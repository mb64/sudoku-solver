#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Idx {
    pub regx: u8,
    pub regy: u8,
    pub subx: u8,
    pub suby: u8,
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
            })
            .filter(move |&idx| idx != self)
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
            )
            .chain(
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
