import("stdfaust.lib");

maxdel=16;
del=5.5;
aN=0.8;
process =  fi.allpass_fcomb(maxdel,del,aN) ;
