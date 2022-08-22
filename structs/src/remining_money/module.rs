#![allow(dead_code)]
use super::*;
//public
impl ReminingMoney{
    pub fn new()->Self{
        ReminingMoney { logs: BTreeMap::new(),
             remining_money: HashMap::new() }
    }
}

//protected
impl ReminingMoney{
    
}