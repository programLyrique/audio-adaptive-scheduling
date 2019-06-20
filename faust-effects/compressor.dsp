import("stdfaust.lib");

// We don't care about the values here. Just need it for the generation

ratio = hslider("ratio", 2, 1, 1000, 0.1);
thresh = hslider("thresh", 20, 0, 100, 1);
att = hslider("att", 0.5, 0, 100, 0.1);
rel = hslider("rel", 1, 0, 100, 0.1);


process =  co.compressor_mono(ratio,thresh,att,rel) ;
