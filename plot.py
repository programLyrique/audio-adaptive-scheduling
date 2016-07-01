#!/usr/bin/python
from __future__ import print_function
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np
import sys
from datetime import timedelta

def cluster_per_ratio(data, callback_durations):
    ratios = {}
    durations = {}
    for row, dur in zip(data, callback_durations):
        ratios.setdefault(row['ratio'], []).append(row)
        durations.setdefault(row['ratio'], []).append(dur)
    for key in ratios.keys():
            ratios[key] = np.array(ratios[key])
            durations[key] = np.array(durations[key])
    return (ratios,durations)

fileName = sys.argv[1]

data = np.genfromtxt(fileName, delimiter='\t', dtype=float, names=['start', 'end', 'processing', 'ratio'], skip_header=1)


callback_durations = data['end'] - data['start']

clusters,durations = cluster_per_ratio(data, callback_durations)

duration_callback = timedelta(seconds = callback_durations.mean())
print("Mean duration for callback: ", duration_callback.microseconds, "microseconds")

for key in clusters.keys():
    print("Mean duration for resampling with ratio ", key, " is: ", clusters[key]['processing'].mean() / 1000)


duration_processing = data['processing'].mean()#In nanoseconds
print("Mean duration for processing: ", duration_processing / 1000, "microseconds")

callback_durations = callback_durations * 1000 # from seconds to microseconds
#plt.plot(data['start'], callback_durations)
#plt.plot(data['start'], callback_durations, label="Duration of callback")
plt.plot(data['start'], data['processing'], label="Duration of audio processing")

#plt.axis([0,1,0,2])
plt.legend(loc='best')
plt.xlabel('Start time (s)')
plt.ylabel('Duration (ms))')
plt.show()
