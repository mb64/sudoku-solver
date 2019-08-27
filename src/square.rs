use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(u16);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_known() {
            write!(f, "{}", self.0)
        } else {
            write!(f, " ")
        }
    }
}

impl Square {
    pub fn any() -> Self {
        Self(0x1ff0)
    }
    pub fn new(x: u8) -> Self {
        debug_assert!(x <= 9 && x >= 1);
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
    pub fn possibilities(self) -> impl Iterator<Item = u8> {
        debug_assert!(self.is_unknown());
        (1..=9).filter(move |&x| self.0 & (1 << (3 + x)) != 0)
    }
}
