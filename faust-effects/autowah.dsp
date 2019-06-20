import("stdfaust.lib");

// We don't care about the values here. Just need it for the generation

level = hslider("level", 0.9, 0, 1, 0.01);

process =  ve.autowah(level) ;
