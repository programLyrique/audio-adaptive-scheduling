import("stdfaust.lib");

rdel = hslider("rdel", 6, 0, 1000, 0.1);
f1 = hslider("f1", 100, 15, 30000, 1);
f2 = hslider("f2", 600, 15, 30000, 1);
t60dc = hslider("t60dc", 3, 0, 1000, 0.1);
t60m = hslider("t60m", 6, 0, 1000, 0.1);
fsmax = hslider("fsmax", 44100, 1, 96000, 1);

process = re.zita_rev1_stereo(rdel, f1, f2, t60dc, t60m, fsmax);
