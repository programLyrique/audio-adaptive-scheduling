import("stdfaust.lib");

w = hslider("Window length", 128, 1, 4096, 1);
x = hslider("Crossfade duration", 32, 1, 4096, 1);
s = hslider("Shift", 5, -12 * 4, 12 * 4, 1);

process = ef.transpose(w, x, s);
