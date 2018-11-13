#![feature(trace_macros)]
#![recursion_limit="256"]

#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

mod nomparse;
mod line;

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile: File;
    let infilename: String;
    let outfile: File;
    let mut option = String::new();
    match args.len() {
        1 => {
            eprintln!("No input files specified, exiting.");
            println!("Proper syntax-
yacc.exe <input file name> [-text]

Options:
-text    - Generate object code as ASCII characters.\n");
            return
        }
        2 => {
            infilename = args[1].clone();
            infile = File::open(args.clone().pop().unwrap()).unwrap();
        }
        3 => {
            infilename = args[1].clone();
            option = args.iter().find(|x| x.contains('-')).unwrap().to_string();
            infile = File::open(infilename.clone()).unwrap();
        }
        _ => return
    }
    let input = BufReader::new(infile);
    let mut intfile = String::new();
    let mut parsed: File = File::create(infilename.clone() + "_out").unwrap();
    let mut err_vec: Vec<Result<(), String> > = Vec::new();
    let mut parse_vec: Vec<line::Line> = Vec::new();
    let mut curr_line = 0;
    let mut curr_mem_loc: u32 = 0u32;
    let mut sym_tab: line::Symtab = line::Symtab::new();
    for line in input.lines() {
        let res = nomparse::statement(
            &(line.unwrap() + "\n").as_bytes(),
            &mut curr_mem_loc,
            &mut curr_line,
            &mut sym_tab,
            &mut err_vec
        ).unwrap().1;

        parse_vec.push(res);

    }

    if err_vec.iter().map(|x| {
        match x { 
            Ok(_) => x, 
            Err(ref e) => { 
                eprintln!("{}", e);
                x 
            } 
        }
    }).any(|x| x.is_err()) {
        eprintln!("Errors found, exiting.");
        return
    }

    //println!("{:#?}", sym_tab);

    //for mut res in &mut parse_vec {
    //    nomparse::gen_obj_code(res, &mut sym_tab, &mut base);
    //    //write!(parsed, "{:<4}{:<8X}{:<8}{:<8}{:<8}{:<8}\n", res.line_no, res.mem_loc, res.label.clone().unwrap_or("".to_owned()), res.operation, display_vec(&res.args), display_vec_nums(&res.obj_code));
    //}
    //println!("{}", nomparse::gen_records(&mut parse_vec, &mut sym_tab, &mut intfile));

    use std::io::prelude::*;
    use std::ops::Deref;

    if option == "-text" {
        write!(parsed, "{}", nomparse::gen_records(&mut parse_vec, &mut sym_tab, &mut intfile));
    } else {
        parsed.write_all(nomparse::vec_gen_records(&mut parse_vec, &mut sym_tab, &mut intfile).deref());

    }

    let mut intermediate = File::create(infilename.clone() + "__intermediate").unwrap();
    write!(intermediate, "{}", intfile);

    // let mut curr = line::Line::new()
    // .operation(
    //     line::source_op::Instruction(
    //         line::op_struct { 
    //             opcode: 0x10,
    //             name: "STX",
    //             long: true
    //         }
    //     )
    // ).args(
    //     vec![
    //         line::arg_struct{ 
    //             val: line::arg::Label("LENGTH".to_owned()), 
    //             reg_code: 255, 
    //             modifier: line::addr_mod::Direct 
    //         }
    //     ]
    // ).line_no(30)
    // .mem_loc(0x104E)
    // .format(line::format::Long);

    // let mut sym_tab = line::Symtab::new();
    // sym_tab.insert("LENGTH".to_owned(), line::Pos {
    //     line_no: 38,
    //     mem_loc: 0x0127
    // });
    
    // nomparse::gen_obj_code(&mut curr, &mut sym_tab, 0xFFFFFFFF);
    // println!("{:#?}", curr);


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

    // let mut sym_tab = line::Symtab::new();
    // sym_tab.insert("LENGTH".to_owned(), line::Pos {
    //     line_no: 38,
    //     mem_loc: 0x0127,

    // });
    // let mut d = 0xFFFFFFFFu32;
    // nomparse::gen_obj_code(&mut curr, &mut sym_tab, &mut d);
    // println!("{:#?}", curr);
}