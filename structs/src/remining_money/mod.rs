#![allow(dead_code)]
mod module;
use std::collections::{HashMap, BTreeMap};
use super::cash_history::{CashHistory, Badget};
use super::day::Day;
struct ReminingMoney{
    logs:BTreeMap<Day, Vec<CashHistory>>,
    remining_money:HashMap<Badget,u64>,
}

#[test]
fn remining_money_test(){

}