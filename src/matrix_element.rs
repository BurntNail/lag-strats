use crate::{Fl, Int};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct El {
    pub top: Int,
    pub btm: Int,
    pub res: Fl,
}

impl El {
    pub fn new(top: Int, btm: Int) -> Self {
        Self {
            top,
            btm,
            res: (top as Fl) / (btm as Fl).sqrt(),
        }
    }
}
impl Debug for El {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/sqrt({})", self.top, self.btm)
    }
}
