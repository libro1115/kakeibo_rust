use super::*;
use chrono::{Local,Weekday,Datelike,TimeZone};
use std::cmp::{PartialOrd, Ordering};
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
//==演算子
impl PartialEq for Day{
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day
    }
}
//比較演算子
impl PartialOrd for Day{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year > other.year{
            return Some(Ordering::Greater)
        }
        else if self.year < other.year{
            return Some(Ordering::Less)
        }
        if self.month > other.month{
            return Some(Ordering::Greater)
        }
        else if self.month < other.month{
            return Some(Ordering::Less)
        }
        if self.day < other.day{
            Some(Ordering::Greater)
        }
        else if self.month > other.month{
            Some(Ordering::Less)
        }
        else if self == other{
            Some(Ordering::Equal)
        }
        else {None}
    }
}