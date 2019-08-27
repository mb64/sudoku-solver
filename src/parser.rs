use crate::square::Square;

struct State<'a> {
    text: &'a str,
}

impl<'a> State<'a> {
    fn whitespace(&mut self) -> Option<()> {
        if self.consume()? == ' ' {
            Some(())
        } else {
            None
        }
    }
    fn newline(&mut self) -> Option<()> {
        if self.consume()? == '\n' {
            Some(())
        } else {
            None
        }
    }
    fn consume(&mut self) -> Option<char> {
        let ch = self.text.chars().nth(0)?;
        self.text = &self.text[1..];
        Some(ch)
    }

    fn square(&mut self) -> Option<Square> {
        match self.consume()? {
            ' ' => Some(Square::any()),
            '1' => Some(Square::new(1)),
            '2' => Some(Square::new(2)),
            '3' => Some(Square::new(3)),
            '4' => Some(Square::new(4)),
            '5' => Some(Square::new(5)),
            '6' => Some(Square::new(6)),
            '7' => Some(Square::new(7)),
            '8' => Some(Square::new(8)),
            '9' => Some(Square::new(9)),
            _ => None,
        }
    }

    fn line(&mut self) -> Option<[Square; 9]> {
        let mut res = [Square::any(); 9];
        res[0] = self.square()?;
        for i in 1..9 {
            self.whitespace()?;
            res[i] = self.square()?;
        }
        self.newline()?;
        Some(res)
    }

    fn board(&mut self) -> Option<[[Square; 9]; 9]> {
        let mut res = [[Square::any(); 9]; 9];
        for line in &mut res[..] {
            *line = self.line()?;
        }
        Some(res)
    }
}

pub fn parse(text: &str) -> Option<[[Square; 9]; 9]> {
    State { text }.board()
}
