use nom::{is_hex_digit, is_digit, is_alphabetic, is_alphanumeric};

use std::str;

use super::line::*;

pub fn add_to_symtab(curr: &mut Line, symtab: &mut Symtab, panic: (bool, &'static str)) -> Result<(), String> {
    match curr.label {
        Some(ref l) => {
            if !symtab.contains_key(l) {
                symtab.insert(l.to_string(), Pos { line_no: curr.line_no, mem_loc: curr.mem_loc });
                Ok(())
            } else {
                Err(
                    format!(
                        "Duplicate definition of symbol {}, first defined in line {}.", 
                        l, 
                        symtab.get(l).unwrap().mem_loc
                    )
                )
            }
        }
        None => {
            if panic.0 {
                Err(panic.1.to_owned())
            } else {
                Ok(())
            }
        }
    }
}

pub fn gen_header_record(parsed_vec: &Vec<Line>) -> String {
    let start = parsed_vec.iter().find(|i| i.operation.unwrap_as_directive() == "START".to_owned()).unwrap();
    let st = start.args[0].val.unwrap_as_int();
    let end = parsed_vec.iter().find(|i| i.operation.unwrap_as_directive() == "END".to_owned()).unwrap().mem_loc;
    String::new() + "H" + &*format!("{:>6}{:0>6X}{:06X}\n", start.label.clone().unwrap(), st, end as u32)
}

pub fn gen_obj_code(curr: &mut Line, symtab: &Symtab, base: u32) {
    match curr.format {
        format::Opless => {
            let tmp = curr.operation.clone();
            match tmp {
                source_op::Instruction(x) => curr.obj_code.push(x.opcode),
                _ => panic!("NOT ALLOWED")
            }
        },
        format::Register => {
            let tmp = curr.operation.clone();
            match tmp {
                source_op::Instruction(x) => curr.obj_code.push(x.opcode),
                _ => panic!("NOT ALLOWED")
            }
            let mut tmp = 0u8;
            match curr.args.len() {
                1 => curr.obj_code.push(curr.args[0].reg_code << 4),
                2 => curr.obj_code.push((curr.args[0].reg_code << 4) | (curr.args[1].reg_code)),
                _ => panic!("NOT ALLOWED")
            }
        },
        // opcode n i | x b p e offset
        // 000000 0 0 | 0 0 0 0 0000 0000 0000
        format::Normal => {
            let mut opcode = match curr.operation.clone() {
                source_op::Instruction(x) => x.opcode,
                _ => panic!()
            };
            match curr.args.len() {
                1 | 2 => {
                    opcode |= curr.args[0].modifier.clone() as u8;
                    let mut disp = 0u16;

                    match curr.args[0].modifier {
                        addr_mod::Direct | addr_mod::Indirect => match curr.args[0].val {
                            arg::Label(ref x) => {
                                let target = symtab.get(x).unwrap().mem_loc;
                                if  base != 0xFFFFFFFF  {
                                    disp = ((target - base) as u16 & 0x0FFF) | 0x4000u16; // OR with 0x4000 for base flag
                                } else {
                                    disp = (((target as i32 - curr.mem_loc as i32) as i16 & 0x0FFF) | 0x2000) as u16 ; // OR with 0x2000 for PC flag
                                }
                            }
                            arg::IntLit(ref x) => {
                                let target = *x;
                                if  base != 0xFFFFFFFF  {
                                    disp = ((target as u32 - base) as u16 & 0x0FFF) | 0x4000u16;
                                } else {
                                    disp = (((target - curr.mem_loc as i32) as i16 & 0x0FFF) | 0x2000) as u16;
                                }
                            }
                            _ => panic!()
                        }
                        addr_mod::Immediate => match curr.args[0].val {
                            arg::Label(ref x) => {
                                let target = symtab.get(x).unwrap().mem_loc;
                                if  base != 0xFFFFFFFF  {
                                    disp = ((target - base) as u16 & 0x0FFF) | 0x4000u16; // OR with 0x4000 for base flag
                                } else {
                                    disp = (((target as i32 - curr.mem_loc as i32) as i16 & 0x0FFF) | 0x2000) as u16 ; // OR with 0x2000 for PC flag
                                }
                            }
                            arg::IntLit(ref x) => {
                                if *x > 0x0FFF {
                                    panic!();
                                } else {
                                    disp = *x as u16;
                                }
                            }
                            _ => panic!()
                        }
                        _ => panic!()
                    }

                    if curr.args.len() == 2 {
                        if curr.args[1].reg_code == 0x01 {
                            disp |= 0x8000u16;
                        } else {
                            panic!()
                        }
                    }
                    curr.obj_code.push(opcode);
                    curr.obj_code.push((disp >> 8) as u8);
                    curr.obj_code.push((disp & 0x00FF) as u8);

                },
                _ => panic!()
            }

        },
        // opcode n i | x b p e addr |           |          
        // 000000 0 0 | 0 n n 1 0000 | 0000 0000 | 0000 0000
        //              8 4 2 1  0      0    0      0    0  
        format::Long => {
            let mut opcode = match curr.operation.clone() {
                source_op::Instruction(x) => x.opcode,
                _ => panic!()
            };
            match curr.args.len() {
                1 | 2 => {
                    opcode |= curr.args[0].modifier.clone() as u8;
                    let mut ta = 0x00100000u32;

                    match curr.args[0].modifier {
                        addr_mod::Direct | addr_mod::Indirect => match curr.args[0].val {
                            arg::Label(ref x) => {
                                ta |= symtab.get(x).unwrap().mem_loc & 0x000FFFFFu32;
                            }
                            arg::IntLit(ref x) => {
                                ta |= *x as u32 & 0x000FFFFFu32;
                            }
                            _ => panic!()
                        }
                        addr_mod::Immediate => match curr.args[0].val {
                            arg::Label(ref x) => {
                                ta |= symtab.get(x).unwrap().mem_loc & 0x000FFFFFu32;
                            }
                            arg::IntLit(ref x) => {
                                ta |= *x as u32 & 0x000FFFFFu32;
                            }
                            _ => panic!()
                        }
                        _ => panic!()
                    }

                    if curr.args.len() == 2 {
                        if curr.args[1].reg_code == 0x01 {
                            ta |= 0x800000u32;
                        } else {
                            panic!()
                        }
                    }
                    curr.obj_code.push(opcode);
                    curr.obj_code.push(((ta & 0x00FF0000) >> 16) as u8);
                    curr.obj_code.push(((ta & 0x0000FF00) >> 8) as u8);
                    curr.obj_code.push(((ta & 0x000000FF) >> 0) as u8);

                },
                _ => panic!()
            }
        },
        _ => return
    }
}

#[allow(dead_code)]
const IDENTIFIER_CHARS: &'static [u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

macro_rules! tag_max {
  ($i:expr, $tg:expr) => ( terminated!($i, tag_no_case!($tg), peek!(none_of!(IDENTIFIER_CHARS))));
}

fn make_str(x: &[u8]) -> &str {
    str::from_utf8(x).unwrap()
}

fn make_dec<'a>(sign: Option<&'a[u8]>, mag: &'a[u8]) -> i32 {
    match sign {
        Some(sign) => {
            let res = make_str(sign).to_owned() + make_str(mag);
            i32::from_str_radix(&res, 10).unwrap() & 0x00ffffffi32
        },
        None => i32::from_str_radix(make_str(mag), 10).unwrap() & 0x00ffffffi32
    }
}

