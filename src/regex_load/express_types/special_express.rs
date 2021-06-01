use crate::regex_load::FollowChars;
#[derive(Debug, PartialEq,Clone)]
pub enum SpecialExpress {
    AnyChar, // .

    StringStart, // ^
    StringEnd,   // $

    AnyDigit,    // \d
    AnyNotDigit, // \D
    AnyNoPrint,  // \s
    AnyPrint,    // \S

    Word, // \w
    NotWord, // \W

    WordBoundary, // \b
    NotWordBoundary, // \B
}

impl SpecialExpress {
    //TODO: differernt space special char
    pub fn get_in_type(c: &FollowChars) -> Option<SpecialExpress> {
        match SpecialExpress::transed_type(c) {
            Some(s) => Some(s),
            None => None,
        }
    }
    pub fn get_out_type(c: &FollowChars) -> Option<SpecialExpress> {
        match SpecialExpress::transed_type(c) {
            Some(s) => Some(s),
            None => match (c.data, c.trans_sign) {
                ('^', false) => Some(SpecialExpress::StringStart),
                ('$', false) => Some(SpecialExpress::StringEnd),
                _ => None,
            },
        }
    }
    fn transed_type(c: &FollowChars) -> Option<SpecialExpress> {
        match (c.data, c.trans_sign) {
            ('d', true) => Some(SpecialExpress::AnyDigit),
            ('D', true) => Some(SpecialExpress::AnyNotDigit),
            ('s', true) => Some(SpecialExpress::AnyNoPrint),
            ('S', true) => Some(SpecialExpress::AnyPrint),

            ('.', false) => Some(SpecialExpress::AnyChar),
            _ => None,
        }
    }
}
