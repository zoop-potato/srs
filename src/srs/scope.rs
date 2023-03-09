use std::ops::RangeInclusive;

/// Fast and simple way to have both start and end indexs into a string AND all the functinality of &str
#[derive(Debug, Clone, Copy)]
pub struct Scope<'a> {
    slice: &'a str,
    start_index: usize,
}

impl<'a> Scope<'a> {
    pub fn new(scope: RangeInclusive<usize>, string: &'a String) -> Result<Self, String> {
        if *scope.end() >= string.len() {
            return Err("Range out of bounds".to_string());
        }
        if scope.end() - scope.start() == 0 {
            return Err("Range too small".to_string());
        }
        Ok(Scope {
            slice: &string[*scope.start()..=*scope.end()],
            start_index: *scope.start(),
        })
    }

    pub fn start(&self) -> usize {
        self.start_index
    }

    pub fn end(&self) -> usize {
        self.start_index + self.slice.len()
    }

    pub fn range(&self) -> RangeInclusive<usize> {
        self.start_index..=self.end()
    }
}

impl<'a> From<Scope<'a>> for &'a str {
    fn from(val: Scope<'a>) -> Self {
        val.slice
    }
}

struct Bracket {
    style: BracketStyle,
    side: BracketSide,
}

enum BracketStyle {
    Round,
    Curly,
    Angle,
    Square,
}

enum BracketSide {
    Start,
    End,
}

impl From<Bracket> for char {
    fn from(val: Bracket) -> char {
        match val.side {
            BracketSide::Start => match val.style {
                BracketStyle::Round => '(',
                BracketStyle::Curly => '{',
                BracketStyle::Angle => '<',
                BracketStyle::Square => '[',
            },
            BracketSide::End => match val.style {
                BracketStyle::Round => ')',
                BracketStyle::Curly => '}',
                BracketStyle::Angle => '>',
                BracketStyle::Square => ']',
            },
        }
    }
}

impl Bracket {
    fn from(val: char) -> Result<Bracket, ()> {
        match val {
            '(' => Ok(Bracket{style: BracketStyle::Round, side: BracketSide::Start}),
            ')' => Ok(Bracket{style: BracketStyle::Round, side: BracketSide::End}),
            '{' => Ok(Bracket { style: BracketStyle::Curly, side: BracketSide::Start }),
            '}' => Ok(Bracket { style: BracketStyle::Curly, side: BracketSide::End }),
            '<' => Ok(Bracket { style: BracketStyle::Angle, side: BracketSide::Start }),
            '>' => Ok(Bracket { style: BracketStyle::Angle, side: BracketSide::End }),
            '[' => Ok(Bracket { style: BracketStyle::Square, side: BracketSide::Start }),
            ']' => Ok(Bracket { style: BracketStyle::Square, side: BracketSide::End }),
            _ => Err(()),
        }
    }
}