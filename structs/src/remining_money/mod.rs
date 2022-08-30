#![allow(dead_code)]
mod module;
use std::collections::{HashMap, BTreeMap};
use super::cash_history::{CashHistory, Badget};
use super::date::Date;
struct ReminingMoney{
    logs:BTreeMap<Date, Vec<CashHistory>>,
    remining_money:HashMap<Badget,i128>,
}

#[test]
fn remining_money_test(){
    let mut rem = ReminingMoney::new();
    let day = Date::new(2022, 1, 1);
    let log1 = CashHistory::income(1000, 
                                                day,
                                                 Badget::Monthly, 
                                                 String::new());
    let log2 = CashHistory::income(10000,
                                                 day,
                                                Badget::Weekly,
                                                String::new());
     
    let log3 = CashHistory::spending(5000,
                                                 day,
                                                Badget::Weekly,
                                                String::new());
     let log4 = CashHistory::spending(100000,
                                                 day,
                                                Badget::Monthly,
                                                String::new());
    
    rem.push(log1);
    assert_eq!(rem.get_remining(Badget::Monthly), 1000);
    assert_eq!(rem.get_remining(Badget::Weekly),0);
    rem.push(log2);
    assert_eq!(rem.get_remining(Badget::Monthly), 1000);
    assert_eq!(rem.get_remining(Badget::Weekly),10000);
    rem.push(log3);
    assert_eq!(rem.get_remining(Badget::Monthly), 1000);
    assert_eq!(rem.get_remining(Badget::Weekly),5000);
    rem.push(log4);
    assert_eq!(rem.get_remining(Badget::Monthly), -99000);
    assert_eq!(rem.get_remining(Badget::Weekly),5000);
    match rem.get_logs(day){
        Some(v)=>{
            assert_eq!(v.len(), 4);
        },
        None=>{panic!("err 49");}
    }
    match rem.get_logs(day){
       Some(v)=>{
           assert_eq!(v.len(), 4);
       },
       None=>{panic!("err 55");}
    }
    println!("XXX");
    match rem.get_logs(Date::new(2021,1,1)){
        Some(v)=>{
            println!("A");
            if v.len() != 0{panic!("err 61");}
            panic!("err 62");
        },
        None=>{println!("B");}
    }
}