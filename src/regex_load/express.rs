use crate::regex_iter::RegexIter;
use crate::regex_load::conbin_express::ConbinExpress;
use crate::regex_load::load_follow_charact;
use crate::regex_load::special_express::SpecialExpress;
use crate::regex_load::sub_express::SubExpress;

#[derive(Debug, PartialEq)]
pub enum Express {
    //普通空白
    NormalEmpty,
    //普通的字面字符或者转义后的字面字符
    Normal(char),
    //特殊的字符，包括特殊标记: . 。^ $ 等
    Special(SpecialExpress),
    //组合表达式
    Conbin(ConbinExpress),
    //内部子表达式
    Sub(SubExpress),
}

impl Express {
    pub fn new_regex(regex_iter: &mut RegexIter) -> Vec<Express> {
        match regex_iter.has_next() {
            true => {
                if let Some(exp) = Express::express_capture(regex_iter) {
                    ConbinExpress::capture_or(regex_iter, exp)
                } else {
                    Vec::new()
                }
            }
            false => Vec::new(),
        }
    }

    pub fn express_capture(regex_iter: &mut RegexIter) -> Option<Express> {
        //if c is a transform sign read next charcater to get what it is
        match load_follow_charact(regex_iter) {
            Some(follow_c) => {
                //try look as spectial charct
                match SpecialExpress::get_type(&follow_c) {
                    Some(se) => Some(Express::Special(se)),
                    None => match follow_c.data {
                        c if c == '(' && !follow_c.trans_sign => {
                            match SubExpress::capture_sub(regex_iter) {
                                Some(exp) => Some(Express::Sub(exp)),
                                None => None,
                            }
                        }
                        c if c == '[' && !follow_c.trans_sign => {
                            match ConbinExpress::capture_in(regex_iter) {
                                Some(ce) => Some(Express::Conbin(ce)),
                                None => None,
                            }
                        }
                        d => Some(Express::Normal(d)),
                    },
                }
            }
            None => None,
        }
    }
}

#[test]
fn test_express_cap_in() {
    let mut iter = RegexIter::new("[abcde\\d]");
    let express = Express::express_capture(&mut iter).unwrap();

    println!("{:#?}", express);
}
#[test]
fn test_express_cap_normal() {
    let mut iter = RegexIter::new("\\[abcde\\d]");
    let express = Express::express_capture(&mut iter).unwrap();

    println!("{:#?}", express);
}
