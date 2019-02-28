#!/usr/bin/python3

"""
Compares two audio files, one seen as a baseline,the other one degraded,
using their spectrograms psychoacoustically weighted.
"""
import sys
import librosa
import numpy as np
import math

def ITU_weighting(freqs):
    """ ITU_R_468 vectorized
    https://en.wikipedia.org/wiki/ITU-R_468_noise_weighting
    Supposedly better than A-weighting """
    h1 = -4.737338981378384e-24 *  freqs**6 + 2.043828333606125e-15 * freqs**4 - 1.363894795463638e-7 * freqs**2 + 1
    h2 = 1.306612257412824e-19 * freqs**5 - 2.118150887518656e-11 * freqs**3 + 5.559488023498642e-4 * freqs

    R_ITU = 1.246332637532143e-4 * freqs / np.sqrt(h1**2 + h2**2)
    return 18.2 + 20 * np.log10(R_ITU)

def perceptive_weighting_ITU(y, sr):
    pass

# Courbe isophonique ?
# https://www.iso.org/fr/standard/34222.html

# See here: http://librosa.github.io/librosa/generated/librosa.core.perceptual_weighting.html
#https://www.iso.org/fr/standard/34222.html
def perceptual_cqt(y,sr):
    C = np.abs(librosa.cqt(y, sr=sr, fmin=librosa.note_to_hz('A1')))
    freqs = librosa.cqt_frequencies(C.shape[0], fmin=librosa.note_to_hz('A1'))#Adapted to music
    perceptual_CQT = librosa.perceptual_weighting(C**2, freqs, ref=np.max)# Uses
    return perceptual_CQT

def compare_specto(y1, sr1, y2, sr2):
    base_pcat = perceptual_cqt(y1, sr1)
    degraded_pcat = perceptual_cqt(y2, sr2)
    size = len(base_pcat)**2.
    # To get a quality between 0 and 1, with 0 the worst one and 1 the best one.
    distance = np.linalg.norm(base_pcat - degraded_pcat)
    quality = np.exp(- distance / float(size))
    return quality

def quality(base, degraded):
    bf,sr1 = librosa.load(base, sr=None)
    df,sr2 = librosa.load(degraded, sr=None)
    assert(sr1 == sr2)
    return compare_specto(bf, sr1, df, sr2)

def load_file(filename, duration=None):
    return librosa.load(filename, sr=None, duration=duration)

if __name__ == "__main__":
    basefile = sys.argv[1]
    degradedfile = sys.argv[2]

    print("Comparing basefile ", basefile, " and degraded file ", degradedfile)

    print("Quality is ", quality(basefile, degradedfile))
