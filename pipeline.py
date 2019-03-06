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
        print("Loading pipeline.json")
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
    tqdm.write("Executing graph " + graph)
    subprocess.run([graph_exec, "-m", "-b", "-c", "10000", graph], check=True)
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
    tqdm.write("Degraded files are: " + str(audiofiles))
    tqdm.write("Comparing degraded versions with non-degraded one.")
    y_nd,sr_nd = quality.load_file(non_degraded, duration=2)
    qualities = {}
    for degraded in tqdm(sorted(audiofiles)):
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
        basename,_ = os.path.splitext(os.path.basename(graph))
        costs[basename] = execute_graph(graph)
    # Get resulting audiofiles
    audiofiles = glob.glob("*.wav")
    qualities = compare_audio_files(audiofiles)
    return qualities, costs

class GraphResults:
    def __init__(self, name):
        self.name = name
        self.costs=None
        self.quality=None

    def __repr__(self):
        return "{}: {}, {}".format(self.name, self.costs, self.quality)

def process_all_graphs(nb_nodes, dirname):
    """Process on all weakly connected Dags up to nb_nodes"""
    tqdm.write("Enumerating weakily DAGs up to " + str(nb_nodes) + " nodes with result in " + dirname)
    #./main.native -dewx -n 5 --node-file ../nodes.ag
    subprocess.run([graph_enum, "-dewxr",  "-n", str(nb_nodes), "--node-file="+nodes_dic], check=True)

    results={}
    tqdm.write("Executing graphs")
    #Group them by non-degraded graphs
    for non_degraded_graph in tqdm(glob.iglob("*-0.ag")):
        # Get the prefix for this graph
        prefix = non_degraded_graph.rsplit("-", maxsplit=1)[0]
        tqdm.write(prefix)
        result_graph=[]
        for graph in tqdm(sorted(glob.iglob(prefix+"*.ag"))):
            basename,_ = os.path.splitext(graph)
            result= GraphResults(basename)
            costs = execute_graph(graph)
            result.costs = costs
            result_graph.append(result)

        # We also want to get the following measures:
        # - are the worst/best graphs in terms of costs and quality the same in
        #   the theoretical models and in the experiments. How close are they in both vectors? (in inversions? In position distances?)
        # - are costs and qualities correlated? In the experimental model first. And in the theoretical one? (We could even prove it)
        # - are all the degraded graphs faster than the non-degraded one? And at least one? How many? Which percentage?
        # Shape questions:
        # - how many degraded graphs in average for one graph?
        # - how many resamplers have been inserted? Downsamplers? Upsamplers?
        # TODO later: try to degrade in same order as heuristics and see if it correlates with the order in quality and in cost
        # TODO: case of a source => use a real audio file? Or generate a sin wave? Or just noise? Or don't generate sources here?
        # Because for now, sources just output a 0 signal, so we get the same quality for each version and
        # it does not give an useful ranking for the measured quality.
        # TODO: apply merge operation for resampler (the one that inserts a mixer and then a resampler instead of several resamplers)

        # Meaningless to compute rank correlation on a vector of size 1
        if len(result_graph) > 1:
            # Get audio files
            audiofiles = glob.glob(prefix+"*.wav")
            qualities = compare_audio_files(audiofiles)

            # For the correlation, we want the graphs in increasing rank
            result_graph.sort(key=lambda res: int(res.name.rsplit("-", maxsplit=1)[1]))

            # Update results
            for result in result_graph:
                #If we've not computed a quality for it, it is the non-degraded graph
                result.quality = qualities.get(result.name, 1.0)

            costs_mes=[]
            qualities_mes=[]
            for result in result_graph:
                cost,_ = result.costs
                costs_mes.append(cost)
                qualities_mes.append(result.quality)

            # We should get them in the same graph order as in the measured one (non-degraded first)
            csvname = prefix.rsplit("-", maxsplit=1)[0] + "-theo.csv"
            qualities_th, costs_th = load_csv(csvname)

            # print("Results:", result_graph)
            #
            # print("Theoretical costs: ", costs_th)
            # print("Measured costs: ", costs_mes)
            # print("Theoretical qualities: ", qualities_th)
            # print("Measured qualities: ", qualities_mes)

            kendalltau = GraphResults(prefix)
            kendalltau.costs = stats.kendalltau(costs_mes, costs_th, nan_policy='raise')
            kendalltau.quality = stats.kendalltau(qualities_mes, qualities_th, nan_policy='raise')

            spearmanr = GraphResults(prefix)
            spearmanr.costs = stats.spearmanr(costs_mes, costs_th, nan_policy='raise')
            spearmanr.quality = stats.spearmanr(qualities_mes, qualities_th, nan_policy='raise')

            print(kendalltau.name, " Kendal's tau: cost=", kendalltau.costs, " and quality=", kendalltau.quality)
            print(spearmanr.name, " Spearman's r: cost=", spearmanr.costs, " and quality=", spearmanr.quality)

            #input("Press a key to continue.")

            results[prefix] = (kendalltau, spearmanr)

        # We remove the audio files here as they can take a log of space
        audiofiles = glob.glob(prefix+"*.wav")
        for audiofile in audiofiles:
            os.remove(audiofile)

    return results


