import("stdfaust.lib");

// We don't care about the values here. Just need it for the generation

fb1 = hslider("fb1", 0.5, 0, 1, 0.01);
fb2 = hslider("fb2", 0.5, 0, 1, 0.01);
damp = hslider("damp", 0.5, 0, 1, 0.01);
spread = hslider("spread", 0.5, 0, 1, 0.01);


process =  re.mono_freeverb(fb1, fb2, damp, spread) ;
