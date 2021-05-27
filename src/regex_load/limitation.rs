#[derive(Debug,PartialEq)]
pub enum Limitation {
    One,
    OneOrMore,
    ZeroOrMore,
    OneLen(usize),
    Range(usize, usize),
}

