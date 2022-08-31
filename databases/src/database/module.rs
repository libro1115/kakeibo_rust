use super::Database;
use rusqlite::{Connection, Error};
use structs::{cash_history::CashHistory};

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