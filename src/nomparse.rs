use nom::*;
use std::str;
use super::line;

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
            tag!("x'"),
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
        a: take_while!(|c| is_alphabetic(c) | (c == '_' as u8))
     >> b: take_while!(|c| is_alphanumeric(c) | (c == '_' as u8))
     >> (make_str(a).to_owned() + make_str(b))
    )
);

named!(
    pub str_lit(&[u8]) -> String,
    do_parse!(
        val: delimited!(
            tag!("c'"),
            take_until!("'"),
            tag!("'")
        )
     >> (make_str(val).to_owned())
    )
);

named!(
    pub arg(&[u8]) -> line::arg,
    alt_complete!(
        num     => { |n| line::arg::IntLit(n) }
      | str_lit => { |s| line::arg::StrLit(s) }
      | label   => { |l| line::arg::Label(l)  }
    )
);

named!(
    pub asm_directive(&[u8]) -> line::source_op,
    alt_complete!(
        tag_no_case!("START") => { |_| line::source_op::directive(0x01) } 
      | tag_no_case!("END"  ) => { |_| line::source_op::directive(0x02) } 
      | tag_no_case!("BYTE" ) => { |_| line::source_op::directive(0x03) } 
      | tag_no_case!("WORD" ) => { |_| line::source_op::directive(0x04) } 
      | tag_no_case!("RESB" ) => { |_| line::source_op::directive(0x05) } 
      | tag_no_case!("RESW" ) => { |_| line::source_op::directive(0x06) }
    )
);

named!(
    pub instruction(&[u8]) -> line::source_op,
    alt_complete!(
        tag_no_case!("ADD"   ) => { |_| line::source_op::instruction(0x18) }
      | tag_no_case!("ADDF"  ) => { |_| line::source_op::instruction(0x58) }
      | tag_no_case!("ADDR"  ) => { |_| line::source_op::instruction(0x90) }
      | tag_no_case!("AND"   ) => { |_| line::source_op::instruction(0x40) }
      | tag_no_case!("CLEAR" ) => { |_| line::source_op::instruction(0xB4) }
      | tag_no_case!("COMP"  ) => { |_| line::source_op::instruction(0x28) }
      | tag_no_case!("COMPR" ) => { |_| line::source_op::instruction(0x88) }
      | tag_no_case!("DIV"   ) => { |_| line::source_op::instruction(0xA0) }
      | tag_no_case!("DIVF"  ) => { |_| line::source_op::instruction(0x24) }
      | tag_no_case!("DIVR"  ) => { |_| line::source_op::instruction(0x64) }
      | tag_no_case!("FIX"   ) => { |_| line::source_op::instruction(0xC4) }
      | tag_no_case!("FLOAT" ) => { |_| line::source_op::instruction(0xC0) }
      | tag_no_case!("HIO"   ) => { |_| line::source_op::instruction(0xF4) }
      | tag_no_case!("J"     ) => { |_| line::source_op::instruction(0x3C) }
      | tag_no_case!("JEQ"   ) => { |_| line::source_op::instruction(0x30) }
      | tag_no_case!("JLT"   ) => { |_| line::source_op::instruction(0x34) }
      | tag_no_case!("JGT"   ) => { |_| line::source_op::instruction(0x38) }
      | tag_no_case!("JSUB"  ) => { |_| line::source_op::instruction(0x48) }
      | tag_no_case!("LDA"   ) => { |_| line::source_op::instruction(0x00) }
      | tag_no_case!("LDB"   ) => { |_| line::source_op::instruction(0x68) }
      | tag_no_case!("LDCH"  ) => { |_| line::source_op::instruction(0x50) }
      | tag_no_case!("LDF"   ) => { |_| line::source_op::instruction(0x70) }
      | tag_no_case!("LDL"   ) => { |_| line::source_op::instruction(0x08) }
      | tag_no_case!("LDS"   ) => { |_| line::source_op::instruction(0x6C) }
      | tag_no_case!("LDT"   ) => { |_| line::source_op::instruction(0x74) }
      | tag_no_case!("LDX"   ) => { |_| line::source_op::instruction(0x04) }
      | tag_no_case!("LPS"   ) => { |_| line::source_op::instruction(0xD0) }
      | tag_no_case!("MUL"   ) => { |_| line::source_op::instruction(0x20) }
      | tag_no_case!("MULF"  ) => { |_| line::source_op::instruction(0x60) }
      | tag_no_case!("MULR"  ) => { |_| line::source_op::instruction(0x98) }
      | tag_no_case!("NORM"  ) => { |_| line::source_op::instruction(0xC8) }
      | tag_no_case!("OR"    ) => { |_| line::source_op::instruction(0x44) }
      | tag_no_case!("RD"    ) => { |_| line::source_op::instruction(0xD8) }
      | tag_no_case!("RMO"   ) => { |_| line::source_op::instruction(0xAC) }
      | tag_no_case!("RSUB"  ) => { |_| line::source_op::instruction(0x4C) }
      | tag_no_case!("SHIFTL") => { |_| line::source_op::instruction(0xA4) }
      | tag_no_case!("SHIFTR") => { |_| line::source_op::instruction(0xA8) }
      | tag_no_case!("SIO"   ) => { |_| line::source_op::instruction(0xF0) }
      | tag_no_case!("SSK"   ) => { |_| line::source_op::instruction(0xEC) }
      | tag_no_case!("STA"   ) => { |_| line::source_op::instruction(0x0C) }
      | tag_no_case!("STB"   ) => { |_| line::source_op::instruction(0x78) }
      | tag_no_case!("STCH"  ) => { |_| line::source_op::instruction(0x54) }
      | tag_no_case!("STF"   ) => { |_| line::source_op::instruction(0x80) }
      | tag_no_case!("STI"   ) => { |_| line::source_op::instruction(0xD4) }
      | tag_no_case!("STL"   ) => { |_| line::source_op::instruction(0x14) }
      | tag_no_case!("STS"   ) => { |_| line::source_op::instruction(0x7C) }
      | tag_no_case!("STSW"  ) => { |_| line::source_op::instruction(0xE8) }
      | tag_no_case!("STT"   ) => { |_| line::source_op::instruction(0x84) }
      | tag_no_case!("STX"   ) => { |_| line::source_op::instruction(0x10) }
      | tag_no_case!("SUB"   ) => { |_| line::source_op::instruction(0x1C) }
      | tag_no_case!("SUBF"  ) => { |_| line::source_op::instruction(0x5C) }
      | tag_no_case!("SUBR"  ) => { |_| line::source_op::instruction(0x94) }
      | tag_no_case!("SVC"   ) => { |_| line::source_op::instruction(0xB0) }
      | tag_no_case!("TD"    ) => { |_| line::source_op::instruction(0xE0) }
      | tag_no_case!("TIO"   ) => { |_| line::source_op::instruction(0xF8) }
      | tag_no_case!("TIX"   ) => { |_| line::source_op::instruction(0x2C) }
      | tag_no_case!("TIXR"  ) => { |_| line::source_op::instruction(0xB8) }
      | tag_no_case!("WD"    ) => { |_| line::source_op::instruction(0xDC) } 
    )
);

