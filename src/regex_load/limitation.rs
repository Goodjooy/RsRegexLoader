use crate::regex_iter::RegexIter;
#[derive(Debug, PartialEq, Clone)]

pub enum Limitation {
    One,
    ZeroOrOne,
    OneOrMore(bool),
    ZeroOrMore(bool),
    OneLen(usize),
    Range((usize, usize), bool),
}

impl Limitation {
    pub fn load_limitaion(iter: &mut RegexIter) -> Limitation {
        match iter.per_look(1) {
            Some(st) => match st {
                "+" => {
                    iter.next();
                    Limitation::OneOrMore(Limitation::is_avarice(iter))
                }
                "*" => {
                    iter.next();
                    Limitation::ZeroOrMore(Limitation::is_avarice(iter))
                }
                "?" => {
                    iter.next();
                    Limitation::ZeroOrOne
                }
                "{" => {
                    todo!()
                }
                _ => Limitation::One,
            },
            None => Limitation::One,
        }
    }

    fn is_avarice(iter: &mut RegexIter) -> bool {
        match iter.per_look(1) {
            Some(c) => match c {
                "?" => {
                    iter.next();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
