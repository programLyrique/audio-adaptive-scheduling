#!/usr/bin/python3

"""
This script agregates and analyzes the results of the experiments launched by experiments.py

theformatting of the final tsv file is done by another script, format.py
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

columnNames2 = ["Budget", "ExpectRemainingTime", "Deadline", "NbNodes", "ExecutionTime", "ChoosingDuration"]
columnNames1 = ["NbCycles", "NotDegraded", "nbEdges",]
columnNames =  columnNames1 + columnNames2
meanNames = ["mean"+s for s in columnNames]
stdNames = ["std"+s for s in columnNames]
fieldnames = ["nbNodes", "Mode"] + [val for pair in zip(meanNames, stdNames) for val in pair]

# Retrieve folder where the files to analyze are located
folderName = sys.argv[1]

os.chdir(folderName)

resultFiles = glob.glob("*.tsv")

results=defaultdict(list)

mode=""

dtyp=[]

print("Agregating results")
for f in tqdm(resultFiles):
    if "EX" in f:
        mode="EX"
    elif "PROG" in f:
        mode="PROG"
    else:
        print("Ignoring file: ", f)
        continue
    data = np.genfromtxt(f, delimiter="\t", encoding=None, dtype=None, names=True)
    dtyp = data.dtype
    for line in tqdm(data):
        results[(line['nbNodes'], mode)].append(line)

#print(dtyp)

uncapitalize = lambda s: s[:1].lower() + s[1:] if s else ''

print("Analyzing results")
results2 = []
for (nN, mode),result in tqdm(results.items()):
    result=np.array(result, dtype=dtyp)
    cycles = result["nbCycles"]#The number of elements used to compute the average for this line
    res = {}
    ptp = np.ptp(result["nbNodes"])
    assert ptp == 0
    assert result["nbNodes"][0] == nN
    res["nbNodes"] = result["nbNodes"][0]
    res["Mode"] = mode
    # Mean of choosing duration should be computed on only degraded cycles!!
    for columnName in columnNames1:
        res["mean" + columnName] = result[uncapitalize(columnName)].mean()
        res["std"+ columnName] = result[uncapitalize(columnName)].std(ddof=1)
    for columnName in columnNames2:
        res["mean"+columnName] = np.average(result["mean"+columnName], weights=cycles)
        res["std"+columnName] = np.sqrt(np.sum(np.square(result["std"+columnName])))
    results2.append(res)


print("Writing final result file")
with open(folderName+".tsv", 'w') as tsvfile:
    writer = csv.DictWriter(tsvfile, fieldnames,dialect="excel-tab")
    writer.writeheader()
    for result in tqdm(results2):
        writer.writerow(result)
