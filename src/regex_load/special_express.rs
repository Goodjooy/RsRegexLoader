use crate::regex_load::FollowChars;
#[derive(Debug,PartialEq)]
pub enum SpecialExpress {
    AnyChar, // .

    StringStart, // ^
    StringEnd,   // $

    AnyDigit,    // \d
    AnyNotDigit, // \D
    AnyNoPrint,  // \s
    AnyPrint,    // \S

    
}



impl SpecialExpress {
    pub fn get_type(c: &FollowChars) -> Option<SpecialExpress> {
        if c.trans_sign {
            //TODO: more sign support
            match c.data {
                'd' => Some(SpecialExpress::AnyDigit),
                'D' => Some(SpecialExpress::AnyNotDigit),
                's' => Some(SpecialExpress::AnyNoPrint),
                'S' => Some(SpecialExpress::AnyPrint),
                _ => None,
            }
        } else {
            match c.data {
                '.' => Some(SpecialExpress::AnyChar),
                _ => None,
            }
        }
    }
}
