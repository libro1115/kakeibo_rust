#![allow(dead_code)]
mod module;
use rusqlite::{Connection};
use structs::{cash_history::{CashHistory, Badget}, date::Date}; 


pub struct Database{
    access_point:Connection,
}


#[test]
fn database_test(){
    let db = Database::connect();
    db.create_table();
    let log = CashHistory::income(1000, Date::new(2022,1,1), Badget::Weekly, "memo".to_string());
    match db.save_log(&log){
        Ok(_)=>{},
        Err(o)=>{panic!("err save database: {}",o.to_string());}
    }
}