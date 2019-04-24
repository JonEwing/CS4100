use std::string::{ToString};

/********************************************
 * Expression language
 ********************************************/

/*
trait ToString {
    fn show(&self) -> String;
}
*/

#[derive(Debug,Clone)]
pub enum Binop {
    BPlus,
    BTimes,
    BMinus,
    BDivide,
    BLess_Than,
    BEqual
}

#[derive(Debug,Clone)]
pub enum Unop {
    Neg
}

#[derive(Debug,Clone)]
pub enum Let {
    Let_x
}

#[derive(Debug,Clone)]
pub enum Seq {
    Seq_x
}

use types::Binop::*;
use types::Unop::*;
use types::Let::*;
use types::Seq::*;

impl ToString for Binop {
    fn to_string(&self) -> String {
        match self {
            BPlus => "+".to_string(),
            BTimes => "*".to_string(),
            BMinus => "-".to_string(),
            BDivide => "/".to_string(),
            BLess_Than => "<".to_string(),
            BEqual => "==".to_string()
        }
    }
}

impl ToString for Unop {
    fn to_string(&self) -> String {
        match self {
            Neg => "neg".to_string()
        }
    }
}

impl ToString for Let {
    fn to_string(&self) -> String {
        match self {
            Let_x => "let".to_string()
        }
    }
}

impl ToString for Seq {
    fn to_string(&self) -> String {
        match self {
            Seq_x => "seq".to_string()
        }
    }
}

#[derive(Debug,Clone)]
pub struct Binexp {
    pub op: Binop,
    pub lhs: Exp,
    pub rhs: Exp
}

#[derive(Debug,Clone)]
pub struct Unexp {
    pub op: Unop,
    pub e: Exp
}

#[derive(Debug,Clone)]
pub struct Letexp {
    pub op: Let,
    pub var: Exp,
    pub e1: Exp,
    pub e2: Exp
}

#[derive(Debug,Clone)]
pub struct Seqexp {
    pub op: Seq,
    pub e1: Exp,
    pub e2: Exp
}

impl ToString for Binexp {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.op.to_string(), self.lhs.to_string(), self.rhs.to_string())
    }
}

impl ToString for Unexp {
    fn to_string(&self) -> String {
        format!("({} {})", self.op.to_string(), self.e.to_string())
    }
}

impl ToString for Letexp {
    fn to_string(&self) -> String {
        format!("({} {} {} {})", self.op.to_string(),self.var.to_string(),self.e1.to_string(),self.e2.to_string())
    }
}

impl ToString for Seqexp {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.op.to_string(),self.e1.to_string(),self.e2.to_string())
    }
}

#[derive(Debug,Clone)]
pub enum Exp {
    EI32(i32),
    EBinop(Box<Binexp>),
    EBool(bool),
    EUnop(Box<Unexp>),
    ELet(Box<Letexp>),
    ESeq(Box<Seqexp>),
    EVar(String),
    EPercent
}

use types::Exp::*;

impl ToString for Exp {
    fn to_string(&self) -> String {
        match self {
            EI32(i) => i.to_string(),
            EVar(v) => v.to_string(),
            EBinop(b) => b.to_string(),
            EBool(b) => b.to_string(),
            EUnop(u) => u.to_string(),
            ELet(l) => l.to_string(),
            ESeq(s) => s.to_string(),
            _ => "oof".to_string()
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Val {
    //Value types that may appear in GrumpyVM programs:
    Vunit,          //The unit value
    Vi32(i32),      //32-bit signed integers
    Vbool(bool),    //Booleans
    Vloc(u32),      //Stack or instruction locations
    Vundef,         //The undefined value

    //Value types that are used internally by the language implementation, and may not appear in GrumpyVM programs:
    Vsize(i32),     //Metadata for heap objects that span multiple values
    Vaddr(u32), //Pointers to heap locations
}

#[derive(Debug,Clone)]
pub enum Instr {
    Push(Val),     //Push(v): Push value v onto the stack
    Pop,           //Pop a value from the stack, discarding it
    Peek(u32),     //Peek(i): Push onto the stack the ith value from the top
    Unary(Unop),   //Unary(u): Apply u to the top value on the stack
    Binary(Binop), //Binary(b): Apply b to the top two values on the stack, replacing them with the result
    Swap,          //Swap the top two values
    Alloc,         //Allocate an array on the heap
    Set,           //Write to a heap-allocated array
    Get,           //Read from a heap-allocated array
    Var(u32),      //Var(i): Get the value at stack position fp+i
    Store(u32),    //Store(i): Store a value at stack position fp+i
    SetFrame(u32), //SetFrame(i): Set fp = s.stack.len() - i
    Call,          //Function call
    Ret,           //Function return
    Branch,        //Conditional jump
    Halt           //Halt the machine
}

use types::Instr::*;
