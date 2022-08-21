use chrono::Weekday;

mod module;
#[derive(Debug,Clone, Copy)]
#[allow(dead_code)]
pub struct Day{
    year:u16,
    day:u8,
    week:Weekday,
    month:u8,
}

 #[test]
fn make_day(){
    let day = Day::new(2022,8,21);
    assert_eq!(day.year,2022);
    assert_eq!(day.month,8);
    assert_eq!(day.day,21);
    assert_eq!(day.week, Weekday::Sun);
 }