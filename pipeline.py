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
import quality

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
graph_enum = "../ims_analysis/main.native"
try:
    with open("pipeline.json", "r") as f:
        json = json.load(f)
        graph_exec = json.get("audiograph", graph_exec)
        graph_enum = json.get("enumerate", graph_enum)
except:
    print("No specific paths provided.")
finally:
    print("Graph execution with: ", graph_exec)
    print("Graph enumeration with: ", graph_enum)

if args.graph:
    print("Processing graphs: ", args.graph)
elif args.nodes:
    print("Processing all graphs ", end="")
    if args.all:
        print("up to size ", args.nodes)
    else:
        print("of size ", args.nodes)
