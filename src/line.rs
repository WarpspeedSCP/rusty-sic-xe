use std::mem::discriminant;
use std::collections::HashMap;

pub type Symtab = HashMap<String, (u32, u32)>;

#[derive(Debug, Eq, Clone)]
pub enum format {
    Opless,
    Register,
    Normal,
    Long,
    None,
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
    pub format: format
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
            format: format::None
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
    Direct,
    Indirect,
    Immediate,
    Literal
}

#[derive(Debug, Eq, Clone)]
pub struct arg_struct {
    pub val: arg,
    pub modifier: addr_mod
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

#[derive(Debug, Eq, Clone)]
pub struct op_struct { pub opcode: u8, pub name: &'static str, pub long: bool }

#[derive(Debug, Eq, Clone)]
pub enum source_op {
    Directive(op_struct),
    Instruction(op_struct),
    Neh,
    Error
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
