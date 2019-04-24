#!/usr/bin/python3

"""
Convert a faust effect to an audio effect (in Rust)

For one file or for a bunch of files (and put them all in one Rust file)
"""


import argparse
import subprocess
import os

parser = argprase.ArgumentParser(description="Convert a faust effect to an audio effect (in Rust)")

parser.add_argument("-d", "--directory", help="Process all Faust dsp files in the given directory and bundle them in one Rust file.")
parser.add_argument("-o", "--output", help="Name of the output file")


args = parser.parse_args()
