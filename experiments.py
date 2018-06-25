#!/usr/bin/python3

"""
This script launches experiments and save results.

Another script is used to agregate results.
"""

import sys
import os
import glob
import subprocess
from tqdm import tqdm
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np
from datetime import timedelta
import csv


flags_to_num = {}
i = 0
for name in ["NO_FLAG", "INPUT_UNDERFLOW", "INPUT_OVERFLOW", "OUTPUT_UNDERFLOW", "OUTPUT_OVERFLOW", "PRIMING_OUTPUT"]:
    flags_to_num[name] = i
    i += 1

# key: (nbNodes, mode)
# value: a list with all the values
results={}

def launch_experiments(where, mode, nbNodes, nbRuns):
    folderName = str(nb)+mode
    os.makedirs(where)
    os.chdir(where)
    print("Experiment in mode ", mode, " with ", nbNodes, " nodes with ", nbRuns, "runs")
    for i in tqdm(nbRuns):
        os.subprocess(programPath, mode, nbNodes, check=True, stdout=subprocess.DEVNULL)

    print("Analyzing")
    files= glob.glob("complex_graph*.csv")
    for f in tqdm(files):
        data = np.genfromtxt(f, delimiter="\t", dtype=[('Quality', '<i8'), ('Budget', '<i8'), ('ExpectRemainingTime', '<i8'), ('Deadline', '<i8'), ('NbNodes', '<i8'), ('ExecutionTime', '<i8'), ('ChoosingDuration', '<i8'), ('CallbackFlags', 'S16')], names=True)
        nbCycles = data.size() #data should be 1D (each element is a dictionary)
        notDegraded = data["Quality"].count_nonzero()

        result={}

        columNames = ["Budget", "ExpectRemainingTime", "Deadline", "NbNodes", "ExecutionTime", "ChoosingDuration"]
        for columnName in columnNames:
            result["mean" + columnName] = data[columnName].mean
            result["std" + columnName] =data[columnName].std(ddof=1)

        result["nbCycles"] = nbCycles
        result["notDegraded"] = notDegraded
        results[(nbNodes, mode)].append(result)

    os.chdir("..")
    print("Writing results")
    with open(where+".tsv", 'w') as tsvfile:
        meanNames = ["mean"+s for s in columNames]
        stdNames = ["std"+s for s in columNames]
        fieldnames = ["nbCycles", "notDegraded"].extend([val for pair in zip(meanNames, stdNames) for val in pair])
        writer = csv.DictWriter(tsvfile, dialect="excel-tab", field)
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
nbRuns = sys.argv[2]

programPath= "../../target/release/complex_graph "

nbNodes = [10, 100, 1000]

# Launch experiments
for nb in nbNodes:
    launch_experiments("EX", nb, nbRuns)
    launch_experiments("PROG", nb, nbRuns)
