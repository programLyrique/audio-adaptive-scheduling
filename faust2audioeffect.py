#!/usr/bin/python3

"""
Convert a faust effect to an audio effect (in Rust)

For one file or for a bunch of files (and put them all in one Rust file)
"""


import argparse
import subprocess
import os
import os.path
from tqdm import tqdm
import glob

import lark
from lark import Lark

block_parser= Lark(r"""
    block: "{" decls* "}"

    pointer: "&" "mut"?

    array: "[" ident ";" /[1-9]{1}[0-9]*/ "]"
    slice: pointer "[" pointer? (ident | slice) "]"
    generic: "<" ident ">"
    rtype: pointer? ident generic? | array | slice
    typedecl: ident ":" rtype

    arg: rtype | typedecl
    args: (arg ",")* arg?
    rettype: "->" rtype

    //for: "for" ident "in" _string function_block

    //if: "if" "(" _ifcond ")" function_block ["else" function_block] ";"?

    //index: "[" INT "]"
    //assign: ident index? (":" rtype)? "=" expr
    //mut: "mut"
    //let: "let" mut? assign
    //return: "return" (ident | value) ";"?
    //statement: struct | match | let | return | assign | _line
    //expr: (if | value | ident | _line)+

    //function_block: "{" (statement ";" |  for | if )* statement? return? "}"
    function_block: "{" insideblock? | (insideblock? function_block insideblock?) "}"
    function: "pub"? "fn" ident "(" args ")" rettype? function_block

    //struct_decl: "pub"? "struct" ident "{" (typedecl ",")* "}"
    struct_decl: "pub"? "struct" ident function_block
    //array_val: "[" (SIGNED_INT |DECIMAL) ";" INT "]"
    //struct_elem: ident ":" (value | array_val) ["as" ident]
    //struct: ident "{" (struct_elem ",")* "}"

    //value: INT | SIGNED_INT | DECIMAL | ident | _ifcond

    //match_clause: (value | "_") "=>" (function_block | _string)
    //match: "match" "(" ident ")" "{" (match_clause ",")* "}"

    impl: "impl" ident block

    decls: function | struct_decl | impl

    start: decls*

    ident: /[a-zA-Z][a-zA-Z0-9_\.-]*/
    

    insideblock: /[^{}]+/
    //_string: /[^{};]+/
    //_line: /[^\n\t{};]+/
    //ifcond: /[^\n\t(){};]+/ | "(" ifcond ")" ifcond*

    %import common.INT
    %import common.SIGNED_INT
    %import common.DECIMAL
    %import common.LETTER
    %import common.DIGIT
    %import common.WS
    %ignore WS
""", debug=True)


def extension(path):
    return os.path.splitext(path)[1]

def show_tokens(s):
    tokens = block_parser.lex(s)
    for tok in tokens:
        print(tok.type, "(", tok, ")")

# Convert faust files to rust
def convert_file(filename):
    proc = subprocess.run(["faust", "-lang", "rust", filename], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    return proc.stdout#This is the rust content

def process(filename):
    rust_file = convert_file(filename)
    parse_tree = block_parser.parse(rust_file)
    print(parse_tree)

if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="Convert a faust effect to an audio effect (in Rust)")

    #parser.add_argument("-d", "--directory", help="Process all Faust dsp files in the given directory and bundle them in one Rust file.", action="store_true")
    parser.add_argument("-o", "--output", help="Name of the output file")
    parser.add_argument("name", help="Name of the Faust file or of the directory")

    args = parser.parse_args()

    if os.path.isdir(args.name):
        os.chdir(args.name)
        for file in tqdm(glob.iglob("*.dsp")):
            tqdm.write("Processing " + file)
            process(file)
    elif os.path.isfile(args.name):
        ext = extension(args.name)
        if ext == ".dsp":
            process(args.name)
        elif ext == ".rs":
            with open(args.name, "r") as f:
                parse_tree = block_parser.parse(f.read())
                print(parse_tree)
        else:
            print(args.name, " is not a known format file.")
            exit(-1)
    else:
        print(args.name, " does not exist")
        exit(-1)

    # Extract the relevant rust code (parsing with lark)

    # Generate the audio effect