fn make_hex<'a>(sign: Option<&'a[u8]>, mag: &'a[u8]) -> i32 {
    match sign {
        Some(sign) => {
            let res = make_str(sign).to_owned() + make_str(mag);
            i32::from_str_radix(&res, 16).unwrap() & 0x00ffffffi32
        },
        None => i32::from_str_radix(make_str(mag), 16).unwrap() & 0x00ffffffi32
    }
}

//trace_macros!(true);
named!(
    pub dec(&[u8]) -> i32, 
    do_parse!(
        sign: opt!(
            alt_complete!(
                tag!("+") | tag!("-")
            )
        ) 
     >> mag: take_while1!(is_digit)
     >> (make_dec(sign, mag))
     )
);

named!(
    hex_base(&[u8]) -> i32,
    do_parse!(
        sign: opt!(
            alt_complete!(
                tag!("+") | tag!("-")
            )
        )
     >> mag: take_while1!(is_hex_digit)
     >> (make_hex(sign, mag))
    )
);

named!(
    pub hex(&[u8]) -> i32,
    do_parse!(
        val: terminated!(
            hex_base,
            tag!("h")
        )
     >> (val)
    )
);

named!(
    pub hex_lit(&[u8]) -> i32,
    do_parse!(
        val: delimited!(
            tag_no_case!("x'"),
            hex_base,
            tag!("'")
        )
     >> (val)
    )
);

