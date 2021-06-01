use crate::regex_load::express_types::sub_express::{SubExpress, SubExpressMode};
use crate::regex_load::limited_express::LimitExrepss;
#[derive(Debug, PartialEq, Default,Clone)]
pub struct Context {
    express: Vec<LimitExrepss>,
    mode: ContextMode,
}
#[derive(Debug, PartialEq,Clone)]
pub enum ContextMode {
    NoLookAround,

    PositiveLookahead,
    NegativeLookahead,

    PostiveLookBehind,
    NegativeLookBehind,
}
impl Default for ContextMode {
    fn default() -> Self {
        ContextMode::NoLookAround
    }
}

impl Context {
    pub fn from_sub(sub: SubExpress) -> Option<Context> {
        let (sub, mode) = sub.get_mode();
        let mode = match mode {
            SubExpressMode::PositiveLookahead => ContextMode::PositiveLookahead,
            SubExpressMode::NegativeLookahead => ContextMode::NegativeLookahead,
            SubExpressMode::PostiveLookBehind => ContextMode::PostiveLookBehind,
            SubExpressMode::NegativeLookBehind => ContextMode::NegativeLookBehind,
            _ => ContextMode::NoLookAround,
        };

        let (_, express) = sub.get_expresses();
        Some(Context { express, mode })
    }

    pub fn get_mode(&self)->&ContextMode{
        &self.mode
    }
}
