pub mod table;

use super::{
    chunk::{Chunk, Values, Obj, ObjType, ObjString},
    opcode::*,
    compiler::{self},
};
use std::alloc::{self, Layout};

macro_rules! binary_op {
    ($stack:expr, $op:tt, $line:expr) => {{
        match (unsafe { $stack.pop().unwrap_unchecked() }, unsafe { $stack.pop().unwrap_unchecked() }) {
            (Values::Number(b), Values::Number(a)) => {
                $stack.push(Values::Number(a $op b));
            },
            _ => return (InterpretResult::RuntimeError { message: String::from("failed to binary_op!"), line: $line, }),
        }
    }};
}

pub enum InterpretResult {
    Done,
    CompileError,
    RuntimeError {
        message: String,
        line: u32,
    },
}

pub struct VM {
    pub chunk: Chunk,
    ip: *const u8,
    stack: Vec<Values>,
    objects: *mut Obj,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        let ip = chunk.code.as_ptr();
        let stack_capacity = chunk.code.len();
        Self {
            chunk,
            ip,
            stack: Vec::with_capacity(stack_capacity * 2),
            objects: std::ptr::null_mut(),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        #[cfg(debug_assertions)]
        {
            println!("== test at VM(in process) ==");
        }

        loop {
            #[cfg(debug_assertions)]
            {
                let offset = unsafe { self.ip.offset_from(self.chunk.code.as_ptr()) } as usize;
                self.chunk.chunk_match(offset);
            }

            let instruction = unsafe {
                let instruction = *self.ip;
                self.ip = self.ip.add(1);
                instruction
            };

            let offset = unsafe { self.ip.offset_from(self.chunk.code.as_ptr()) } as usize;
            let line = self.chunk.get_line(offset);
            
            match instruction {
                OP_RETURN => {
                    #[cfg(debug_assertions)]
                    {
                        println!("== test at VM(ended - before poping last value) ==");
                        for i in &self.stack {
                            println!("{:?}", i);
                        }
                    }
                    match self.stack.pop() {
                        Some(_) => return InterpretResult::Done,
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("Oops!  You couldn't make return from [Brr brr patapim!]..."),
                            line,
                        },
                    }
                },
                OP_CONSTANT => {
                    let constant_idx = unsafe { *self.ip } as usize;
                    self.ip = unsafe { self.ip.add(1) };
                    if let Values::Obj(obj) = &self.chunk.constants[constant_idx] {
                        unsafe {
                            (*(*obj)).next = self.objects;
                            self.objects = *obj;
                        }
                    }
                    self.stack.push(self.chunk.constants[constant_idx].clone());
                },
                OP_CONSTANT_LONG => {
                    let b0 = unsafe { *self.ip } as u32;
                    let b1 = unsafe { *self.ip.add(1) } as u32;
                    let b2 = unsafe { *self.ip.add(2) } as u32;
                    let index = ((b0 << 16) | (b1 << 8) | b2) as usize;
                    self.ip = unsafe { self.ip.add(3) };
                    if let Values::Obj(obj) = &self.chunk.constants[index] {
                        unsafe {
                            (*(*obj)).next = self.objects;
                            self.objects = *obj;
                        }
                    }
                    self.stack.push(self.chunk.constants[index].clone());
                },
                OP_NEGATE => {
                    match self.stack.pop() {
                        Some(Values::Number(i)) => self.stack.push(Values::Number(-i)),
                        Some(Values::Bool(i)) => self.stack.push(Values::Bool(!i)),
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("You tried to negate the value, which does not support.."),
                            line,
                        },
                    }
                },
                OP_ADD => {
                    let b = unsafe { self.stack.pop().unwrap_unchecked() };
                    let a = unsafe { self.stack.pop().unwrap_unchecked() };

                    match (a, b) {
                        (Values::Number(aa), Values::Number(bb)) => self.stack.push(Values::Number(aa + bb)),
                        (Values::Obj(pa), Values::Obj(pb)) => {
                            unsafe {
                                match ((*pa).type_obj, (*pb).type_obj) {
                                    (ObjType::String, ObjType::String) => {
                                        let pa: *mut ObjString = pa as *mut ObjString;
                                        let pb: *mut ObjString = pb as *mut ObjString;
                                        let cap: usize = (*pa).length + (*pb).length;
                                        let layout = Layout::array::<u8>(cap).unwrap();
                                        let ptr: *mut u8 = alloc::alloc(layout);
                                        std::ptr::copy_nonoverlapping((*pa).chars, ptr, (*pa).length);
                                        std::ptr::copy_nonoverlapping((*pb).chars, ptr.add((*pa).length), (*pb).length);
                                        let obj_str_ptr = compiler::make_obj_str(ptr, cap) as *mut Obj;
                                        (*obj_str_ptr).next = self.objects;
                                        self.objects = obj_str_ptr;
                                        self.stack.push(Values::Obj(obj_str_ptr));
                                    },
                                    _ => return InterpretResult::RuntimeError {
                                        message: String::from("You cannot compare non-comparable value! (only numbers and strings)"),
                                        line,
                                    },
                                }
                            }
                        },
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("You cannot compare non-comparable value! (only numbers and strings)"),
                            line,
                        },
                    }
                },
                OP_SUBTRACT => binary_op!(self.stack, -, line),
                OP_MULTIPLY => binary_op!(self.stack, *, line),
                OP_DIVIDE => binary_op!(self.stack, /, line),
                OP_TRUE => self.stack.push(Values::Bool(true)),
                OP_FALSE => self.stack.push(Values::Bool(false)),
                OP_NIL => self.stack.push(Values::Nil),
                OP_EQUAL => {
                    let b = unsafe { self.stack.pop().unwrap_unchecked() };
                    let a = unsafe { self.stack.pop().unwrap_unchecked() };

                    match (a, b) {
                        (Values::Number(aa), Values::Number(bb)) => self.stack.push(Values::Bool(aa == bb)),
                        (Values::Obj(pa), Values::Obj(pb)) => {
                            unsafe {
                                match ((*pa).type_obj, (*pb).type_obj) {
                                    (ObjType::String, ObjType::String) => {
                                        let pa: *mut ObjString = pa as *mut ObjString;
                                        let pb: *mut ObjString = pb as *mut ObjString;
                                        let sa = std::slice::from_raw_parts((*pa).chars, (*pa).length);
                                        let sb = std::slice::from_raw_parts((*pb).chars, (*pb).length);
                                        self.stack.push(Values::Bool(sa == sb));
                                    },
                                    _ => return InterpretResult::RuntimeError {
                                        message: String::from("You cannot add non-addable value! (only numbers and strings)"),
                                        line,
                                    },
                                }
                            }
                        },
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("You cannot add non-addable value! (only numbers and strings)"),
                            line,
                        },
                    }
                },
                OP_GREATER => {
                    let b = unsafe { self.stack.pop().unwrap_unchecked() };
                    let a = unsafe { self.stack.pop().unwrap_unchecked() };
                    match (a, b) {
                        (Values::Number(a), Values::Number(b)) => self.stack.push(Values::Bool(a > b)),
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("You cannot compare non-comparable value! (only numbers)"),
                            line,
                        },
                    }
                },
                OP_LESS => {
                    let b = unsafe { self.stack.pop().unwrap_unchecked() };
                    let a = unsafe { self.stack.pop().unwrap_unchecked() };
                    match (a, b) {
                        (Values::Number(a), Values::Number(b)) => self.stack.push(Values::Bool(a < b)),
                        _ => return InterpretResult::RuntimeError {
                            message: String::from("You cannot compare non-comparable value! (only numbers)"),
                            line,
                        }
                    }
                },
                _ => return InterpretResult::RuntimeError {
                    message : String::from("Fatal! Something wrong happened..."),
                    line,
                }
            }
        }
    }

    pub fn free_objects(&mut self) {
        let mut object = self.objects;
        unsafe {
            while !object.is_null() {
                let next: *mut Obj = (*object).next;
                match (*object).type_obj {
                    ObjType::String => {
                        let s = object as *mut ObjString;
                        let layout = Layout::array::<u8>((*s).length).unwrap();
                        alloc::dealloc((*s).chars, layout);
                        let layout = Layout::new::<ObjString>();
                        alloc::dealloc(s as *mut u8, layout);
                    },
                    _ => panic!("memory --> leaked!!!!!!"),
                }
                object = next;
            }
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        self.free_objects();
    }
}

