#![feature(trace_macros)]

#[macro_use]
extern crate nom;

mod nomparse;
//mod tookens;
mod line;


fn main() {
    println!("{:#?}", nomparse::statement(b"abc add \n"));
}


#[test]
fn tst() {    
    assert_eq!(nomparse::num(b"x'10'").unwrap().1, 0x10);
    assert_eq!(nomparse::num(b"16 ").unwrap().1, 0x10);
    assert_eq!(nomparse::num(b"10h").unwrap().1, 0x10);

    assert_eq!(nomparse::arg(b"herp ").unwrap().1, line::arg::Label("herp".to_owned()));
    assert_eq!(nomparse::arg(b"c'herp'").unwrap().1, line::arg::StrLit("herp".to_owned()));
    assert_eq!(nomparse::asm_directive(b"start").unwrap().1, line::source_op::directive(0x01));
    assert_eq!(nomparse::instruction(b"add").unwrap().1, line::source_op::instruction(0x18));

    
}