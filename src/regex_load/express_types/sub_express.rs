use crate::regex_iter::RegexIter;
use crate::regex_load::limited_express::LimitExrepss;

#[derive(Debug, PartialEq, Clone)]
pub struct SubExpress {
    expresses: Vec<LimitExrepss>,
    mode: SubExpressMode,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubExpressMode {
    Capture,
    NoCapture,

    PositiveLookahead,
    NegativeLookahead,

    PostiveLookBehind,
    NegativeLookBehind,
}

impl SubExpress {
    pub fn capture_sub(regex_iter: &mut RegexIter) -> Option<SubExpress> {
        let mode: SubExpressMode;
        //左括号已经被读取了
        match SubExpressMode::load_mod(regex_iter) {
            Some(m) => mode = m,
            None => return None,
        };

        let mut expresses = Vec::new();

        loop {
            match regex_iter.per_look(1) {
                Some(st) => {
                    if st == ")" {
                        regex_iter.next();
                        break Some(SubExpress { expresses, mode });
                    } else {
                        match LimitExrepss::new(regex_iter) {
                            Some(exp) => expresses.push(exp),
                            None => return None,
                        };
                    }
                }
                None => return None,
            }
            //break Some(SubExpress { expresses, mode });
        }
    }

    pub fn get_mode(self) -> (Self, SubExpressMode) {
        (self.clone(), self.mode)
    }
    pub fn get_expresses(self) -> (Self, Vec<LimitExrepss>) {
        (self.clone(), self.expresses)
    }
}
impl SubExpressMode {
    fn load_mod(regex_iter: &mut RegexIter) -> Option<SubExpressMode> {
        let mode = match regex_iter.per_look(1) {
            Some(ch) => {
                if ch == "?" {
                    regex_iter.next();
                    match regex_iter.next() {
                        Some(c) => {
                            if c == ':' {
                                SubExpressMode::NoCapture
                            } else {
                                if c == '=' {
                                    SubExpressMode::PositiveLookahead
                                } else if c == '!' {
                                    SubExpressMode::NegativeLookahead
                                } else if c == '<' {
                                    match regex_iter.next() {
                                        Some(c) => {
                                            if c == '=' {
                                                SubExpressMode::PostiveLookBehind
                                            } else if c == '!' {
                                                SubExpressMode::NegativeLookBehind
                                            } else {
                                                return None;
                                            }
                                        }
                                        None => return None,
                                    }
                                } else {
                                    return None;
                                }
                            }
                        }
                        None => return None,
                    }
                } else {
                    SubExpressMode::Capture
                }
            }
            None => return None,
        };
        Some(mode)
    }
}

#[cfg(test)]
mod sub_express {
    use crate::regex_iter::RegexIter;
    use crate::regex_load::express_types::sub_express::SubExpress;
    #[test]
    fn sub_normal() {
        let mut iter = RegexIter::new("?:abc)");

        let v = SubExpress::capture_sub(&mut iter);

        println!("{:#?}", v)
    }

    #[test]
    fn sub_inside_anorther() {
        let mut iter = RegexIter::new("?:ab(?:bbc)c[\\d]+)");

        let v = SubExpress::capture_sub(&mut iter);

        println!("{:#?}", v)
    }
}
