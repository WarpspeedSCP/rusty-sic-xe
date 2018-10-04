use std::mem::discriminant;

#[derive(Debug)]
pub struct Line {
    label: Option<String>,
    operation: source_op,
    args: Vec<arg>,
    line_no: u32,
    mem_loc: u32
}

impl Line {
    pub fn new() -> Line {
        Line {
            label:None,
            operation: source_op::Neh, 
            args: Vec::new(),
            line_no: 0,
            mem_loc: 0
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

    pub fn args(mut self, a: Vec<arg>) -> Self {
        self.args = a;
        self
    }

    pub fn line_no(mut self, l: u32) -> Self {
        self.line_no = 0;
        self
    }

    pub fn mem_loc(mut self, m: u32) -> Self {
        self.mem_loc = 0;
        self
    }

}

#[derive(Debug, Eq)]
pub enum arg {
    Label(String),
    StrLit(String),
    IntLit(i32)
}

#[derive(Debug, Eq)]
pub enum source_op {
    directive(u8),
    instruction(u8),
    Neh
}

impl PartialEq for arg {
    fn eq(&self, other: &arg) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl PartialEq for source_op {
    fn eq(&self, other: &source_op) -> bool {
        discriminant(self) == discriminant(other)
    }
}