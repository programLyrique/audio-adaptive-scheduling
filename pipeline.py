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
import csv
import matplotlib.pyplot as plt
import numpy as np


parser = argparse.ArgumentParser(description="Generate graphs, execute them, and then evaluate their quality", \
    epilog="Please indicate in a pipeline.json file where the process thqt executes graphs is located.")
group = parser.add_mutually_exclusive_group(required=True)
group.add_argument("-g", "--graph", help="Specify non-degraded graphs to explore the quality of.", action="append")
group.add_argument("-n", "--nodes", help="Explore all grqphs of size nodes", type=int)
parser.add_argument("-a", "--all", help="Explore all sizes up to the one precised by --nodes", action="store_true")
parser.add_argument("-d", "--draw", help="Draw graph of quality and cost.", action="store_true")

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

def get_costs(csvname):
    """Get csv file with execution times and compute average execution time
        for a cycle as well as total execution time"""
    with open(csvname, "r") as csvfile:
        csvfile.readline() # To remove first line where there are number of ndoes and edges
        csvreader = csv.DictReader(csvfile, delimiter='\t')
        total=0.
        nb_rows=0
        for row in csvreader:
            total += float(row["Execution time"])
            nb_rows += 1
        return total / float(nb_rows), total



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
    # Export both as .ag and as .dot
    subprocess.run([graph_enum, "-w", "-x", "-edr", graph], check=True)
    costs={}
    tqdm.write("Executing graphs")
    for graph in tqdm(glob.iglob("*.ag")):
        tqdm.write(graph)
        subprocess.run([graph_exec, "-m", "-b", graph], check=True)
        # Get execution times for reports (-m option)
        basename,_ = os.path.splitext(os.path.basename(graph))
        reports = glob.glob("*"+basename + "*.csv")
        reports.sort(reverse=True, key= lambda f: os.path.getmtime(f))
        csvfile = reports[0]
        tqdm.write("Retrieving monitoring info from "+ csvfile)
        costs[basename] = get_costs(csvfile)
    # Get resulting audiofiles
    audiofiles = glob.glob("*.wav")
    audiofiles.sort()# Number 0 is always the non-degraded file
    tqdm.write(str(audiofiles))
    non_degraded = audiofiles.pop(0)
    tqdm.write("Non degraded file is: " + non_degraded)
    tqdm.write("Comparing degraded versions with non-degraded one.")
    y_nd,sr_nd = quality.load_file(non_degraded)
    qualities = {}
    for degraded in tqdm(audiofiles):
        y,sr = quality.load_file(degraded)
        basename,_ = os.path.splitext(degraded)
        qualities[basename] = quality.compare_specto(y_nd, sr_nd, y, sr)
    # Get execution time
    execFiles = glob.glob("*.csv")

    return qualities, costs

def results_to_csv(graphname, qualities, costs):
    fieldnames=["Quality", "Cost", "Total"]
    with open(graphname+".csv", "w", newline='') as csvfile:
        result={}
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames, delimiter='\t')
        writer.writeheader()
        #Get cost original graph
        name = list(sorted(costs.keys()))[0]
        cost, total = costs[name]
        result["Quality"] = 1.0
        result["Cost"] = cost
        result["Total"] = total
        writer.writerow(result)
        for graph in qualities.keys():
            result={}
            result["Quality"] = qualities[graph]
            cost, total = costs[graph]
            result["Cost"] = cost
            result["Total"] = total
            writer.writerow(result)

def plot(qualities, costs):
    q = []
    c_cycle = []
    c_total = []
    for k in sorted(qualities.keys()):
        q.append(qualities[k])
        cycle, total = costs[k]
        c_cycle.append(cycle)
        c_total.append(total)
    q.append(1.)
    name = list(sorted(costs.keys()))[0]
    cost, total = costs[name]
    c_cycle.append(cost)
    c_total.append(total)
    plt.plot(q, c_cycle)
    #plt.plot(q, c_total)
    plt.ylabel("cost per cycle (ms)")
    plt.xlabel("quality")
    plt.show()

if args.graph:
    for graph in tqdm(args.graph):
        absgraph = os.path.abspath(graph)
        qualities,costs = process_graph(absgraph)
        tqdm.write("Qualities are " + str(qualities))
        tqdm.write("Costs are " + str(costs))
        basename,_ = os.path.splitext(os.path.basename(graph))# We stay in the directory created in process_graph
        results_to_csv(basename + "-exec-report", qualities, costs)
        # Display in a graph
        plot(qualities, costs)
        os.chdir("..")
elif args.nodes:
    print("Processing all graphs ", end="")
    if args.all:
        print("up to size ", args.nodes)
    else:
        print("of size ", args.nodes)
