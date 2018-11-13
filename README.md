# rusty-sic-xe
An SIC XE assembler written in Rust.

This software was made as a project for the 5th semester course in Systems Software as a part of the B.E. Information Science degree at the National Institute of Engineering, Mysore.

This SIC/XE assembler has the following features-

* Generates position independent code through modification records.
* Correctly parses all SIC/XE instructions apart from floating point instructions.
* Generates an intermediate file listing each line and the generated object code for that line.
* Outputs object code in ASCII (through the `-text` option) for easy readability, as well as the default binary format.
* Allows definition of symbolic constants vis the `EQU` keyword.
* As of now, this project is in a stable state, but I intend to implement more features.

## Features to be implemented

* Program blocks
* Control sections
* Literals
* Constant expressions
* Full SIC/XE instruction set support
* External references


## References

* [nom reference pages](https://github.com/Geal/nom)
* [Rust language reference](https://doc.rust-lang.org)
* System Software: An Introduction to Systems Programming (3rd Edition) by Leland L. Beck