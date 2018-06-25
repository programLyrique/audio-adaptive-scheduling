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


flags_to_num = {}
i = 0
for name in ["NO_FLAG", "INPUT_UNDERFLOW", "INPUT_OVERFLOW", "OUTPUT_UNDERFLOW", "OUTPUT_OVERFLOW", "PRIMING_OUTPUT"]:
    flags_to_num[name] = i
    i += 1

columnNames = ["Budget", "ExpectRemainingTime", "Deadline", "NbNodes", "ExecutionTime", "ChoosingDuration"]
meanNames = ["mean"+s for s in columnNames]
stdNames = ["std"+s for s in columnNames]
fieldnames = ["nbCycles", "notDegraded", "nbNodes", "nbEdges"] + [val for pair in zip(meanNames, stdNames) for val in pair]
# key: (nbNodes, mode)
# value: a list with all the values


def launch_experiments(mode, nbNodes, nbRuns):
    folderName = str(nb)+mode
    os.makedirs(folderName, exist_ok=True)
    os.chdir(folderName)
    print("Experiment in mode ", mode, " with ", nbNodes, " nodes with ", nbRuns, "runs")
    for i in trange(nbRuns):
        subprocess.run(programPath + " " + mode + " " + str(nbNodes),  stdout=subprocess.DEVNULL, shell=True)
        #subprocess.run(programPath + " " + mode + " " + str(nbNodes), check=True, stdout=subprocess.DEVNULL, shell=True)

    print("Analyzing")
    results=[]
    files= glob.glob("complex_graph*.csv")
    for f in tqdm(files):
        #data = np.genfromtxt(f, delimiter="\t", encoding=None, dtype=[('Quality', '<i8'), ('Budget', '<i8'), ('ExpectRemainingTime', '<i8'), ('Deadline', '<i8'), ('NbNodes', '<i8'), ('ExecutionTime', '<i8'), ('ChoosingDuration', '<i8'), ('CallbackFlags', 'S16')], names=True)
        data = np.genfromtxt(f, delimiter="\t", encoding=None, dtype=None, names=True, skip_header=1)
        nbActualNodes=-1
        nbActualEdges=-1
        with open(f, "r") as datafile:
            line1 = datafile.readline().split(' ')
            nbActualNodes = int(line1[0])
            nbActualEdges = int(line1[1])

        nbCycles = data.size #data should be 1D (each element is a dictionary)
        notDegraded = np.count_nonzero(data["Quality"])

        result={}


        for columnName in columnNames:
            result["mean" + columnName] = data[columnName].mean()
            result["std" + columnName] =data[columnName].std(ddof=1)

        result["nbCycles"] = nbCycles
        result["nbNodes"] = nbActualNodes
        result["nbEdges"] = nbActualEdges
        result["notDegraded"] = notDegraded
        results.append(result)

    os.chdir("..")
    print("Writing results")
    with open(folderName+".tsv", 'w') as tsvfile:
        writer = csv.DictWriter(tsvfile, fieldnames,dialect="excel-tab")
        writer.writeheader()
        for result in tqdm(results):
            writer.writerow(result)
    print("Done")

# Prepare folder for experiments
# Must be in the base directory of the source
baseFolder = sys.argv[1]
os.makedirs(baseFolder, exist_ok=True)
os.chdir(baseFolder)

# Nb runs per config (mode, nbNodes)
nbRuns = int(sys.argv[2])

programPath= "/Users/pierre/Documents/Salzburg/audio_adaptive_scheduling/target/release/complex_graph "

nbNodes = [10, 100, 1000]

print("##### Experiments starting in folder ", baseFolder, " with ", nbRuns, " runs per experiment #####\n")

# Launch experiments
for nb in nbNodes:
    launch_experiments("EX", nb, nbRuns)
    launch_experiments("PROG", nb, nbRuns)