def results_to_csv(graphname, qualities, costs):
    fieldnames=["Name", "Quality", "Cost", "Total"]
    with open(graphname+".csv", "w", newline='') as csvfile:
        result={}
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames, delimiter='\t')
        writer.writeheader()
        #Get cost original graph
        name = list(sorted(costs.keys()))[0]
        result["Name"] = name
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

def result_all_graphs_to_csv(name, results):
    fieldnames = ["Name", "CostKT", "CostPKT", "QualityKT", "QualityPKT", "CostSR", "CostPSR", "QualitySR", "QualityPSR"]
    with open(name, "w", newline='') as csvfile:
        result={}
        writer = csv.DictWriter(csvfile, fieldnames, delimiter='\t')
        writer.writeheader()
        for (name, (kendalltau, spearmanr)) in tqdm(results.items()):
            assert(kendalltau.name == spearmanr.name)
            result["Name"]= name

            result["CostKT"] = kendalltau.costs[0]
            result["CostPKT"] = kendalltau.costs[1]

            result["QualityKT"] = kendalltau.quality[0]
            result["QualityPKT"] = kendalltau.quality[1]

            result["CostSR"] = spearmanr.costs[0]
            result["CostPSR"] = spearmanr.costs[1]

            result["QualitySR"] = spearmanr.quality[0]
            result["QualityPSR"] = spearmanr.quality[1]
            writer.writerow(result)

def load_correlations(filename):
    """LOad correlations from csv file"""
    with open(filename, "r", newline='') as csvfile:
        csvreader = csv.DictReader(csvfile, delimiter='\t')
        c_kt = []
        q_kt = []
        c_sr = []
        q_sr = []
        for row in csvreader:
            c_kt.append(float(row["CostKT"]))
            q_kt.append(float(row["QualityKT"]))
            c_sr.append(float(row["CostSR"]))
            q_sr.append(float(row["QualitySR"]))
        return c_kt, q_kt, c_sr, q_sr

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

def nan_in(l, name):
    for (i,e) in enumerate(l):
        if np.isnan(e):
            print("NaN detected in", name, " at iteration ", i)



