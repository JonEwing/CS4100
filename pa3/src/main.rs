/*
//  CS4100
//  PA3: Runtime
//  Marc Baltes & Jonathan Feige
*/

#![allow(non_snake_case)] //get rid of snake case warning
#![allow(warnings)] //get rid of warnings in general

use byteorder::ByteOrder;
use byteorder::{BigEndian};
use std::slice::Iter;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::env;
use std::fs;

//GrumpyVM types

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Val {
    //Value types that may appear in GrumpyVM s.programs:
    Vunit,       //The unit value
    Vi32(i32),   //32-bit signed integers
    Vbool(bool), //Booleans
    Vloc(u32),   //Stack or instruction locations
    Vundef,      //The undefined value

    //Value types that are used internally by the language implementation, and may not appear in GrumpyVM s.programs:
    Vsize(i32), //Metadata for heap objects that span multiple values
                //Vaddr(Address), //Pointers to heap locations
    Vaddr(u32), //for heaps
}

#[derive(Copy, Debug, Clone)]
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
    Halt,          //Halt the machine
    Spawn,         //Multithreads code
    Print          //Prints to terminal
}
#[derive(Copy, Debug, Clone)]
pub enum Binop {
    Add, //i32 addition
    Mul, //i32 multiplication
    Sub, //i32 subtraction
    Div, //i32 division (raises an error on divide by zero)
    Lt,  //Returns true if one i32 is less than another, otherwise false
    Eq,  //Returns true if one i32 is equal another, otherwise false
}
#[derive(Copy, Debug, Clone)]
pub enum Unop {
    Neg, //Boolean negation
}

#[derive(Debug,Clone)]
pub struct State {
    pub halt: bool, //Has the machine halted?
    pub pc: u32, //The current s.program counter, a 32-bit unsigned integer
    pub fp: u32, //The current frame pointer
    pub stack: Vec<Val>, //The stack, with maximum size STACK_SIZE
    pub heap: Vec<Val>, //The heap
    pub program: Vec<Instr>, //The s.program being executed, a list of instructions
    pub to_heap: Vec<Val>,   //heap for gc function
}

pub enum Debug {
    DEBUG,
    NODEBUG
}

trait Binconvert{
	fn toinstr(binary: &mut Iter<u8>) -> Self;
}

//u32 convert
impl Binconvert for u32{
	fn toinstr(binary: &mut Iter<u8>) -> Self{
		let mut v = Vec::new();
		v.push(*binary.next().expect("Error: Could not convert to interger of size u32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size u32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size u32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size u32"));
		let ret = BigEndian::read_u32(&v);
		return ret;
	}
}

//i32 convert
impl Binconvert for i32{
	fn toinstr(binary: &mut Iter<u8>) -> Self{
		let mut v = Vec::new();
		v.push(*binary.next().expect("Error: Could not convert to interger of size i32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size i32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size i32"));
		v.push(*binary.next().expect("Error: Could not convert to interger of size i32"));
		let ret = BigEndian::read_i32(&v);
		return ret;
	}
}
//instructions convert
impl Binconvert for Instr{
    fn toinstr(binary: & mut Iter<u8>) -> Self{
        match binary.next().unwrap(){
			0 => return Instr::Push(<Val as Binconvert>::toinstr(binary)),
			1 => return Instr::Pop,
			2 => return Instr::Peek(<u32 as Binconvert>::toinstr(binary)),
			3 => return Instr::Unary(<Unop as Binconvert>::toinstr(binary)),
			4 => return Instr::Binary(<Binop as Binconvert>::toinstr(binary)),
			5 => return Instr::Swap,
			6 => return Instr::Alloc,
			7 => return Instr::Set,
			8 => return Instr::Get,
			9 => return Instr::Var(<u32 as Binconvert>::toinstr(binary)),
			10 => return Instr::Store(<u32 as Binconvert>::toinstr(binary)),
			11 => return Instr::SetFrame(<u32 as Binconvert>::toinstr(binary)),
			12 => return Instr::Call,
			13 => return Instr::Ret,
			14 => return Instr::Branch,
			15 => return Instr::Halt,
            16 => return Instr::Spawn,
            20 => return Instr::Print,
			_ => panic!("Error: Invalid instruction"),
		}
	}
}

