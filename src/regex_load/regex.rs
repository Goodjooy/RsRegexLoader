use crate::regex_iter::RegexIter;
use crate::regex_load::context::Context;
use crate::regex_load::express::Express;
use crate::regex_load::express_types::sub_express::SubExpressMode;
use crate::regex_load::meta_express::MetaExpress;
#[derive(Debug)]
struct RegexExpress {
    expresses: Vec<MetaExpress>,

    context: Option<Context>,
}

impl RegexExpress {
    fn new(regex: &str) -> Option<RegexExpress> {
        let mut iter = RegexIter::new(regex);
        let mut regex = RegexExpress {
            expresses: Vec::new(),
            context: None,
        };

        loop {
            match MetaExpress::new(&mut iter) {
                Some(mut meta) => {
                    match meta.get_express() {
                        Express::Sub(sub) => {
                            let (sub, mode) = sub.clone().get_mode();
                            if mode != SubExpressMode::Capture && mode != SubExpressMode::NoCapture
                            {
                                if mode == SubExpressMode::NegativeLookBehind
                                    || mode == SubExpressMode::PostiveLookBehind
                                {
                                    regex.context = Context::from_sub(sub);
                                } else {
                                    match regex.expresses.last_mut() {
                                        Some(exp) => {
                                            exp.set_context(match Context::from_sub(sub) {
                                                Some(c) => c,
                                                None => return None,
                                            });
                                        }
                                        None => return None,
                                    }
                                }
                                continue;
                            }
                        }
                        _ => {}
                    };

                    if let Some(ref cont) = regex.context {
                        meta.set_context((*cont).clone());
                        regex.context = None
                    }
                    regex.expresses.push(meta);
                }
                None => {
                    if iter.has_next() {
                        return None;
                    } else {
                        regex.context = None;
                        break Some(regex);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod regex {
    use crate::regex_load::regex::RegexExpress;
    #[test]

    fn regex_test_normal() {
        let pattern = RegexExpress::new("(https?:[^<>]+)(?=\\d)");

        println!("{:#?}", pattern)
    }
}
