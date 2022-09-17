use super::*;
#[allow(dead_code)]
//public
impl CashHistory{
    ///収入を作成
    /// expenses:金額
    /// day:日にち
    /// badget:予算枠
    /// memo:メモ
    pub fn income(expenses_:u64, day_:Date, badget_:Badget, memo_:String)->Self{
        CashHistory{usage:Usage::Income, expenses:expenses_, day:day_, tag:MejorTag::NONE, badget:badget_, memo:memo_}
    }
    ///支出を作成
    /// expenses:金額
    /// day:日にち
    /// badget:予算枠
    /// memo:メモ
    pub fn spending(expenses_:u64, day_:Date,badget_:Badget, memo_:String)->Self{
        CashHistory{usage:Usage::Spending, expenses:expenses_, day:day_, tag:MejorTag::NONE, badget:badget_, memo:memo_}
    }
    ///動かした額の参照
    pub fn expenses(&self)->i128{
        if self.usage == Usage::Income{
            self.expenses as i128
        }
        else{
            -(self.expenses as i128)
        }
    }
    ///日にち参照
    pub fn day(&self)->Date{
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
    ///タグをintから設定
    pub fn set_tag(&mut self, tag:u64){
        self.tag.bits = tag;
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
impl  Badget {
    pub fn from_int(i:u64)->Self{
        match i{
            0=>{return Badget::Weekly;}
            1=>{return Badget::Monthly;}
            2=>{return Badget::TwoMonthly;}
            3=>{return Badget::ThreeMonthly;}
            4=>{return Badget::ForuMonthly;}
            5=>{return Badget::HalfAnnual;}
            6=>{return Badget::Annual;}
            _=>{return Badget::None;}
        }
    }
}

impl  Usage {
    pub fn from_int(i:u64)->Self{
        match i{
            0=>{return Usage::Income;}
            _=>{return Usage::Spending;}
        }
    }
}