//val convert
impl Binconvert for Val{
	fn toinstr(binary: &mut Iter<u8>) -> Self{
		match binary.next().unwrap(){
			0 => return Val::Vunit,
			1 => return Val::Vi32(<i32 as Binconvert>::toinstr(binary)),
			2 => return Val::Vbool(true),
			3 => return Val::Vbool(false),
			4 => return Val::Vloc(<u32 as Binconvert>::toinstr(binary)),
			5 => return Val::Vundef,
			_ => panic!("Error: Invalid value type representation"),
		}
	}
}

//binop convert
impl Binconvert for Binop{
	fn toinstr(binary: &mut Iter<u8>) -> Self{
		match binary.next().unwrap(){
			0 => return Binop::Add,
			1 => return Binop::Mul,
			2 => return Binop::Sub,
			3 => return Binop::Div,
			4 => return Binop::Lt,
			5 => return Binop::Eq,
			_ => panic!("Error: Invalid binary operation"),
		}
	}
}

//unary convert
impl Binconvert for Unop{
	fn toinstr(binary: &mut Iter<u8>) -> Self{
		match binary.next().unwrap(){
			0 => return Unop::Neg,
			_ => panic!("Error: Invalid unary operation"),
		}
	}
}

//get l from Vloc(l) or Vaddr(l)
fn get_loc(l: Val) -> u32{
    match l{
        Val::Vloc(loc) => loc,
        Val::Vaddr(loc) => loc,
        _ => panic!("Error: expected Location"),
    }

}

//get b from Vi32(b)
fn get_bool(b: Val) -> bool{
    match b{
        Val::Vbool(boolean) => boolean,
        _ => panic!("Error: expected boolean"),
    }

}

//get n from Vi32(n)
fn get_num(n: Val) -> i32{
    match n{
        Val::Vi32(num) => num,
        _ => panic!("Error: expected integer of size i32"),
    }

}

//get char
fn get_char(n: Val) -> u8{
    match n{
        Val::Vi32(num) => num as u8,
        _ => panic!("Error: expected integer of size i32"),
    }

}

//collect garbage function
pub fn gc(s: &mut State){
    let original_size = s.heap.len();
    let mut next = 0;
    let mut scan = 0;
    s.to_heap.clear();    //remove old values

    //pass 1
    for v in 0..s.stack.len(){
        if let Val::Vaddr(base) = s.stack[v]{
            if let Val::Vsize(size) = &s.heap[base as usize] {
                s.stack[v] = Val::Vaddr(next);
                for i in base..(base + *size as u32 + 1){
                    s.to_heap.push(s.heap[i as usize].clone());
                }
                next += *size as u32 + 1;
            }
        }
    }

    //pass 2
    let mut map: HashMap<u32, u32> = HashMap::new();
    while scan < next{
        if let Val::Vaddr(base) = s.to_heap[scan as usize]{
            //check if alread copied
            if let Some(&key) = map.get(&base){
                s.to_heap[scan as usize] = Val::Vaddr(key);
            }
            else{
                if let Val::Vsize(size) = &s.heap[base as usize]{
                    for i in base..(base+*size as u32+1){
                        s.to_heap.push(s.heap[i as usize].clone());
                    }
                    next+=*size as u32;
                    map.insert(base, scan);
                }
            }
        }
        scan +=1;
    }
    s.heap = s.to_heap.clone();
    //uncomment to view gc log on stdout
    //println!("GC end: heap_size = {} values", s.heap.len());

    //gc couldn't collect garbage so error
    if s.heap.len() == original_size {
        panic!("Error: Garbage collection failed to free enough space");
    }
}

