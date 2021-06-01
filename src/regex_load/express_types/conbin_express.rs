use crate::regex_iter::RegexIter;
use crate::regex_load::express::Express;
use crate::regex_load::express_types::special_express::SpecialExpress;
use crate::regex_load::load_follow_charact;
use crate::regex_load::pre_look_charact;
#[derive(Debug, PartialEq,Clone)]
pub enum ConbinExpress {
    Or(Box<[Express; 2]>),
    In(Vec<Express>, bool),
}

impl ConbinExpress {
    // 捕获in形式的表达式
    // 不实现简写模式（a-z）
    pub fn capture_in(iter: &mut RegexIter) -> Option<ConbinExpress> {
        // 是否去非
        let revent = match iter.per_look(1) {
            Some(c) => {
                if c == "^" {
                    iter.next();
                    true
                } else {
                    false
                }
            }
            None => return None,
        };
        let mut expresses = Vec::new();
        loop {
            match load_follow_charact(iter) {
                Some(c) => {
                    //close sign exit,no trans
                    if !c.trans_sign && c.data == ']' {
                        break;
                    }
                    // special sign
                    match SpecialExpress::get_in_type(&c) {
                        Some(se) => expresses.push(Express::Special(se)),
                        //not a special sign ,as Normal
                        None => {
                            if !c.trans_sign && c.data == '-' {
                                match expresses.last() {
                                    Some(last) => match last {
                                        Express::Normal(ch) => match load_follow_charact(iter) {
                                            Some(end_c) => {
                                                if !end_c.trans_sign {
                                                    let start_int = *ch as u32;
                                                    let end_int = end_c.data as u32;

                                                    for char_code in start_int + 1..end_int + 1 {
                                                        let in_char = char_code as u8 as char;
                                                        expresses.push(Express::Normal(in_char))
                                                    }
                                                } else {
                                                    expresses.push(Express::Normal(c.data))
                                                }
                                            }
                                            None => expresses.push(Express::Normal(c.data)),
                                        },
                                        _ => {}
                                    },
                                    None => expresses.push(Express::Normal(c.data)),
                                }
                            } else {
                                expresses.push(Express::Normal(c.data))
                            }
                        }
                    }
                }
                //end of string with no close sign
                None => return None,
            }
        }
        Some(ConbinExpress::In(expresses, revent))
    }

    pub fn capture_or(iter: &mut RegexIter, previde: Express) -> Option<Express> {
        let expresses = match pre_look_charact(iter) {
            Some(fc) => {
                // or sign
                if !fc.trans_sign && fc.data == "|" {
                    iter.next();
                    let next_express = match Express::express_capture(iter) {
                        Some(e) => e,
                        None => Express::NormalEmpty,
                    };
                    let or_exp = [previde, next_express];
                    let or_exp = Box::new(or_exp);
                    let or_exp = ConbinExpress::Or(or_exp);
                    Some(Express::Conbin(or_exp))
                } else {
                    Some(previde)
                }
            }
            None => Some(previde),
        };

        expresses
    }
}

#[cfg(test)]
mod conbin_test {
    use crate::regex_iter::RegexIter;
    use crate::regex_load::express_types::conbin_express::ConbinExpress;
    use crate::regex_load::express_types::conbin_express::Express;
    #[test]
    fn caputure_in_normal() {
        let mut iter = RegexIter::new("12345\\S]");
        match ConbinExpress::capture_in(&mut iter) {
            Some(ce) => match &ce {
                ConbinExpress::In(_exps, revent) => {
                    println!("{:#?}", ce);
                    assert_eq!(*revent, false);
                    //assert_eq!(exps[0], Express::Normal('1'));
                    //assert_eq!(exps[5], Express::Special(SpecialExpress::AnyPrint));
                }
                _ => {}
            },
            None => {}
        }
    }
    #[test]
    fn caputure_in_not_close() {
        let mut iter = RegexIter::new("12345\\S");
        assert_eq!(None, ConbinExpress::capture_in(&mut iter));
    }
    #[test]
    fn caputure_in_revent() {
        let mut iter = RegexIter::new("^12345\\S]");
        match ConbinExpress::capture_in(&mut iter) {
            Some(ce) => match &ce {
                ConbinExpress::In(_exps, revent) => {
                    println!("{:#?}", ce);
                    assert_eq!(*revent, true);
                    //assert_eq!(exps[0], Express::Normal('1'));
                    //assert_eq!(exps[5], Express::Special(SpecialExpress::AnyPrint));
                }
                _ => {}
            },
            None => {}
        }
    }
    #[test]
    fn caputure_in_range() {
        let mut iter = RegexIter::new("^0-9\\S王]");
        match ConbinExpress::capture_in(&mut iter) {
            Some(ce) => match &ce {
                ConbinExpress::In(_exps, revent) => {
                    println!("{:#?}", ce);
                    assert_eq!(*revent, true);
                    //assert_eq!(exps[0], Express::Normal('1'));
                    //assert_eq!(exps[5], Express::Special(SpecialExpress::AnyPrint));
                }
                _ => {}
            },
            None => {}
        }
    }

    #[test]
    fn capture_or_normoal() {
        let mut iter = RegexIter::new("好|[凊弦凝绝]");
        match Express::express_capture(&mut iter) {
            Some(previde) => {
                let totle = ConbinExpress::capture_or(&mut iter, previde);
                println!("{:#?}", totle)
            }
            None => {}
        }
    }
}
