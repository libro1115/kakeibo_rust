use super::*;
use chrono::{Local,Weekday,Datelike,TimeZone};
use std::cmp::{Ord, Ordering};
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
impl  Eq for Day {}
//比較演算子
impl Ord for Day{
    fn cmp(&self, other:&Self)->Ordering{
        if self.year > other.year{
            return Ordering::Greater
        }
        else if self.year < other.year{
            return Ordering::Less
        }
        if self.month > other.month{
            return Ordering::Greater
        }
        else if self.month < other.month{
            return Ordering::Less
        }
        if self.day < other.day{
            return Ordering::Greater
        }
        else if self.month > other.month{
            return Ordering::Less
        }
        Ordering::Equal
        
    }
}
impl PartialOrd for Day{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}