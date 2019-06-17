import("stdfaust.lib");

length = hslider("length",2,0.2,10,0.1);
pluckPosition = hslider("pluckPosition", 0.3, 0, 1, 0.01);
gain = hslider("gain", 0.8, 0,1, 0.01);
trigger = button("trigger");

process = pm.guitar(length, pluckPosition, gain, trigger);
