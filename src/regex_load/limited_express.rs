use crate::regex_iter::RegexIter;
use crate::regex_load::express::Express;
use crate::regex_load::limitation::Limitation;
#[derive(Debug, PartialEq,Clone)]
pub struct LimitExrepss {
    limit: Limitation,
    express: Express,
}

impl LimitExrepss {
    pub fn new(iter: &mut RegexIter) -> Option<LimitExrepss> {
        let express = Express::new_regex(iter);
        let limit = Limitation::load_limitaion(iter);

        match express {
            Some(exp) => Some(LimitExrepss {
                limit,
                express: exp,
            }),
            None=>None
        }
    }
}
