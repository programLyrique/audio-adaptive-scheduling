#!/usr/bin/python3

"""
Execute all the stuff:
- generating graphs (enumeration for a given size for instance or for a given non-degraded graph)
- executing graph
- comparing graphs

Dependencies:
pip3 install tqdm numpy matlpotlib adjusttext
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
from adjustText import adjust_text
from operator import itemgetter

from scipy import stats


parser = argparse.ArgumentParser(description="Generate graphs, execute them, and then evaluate their quality", \
    epilog="Please indicate in a pipeline.json file where the process thqt executes graphs is located.")
group = parser.add_mutually_exclusive_group(required=True)
group.add_argument("-g", "--graph", help="Specify non-degraded graphs to explore the quality of.", action="append")
group.add_argument("-n", "--nodes", help="Explore all graphs of size nodes", type=int)
parser.add_argument("-a", "--all", help="Explore all sizes up to the one precised by --nodes", action="store_true")
parser.add_argument("-d", "--draw", help="Draw graph of quality and cost.", action="store_true")
parser.add_argument("--only-draw", help="Only draws graph", action="store_true")
parser.add_argument("--dir", help="Directory where to process")

args = parser.parse_args()

# We indicate where the process used to execute the graphs
# and the one to generate degraded versions
graph_exec = "./target/release/audiograph"
graph_enum = "../ims-analysis/main.native"
nodes_dic = "../ims-analysis/nodes.ag"
try:
    with open("pipeline.json", "r") as f:
        json = json.load(f)
        graph_exec = json.get("audiograph", graph_exec)
        nodes_dic = json.get("nodes", nodes_dic)
        graph_enum = json.get("enumerate", graph_enum)
except:
    print("No specific paths provided.")
finally:
    graph_exec = os.path.abspath(graph_exec)
    graph_enum = os.path.abspath(graph_enum)
    nodes_dic = os.path.abspath(nodes_dic)
    print("Graph execution with: ", graph_exec)
    print("Graph enumeration with: ", graph_enum)
    print("Using node dictionary: ", nodes_dic)

def get_costs(csvname):
    """Get csv file with execution times and compute average execution time
        for a cycle as well as total execution time"""
    with open(csvname, "r") as csvfile:
        csvfile.readline() # To remove first line where there are number of nodes and edges
        csvreader = csv.DictReader(csvfile, delimiter='\t')
        total=0.
        nb_rows=0
        for row in csvreader:
            total += float(row["Execution time"])
            nb_rows += 1
        return total / float(nb_rows), total

def execute_graph(graph):
    subprocess.run([graph_exec, "-m", "-b", "-c", "60000", graph], check=True)
    # Get execution times for reports (-m option)
    basename,_ = os.path.splitext(os.path.basename(graph))
    reports = glob.glob("*"+basename + "*.csv")
    reports.sort(reverse=True, key= lambda f: os.path.getmtime(f))
    csvfile = reports[0]
    tqdm.write("Retrieving monitoring info from "+ csvfile)
    return get_costs(csvfile)

def compare_audio_files(audiofiles):
    audiofiles.sort()# Number 0 is always the non-degraded file
    non_degraded = audiofiles.pop(0)
    tqdm.write("Non degraded file is: " + non_degraded)
    tqdm.write("Comparing degraded versions with non-degraded one.")
    y_nd,sr_nd = quality.load_file(non_degraded, duration=2)
    qualities = {}
    for degraded in tqdm(audiofiles):
        y,sr = quality.load_file(degraded, duration=2)
        basename,_ = os.path.splitext(degraded)
        qualities[basename] = quality.compare_specto(y_nd, sr_nd, y, sr)
    return qualities

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
    subprocess.run([graph_enum, "-w", "-x", "-edr", graph, "--node-file="+nodes_dic], check=True)
    costs={}
    tqdm.write("Executing graphs")
    for graph in tqdm(glob.iglob("*.ag")):
        tqdm.write(graph)
        execute_graph(graph, costs)
    # Get resulting audiofiles
    audiofiles = glob.glob("*.wav")
    qualities = compare_audio_files(audiofiles)
    return qualities, costs

class GraphResults:
    def __init__(self, name):
        self.name = name
        self.costs=None
        self.quality=None

def process_all_graphs(nb_nodes, dirname):
    """Process on all weakly connected Dags up to nb_nodes"""
    tqdm.write("Enumerating weakily DAGs up to " + str(nb_nodes) + " nodes with result in " + dirname)
    #./main.native -dewx -n 5 --node-file ../nodes.ag
    subprocess.run([graph_enum, "-dewxr",  "-n", str(nb_nodes), "--node-file="+nodes_dic], check=True)

    # Getting the costs
    results={}
    tqdm.write("Executing graphs")
    #Group them by non-degraded graphs
    for non_degraded_graph in tqdm(glob.iglob("*-0.ag")):
        # Get the prefix for this graph
        prefix = non_degraded_graph.rsplit("-", maxsplit=1)[0]
        tqdm.write(prefix)
        results[prefix] = []
        for graph in tqdm(glob.iglob(prefix+"*.ag")):
            basename,_ = os.path.splitext(graph)
            result= GraphResults(basename)
            costs = execute_graph(graph)
            result.costs = costs
            results[prefix].append(result)
        # Get audio files
        audiofiles = glob.glob(prefix+"*.wav")
        qualities = compare_audio_files(audiofiles)

        # Update results
        for result in results[prefix]:
            #If we've not computed a quality for it, it is the non-degraded graph
            result.quality = qualities.get(result.name, 1.0)

    #TODO: compute correlation between costs for theoretical and experimental models
    # Idem for qualities
    # And then correlation between costs and qualities
    # rather put that in the results
    return results


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

def load_csv(filename):
    """Load quality and cost from the theoretical model"""
    with open(filename, "r", newline='') as csvfile:
        csvreader = csv.DictReader(csvfile, delimiter='\t')
        qualities=[]
        costs=[]
        for row in csvreader:
            qualities.append(float(row["Quality"]))
            costs.append(float(row["Cost"]))
        return qualities, costs

def sort_by_quality(qualities, costs):
    return [[*x] for x in zip(*sorted(zip(qualities, costs), key=itemgetter(0)))]


def q_c_dict_to_list(qualities, costs):
    "Converts the dict of qualities and costs to lists"
    q = []
    c_cycle = []
    # graph 0
    c_cycle.append
    q.append(1.)
    name = list(sorted(costs.keys()))[0]
    cost, total = costs[name]
    c_cycle.append(cost)
    # Then, from graph 1
    for k in sorted(qualities.keys()):
        q.append(qualities[k])
        cycle, _ = costs[k]
        c_cycle.append(cycle)
        #name = k.split("-")[-1]

    return q, c_cycle

def plot(qualities_mes, costs_mes, qualities_th, costs_th):
    fig, axes = plt.subplots(2,1)

    ax1= axes[0]

    texts_mes= []
    for (i, (quality, cost)) in enumerate(zip(qualities_mes, costs_mes)):
        texts_mes.append(ax1.text(quality, cost, str(i), ha='center', va='center'))

    qualities_mes,costs_mes = sort_by_quality(qualities_mes, costs_mes)

    #print("Measured: ", q, c_cycle)

    color='tab:red'

    ax1.set_ylabel("cost per cycle (µs)")
    ax1.set_xlabel("quality")
    ax1.scatter(qualities_mes, costs_mes,  label="Measured", color=color)
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.grid(True)

    # We will rather use a distance? Such as Kendall Tau... But not a correlation.
    #rho_s,p_s = stats.spearmanr(q, c_cycle)
    #tau_k, p_k = stats.kendalltau(q, c_cycle)
    #tau_w, p_w = stats.weightedtau(q, c_cycle)

    #print("Spearman: rho=", rho_s, ", p=", p_s)
    #print("KendallTau: rho=", tau_k, ", p=", p_k)
    #print("Weighted Kendall: rho=", tau_w, ", p=", p_w)

    ax2 = axes[1]

    texts_th = []
    for (i, (quality, cost)) in enumerate(zip(qualities_th, costs_th)):
        texts_th.append(ax2.text(quality, cost, str(i), ha='center', va='center'))

    color = 'tab:blue'
    ax2.set_ylabel("cost")
    ax2.set_xlabel("quality")

    qualities_th,costs_th = sort_by_quality(qualities_th, costs_th)

    ax2.scatter(qualities_th, costs_th,  label="Model", color=color)
    ax2.tick_params(axis='y', labelcolor=color)
    ax2.grid(True)


    adjust_text(texts_mes, ax=ax1)
    adjust_text(texts_th, ax=ax2)


    fig.tight_layout()
    fig.legend()
    plt.show()
    #print("limits are: x1=[", ax1.get_xlim(), "], x2=[", ax2.get_xlim(), "]")

if args.graph:
    for graph in tqdm(args.graph):
        q_mes=[]
        c_mes=[]
        basename,_ = os.path.splitext(os.path.basename(graph))# We stay in the directory created in process_graph
        if not args.only_draw:
            absgraph = os.path.abspath(graph)
            qualities,costs = process_graph(absgraph)
            tqdm.write("Qualities are " + str(qualities))
            tqdm.write("Costs are " + str(costs))
            results_to_csv(basename + "-exec-report", qualities, costs)
            q_mes, c_mes = q_c_dict_to_list(qualities, costs)
        if args.draw or args.only_draw:
            if args.only_draw:
                dirname = basename + "-degraded"
                os.chdir(dirname)
                q_mes, c_mes = load_csv(basename + "-exec-report.csv")
            q_th, c_th = load_csv(basename + "-theo.csv")
            # Display in a graph
            plot(q_mes, c_mes, q_th, c_th)
        os.chdir("..")
elif args.nodes:
    dirname= os.getcwd()
    if args.dir:
        dirname= args.dir
    try:
        os.mkdir(dirname)
    except:
        pass
    os.chdir(dirname)
    results = process_all_graphs(args.nodes, dirname)
    # Draw histogram of the correlations

    # Try also to do a Fisher transformation to get a better idea
    os.chdir("..")
