#!/usr/bin/python
from __future__ import print_function
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np
import sys
from datetime import timedelta



fileName = sys.argv[1]

data = np.genfromtxt(fileName, delimiter='\t', dtype=float, names=['start', 'end', 'processing'], skip_header=1)

callback_durations = data['end'] - data['start']


duration_callback = timedelta(seconds = callback_durations.mean())
print("Mean duration for callback: ", duration_callback.microseconds, "microseconds")

duration_processing = data['processing'].mean()
print("Mean duration for processing: ", duration_processing / 1000, "ms")

plt.plot(data['start'], callback_durations, label="Duration of callback")
plt.plot(data['start'], data['processing'], label="Duration of audio processing")

#plt.axis([0,1,0,2])
plt.legend(loc='best')
plt.xlabel('Start time (s)')
plt.ylabel('Duration (ns)')
plt.show()
