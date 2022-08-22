use super::*;
#[allow(dead_code)]
//public
impl CashHistory{
    ///収入を作成
    /// expenses:金額
    /// day:日にち
    /// badget:予算枠
    /// memo:メモ
    pub fn income(expenses_:u64, day_:Day, badget_:Badget, memo_:String)->Self{
        CashHistory{usage:Usage::Income, expenses:expenses_, day:day_, tag:MejorTag::NONE, badget:badget_, memo:memo_}
    }
    ///支出を作成
    /// expenses:金額
    /// day:日にち
    /// badget:予算枠
    /// memo:メモ
    pub fn spending(expenses_:u64, day_:Day,badget_:Badget, memo_:String)->Self{
        CashHistory{usage:Usage::Spending, expenses:expenses_, day:day_, tag:MejorTag::NONE, badget:badget_, memo:memo_}
    }
    ///タグを加算
    pub fn add_tag(&mut self, tag_:MejorTag){
        self.tag |= tag_;
    }
    ///タグを消す
    pub fn sub_tag(&mut self, tag_:MejorTag){
        self.tag -= tag_;
    }

}


//protected
impl CashHistory{

}