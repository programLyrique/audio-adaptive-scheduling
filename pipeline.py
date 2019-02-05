#!/usr/bin/python3

"""
Execute all the stuff:
- generating graphs (enumeration for a given size for instance or for a given non-degraded graph)
- executing graph
- comparing graphs
"""

import argparse
import subprocess
import json
import os
import quality
from tqdm import tqdm
import glob


parser = argparse.ArgumentParser(description="Generate graphs, execute them, and then evaluate their quality", \
    epilog="Please indicate in a pipeline.json file where the process thqt executes graphs is located.")
group = parser.add_mutually_exclusive_group(required=True)
group.add_argument("-g", "--graph", help="Specify non-degraded graphs to explore the quality of.", action="append")
group.add_argument("-n", "--nodes", help="Explore all grqphs of size nodes", type=int)
parser.add_argument("-a", "--all", help="Explore all sizes up to the one precised by --nodes", action="store_true")

args = parser.parse_args()

# We indicate where the process used to execute the graphs
# and the one to generate degraded versions
graph_exec = "./target/release/audiograph"
graph_enum = "../ims-analysis/main.native"
try:
    with open("pipeline.json", "r") as f:
        json = json.load(f)
        graph_exec = json.get("audiograph", graph_exec)
        graph_enum = json.get("enumerate", graph_enum)
except:
    print("No specific paths provided.")
finally:
    graph_exec = os.path.abspath(graph_exec)
    graph_enum = os.path.abspath(graph_enum)
    print("Graph execution with: ", graph_exec)
    print("Graph enumeration with: ", graph_enum)


def process_graph(graph):
    tqdm.write("Processing " + graph, end=" ")
    basename,ext = os.path.splitext(graph)
    dirname = basename + "-degraded"
    tqdm.write("with results in " + dirname)
    try:
        os.mkdir(dirname)
    except:
        pass
    os.chdir(dirname)
    tqdm.write("Enumerating degraded versions")
    subprocess.run([graph_enum, "-w", "-x", "-e", graph])
    tqdm.write("Executing graphs")
    for graph in tqdm(glob.iglob("*.ag")):
        subprocess.run([graph_exec, "-m", "-b", graph])
    # Get resulting audiofiles
    audiofiles = glob.glob("*.wav")
    non_degraded = audiofiles.pop(0)
    tqdm.write("Non degraded file is: " + non_degraded)
    tqdm.write("Comparing degraded versions with non-degraded one.")
    y_nd,sr_nd = quality.load_file(non_degraded)
    qualities = {}
    for degraded in tqdm(audiofiles):
        y,sr = quality.load_file(degraded)
        qualities[degraded] = quality.compare_specto(y_nd, sr_nd, y, sr)
    os.chdir("..")
    qualities

if args.graph:
    for graph in tqdm(args.graph):
        absgraph = os.path.abspath(graph)
        qualities = process_graph(absgraph)
        tqdm.write("Qualities are " + str(qualities))
elif args.nodes:
    print("Processing all graphs ", end="")
    if args.all:
        print("up to size ", args.nodes)
    else:
        print("of size ", args.nodes)
