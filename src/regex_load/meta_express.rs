//子表达式控制区

use crate::regex_load::limitation::Limitation;
use crate::regex_load::express::Express;
#[derive(Debug,PartialEq)]
pub struct MetaExpress {
    //表达式核心内容
    //多个匹配符号
    expresses: Express,
    //限制条件组
    limit: Limitation,
    //是否为贪婪匹配
    avarice: bool,
    //capture
    capture: bool,
}




