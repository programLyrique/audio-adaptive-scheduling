import("stdfaust.lib");

//num_outputs = hslider("Number of outputs", 2, 1, 10, 1);
num_outputs = 3;
rotation = hslider("distance", 0.7, 0, 1, 0.01);
distance = hslider("distance", 0.9, 0, 1, 0.01);

process = sp.spat(num_outputs, rotation, distance);
