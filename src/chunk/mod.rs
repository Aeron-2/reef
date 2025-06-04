use crate::opcode::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Values {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(*mut Obj),
    Tombstone,
}

// declaration of Obj starts at [line number 143].

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Values>,
    pub lines: Vec<(usize, u32)>
}

impl Chunk {
    pub fn new(capacity: usize) -> Self {
        Self {
            code: Vec::with_capacity(capacity),
            constants: Vec::with_capacity(capacity * 2),
            lines: Vec::with_capacity(capacity),
        }
    }

    pub fn write_byte(&mut self, opcode: u8, line: u32) {
        let offset = self.code.len();
        self.code.push(opcode);

        if self.lines.last().map(|&(_, l)| l) != Some(line) {
            self.lines.push((offset, line));
        }
    }

    pub fn write_constant(&mut self, constant: Values, line: u32) {
        let index = self.constants.len();
        self.constants.push(constant);

        if index < 256 {
            self.write_byte(OP_CONSTANT, line);
            self.write_byte(index as u8, line);
        } else {
            let bytes = u32_to_u24(index as u32);
            self.write_byte(OP_CONSTANT_LONG, line);
            for &b in &bytes {
                self.write_byte(b, line);
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn chunk_peek(&self, name: &str) {
        println!("== {} ==", name);
        let mut i = 0;
        while i < self.code.len() {
            i += self.chunk_match(i);
        }
    }

    #[cfg(debug_assertions)]
    pub fn chunk_match(&self, idx: usize) -> usize {
        let opcode: u8 = self.code[idx];
        let line: u32 = self.get_line(idx);

        let offset = match opcode {
            OP_RETURN => self.return_instruction(idx, line, "OP_RETURN"),
            OP_CONSTANT => self.constant_instruction(idx, line, "OP_CONSTANT"),
            OP_CONSTANT_LONG => self.constant_long_instruction(idx, line, "OP_CONSTANT_LONG"),
            OP_NEGATE => self.return_instruction(idx, line, "OP_NEGATE"),
            OP_ADD => self.return_instruction(idx, line, "OP_ADD"),
            OP_SUBTRACT => self.return_instruction(idx, line, "OP_SUBTRACT"),
            OP_MULTIPLY => self.return_instruction(idx, line, "OP_MULTIPLY"),
            OP_DIVIDE => self.return_instruction(idx, line, "OP_DIVIDE"),
            OP_TRUE => self.return_instruction(idx, line, "OP_TRUE"),
            OP_FALSE => self.return_instruction(idx, line, "OP_FALSE"),
            OP_NIL => self.return_instruction(idx, line, "OP_NIL"),
            OP_EQUAL => self.return_instruction(idx, line, "OP_EQUAL"),
            OP_GREATER => self.return_instruction(idx, line, "OP_GREATER"),
            OP_LESS => self.return_instruction(idx, line, "OP_LESS"),
            OP_POP => self.return_instruction(idx, line, "OP_POP"),
            _ => {
                panic!("Lexer: Unknown Opcode {}", opcode);
            },
        };

        offset
    }

    #[cfg(debug_assertions)]
    fn constant_instruction(&self, idx: usize, line: u32, name: &str) -> usize {
        let constant_idx = self.code[idx + 1] as usize;
        match &self.constants[constant_idx] {
            Values::Number(num) => {
                println!("{:04} (line {}) {} {} {}", idx, line, name, constant_idx, num);
            },
            Values::Obj(obj_ptr) => {
                unsafe {
                    match (*(*obj_ptr)).type_obj {
                        ObjType::String => {
                            let str_ptr: *mut ObjString = *obj_ptr as *mut ObjString;
                            let string = std::str::from_utf8_unchecked(std::slice::from_raw_parts((*str_ptr).chars, (*str_ptr).length));

                            println!("{:04} (line {}) {} {} {}", idx, line, name, constant_idx, string);
                        },
                        _ => panic!("you cannot add non-obj value as an obj"),
                    }
                }
            },
            _ => panic!("You cannot push non-literal-value"),
        }
        2
    }

    #[cfg(debug_assertions)]
    fn constant_long_instruction(&self, idx: usize, line: u32, name: &str) -> usize {
        let constant_idx = u24_to_u32([
            self.code[idx + 1],
            self.code[idx + 2],
            self.code[idx + 3],
        ]) as usize;

        match &self.constants[constant_idx] {
            Values::Number(num) => {
                println!("{:04} (line {}) {} {} {}", idx, line, name, constant_idx, num);
            },
            _ => panic!("You cannot push non-literal-value"),
        }
        4
    }
    
    #[cfg(debug_assertions)]
    fn return_instruction(&self, idx: usize, line: u32, name: &str) -> usize {
        println!("{:04} (line {}) {}", idx, line, name);
        1
    }
    
    #[cfg(debug_assertions)]
    pub fn get_line(&self, idx: usize) -> u32 {
        match self.lines.binary_search_by_key(&idx, |&(off, _)| off) {
            Ok(i) => self.lines[i].1,
            Err(0) => 0,
            Err(i) => self.lines[i - 1].1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjType {
    String,
    Dummy,
}

#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub length: usize,
    pub chars: *mut u8,
    pub hash: u32,
}

#[repr(C)]
pub struct Obj {
    pub type_obj: ObjType,
    pub next: *mut Obj,
}

impl Obj {
    pub fn dummy() -> *mut Obj {
        let obj = Obj {
            type_obj: ObjType::Dummy,
            next: std::ptr::null_mut(),
        };
        Box::into_raw(Box::new(obj))
    }
}

fn u32_to_u24(value: u32) -> [u8; 3] {
    [
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ]
}

fn u24_to_u32(bytes: [u8; 3]) -> u32 {
    ((bytes[0] as u32) << 16)
        | ((bytes[1] as u32) << 8)
        | (bytes[2] as u32)
}

