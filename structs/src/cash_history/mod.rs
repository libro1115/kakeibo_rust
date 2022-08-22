#![allow(dead_code)]
mod module;
use super::day::Day;
use bitflags::bitflags;
#[derive(Clone)]
pub struct CashHistory{
    expenses:u64,
    usage:Usage,
    badget:Badget,
    tag:MejorTag,
    pub memo:String,
    day:Day,
}
//収支どちらか
#[derive(Clone, Copy, PartialEq)]
pub(crate)enum Usage{
    Income = 0,
    Spending = 1,
}
//予算枠
#[derive(Clone, Copy, Hash,PartialEq, Eq)]
pub enum Badget{
    None,
}
//タグ
bitflags!{
    pub struct MejorTag:u64{
        const NONE = 0;
    }
}

#[test]
fn cash_history_test(){
    let day = Day::new(2022,8,21);
    let income = CashHistory::income(500, day, Badget::None, String::new());
    assert_eq!(income.tag, MejorTag::NONE);


}