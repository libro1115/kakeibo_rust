#![allow(dead_code)]
mod module;
use rusqlite::{Connection}; 


pub struct Database{
    access_point:Connection,
}


#[test]
fn database_test(){
    let db = Database::connect();
    db.create_table();
}