named!(
    pub num(&[u8]) -> i32,
    do_parse!(
        val: alt_complete!(
            hex_lit | hex | dec
        )
     >> (val)
    )
);

named!(
    pub label(&[u8]) -> String,
    do_parse!(
        not!(alt!(asm_directive | instruction))
     >> a: take_while!(|c| is_alphabetic(c) | (c == '_' as u8))
     >> b: take_while!(|c| is_alphanumeric(c) | (c == '_' as u8))
     >> (make_str(a).to_owned() + make_str(b))
    )
);

named!(
    pub str_lit(&[u8]) -> String,
    do_parse!(
        val: delimited!(
            tag_no_case!("c'"),
            take_until!("'"),
            tag!("'")
        )
     >> (make_str(val).to_owned())
    )
);

named!(
    pub arg(&[u8]) -> arg_struct,
    do_parse!(
        mode:
            alt!(
                value!(1, tag!("#")) | value!(2, tag!("@")) | value!(3, tag!("=")) | value!(0)
            )
     >> content: alt_complete!(
           num     => { |n| arg::IntLit(n) }
         | str_lit => { |s| arg::StrLit(s) }
         | label   => { |l| arg::Label(l)  }
        )
     >> (
                arg_struct{
                    reg_code: match content {
                        arg::Label(ref x) => match x.to_uppercase().as_str() {
                            "A"     => 0x00,
                            "X"     => 0x01,
                            "L"     => 0x02,
                            "B"     => 0x03,
                            "S"     => 0x04,
                            "T"     => 0x05,
                            "F"     => 0x06,
                            "PC"    => 0x08,
                            "SW"    => 0x09,
                            _       => 0xFF
                        }
                        _ => 0xFF
                    }, 
                    val: content,
                    modifier: match mode {
                        0 => addr_mod::Direct,
                        1 => addr_mod::Immediate,
                        2 => addr_mod::Indirect,
                        3 => addr_mod::Literal,
                        _ => panic!()
                    }
                })
    )

);



named!(
    pub asm_directive(&[u8]) -> source_op,
        alt_complete!(
            tag_max!("START") => { |_| source_op::Directive(op_struct::new(0x01, "START")) } 
        |   tag_max!("END"  ) => { |_| source_op::Directive(op_struct::new(0x02, "END"  )) } 
        |   tag_max!("BYTE" ) => { |_| source_op::Directive(op_struct::new(0x03, "BYTE" )) } 
        |   tag_max!("WORD" ) => { |_| source_op::Directive(op_struct::new(0x04, "WORD" )) } 
        |   tag_max!("RESB" ) => { |_| source_op::Directive(op_struct::new(0x05, "RESB" )) } 
        |   tag_max!("RESW" ) => { |_| source_op::Directive(op_struct::new(0x06, "RESW" )) }
        |   tag_max!("BASE" ) => { |_| source_op::Directive(op_struct::new(0x07, "BASE" )) }
        )
);


