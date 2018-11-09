use std::mem::discriminant;
use std::collections::HashMap;


use std::fmt::Write;
use std::fmt;

#[derive(Debug)]
pub struct Pos { pub line_no: u32, pub mem_loc: u32, pub val: Option<i32> }

pub type Symtab = HashMap<String, Pos>;
pub type Modtab = HashMap<u32, mod_rec>;

pub struct mod_rec {
    pub mem_loc: u32,
    pub length: u8,
    pub pos: bool,
    pub symbol: String
}

impl mod_rec {
    pub fn new() -> mod_rec {
        mod_rec {
            mem_loc: 0,
            length: 0,
            pos: true,
            symbol: String::default()
        }
    }

    pub fn mem_loc(mut self, m: u32) -> Self {
        self.mem_loc = m;
        self
    }

    pub fn length(mut self, m: u8) -> Self {
        self.length = m;
        self
    }

    pub fn positive(mut self, m: bool) -> Self {
        self.pos = m;
        self
    }

    pub fn symbol(mut self, m: String) -> Self {
        self.symbol = m.clone();
        self
    }
}

#[derive(Debug, Eq, Clone)]
pub enum format {
    Opless,
    Register,
    Normal,
    Long,
    None,
    Directive,
    Comment
}

impl PartialEq for format {
    fn eq(&self, other: &format) -> bool {
        discriminant(self) == discriminant(other)
    }
}

#[derive(Debug ,Clone)]
pub struct Line {
    pub label: Option<String>,
    pub operation: source_op,
    pub args: Vec<arg_struct>,
    pub comment: String,
    pub line_no: u32,
    pub mem_loc: u32,
    pub format: format,
    pub obj_code: Vec<u8>,
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.args == other.args
     && self.comment == other.comment
     && self.label == other.label
     && self.line_no == other.line_no
     && self.mem_loc == other.mem_loc
     && self.format == other.format
    }
}

impl Line {
    pub fn new() -> Line {
        Line {
            label:None,
            operation: source_op::Neh, 
            args: Vec::new(),
            comment: String::default(),
            line_no: 0,
            mem_loc: 0,
            format: format::None,
            obj_code: Vec::new()
        }
    }

    pub fn label(mut self, l: Option<String>) -> Self {
        self.label = l;
        self
    }

    pub fn operation(mut self, op: source_op) -> Self {
        self.operation = op;
        self
    }

    pub fn args(mut self, a: Vec<arg_struct>) -> Self {
        self.args = a;
        self
    }

    pub fn line_no(mut self, l: u32) -> Self {
        self.line_no = l;
        self
    }

    pub fn mem_loc(mut self, m: u32) -> Self {
        self.mem_loc = m;
        self
    }

    pub fn comment(mut self, c: &str) -> Self {
        self.comment = c.to_owned();
        self
    }

    pub fn format(mut self, f: format) -> Self {
        self.format = f;
        self
    }

}

#[derive(Debug, Eq, Clone)]
pub enum addr_mod {
    Direct = 0x00,
    Indirect = 0x02,
    Immediate = 0x01,
    Literal = 0x12
}

#[derive(Debug, Eq, Clone)]
pub struct arg_struct {
    pub val: arg,
    pub reg_code: u8,
    pub modifier: addr_mod
}

pub fn display_vec<T: fmt::Display>(v: &Vec<T>) -> String {
        let mut a = String::new();
        for i in v {
            write!(a, "{}", i);
        }
        for i in 0..(if v.len() < 2 { 2 - v.len() } else { 1 }) {
            write!(a, "      ");
        }

        a
}

pub fn display_vec_nums(v: &Vec<u8>) -> String {
        let mut a = String::new();
        for i in v {
            write!(a, "{:02X}", i);
        }
        for i in 0..(if v.len() < 2 { 2 - v.len() } else { 0 }) {
            write!(a, "      ");
        }
        a
}

impl fmt::Display for arg_struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<8},", self.val)
    }        
}

#[derive(Debug, Eq, Clone)]
pub struct expr_struct {
    pub lhs: arg,
    pub op: u8,
    pub rhs: arg
}

#[derive(Debug, Eq, Clone)]
pub enum arg {
    Label(String),
    StrLit(String),
    IntLit(i32),
    Expr(Box<expr_struct>)
}

impl arg {
    pub fn unwrap_as_int(&self) -> Option<i32> {
        match self {
            arg::IntLit(x) => Some(*x),
            _ => None
        }
    }

    pub fn unwrap_as_string(&self) -> &str {
        match self {
            arg::StrLit(ref x) => x,
            _ => ""
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct op_struct { pub opcode: u8, pub name: &'static str, pub long: bool }

#[derive(Debug, Eq, Clone)]
pub enum source_op {
    Directive(op_struct),
    Instruction(op_struct),
    Neh,
    Error
}

impl source_op {
    pub fn unwrap_as_directive(&self) -> String {
        match self {
            source_op::Directive(x) => x.name.to_owned(),
            _ => "not directive".to_owned(),
        }
    }

    pub fn unwrap_as_instruction(&self) -> String {
        match self {
            source_op::Instruction(x) => x.name.to_owned(),
            _ => "not instruction".to_owned(),
        }
    }
}

impl fmt::Display for arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            arg::Label(x) => write!(f, "{:<8}", x),
            arg::StrLit(x) => write!(f, "{X:<8}", X = format!(r#""{}""#, x)),
            arg::IntLit(x) => write!(f, "{:<8}", x),
            arg::Expr(_) => write!(f, "Comment")
        }
    }
}

impl fmt::Display for source_op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            source_op::Directive(x) => write!(f, "{:<12}", x.name),
            source_op::Instruction(x) => write!(f, "{:<12}", x.name),
            source_op::Error => write!(f, "Error_op"),
            source_op::Neh => write!(f, "Comment")
        }
    }
}

impl PartialEq for arg {
    fn eq(&self, other: &arg) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PartialEq for addr_mod {
    fn eq(&self, other: &addr_mod) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PartialEq for source_op {
    fn eq(&self, other: &source_op) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PartialEq for op_struct {
    fn eq(&self, other: &op_struct) -> bool {
        self.name == other.name && self.opcode == other.opcode && self.long == other.long
    }
}

impl PartialEq for arg_struct {
    fn eq(&self, other: &arg_struct) -> bool {
        self.val == other.val && self.modifier == other.modifier
    }
}

impl PartialEq for expr_struct {
    fn eq(&self, other: &expr_struct) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs && self.op == other.op
    }
}


impl op_struct {
    pub fn new(oc: u8, n: &'static str) -> Self {
        op_struct {
            opcode: oc,
            name: n,
            long: false
        }
    }

    pub fn long(mut self, l: bool) -> Self {
        self.long = l;
        self
    }
}

impl source_op {
    pub fn instr_long_mode(self, l: bool) -> Self {
        match self {
            source_op::Instruction(x) => {
                source_op::Instruction(x.long(l))
            },
            _ => self
        }
    }
}
