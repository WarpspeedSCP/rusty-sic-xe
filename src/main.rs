#![feature(trace_macros)]
#![recursion_limit="256"]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate quick_error;

mod nomparse;
mod assembler;
mod line;


fn main() {
    let input = "COPY   START  1000\nFIRST  STL    RETADR\nCLOOP  JSUB   RDREC\n       LDA    LENGTH\n       COMP   ZERO\n       JEQ    ENDFIL\n       JSUB   WRREC\n       J      CLOOP\nENDFIL LDA    EOF\n       STA    BUFFER\n       LDA    THREE\n       STA    LENGTH\n       JSUB   WRREC\n       LDL    RETADR\n       RSUB\nEOF    BYTE   C'EOF'\nTHREE  WORD   3\nZERO   WORD   0\nRETADR RESW   1\nLENGTH RESW   1\nBUFFER RESB   4096\n.\n.      SUBROUTINE TO READ RECORD INTO BUFFER\n.\nRDREC  LDX    ZERO\n       LDA    ZERO\nRLOOP  TD     INPUT\n       JEQ    RLOOP\n       RD     INPUT\n       COMP   ZERO\n       JEQ    EXIT\n       STCH   BUFFER,X\n       TIX    MAXLEN\n       JLT    RLOOP\nEXIT   STX    LENGTH\n       RSUB\nINPUT  BYTE   X'F1'\nMAXLEN WORD   4096\n.\n.      SUBROUTINE TO WRITE RECORD FROM BUFFER\n.\nWRREC  LDX    ZERO\nWLOOP  TD     OUTPUT\n       JEQ    WLOOP\n       LDCH   BUFFER,X\n       WD     OUTPUT\n       TIX    LENGTH\n       JLT    WLOOP\n       RSUB\nOUTPUT BYTE   X'06'\n       END    FIRST\n";
    let mut parsed: Vec<line::Line> = Vec::new();
    let mut err_vec: Vec<Result<(), String> > = Vec::new();
    let mut curr_line = 0;
    let mut curr_mem_loc: u32 = 0u32;
    let mut sym_tab: line::Symtab = line::Symtab::new();
    for line in input.lines() {
        parsed.push(
               nomparse::statement(
                   &(line.to_owned() + "\n").as_bytes(),
                   &mut curr_mem_loc,
                   &mut curr_line,
                   &mut sym_tab,
                   &mut err_vec
                   ).unwrap().1
        );

        println!("{:#?}", parsed.pop().unwrap()); // nomparse::statement(b"STCH   BUFFER, X\n"));
    }

    println!("{:#?}", err_vec);
    println!("{:#?}", sym_tab);



}

#[test]
fn tst() {

}