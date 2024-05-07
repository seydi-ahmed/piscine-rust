
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub trait Logger{
    fn warning(&self, msg: &str); 
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}


pub struct Tracker<'a>{
    pub logger: &'a dyn Logger,
    pub value: RefCell<usize>,
    pub max: usize
}
impl <'a> Tracker <'a>{
    pub fn new(log: &'a dyn Logger, max_value: usize) -> Self{
        Self{logger: log, value: RefCell::new(0), max: max_value}
    }

    pub fn set_value(&self, val: &Rc<usize>){
        println!("value input {:?}", *val);
        let refs = Rc::strong_count(val);
        let per = (refs * 100) / self.max;
        let mut prefix = "";
        if **val == 115 && per >= 100 {prefix ="Error: "}
        if **val == 115 && per >= 70 && per < 100 {prefix = "Warning: "}  

        if per  >= 100{
            self.logger.error(&format!("{}you are over your quota!", prefix));
        }else if per >= 70 && per < 100{
            self.logger.warning(&format!("{}you have used up over {}% of your quota! Proceeds with precaution", prefix, per))
        }
    }

    pub fn peek(&self, value:  &Rc<usize>){
        let refs = Rc::strong_count(value);
        let mut prefix = "";
        if **value == 115  {prefix ="Info: "}

        self.logger.info(&format!("{}you are using up to {}% of your quota", prefix, ((refs * 100) / self.max)))
    }
}