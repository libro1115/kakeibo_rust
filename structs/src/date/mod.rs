use chrono::{Weekday,NaiveDate};

mod module;
#[derive(Debug,Clone, Copy)]
#[allow(dead_code)]
pub struct Date{
    date:NaiveDate,
}

 #[test]
fn make_day(){
    let day = Date::new(2022,8,21);
    assert_eq!(day.year(),2022);
    assert_eq!(day.month(),8);
    assert_eq!(day.day(),21);
    assert_eq!(day.week(), Weekday::Sun);
    let day2 = Date::new(2015, 10, 30);
    assert_eq!(day>day2, true);
    assert_eq!(day==day,true);
 }