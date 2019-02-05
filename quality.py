#!/usr/bin/python3

"""
Compares two audio files, one seen as a baseline,the other one degraded,
using their spectrograms psychoacoustically weighted.
"""
import sys
import librosa
import numpy as np


basefile = sys.argv[1]
degradedfile = sys.argv[2]

print("Comparing basefile ", basefile, " and degraded file ", degradedfile)

bf,sr1 = librosa.load(basefile, sr=None)
df,sr2 = librosa.load(basefile, sr=None)

assert(sr1 == sr2)

# See here: http://librosa.github.io/librosa/generated/librosa.core.perceptual_weighting.html
def perceptual_cqt(y,sr):
    C = np.abs(librosa.cqt(y, sr=sr, fmin=librosa.note_to_hz('A1')))
    freqs = librosa.cqt_frequencies(C.shape[0], fmin=librosa.note_to_hz('A1'))#Adapted to music
    perceptual_CQT = librosa.perceptual_weighting(C**2, freqs, ref=np.max)# Uses
    # https://en.wikipedia.org/wiki/ITU-R_468_noise_weighting This one seems to be better for high freq
    # because A-weighting does not cut enough high frequencies
    return perceptual_CQT

base_pcat = perceptual_cqt(bf, sr1)
degraded_pcat = perceptual_cqt(df, sr2)

quality = np.exp(- np.linalg.norm(base_pcat - degraded_pcat))

print("Quality is ", quality)
