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

from pyparsing import *

def trim(s, l, tokens):
    #print("tok:", tokens[0])
    return [tokens[0].strip()]

#insideBlock = CharsNotIn("{}").setParseAction(trim)
insideBlock = CharsNotIn("{}")
block_parser = nestedExpr('{','}', content=insideBlock)



def extension(path):
    return os.path.splitext(path)[1]

def without_ext(path):
    return os.path.splitext(os.path.basename(path))[0]

def show_tokens(s):
    tokens = block_parser.lex(s)
    for tok in tokens:
        print(tok.type, "(", tok, ")")

# Convert faust files to rust
def convert_file(filename, struct_name):
    proc = subprocess.run(["faust", "-lang", "rust", filename, "-cn", struct_name], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    return proc.stdout#This is the rust content

def to_audioeffect(parse_tree):
    """Generate the audio effect"""
    #Filter out what we don't need
    i = 0
    nb_uplevel_tok = len(parse_tree)
    tok = parse_tree
    while i < nb_uplevel_tok:
        if "fn metadata" in tok[i] or "fn buildUserInterface" in tok[i] or "fn instanceResetUserInterface" in tok[i]:
            del tok[i]
            del tok[i+1]
            i = i + 2
        elif isinstance(tok[i], list):
            pass

def process(filename):
    struct_name = without_ext(filename)
    rust_file = convert_file(filename)
    parse_tree = block_parser.parseString("{" + rust_file + "}")
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
            parse_tree = block_parser.parseFile(args.name)
            print(parse_tree)
        else:
            print(args.name, " is not a known format file.")
            exit(-1)
    else:
        print(args.name, " does not exist")
        exit(-1)


    # Generate the audio effect
