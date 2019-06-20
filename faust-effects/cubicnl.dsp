import("stdfaust.lib");

// We don't care about the values here. Just need it for the generation

drive = hslider("drive", 0.9, 0, 1, 0.01);
offset = hslider("offset", 0.9, 0, 100, 0.01);

process =  ef.cubicnl(drive,offset) ;
