use std::fmt;
use std::ops::{Index, IndexMut};

pub mod idx;
mod parser;
pub mod square;

use idx::Idx;
use square::Square;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board(pub [[Square; 9]; 9]);

impl Index<Idx> for Board {
    type Output = Square;

    fn index(&self, i: Idx) -> &Square {
        &self.0[i.regx as usize * 3 + i.subx as usize][i.regy as usize * 3 + i.suby as usize]
    }
}
impl IndexMut<Idx> for Board {
    fn index_mut(&mut self, i: Idx) -> &mut Square {
        &mut self.0[i.regx as usize * 3 + i.subx as usize][i.regy as usize * 3 + i.suby as usize]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.0[..] {
            for s in &line[..] {
                write!(f, "{} ", s)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn from_text(text: &str) -> Option<Self> {
        Some(Self(parser::parse(text)?))
    }

    pub fn solve(mut self, all: bool) -> Vec<Self> {
        for i in Idx::all() {
            let self_i = self[i];
            if self_i.is_known() {
                for n in i.neighbors() {
                    self[n].update_wrt(self_i);
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
        // find unknown
        let u_idx = match Idx::all()
            .filter(|&i| self[i].is_unknown())
            .min_by_key(|&i| self[i].possibilities().count())
        {
            Some(i) => i,
            None => return vec![self],
        };
        // loop thru possibilities: solve_rec()
        let mut res = vec![];
        for pos in self[u_idx].possibilities() {
            let mut test_board = self.clone();
            let sq = Square::new(pos);
            test_board[u_idx] = sq;
            for n in u_idx.neighbors() {
                test_board[n].update_wrt(sq);
            }
            res.append(&mut test_board.solve_rec(all));
            if !all && !res.is_empty() {
                return res;
            }
        }
        return res;
    }
}
