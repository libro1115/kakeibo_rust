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
    ///動かした額の参照
    pub fn expenses(&self)->i128{
        if(self.usage == Usage::Income){
            self.expenses as i128
        }
        else{
            -(self.expenses as i128)
        }
    }
    ///日にち参照
    pub fn day(&self)->Day{
        self.day.clone()
    }
    ///収支属性参照
    pub fn usage(&self)->Usage{
        self.usage.clone()
    }
    ///予算枠参照
    pub fn badget(&self)->Badget{
        self.badget.clone()
    }
    ///タグを加算
    pub fn add_tag(&mut self, tag_:MejorTag){
        self.tag |= tag_;
    }
    ///タグを消す
    pub fn sub_tag(&mut self, tag_:MejorTag){
        self.tag -= tag_;
    }
    ///タグを参照
    pub fn tag(&self)->MejorTag{
        self.tag.clone()
    }

}


//protected
impl CashHistory{

}