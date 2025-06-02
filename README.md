# The Reef Programming Language

Reef is a dynamically typed, Rust-inspired language that supports concepts like struct(Form) and impl(Item).

<div align="center">
  <img src="https://github.com/user-attachments/assets/ea384868-f0f5-4f89-83a9-73f4684bf827"
       width="400"
       height="400"
  />
</div>

# 🧱 Architecture

## Lexer
- Hand-written lexer using a raw character stream
- Zero-allocation slicing (no string copies)

## Compiler
- Pratt parser with a `ParseRule` table (prefix/infix functions + precedence)
- Constant pool and runtime object pointer emission (`ObjString`)
- No external dependencies

## Virtual Machine (VM)
- Stack-based bytecode interpreter
- Runtime type dispatch for numbers and strings

## Object System
- String object (`ObjString`) with custom layout
- Equality comparisons done via raw slice comparison

## GC *(planned)*

# ⚙️ Optimizations

- Open-addressing hash table (`Table`) with tombstones
- Line number RLE compression in `Chunk`
- No redundant allocations for identical string literals

# 🧪 Debugging Tools

- Bytecode disassembler with opcode/line printing
- Debug trace of VM execution (only in debug mode)
- Runtime error propagation with line info

---
# 🌊 Reef Language Syntax

Reef is a Rust-inspired dynamically typed language focused on simplicity and low-level performance. It supports:

### 🔤 Literals
```reef
123         // number
"hello"     // string
true, false // boolean
nil         // null value
```

### ➕ Operators
```reef
1 + 2 * 3 == 7   // arithmetic and equality
!true == false   // logical not and comparison
!3 == -3         // negate number
```

### 🧠 Variables & Constants *(planned)*
```reef
Fn hello() {}
let x = 10;
x = x + 1;
```

### 🧱 Blocks & Conditionals *(planned)*
```reef
if (x > 0) {
  print "positive";
} else {
  print "non-positive";
}
```

### 📦 Struct & Impl *(planned)*
```reef
Form Point {
  x,
  y
}

Item Point {
  Fn distance() {
    // method body
  }
}
```

More features like functions, closures, and modules will be supported soon.
