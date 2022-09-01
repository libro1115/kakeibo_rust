#![allow(dead_code)]
mod module;
use rusqlite::{Connection};



pub struct Database{
    access_point:Connection,
    user:String,
}


#[test]
fn database_test(){
    use structs::{cash_history::{CashHistory, Badget}, date::Date};
    let db = Database::connect("user1".to_string());
    db.create_table();
    let d1 =  Date::new(2022,1,1);
    let log = CashHistory::income(1000, 
       d1, 
        Badget::Weekly, "memo".to_string());
    match db.save_log(&log){
        Ok(_)=>{},
        Err(o)=>{panic!("err save database: {}",o.to_string());}
    }
    let d2 = Date::new(2022,1,5);
    let d3 = Date::new(2022, 4, 1);
    let log2 = CashHistory::spending(5000, 
        d2, Badget::Monthly, "a".to_string());
    db.save_log(&log2).unwrap();
    let log3 = CashHistory::income(2000,
        d3, 
        Badget::ForuMonthly, "".to_string());
    db.save_log(&log3);
    
    match db.load_logs(&d1,
                 &Date::new(2022, 2, 1))
                 {
                    Ok(map)=>{
                        if map.len() != 2{
                            println!("{:?}",map);
                            panic!("load length is {}", map.len());
                        }
                        assert_eq!(map[&d1], log);
                        assert_eq!(map[&d2], log2);
                    },
                    Err(e)=>{panic!("load error: {}", e.to_string());}
                 }
}