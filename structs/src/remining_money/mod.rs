#![allow(dead_code)]
mod module;
use std::collections::BTreeMap;
use super::cash_history::{CashHistory, Badget};
use super::day::Day;
struct ReminingMoney{
    logs:BTreeMap<Day, CashHistory>,
    remining_money:BTreeMap<Badget,u64>,
}

#[test]
fn remining_money_test(){

}