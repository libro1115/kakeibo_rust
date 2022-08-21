use super::*;
use chrono::{Local,Weekday,Datelike,TimeZone};

//public
#[allow(dead_code)]
impl Day{
    ///指定日で作成
    pub fn new(y:u16,m:u8,d:u8)->Self{
        Day {week:Day::get_week(&y, &m, &d), year: y, month: m, day: d}
    }
}

//protected
impl Day{
    fn get_week(y:&u16, m:&u8,d:&u8)->Weekday{
        Local.ymd(y.clone() as i32, m.clone() as u32, d.clone() as u32).weekday()
    }
}