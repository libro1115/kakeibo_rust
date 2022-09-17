#![allow(dead_code)]
use std::vec;

use super::*;
//public
impl ReminingMoney{
    pub fn new()->Self{
        ReminingMoney { logs: BTreeMap::new(),
             remining_money: HashMap::new() }
    }
    ///資金の操作
    ///log:資金をいつどう動かしたか
    pub fn push(&mut self, log:CashHistory){
        self.push_logs(&log);
        self.push_remining_money(&log);
    }
    ///残り資金の取得
    pub fn get_remining(&self, badget:Badget)->i128{
        match self.remining_money.get(&badget){
            Some(i)=>{
                return *i;
            },
            None=>{
                return 0;
            }
        }
    }
    ///指定日のログを参照
    pub fn get_logs(&self, day:Date)->Option<&Vec<CashHistory>>{
        match self.logs.get(&day){
            Some(i)=>{return Some(i);},
            _=>{return None;}
        }
    }
}

//protected
impl ReminingMoney{
    ///ログを書き加える
    fn push_logs(&mut self, log:&CashHistory){
        let day = log.day();
        match self.logs.get_mut(&day){
            Some(i)=>{
                //既にあるならpushする。
                i.push(log.clone());
            },
            //無かったら新たに作る
            _ =>{self.logs.insert(day, vec![log.clone()]);},
        }
    }
    ///各予算枠を更新
    fn push_remining_money(&mut self, log:&CashHistory){
        let badget = log.badget();
        match self.remining_money.get(&badget){
            //既にあるならその値に加算する。収支の＋ーはCashHistoryがやってくれる
            Some(_)=>{
                self.remining_money.insert(
                            badget,
                            self.remining_money[&badget]+log.expenses());
            },
            //無かったら新たに作る
            _ =>{
                self.remining_money.insert(badget, log.expenses());
                },
        }
    }
}