named!(
    pub args(&[u8]) -> Vec<line::arg>,
    do_parse!(
        q: opt!(
            do_parse!(
                s: arg
             >> d: opt!(
                    do_parse!(
                        tag!(",")
                     >> opt!(
                            many0!(
                                alt!(
                                    tag!(" ") | tag!("\t")
                                )
                            )
                        )
                     >> a:arg
                     >> (a)
                    )
                )
             >> (
                    match d {
                        Some(d) => vec![s, d],
                        None    => match s {
                            line::arg::Label(ref x)  => if x.len() > 0 {
                                vec![s]
                            } else {
                                Vec::new()
                            },
                            line::arg::StrLit(ref x) => if x.len() > 0 {
                                vec![s]
                            } else {
                                Vec::new()
                            },
                            _                    => vec![s]
                        }
                    }
                )
            )
        )
     >> (
            match q {
                Some(q) => q,
                None => Vec::new()
            }
        )
    )
);

named!(
    pub statement(&[u8]) -> line::Line,
        do_parse!(
            l:      delimited!(
                        many0!(
                            alt!(
                                tag!(" ") | tag!("\t")
                            )
                        ),
                        opt!(label),
                        many0!(
                            alt!(
                                tag!(" ") | tag!("\t")
                            )
                        )
                    )
         >> op:     delimited!(
                        many0!(
                            alt!(
                                tag!(" ") | tag!("\t")
                            )
                        ),
                        alt_complete!( asm_directive | instruction ),
                        many0!(
                            alt!(
                                tag!(" ") | tag!("\t")
                            )
                        )
                    )
         
         >> a:      args
         >> (line::Line::new().label(l).operation(op).args(a))
        )
);

//trace_macros!(false);