pub fn instr(i: &Instr, s: &mut State){
    match i{
        Instr::Push(Val) => {
            match Val{ // if let Val::Vsize(n) = Val { panic!() } else { push val }
                Val::Vsize(n) => panic!("Error: Vsize was pushed"),
                _ => s.stack.push(*Val),
            }
        },
        Instr::Pop => {
            s.stack.pop();
        },
        Instr::Peek(Val) => {
            let v1 = s.stack[s.stack.len() - 1].clone();
            match v1 {
                Val::Vloc(l) => {
                    let loc = s.stack.len() - 1 - l as usize;
                    let copy = s.stack[loc as usize].clone();
                    s.stack.push(copy as Val);
                },
                _ => panic!("Error: Integer not of type i32"),
            }
        },
        Instr::Binary(Binop) => {
            match Binop {
                Binop::Add => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    let total = x + y;
                    s.stack.push(Val::Vi32(total));
                },
                Binop::Mul => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    let total = x * y;
                    s.stack.push(Val::Vi32(total));
                },
                Binop::Sub => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    let total = x - y;
                    s.stack.push(Val::Vi32(total));
                },
                Binop::Div => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    if y == 0 {
                        panic!("Error: Cannot divide by zero");
                    }
                    let total = x / y;
                    s.stack.push(Val::Vi32(total));
                },
                Binop::Lt => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    if x < y {
                        s.stack.push(Val::Vbool(true));
                    }
                    else{
                        s.stack.push(Val::Vbool(false));
                    }
                },
                Binop::Eq => {
                    let v1 = s.stack.pop().expect("Expected Val");
                    let v2 = s.stack.pop().expect("Expected Val");
                    let x = get_num(v1);
                    let y = get_num(v2);
                    if x == y {
                        s.stack.push(Val::Vbool(true));
                    }
                    else{
                        s.stack.push(Val::Vbool(false));
                    }
                }
                _ => panic!("Error: Invalid Binary operator"),
            }
        },
        Instr::Unary(Unop) => {
            match Unop {
                Unop::Neg => {
                    let v1 = s.stack.pop().expect("Expected val");
                    match v1 {
                        Val::Vbool(b) => {
                            let n = get_bool(v1);
                            if n == true{s.stack.push(Val::Vbool(false));}
                            else{s.stack.push(Val::Vbool(true));}
                        },
                        Val::Vi32(n) => {
                            let n = get_num(v1);
                            s.stack.push(Val::Vi32(n * -1));
                        }
                        _ => panic!("Error: Expected integer or boolean for unary operation"),
                    }
                },
                _ => panic!("Error: Unexpected unary operator"),
            }

        },
        Instr::Halt => {
            s.halt = true;
        },
        Instr::Call => {
            let target_v = s.stack.pop().expect("Expected val");
            s.stack.push(Val::Vloc(s.pc));  //save cur pc
            s.pc = get_loc(target_v);
        },
        Instr::Ret => {
            let vret = s.stack.pop().expect("Expected val");    //pop vret
            let caller_pc_v = s.stack.pop().expect("Expected val");
            s.pc = get_loc(caller_pc_v);    //new pc
            let callee_fp = s.fp.clone();
            let caller_fp_v = s.stack.pop().expect("Expected val");
            s.fp = get_loc(caller_fp_v);    //new fp

            //pop args
            for i in callee_fp as usize ..s.stack.len(){
                s.stack.pop();
            }

            //restore vret
            s.stack.push(vret);
        },
        Instr::SetFrame(n) => {
            s.stack.push(Val::Vloc(s.fp));   //push current fp
            s.fp = (s.stack.len() - *n as usize - 1) as u32;   //new fp
        },
        Instr::Alloc =>{
            let vinit = s.stack.pop().expect("Expected val");
            let size_v = s.stack.pop().expect("Expected val");
            let size = get_num(size_v); //number of vals to be added
            let addr = s.heap.len();    //current size of heap

            //check for heap size
            if addr + size as usize >= 1024{
                //output to text file
                //uncomment to output to stdout
                //println!("GC start: heap_size = {} values", addr);
                //pass state
                gc(s);
            }

            //check if the heap is still too Big
            if s.heap.len() >= 1024 {
                panic!("Error: Heap size too large");
            }

            s.stack.push(Val::Vaddr(addr as u32));
            s.heap.push(Val::Vsize(size));
            //push copies of vinit
            for i in 0..size{
                s.heap.push(vinit);
            }
        },
        Instr::Get => {
            let idx_v = s.stack.pop().expect("Expected val");
            let idx = get_num(idx_v);
            let base_v = s.stack.pop().expect("Expected val");
            let base = get_loc(base_v);
            //check location
            if idx + base as i32 + 1 >= s.heap.len() as i32{
                panic!("Error: heap location out of range");
            }
            let v = s.heap[(idx + base as i32 + 1) as usize].clone();
            s.stack.push(v);
        },
        Instr::Set => {
            let v = s.stack.pop().expect("Expected val");
            let idx_v = s.stack.pop().expect("Expected val");
            let idx = get_num(idx_v);
            let base_v = s.stack.pop().expect("Expected val");
            let base = get_loc(base_v);
            //check location
            if idx + base as i32 + 1 >= s.heap.len() as i32{
                panic!("Error: heap location out of range");
            }
            //replace
            s.heap[(idx + base as i32 + 1) as usize] = v;
        },
        Instr::Var(n) => {
            let place = n + s.fp;
            let v = s.stack[place as usize].clone();
            s.stack.push(v);
        },
        Instr::Store(n) => {
            let place = s.fp + n;
            let vnew = s.stack.pop().expect("Expected val");
            s.stack[place as usize] = vnew;
        },
        Instr::Swap => {
            let v1 = s.stack.pop().expect("Expected Val");
            let v2 = s.stack.pop().expect("Expected Val");
            s.stack.push(v1);
            s.stack.push(v2);
        },
        Instr::Branch => {
            let v1 = s.stack[s.stack.len()-1].clone();
            let mut target:u32 = 0;
            let mut check = false;
            match v1 {
                Val::Vloc(l) => {
                    target = l;
                    s.stack.pop();
                },
                _ => panic!("Error: Address must be of type u32"),
            }
            let v2 = s.stack[s.stack.len()-1];
            match v2 {
                Val::Vbool(b) => check = b,
                _ => panic!("Error: Value before target must be boolean"),
            }
            s.stack.pop();
            if check == true{
                s.pc = target;
            }
        }
        Instr::Spawn => {
            let mut done = false;
            let v1 = s.stack.pop().expect("Expected val");
            let closure = get_loc(v1);

            //child thread state
            let mut t = State {
                halt: false,
                pc: get_loc(s.heap[closure as usize + 1]),  //funptr
                fp: 0,
                stack: Vec::new(),
                heap: s.heap.clone(),   //copy of heap
                program: s.program.clone(),
                to_heap: Vec::new(),
            };

            //initial thread state
            t.stack.push(Val::Vaddr(closure));
            t.stack.push(Val::Vunit);
            t.stack.push(Val::Vloc(0)); //ret_fp
            t.stack.push(Val::Vloc(s.pc)); //ret_pc

            thread_exec(&mut t, 2, done);   //2 is the quantum value
        }
        Instr::Print => {
            let v1 = s.stack.pop().expect("Expected val");
            let letter = get_char(v1);
            print!("{}", letter as char)
        }
    }
    //check the sizes of the stack and heap
    if s.heap.len() >= 1024 || s.stack.len() >= 1024{
        panic!("Error: Maximum limit of space used");
    }
}

