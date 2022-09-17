use std::{collections::BTreeMap};

use super::Database;
use rusqlite::{Connection, Error, params};
use structs::{cash_history::{CashHistory, Usage, Badget}, date::Date};

const DB_PATH:&str = "./kakeibo_rust.sqlite3";
//public
impl Database {
    ///データベースと接続
    pub fn connect(user_:String)->Self{
        Database{access_point: Self::connect_database(),
                    user:user_}
    }
    ///ログをデータベースに保存
    pub fn save_log(&self, log:&CashHistory)->Result<usize,Error>{
        self.access_point.execute(format!("INSERT INTO {} VALUES($1,$2,$3,$4,$5,$6,$7,$8,?9)"
        ,&self.user).as_str(),         
       Self::create_log_params(&log))
    }
    ///ログをデータベースから読み込み
    pub fn load_logs(&self, start_date:&Date, end_date:&Date)
    ->Result<BTreeMap<Date,CashHistory>,Error>
    {
        let sql = format!("SELECT * FROM {} WHERE ce >= {}  AND ce < {}", 
                        &self.user, 
                        &start_date.days_from_ce(),
                        &end_date.days_from_ce()
                    );
        let mut map:BTreeMap<Date, CashHistory> = BTreeMap::new();
        let mut stmt = self.access_point.prepare(
            sql.as_str()
            )?;
            let person = stmt.query_map(params![], |row|{
            Ok({
                let ce:i32= row.get(0)?;
                let usage:Usage = Usage::from_int(row.get(4)?);
                let expenses:i64 = row.get(5)?;
                let badget = Badget::from_int(row.get(6)?);
                let tag:u64 = row.get(7)?;
                let memo:String = row.get(8)?;
                
                println!("ce:{}",ce);

                let date = Date::from_ce(ce);
                let mut log:CashHistory;
                if  usage == Usage::Income{   
                    log = CashHistory::income(expenses as u64, 
                                        Date::from_ce(ce), 
                                        badget, 
                                        memo);                            
                }
                else {
                    log = CashHistory::spending(-expenses as u64, 
                                            Date::from_ce(ce), 
                                            badget, 
                                            memo);
                }
                log.set_tag(tag);
                map.insert(date, log);
            })
        }).unwrap();
        for p in person{
            println!("{:?}",p);
        }
        Ok(map)
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
            format!("CREATE TABLE {} (
                                                        ce      INTEGER,
                                                        year    INTEGER,
                                                        month   INTEGER,
                                                        day     INTEGER,
                                                        usage   INTEGER,
                                                        expenses INTEGER,
                                                        badget  INTEGER,
                                                        tag     INTEGER,
                                                        memo    TEXT
                        )", self.user).as_str(), (),            
        ){
            Ok(_)=>{},
            Err(e)=>{println!("create table error :{}",e.to_string());}
        };    
    }
    ///パラメータの作成
    fn create_log_params(log:&CashHistory)->
    (i32, i32, u32, u32, i32, i32, i32, u64, String)
    {
        (log.day().days_from_ce(), log.day().year(), log.day().month(), log.day().day(),
         log.usage() as i32, log.expenses() as i32, log.badget() as i32,
         log.tag().bits(), log.memo.clone())
    }
}