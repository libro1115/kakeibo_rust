use super::*;
use chrono::{Weekday,Datelike};
use std::cmp::{Ord, Ordering};
//public
#[allow(dead_code)]
impl Day{
    ///指定日で作成
    pub fn new(y:i32,m:u32,d:u32)->Self{
        Day {date:NaiveDate::from_ymd(y, m, d)}
    }
    pub fn from_ce(ce:i32)->Self{
        Day{date:NaiveDate::from_num_days_from_ce(ce)}
    }
    ///年の取得
    pub fn year(&self)->i32{
        self.date.year()
    }
    ///月の取得
    pub fn month(&self)->u32{
        self.date.month()
    }
    ///日の取得
    pub fn day(&self)->u32{
        self.date.day()
    }
    ///曜日の取得
    pub fn week(&self)->Weekday{
        self.date.weekday()
    }
    ///1年1月1日からの年月
    pub fn days_from_ce(&self)->i32{
        self.date.num_days_from_ce()
    }
}

//protected
impl Day{

}
//==演算子
impl PartialEq for Day{
    fn eq(&self, other: &Self) -> bool {
        self.days_from_ce() == other.days_from_ce()
    }
}
impl  Eq for Day {}
//比較演算子
impl Ord for Day{
    fn cmp(&self, other:&Self)->Ordering{
        self.days_from_ce().cmp(&other.days_from_ce())        
    }
}
impl PartialOrd for Day{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}