pub fn exec(s: &mut State) {
    //println!("program: {:?}", s.program);
    'mainloop:loop {
        //println!("stack: {:?}", s.stack);
        //println!("heap: {:?}", s.heap);
        if s.halt { break 'mainloop }
        let pc = s.pc;
        s.pc = pc + 1;
        if pc >= s.program.len() as u32{
            panic!("exec: pc out of bounds")
        }
        let i = &s.program[pc as usize].clone();
        instr(i, s);
    }
}

pub fn thread_exec(s: &mut State, q: u32, mut done: bool) {
    if(done == false){
        let mut end = s.pc + q;
        while(s.pc < end && s.pc < s.program.len() as u32){
            let i = &s.program[s.pc as usize].clone();
            match i {
                Instr::Ret => {
                    s.halt = true;
                    done = true;   //suspend thread
                    s.pc+=1
                },
                _ => {
                    instr(i, s);
                    s.pc+=1;
                }
            }
        }
    }
    //check if still not at return yet
    if(done == false){
        //any threads that didn't have ret will need to suspend
        s.halt = true;
        //spawn next thread
        let mut t = s;
        thread_exec(t, 2, done);
    }
}



fn main(){
    let args: Vec<String> = env::args().collect();
    let mut _filename = String::new();
    _filename = args[1].clone();
    let file = fs::read(_filename).expect("Error: Could not open file");
    let mut file_iter = file.iter();

    let lines = <u32 as Binconvert>::toinstr(file_iter.by_ref());
    let mut s = State {
        halt: false,
        pc: 0,
        fp: 0,
        stack: Vec::new(),
        heap: Vec::new(),
        program: Vec::with_capacity(lines as usize),
        to_heap: Vec::new(),
    };
    for _i in 0..lines{
        s.program.push(Instr::toinstr(file_iter.by_ref()));
    }

    //main loop
    exec(&mut s);

    //print final value
    print!("{:?}", s.stack.pop().unwrap());
}
