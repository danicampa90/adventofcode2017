use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Clone)]
pub struct Instruction {
    pub target_reg: String,
    pub target_op : Operation,
    pub target_value : i32,
    pub cond_left: String,
    pub cond_op : Condition,
    pub cond_right : i32,
}

impl Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "Instruction ({} {} {}) if ({} {} {})",
        self.target_reg, self.target_op, self.target_value, self.cond_left, self.cond_op, self.cond_right);
        Ok(())
    }
}


#[derive(Clone)]
#[derive(Copy)]
pub enum Operation {
    Inc,
    Dec,
}
impl Display for Operation {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Inc => write!(fmt, "inc"),
            Dec => write!(fmt, "dec"),
        }
    }
}



#[derive(Clone)]
#[derive(Copy)]
pub enum Condition {
    GT, GE, EQ, NE, LE, LT
}

impl Display for Condition {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", match self {
            GT=> ">",
            GE=> ">=", 
            EQ=> "==",
            NE=> "!=",
            LE=> "<=",
            LT=> "<",
        });
        Ok(())
    }
}