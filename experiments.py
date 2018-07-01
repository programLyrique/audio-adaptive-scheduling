#!/usr/bin/python3

"""
This script launches experiments and save results.

Another script is used to agregate results.
"""

import sys
import os
import glob
import subprocess
from tqdm import tqdm, trange
import numpy as np
from datetime import timedelta
import csv
from collections import defaultdict


columnNames1 = ["Budget", "ExpectRemainingTime", "Deadline", "ExecutionTime"]
columnNames2 = ["ChoosingDuration", "NbResamplers", "NbDegradedNodes"]
columnNames3 = ["NbCycles", "NbDegradedCycles", "NbEdges"]
columnNames = columnNames3 + columnNames1 + columnNames2
meanNames = ["mean"+s for s in columnNames]
stdNames = ["std"+s for s in columnNames]
fieldnames = ["nbNodes", "Mode"] + [val for pair in zip(meanNames, stdNames) for val in pair]

# key: (nbNodes, mode)
# value: a list with all the values

agregate_results = defaultdict(list)

def launch_experiments(ag_results, mode, nbNodes, nbRuns, proba_edge):
    folderName = str(nb)+mode
    os.makedirs(folderName, exist_ok=True)
    os.chdir(folderName)
    print("Experiment in mode ", mode, " with ", nbNodes, " nodes ", end='')
    if nbRuns > 0:
        print("with ", nbRuns, " runs")
        for i in trange(nbRuns):
            subprocess.run(os.path.join(os.path.dirname(sys.path[0]), programPath) + " " + mode + " " + str(nbNodes) + " " + str(proba_edge) + " 2>> errors.txt",  stdout=subprocess.DEVNULL, shell=True)
            #subprocess.run(programPath + " " + mode + " " + str(nbNodes), check=True, stdout=subprocess.DEVNULL, shell=True)
    else:
        print("without runs: reusing results from previous invocation")

    # TODO: rather do that as a 3rd phase? So that it's possible to relaunch experiments and have them included
    results=[]
    files= glob.glob("complex_graph*.csv")
    print("Collating results")
    for f in tqdm(files):
        #data = np.genfromtxt(f, delimiter="\t", encoding=None, dtype=[('Quality', '<i8'), ('Budget', '<i8'), ('ExpectRemainingTime', '<i8'), ('Deadline', '<i8'), ('NbNodes', '<i8'), ('ExecutionTime', '<i8'), ('ChoosingDuration', '<i8'), ('CallbackFlags', 'S16')], names=True)
        data = np.genfromtxt(f, delimiter="\t", encoding=None, dtype=None, names=True, skip_header=1)
        nbActualNodes=-1
        nbActualEdges=-1
        with open(f, "r") as datafile:
            line1 = datafile.readline().split(' ')
            nbActualNodes = int(line1[0]) #Always equal to nbNodes byconstruction of the random graph
            nbActualEdges = int(line1[1])
        nbCycles = data.size #data should be 1D (each element is a dictionary)
        degraded = nbCycles -np.count_nonzero(data["Quality"])
        ag_results[(nbNodes, mode)].append((data, nbActualEdges, nbCycles, degraded))
    os.chdir("..")

def agregate(ag_results):
    results = []
    print("Stats on results")
    bar = tqdm(ag_results.items())
    for (nN, mode),info in bar :
        res, nbEdges, nbCycles, degraded = zip(*info)
        #Concatenate all the ndarrays for this configuration
        data = np.concatenate(res)

        result={}
        for columnName in columnNames1:
            bar.write(columnName)
            result["mean" + columnName] = data[columnName].mean(dtype=np.float64)
            result["std" + columnName] = data[columnName].std(dtype=np.float64,ddof=1)
        for columnName in columnNames2:
            bar.write(columnName)
            degraded_ones = data[columnName][np.nonzero(data[columnName])]
            if degraded_ones.size == 0:#If nothing is degraded, we say that the mean is 0
                result["mean" + columnName] = 0.
                result["std" + columnName] = 0.
            else:
                result["mean" + columnName] = np.nanmean(degraded_ones, dtype=np.float64)
                result["std" + columnName] = np.nanstd(degraded_ones, dtype=np.float64, ddof=1)

        print(result["meanChoosingDuration"])

        nbCycles = np.array(nbCycles)
        result["meanNbCycles"] = nbCycles.mean(dtype=np.float64)
        result["stdNbCycles"] = nbCycles.std(dtype=np.float64, ddof=1)

        result["nbNodes"] = nN
        result["Mode"] = mode

        nbEdges= np.array(nbEdges)
        result["meanNbEdges"] = nbEdges.mean(dtype=np.float64)
        result["stdNbEdges"] = nbEdges.std(dtype=np.float64, ddof=1)

        degraded = np.array(degraded)
        result["meanNbDegradedCycles"] = degraded.mean(dtype=np.float64)
        result["stdNbDegradedCycles"] = degraded.std(dtype=np.float64, ddof=1)

        results.append(result)
    return results



if len(sys.argv) < 3:
    print("Usage: experiments.py destinationFolder nbRuns [proba_edge]")
    print("\tif nbRuns <= 0, uses the csv already computed")
# Prepare folder for experiments
# Must be in the base directory of the source
baseFolder = sys.argv[1]
os.makedirs(baseFolder, exist_ok=True)
os.chdir(baseFolder)

# Nb runs per config (mode, nbNodes)
nbRuns = int(sys.argv[2])

proba_edge = 0.5
if len(sys.argv) == 3:
    proba_edge = float(sys.argv[3])

programPath="audio_adaptive_scheduling/target/release/complex_graph"

#nbNodes = [10, 100, 1000]
#nbNodes = [10, 100, 200, 300, 350, 400, 1000]
nbNodes = [10, 100, 300, 400, 500, 1000]
#nbNodes = [10, 300]

print("##### Experiments starting in folder ", baseFolder, " with ", nbRuns, " runs per experiment #####\n")

# Launch experiments
for nb in nbNodes:
    launch_experiments(agregate_results, "EX", nb, nbRuns, proba_edge)
    launch_experiments(agregate_results, "PROG", nb, nbRuns, proba_edge)

results = agregate(agregate_results)
results.sort(key=lambda res : res["nbNodes"])

print("Writing final result file")
with open(baseFolder+".tsv", 'w') as tsvfile:
    writer = csv.DictWriter(tsvfile, fieldnames,dialect="excel-tab")
    writer.writeheader()
    for result in tqdm(results):
        writer.writerow(result)
