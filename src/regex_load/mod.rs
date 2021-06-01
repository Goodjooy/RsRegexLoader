use crate::regex_iter::RegexIter;

pub mod regex;

pub mod express;
pub mod meta_express;
pub mod limited_express;
pub mod context;

pub mod express_types;


pub mod limitation;

pub const SPECIAL_SINGLE_CHAR: [char; 4] = ['.', '^', '$', '\\'];
pub const CMPLEX_SINGLE_CHAR: [char; 5] = ['[', ']', '(', ')', '|'];
pub const SPECIAL_MUTLI_CHAR: [char; 6] = ['d', 'D', 's', 'S', 'w', 'W'];
#[derive(Debug)]
pub struct FollowChars {
    pub data: char,
    pub trans_sign: bool,
}
#[derive(Debug)]
pub struct PreLookChars<'a> {
    pub data: &'a str,
    pub trans_sign: bool,
}

fn load_follow_charact(iter: &mut RegexIter) -> Option<FollowChars> {
    match iter.next() {
        Some(c) => {
            if c == '\\' {
                match iter.next() {
                    Some(c) => Some(FollowChars {
                        data: c,
                        trans_sign: true,
                    }),
                    None => None,
                }
            } else {
                Some(FollowChars {
                    data: c,
                    trans_sign: false,
                })
            }
        }
        None => None,
    }
}

pub fn pre_look_charact<'a>(iter: &'a  RegexIter) -> Option<PreLookChars<'a>> {
    match iter.per_look(1) {
        Some(c) => {
            if c == "\\" {
                match iter.per_look(1) {
                    Some(c) => Some(PreLookChars {
                        data: c,
                        trans_sign: true,
                    }),
                    None => None,
                }
            } else {
                Some(PreLookChars {
                    data: c,
                    trans_sign: false,
                })
            }
        }
        None => None,
    }
}
