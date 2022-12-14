#![allow(dead_code)]
mod module;
use super::date::Date;
use bitflags::bitflags;
#[derive(Clone, Debug, PartialEq)]
pub struct CashHistory{
    expenses:u64,
    usage:Usage,
    badget:Badget,
    tag:MejorTag,
    pub memo:String,
    day:Date,
}
//収支どちらか
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Usage{
    Income = 0,
    Spending = 1,
}
//予算枠
#[derive(Debug, Clone, Copy, Hash,PartialEq, Eq)]
pub enum Badget{
    None = -1,
    ///週予算
    Weekly = 0,
    ///月予算
    Monthly,
    /// 2か月予算
    TwoMonthly,
    /// 3か月予算
    ThreeMonthly,
    /// 4か月予算
    ForuMonthly,
    /// 半年予算
    HalfAnnual,
    ///年予算
    Annual,
}

//タグ
bitflags!{
    pub struct MejorTag:u64{
        const NONE = 0;
    }
}

#[test]
fn cash_history_test(){
    let day = Date::new(2022,8,21);
    let income = CashHistory::income(500, day, Badget::None, String::new());
    assert_eq!(income.tag, MejorTag::NONE);


}