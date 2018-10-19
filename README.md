# rusty-sic-xe
An SIC XE assembler written in Rust.

This software was made as a project for the 5th semester course in Systems Programming as part of the B.E. Information Science degree at the National Institute of Engineering, Mysore.

This SIC/XE assembler has the following features-

* Generates position independent code through modification records.
* Correctly parses all SIC/XE instructions apart from floating point instructions.
* Generates an intermediate file listing each line and the generated object code for that line.
* Outputs object code in ASCII for easy readability.
* As of now, this project is in a stable state, but I intend to implement more features.

## Features to be implemented

* Program blocks
* Control sections
* Literals
* Constant expressions
* Full SIC/XE instruction set support
* External references