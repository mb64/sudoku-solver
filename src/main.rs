use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Square(u16);

const E: Square = Square(0x1ff0);

impl Square {
    pub fn any() -> Self {
        E
    }
    pub fn new(x: u8) -> Self {
        assert!(x <= 9 && x >= 1);
        Self(x as u16)
    }
    pub fn check_singleton(&mut self) -> bool {
        for i in 1..=9 {
            if self.0 == 1 << (3 + i) {
                self.0 = i;
                return true;
            }
        }
        false
    }
    pub fn update_wrt(&mut self, other: Self) {
        debug_assert!(other.is_known());
        self.0 &= !(1 << (3 + other.0));
    }
    pub fn is_unknown(&self) -> bool {
        self.0 & 0x1ff0 != 0
    }
    pub fn is_known(&self) -> bool {
        1 <= self.0 && self.0 <= 9
    }
    pub fn is_impossible(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Idx {
    regx: u8,
    regy: u8,
    subx: u8,
    suby: u8,
}

impl Idx {
    pub fn neighbors(self) -> impl Iterator<Item = Idx> {
        (0..3)
            .flat_map(move |subx| {
                (0..3).map(move |suby| Idx {
                    regx: self.regx,
                    regy: self.regy,
                    subx,
                    suby,
                })
            }).filter(move |&idx| idx != self)
            .chain(
                (0..3)
                    .filter(move |&regx| regx != self.regx)
                    .flat_map(move |regx| {
                        (0..3).map(move |subx| Idx {
                            regx,
                            regy: self.regy,
                            subx,
                            suby: self.suby,
                        })
                    }),
            ).chain(
                (0..3)
                    .filter(move |&regy| regy != self.regy)
                    .flat_map(move |regy| {
                        (0..3).map(move |suby| Idx {
                            regx: self.regx,
                            regy,
                            subx: self.subx,
                            suby,
                        })
                    }),
            )
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..3).flat_map(move |regx| {
            (0..3).flat_map(move |regy| {
                (0..3).flat_map(move |subx| {
                    (0..3).map(move |suby| Idx {
                        regx,
                        regy,
                        subx,
                        suby,
                    })
                })
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board([[Square; 9]; 9]);

impl Index<Idx> for Board {
    type Output = Square;

    fn index(&self, idx: Idx) -> &Square {
        &self.0[idx.regx as usize * 3 + idx.subx as usize]
            [idx.regy as usize * 3 + idx.suby as usize]
    }
}
impl IndexMut<Idx> for Board {
    fn index_mut(&mut self, idx: Idx) -> &mut Square {
        &mut self.0[idx.regx as usize * 3 + idx.subx as usize]
            [idx.regy as usize * 3 + idx.suby as usize]
    }
}

impl Board {
    fn solve(mut self, all: bool) -> Vec<Self> {
        for idx in Idx::all() {
            let self_idx = self[idx];
            if self_idx.is_known() {
                for n in idx.neighbors() {
                    self[n].update_wrt(self_idx);
                }
            }
        }
        self.solve_rec(all)
    }

    fn solve_rec(mut self, all: bool) -> Vec<Self> {
        // loop {
        //   find singletons; foreach {update}
        // }
        loop {
            let mut progress = false;
            for i in Idx::all() {
                if self[i].check_singleton() {
                    progress = true;
                    let self_i = self[i];
                    for n in i.neighbors() {
                        self[n].update_wrt(self_i);
                        if self[n].is_impossible() {
                            return vec![];
                        }
                    }
                }
            }
            if !progress {
                break;
            }
        }
        // find first unknown
        // loop thru possibilities: solve_rec()
        unimplemented!();
    }
}

fn main() {
    println!("Hello, world!");
}
