#![feature(trace_macros)]
#![recursion_limit="256"]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate bitflags;

use std::io::Write;
use std::fs::File;

mod nomparse;
mod line;

use self::line::display_vec;


fn main() {
    // let input = "\nCOPY   START  1000\nFIRST  STL    RETADR\nCLOOP  JSUB   RDREC\n       LDA    LENGTH\n       COMP   ZERO\n       JEQ    ENDFIL\n       JSUB   WRREC\n       J      CLOOP\nENDFIL LDA    EOF\n       STA    BUFFER\n       LDA    THREE\n       STA    LENGTH\n       JSUB   WRREC\n       LDL    RETADR\n       RSUB\nEOF    BYTE   C'EOF'\nTHREE  WORD   3\nZERO   WORD   0\nRETADR RESW   1\nLENGTH RESW   1\nBUFFER RESB   4096\n.\n.      SUBROUTINE TO READ RECORD INTO BUFFER\n.\nRDREC  LDX    ZERO\n       LDA    ZERO\nRLOOP  TD     INPUT\n       JEQ    RLOOP\n       RD     INPUT\n       COMP   ZERO\n       JEQ    EXIT\n       STCH   BUFFER,X\n       TIX    MAXLEN\n       JLT    RLOOP\nEXIT   STX    LENGTH\n       RSUB\nINPUT  BYTE   X'F1'\nMAXLEN WORD   4096\n.\n.      SUBROUTINE TO WRITE RECORD FROM BUFFER\n.\nWRREC  LDX    ZERO\nWLOOP  TD     OUTPUT\n       JEQ    WLOOP\n       LDCH   BUFFER,X\n       WD     OUTPUT\n       TIX    LENGTH\n       JLT    WLOOP\n       RSUB\nOUTPUT BYTE   X'06'\n       END    FIRST\n";
    // let mut parsed: File = File::create("herp").unwrap();
    // let mut err_vec: Vec<Result<(), String> > = Vec::new();
    // let mut parse_vec: Vec<line::Line> = Vec::new();
    // let mut curr_line = 0;
    // let mut curr_mem_loc: u32 = 0u32;
    // let mut sym_tab: line::Symtab = line::Symtab::new();
    // for line in input.lines() {
    //     let res = nomparse::statement(
    //         &(line.to_owned() + "\n").as_bytes(),
    //         &mut curr_mem_loc,
    //         &mut curr_line,
    //         &mut sym_tab,
    //         &mut err_vec
    //     ).unwrap().1;
    //     println!("{:#?}", res);
    //                      //line_no memloc label opcode args
    //     write!(parsed, "{:<4}{:<8X}{:<8}{:<8}{:<8}{:?}\n", res.line_no, 0x1000 + res.mem_loc, res.label.clone().unwrap_or("".to_owned()), res.operation, display_vec(&res.args), res.format);
    //     parse_vec.push(res);

    // }

    // println!("{:#?}", err_vec);
    // println!("{:#?}", sym_tab);


    let mut curr = line::Line::new()
    .operation(
        line::source_op::Instruction(
            line::op_struct { 
                opcode: 0x10,
                name: "STX",
                long: true
            }
        )
    ).args(
        vec![
            line::arg_struct{ 
                val: line::arg::Label("LENGTH".to_owned()), 
                reg_code: 255, 
                modifier: line::addr_mod::Direct 
            }
        ]
    ).line_no(30)
    .mem_loc(0x104E)
    .format(line::format::Long);

    let mut sym_tab = line::Symtab::new();
    sym_tab.insert("LENGTH".to_owned(), line::Pos {
        line_no: 38,
        mem_loc: 0x0127
    });
    
    nomparse::gen_obj_code(&mut curr, &mut sym_tab, 0xFFFFFFFF);
    println!("{:#?}", curr);


}

#[test]
fn tst() {
    let mut curr = line::Line::new()
    .operation(
        line::source_op::Instruction(
            line::op_struct { 
                opcode: 0x10,
                name: "STX",
                long: true
            }
        )
    ).args(
        vec![
            line::arg_struct{ 
                val: line::arg::Label("LENGTH".to_owned()), 
                reg_code: 255, 
                modifier: line::addr_mod::Direct 
            }
        ]
    ).line_no(30)
    .mem_loc(0x104E)
    .format(line::format::Long);

    let mut sym_tab = line::Symtab::new();
    sym_tab.insert("LENGTH".to_owned(), line::Pos {
        line_no: 38,
        mem_loc: 0x0127
    });

    nomparse::gen_obj_code(&mut curr, &mut sym_tab, 0xFFFFFFFF);
    println!("{:#?}", curr);
}