named!(
    pub instruction(&[u8]) -> source_op,
    do_parse!(
        mode: alt!(
            value!(true, tag!("+")) | value!(false)
        )
     >> content: alt_complete!(
          tag_max!("ADD"   ) => { |_| source_op::Instruction(op_struct::new(0x18, "ADD"   )) }
      //| tag_max!("ADDF"  ) => { |_| source_op::Instruction(op_struct::new(0x58, "ADDF"  )) }
        | tag_max!("ADDR"  ) => { |_| source_op::Instruction(op_struct::new(0x90, "ADDR"  )) }
        | tag_max!("AND"   ) => { |_| source_op::Instruction(op_struct::new(0x40, "AND"   )) }
        | tag_max!("CLEAR" ) => { |_| source_op::Instruction(op_struct::new(0xB4, "CLEAR" )) }
        | tag_max!("COMP"  ) => { |_| source_op::Instruction(op_struct::new(0x28, "COMP"  )) }
        | tag_max!("COMPR" ) => { |_| source_op::Instruction(op_struct::new(0x88, "COMPR" )) }
        | tag_max!("DIV"   ) => { |_| source_op::Instruction(op_struct::new(0xA0, "DIV"   )) }
      //| tag_max!("DIVF"  ) => { |_| source_op::Instruction(op_struct::new(0x24, "DIVF"  )) }
        | tag_max!("DIVR"  ) => { |_| source_op::Instruction(op_struct::new(0x64, "DIVR"  )) }
      //| tag_max!("FIX"   ) => { |_| source_op::Instruction(op_struct::new(0xC4, "FIX"   )) }
      //| tag_max!("FLOAT" ) => { |_| source_op::Instruction(op_struct::new(0xC0, "FLOAT" )) }
        | tag_max!("HIO"   ) => { |_| source_op::Instruction(op_struct::new(0xF4, "HIO"   )) }
        | tag_max!("JEQ"   ) => { |_| source_op::Instruction(op_struct::new(0x30, "JEQ"   )) }
        | tag_max!("JLT"   ) => { |_| source_op::Instruction(op_struct::new(0x34, "JLT"   )) }
        | tag_max!("JGT"   ) => { |_| source_op::Instruction(op_struct::new(0x38, "JGT"   )) }
        | tag_max!("JSUB"  ) => { |_| source_op::Instruction(op_struct::new(0x48, "JSUB"  )) }
        | tag_max!("J"     ) => { |_| source_op::Instruction(op_struct::new(0x3C, "J"     )) }
        | tag_max!("LDA"   ) => { |_| source_op::Instruction(op_struct::new(0x00, "LDA"   )) }
        | tag_max!("LDB"   ) => { |_| source_op::Instruction(op_struct::new(0x68, "LDB"   )) }
        | tag_max!("LDCH"  ) => { |_| source_op::Instruction(op_struct::new(0x50, "LDCH"  )) }
      //| tag_max!("LDF"   ) => { |_| source_op::Instruction(op_struct::new(0x70, "LDF"   )) }
        | tag_max!("LDL"   ) => { |_| source_op::Instruction(op_struct::new(0x08, "LDL"   )) }
        | tag_max!("LDS"   ) => { |_| source_op::Instruction(op_struct::new(0x6C, "LDS"   )) }
        | tag_max!("LDT"   ) => { |_| source_op::Instruction(op_struct::new(0x74, "LDT"   )) }
        | tag_max!("LDX"   ) => { |_| source_op::Instruction(op_struct::new(0x04, "LDX"   )) }
        | tag_max!("LPS"   ) => { |_| source_op::Instruction(op_struct::new(0xD0, "LPS"   )) }
        | tag_max!("MUL"   ) => { |_| source_op::Instruction(op_struct::new(0x20, "MUL"   )) }
      //| tag_max!("MULF"  ) => { |_| source_op::Instruction(op_struct::new(0x60, "MULF"  )) }
        | tag_max!("MULR"  ) => { |_| source_op::Instruction(op_struct::new(0x98, "MULR"  )) }
      //| tag_max!("NORM"  ) => { |_| source_op::Instruction(op_struct::new(0xC8, "NORM"  )) }
        | tag_max!("OR"    ) => { |_| source_op::Instruction(op_struct::new(0x44, "OR"    )) }
        | tag_max!("RD"    ) => { |_| source_op::Instruction(op_struct::new(0xD8, "RD"    )) }
        | tag_max!("RMO"   ) => { |_| source_op::Instruction(op_struct::new(0xAC, "RMO"   )) }
        | tag_max!("RSUB"  ) => { |_| source_op::Instruction(op_struct::new(0x4C, "RSUB"  )) }
        | tag_max!("SHIFTL") => { |_| source_op::Instruction(op_struct::new(0xA4, "SHIFTL")) }
        | tag_max!("SHIFTR") => { |_| source_op::Instruction(op_struct::new(0xA8, "SHIFTR")) }
        | tag_max!("SIO"   ) => { |_| source_op::Instruction(op_struct::new(0xF0, "SIO"   )) }
        | tag_max!("SSK"   ) => { |_| source_op::Instruction(op_struct::new(0xEC, "SSK"   )) }
        | tag_max!("STA"   ) => { |_| source_op::Instruction(op_struct::new(0x0C, "STA"   )) }
        | tag_max!("STB"   ) => { |_| source_op::Instruction(op_struct::new(0x78, "STB"   )) }
        | tag_max!("STCH"  ) => { |_| source_op::Instruction(op_struct::new(0x54, "STCH"  )) }
      //| tag_max!("STF"   ) => { |_| source_op::Instruction(op_struct::new(0x80, "STF"   )) }
        | tag_max!("STI"   ) => { |_| source_op::Instruction(op_struct::new(0xD4, "STI"   )) }
        | tag_max!("STL"   ) => { |_| source_op::Instruction(op_struct::new(0x14, "STL"   )) }
        | tag_max!("STS"   ) => { |_| source_op::Instruction(op_struct::new(0x7C, "STS"   )) }
        | tag_max!("STSW"  ) => { |_| source_op::Instruction(op_struct::new(0xE8, "STSW"  )) }
        | tag_max!("STT"   ) => { |_| source_op::Instruction(op_struct::new(0x84, "STT"   )) }
        | tag_max!("STX"   ) => { |_| source_op::Instruction(op_struct::new(0x10, "STX"   )) }
        | tag_max!("SUB"   ) => { |_| source_op::Instruction(op_struct::new(0x1C, "SUB"   )) }
      //| tag_max!("SUBF"  ) => { |_| source_op::Instruction(op_struct::new(0x5C, "SUBF"  )) }
        | tag_max!("SUBR"  ) => { |_| source_op::Instruction(op_struct::new(0x94, "SUBR"  )) }
        | tag_max!("SVC"   ) => { |_| source_op::Instruction(op_struct::new(0xB0, "SVC"   )) }
        | tag_max!("TD"    ) => { |_| source_op::Instruction(op_struct::new(0xE0, "TD"    )) }
        | tag_max!("TIO"   ) => { |_| source_op::Instruction(op_struct::new(0xF8, "TIO"   )) }
        | tag_max!("TIX"   ) => { |_| source_op::Instruction(op_struct::new(0x2C, "TIX"   )) }
        | tag_max!("TIXR"  ) => { |_| source_op::Instruction(op_struct::new(0xB8, "TIXR"  )) }
        | tag_max!("WD"    ) => { |_| source_op::Instruction(op_struct::new(0xDC, "WD"    )) } 
            
        )
     >> (
         match mode {
             true => content.instr_long_mode(true),
             false => content
         }
        )
    )

);

