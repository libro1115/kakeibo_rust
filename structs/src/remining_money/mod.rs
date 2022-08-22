#![allow(dead_code)]
mod module;
use multimap::MultiMap;
use super::cash_history::{CashHistory, Badget};
use super::day::Day;
struct ReminingMoney{
    logs:MultiMap<Day, CashHistory>,
    remining_money:MultiMap<Badget,u64>,
}

#[test]
fn remining_money_test(){

}