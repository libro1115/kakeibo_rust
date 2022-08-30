use super::Database;
use rusqlite::{Connection, params};


const DB_PATH:&str = "./kakeibo_rust.sqlite3";
//public
impl Database {
    ///データベースと接続
    pub fn connect()->Self{
        Database{access_point: Self::connect_database()}
    }
}

//protected
impl  Database {

    ///データベースがなければ生成して接続
    fn connect_database()->Connection{
       match Connection::open(DB_PATH){
            Ok(i)=>{
                return i;
            },
            Err(_)=>{
                panic!("database not found and can't create error")
            }
        } 
    }
    ///テーブルの作成
    pub(crate)fn create_table(&self){
        match self.access_point.execute(
            "CREATE TABLE users (
                                            year    INTEGER,
                                            month   INTEGER,
                                            day     INTEGER,
                                            usage   INTEGER,
                                            expenses INTEGER,
                                            badget  INTEGER,
                                            tag     INTEGER,
                                            memo    TEXT
            )",
            (),
        ){
            Ok(_)=>{},
            Err(_)=>{println!("create table error");}
        };
    }
}