named!(
    pub args(&[u8]) -> Vec<arg_struct>,
    do_parse!(
        many1!(alt!(tag!(" ") | tag!("\t")))
     >> a: separated_list!(
            delimited!(
                opt!(alt!(
                    tag!(" ") | tag!("\t")
                )), 
                tag!(","), 
                opt!(alt!(
                    tag!(" ") | tag!("\t")
                ))
            ),
            arg
        )
     >> (a)
    )
);
//trace_macros!(true);
named_args!(
    operation_string<'a>(mem_loc: &mut u32, line_no: &mut u32, sym_tab: &mut Symtab, err_vec: &mut Vec<Result<(), String> >)<&'a [u8], Line >,
    do_parse!(
        not!(tag!("\n"))
     >> many0!(
            alt_complete!(
                tag!(" ") | tag!("\t")
            )
        )
     >> not!(tag!("."))
     >> l: opt!(
            terminated!(
                label,
                many1!(
                    alt_complete!(
                        tag!(" ") | tag!("\t")
                    )
                )
            )
        )
     >> op: alt_complete!( asm_directive | instruction | value!(source_op::Error))
     >> a: alt_complete!(args | value!(Vec::new()))
     >> ({
            let mut res = Line::new().mem_loc(*mem_loc).line_no(*line_no).label(l);
            match op {
                source_op::Instruction(ref x) => {
                    err_vec.push(add_to_symtab(&mut res, sym_tab, (false, "")));
                    if a.len() > 0 {
                        match a[0].val {
                            arg::Label(ref y) => match &*y.to_uppercase() {
                                "A" | "B" | "X" | "L" | "S" | "T" | "PC" | "SW" => {
                                    res = res.format(format::Register);
                                    *mem_loc += 2;
                                }
                                _ => {
                                    if x.long {
                                        res = res.format(format::Long);
                                        *mem_loc += 4;
                                        
                                    } else {
                                        res = res.format(format::Normal);
                                        *mem_loc += 3;
                                    }
                                }
                            },
                            _ => {
                                if x.long {
                                    res = res.format(format::Long);
                                    *mem_loc += 4;
                                    
                                } else {
                                    res = res.format(format::Normal);
                                    *mem_loc += 3;
                                }
                            }
                        }
                    } else {
                        if x.name == "RSUB" {
                            *mem_loc += 2;
                        }
                        res = res.format(format::Opless);
                        *mem_loc += 1;
                    }
                }
                source_op::Neh => res = res.format(format::Comment),
                source_op::Directive(ref x) => {
                    match x.name {
                        "BYTE" => {
                            err_vec.push(add_to_symtab(&mut res, sym_tab, (true, "The BYTE directive requires a label!")));
                            match a[0].val {
                                arg::StrLit(ref s) => *mem_loc += s.len() as u32,
                                arg::IntLit(_) => *mem_loc += a.len() as u32,
                                _ => err_vec.push(Err("The BYTE directive does not accept labels as arguments!".to_owned())),
                            };
                        
                        }
                        "WORD" => {
                            err_vec.push(add_to_symtab(&mut res, sym_tab, (true, "The WORD directive requires a label!")));
                            match a[0].val {
                                arg::StrLit(ref s) => *mem_loc += 3 * s.len() as u32,
                                arg::IntLit(_) => *mem_loc += 3 * a.len() as u32,
                                _ => err_vec.push(Err("The WORD directive does not accept labels as arguments!".to_owned())),
                            }

                        } 
                        "RESB" => {
                            err_vec.push(add_to_symtab(&mut res, sym_tab, (true, "The RESB directive requires a label!")));
                            match a[0].val {
                                arg::IntLit(ref x) => *mem_loc += *x as u32,
                                _ => err_vec.push(Err("The RESB directive does not accept labels as arguments!".to_owned())),
                            }

                        }
                        "RESW" => {
                            err_vec.push(add_to_symtab(&mut res, sym_tab, (true, "The RESW directive requires a label!")));
                            match a[0].val {
                                arg::IntLit(ref x) => *mem_loc += 3 * *x as u32,
                                _ => err_vec.push(Err("The RESW directive does not accept labels as arguments!".to_owned())),
                            }

                        }
                        _      =>  {
                            match res.label {
                                Some(ref x) => if x.len() > 0 { err_vec.push(add_to_symtab(&mut res, sym_tab, (false, ""))) },
                                None        => err_vec.push(add_to_symtab(&mut res, sym_tab, (false, "")))
                            }
                        }
                    }
                }
                source_op::Error => err_vec.push(Err("Invalid opcode in this line!".to_owned()))
            }
            res.args(a).operation(op)
        }) 
    )
);



// "rsub\n"
named_args!(
    pub statement<'a>(mem_loc: &mut u32, line_no: &mut u32, sym_tab: &mut Symtab, err_vec: &mut Vec<Result<(), String> >)<&'a [u8], Line>,
    do_parse!(
        x: opt!(
            call!(operation_string, mem_loc, {*line_no += 1; line_no}, sym_tab, err_vec)
        )
     >> c: opt!(comment)
     >> ({
            let mut temp = match x {
                Some(l) => l,
                None => Line::new().format(format::Comment)
            };

            temp = match c {
                Some(c) => temp.comment(&str::from_utf8(c).unwrap()),
                None => temp
            };

            temp
        })
    )
);

named!(
    pub comment(&[u8]) -> &[u8],
    do_parse!(
        many0!(
            alt!(
                tag!(" ") | tag!("\t")
            )
        )
     >> tag!(".")
     >> content: take_until!("\n")
     >> (content)
    )
);

trace_macros!(false);