def plot(qualities_mes, costs_mes, qualities_th, costs_th):
    fig, axes = plt.subplots(2,1)

    ax1= axes[0]

    texts_mes= []
    for (i, (quality, cost)) in enumerate(zip(qualities_mes, costs_mes)):
        texts_mes.append(ax1.text(quality, cost, str(i), ha='center', va='center'))

    #print("Measured: ", q, c_cycle)

    color='tab:red'

    ax1.set_ylabel("cost per cycle (µs)")
    ax1.set_xlabel("quality")
    ax1.scatter(qualities_mes, costs_mes,  label="Measured", color=color)
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.grid(True)

    ax2 = axes[1]

    texts_th = []
    for (i, (quality, cost)) in enumerate(zip(qualities_th, costs_th)):
        texts_th.append(ax2.text(quality, cost, str(i), ha='center', va='center'))

    color = 'tab:blue'
    ax2.set_ylabel("cost")
    ax2.set_xlabel("quality")

    ax2.scatter(qualities_th, costs_th,  label="Model", color=color)
    ax2.tick_params(axis='y', labelcolor=color)
    ax2.grid(True)


    adjust_text(texts_mes, ax=ax1)
    adjust_text(texts_th, ax=ax2)

    kendalltau = GraphResults("Kendall's tau")
    kendalltau.costs = stats.kendalltau(costs_mes, costs_th, nan_policy='raise')
    kendalltau.quality = stats.kendalltau(qualities_mes, qualities_th, nan_policy='raise')

    spearmanr = GraphResults("Spearman's R")
    spearmanr.costs = stats.spearmanr(costs_mes, costs_th, nan_policy='raise')
    spearmanr.quality = stats.spearmanr(qualities_mes, qualities_th, nan_policy='raise')

    print(kendalltau.name, " Kendal's tau: cost=", kendalltau.costs, " and quality=", kendalltau.quality)
    print(spearmanr.name, " Spearman's r: cost=", spearmanr.costs, " and quality=", spearmanr.quality)


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
    kendalltau_costs_rhos = []
    kendalltau_qualities_rhos = []
    spearmanr_costs_rhos = []
    spearmanr_qualities_rhos = []
    if not args.only_draw:
        results = process_all_graphs(args.nodes, dirname)
        for  (kendaltau, spearmanr) in tqdm(results.values()):
            #print(kendaltau.name, " Kendal's tau: cost=", kendaltau.costs, " and quality=", kendaltau.quality)
            #print(spearmanr.name, " Spearman's r: cost=", spearmanr.costs, " and quality=", spearmanr.quality)
            kendalltau_costs_rhos.append(kendaltau.costs[0])
            kendalltau_qualities_rhos.append(kendaltau.quality[0])
            spearmanr_costs_rhos.append(spearmanr.costs[0])
            spearmanr_qualities_rhos.append(spearmanr.quality[0])
        result_all_graphs_to_csv(dirname + "-correlations.csv", results)
    else:
        kendalltau_costs_rhos,kendalltau_qualities_rhos,spearmanr_costs_rhos,spearmanr_qualities_rhos=load_correlations(dirname + "-correlations.csv")

    # nan_in(kendalltau_costs_rhos, "kendalltau_costs_rhos")
    # nan_in(kendalltau_qualities_rhos, "kendalltau_qualities_rhos")
    # nan_in(spearmanr_costs_rhos, "spearmanr_costs_rhos")
    # nan_in(spearmanr_qualities_rhos, "spearmanr_qualities_rhos")

    kendalltau_costs_rhos = np.array(kendalltau_costs_rhos)
    kendalltau_costs_rhos = kendalltau_costs_rhos[~np.isnan(kendalltau_costs_rhos)]
    kendalltau_qualities_rhos = np.array(kendalltau_qualities_rhos)
    kendalltau_qualities_rhos = kendalltau_qualities_rhos[~np.isnan(kendalltau_qualities_rhos)]
    spearmanr_costs_rhos = np.array(spearmanr_costs_rhos)
    spearmanr_costs_rhos = spearmanr_costs_rhos[~np.isnan(spearmanr_costs_rhos)]
    spearmanr_qualities_rhos = np.array(spearmanr_qualities_rhos)
    spearmanr_qualities_rhos = spearmanr_qualities_rhos[~np.isnan(spearmanr_qualities_rhos)]


    if args.draw or args.only_draw:
        fig, axes = plt.subplots(2,2)

        axes[0][0].hist(kendalltau_costs_rhos, bins=20, label="Costs (Kendall's Tau)", color="red", range=[-1,1])
        axes[0][1].hist(spearmanr_costs_rhos, bins=20, label="Costs (Spearman's R)", color="red", range=[-1,1])

        axes[1][0].hist(kendalltau_qualities_rhos, bins=20, label="Qualities (Kendall's Tau)", range=[-1,1])
        axes[1][1].hist(spearmanr_qualities_rhos, bins=20, label="Qualities (Spearman's r)", range=[-1,1])

        fig.legend()

        plt.show()

    # Try also to do a Fisher transformation to get a better idea
    os.chdir("..")
