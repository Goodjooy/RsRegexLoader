//子表达式控制区

use crate::regex_iter::RegexIter;
use crate::regex_load::context::{Context, ContextMode};
use crate::regex_load::express::Express;
use crate::regex_load::limitation::Limitation;
#[derive(Debug, PartialEq)]
pub struct MetaExpress {
    //表达式核心内容
    //多个匹配符号
    expresses: Express,
    //限制条件组
    limit: Limitation,
    //上下文组
    context: (Context, Context),
}

impl MetaExpress {
   pub  fn new(iter: &mut RegexIter) -> Option<MetaExpress> {
        let expresses = match Express::new_regex(iter) {
            Some(exp) => exp,
            None => return None,
        };
        let limit = Limitation::load_limitaion(iter);
        let context = (Context::default(), Context::default());

        Some(MetaExpress {
            expresses,
            limit,
            context,
        })
    }

    pub fn set_context(&mut self, context: Context) {
        match context.get_mode() {
            m if *m == ContextMode::PositiveLookahead || *m == ContextMode::NegativeLookahead => {
                self.context.1 = context;
            }
            m if *m == ContextMode::PostiveLookBehind || *m == ContextMode::NegativeLookBehind => {
                self.context.0 = context;
            }
            _ => {}
        };
    }

    pub fn get_express(&self)->&Express{
        &self.expresses
    }
}
