//! Audio effects
//! Generated using the rust backend from Faust code

use audiograph::*;
use audiograph_parser;

use std::fmt;

use amath;

pub fn faustpower2_f(value: f32) -> f32 {
    return value * value;
}

pub struct Guitar {
    fDummy: f32,
    iRec10: [i32; 2],
    fSampleRate: i32,
    fConst0: f32,
    fConst1: f32,
    fHslider0: f32,
    fHslider1: f32,
    fConst2: f32,
    fRec22: [f32; 2],
    fRec25: [f32; 2],
    fRec27: [f32; 4],
    IOTA: i32,
    fRec28: [f32; 2048],
    fVec0: [f32; 2],
    fHslider2: f32,
    fConst3: f32,
    iRec30: [i32; 2],
    fRec29: [f32; 3],
    fButton0: f32,
    fVec1: [f32; 2],
    fConst4: f32,
    fVec2: [f32; 2],
    fRec31: [f32; 2],
    fConst5: f32,
    fConst6: f32,
    fVec3: [f32; 2],
    fRec26: [f32; 2048],
    fRec19: [f32; 2],
    fRec16: [f32; 2048],
    fRec18: [f32; 2],
    fRec15: [f32; 4],
    iRec6: [i32; 2],
    fRec2: [f32; 2048],
    fRec0: [f32; 2],
}

impl Guitar {
    pub fn init() -> Guitar {
        Guitar {
            fDummy: 0 as f32,
            iRec10: [0; 2],
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fHslider0: 0.0,
            fHslider1: 0.0,
            fConst2: 0.0,
            fRec22: [0.0; 2],
            fRec25: [0.0; 2],
            fRec27: [0.0; 4],
            IOTA: 0,
            fRec28: [0.0; 2048],
            fVec0: [0.0; 2],
            fHslider2: 0.0,
            fConst3: 0.0,
            iRec30: [0; 2],
            fRec29: [0.0; 3],
            fButton0: 0.0,
            fVec1: [0.0; 2],
            fConst4: 0.0,
            fVec2: [0.0; 2],
            fRec31: [0.0; 2],
            fConst5: 0.0,
            fConst6: 0.0,
            fVec3: [0.0; 2],
            fRec26: [0.0; 2048],
            fRec19: [0.0; 2],
            fRec16: [0.0; 2048],
            fRec18: [0.0; 2],
            fRec15: [0.0; 4],
            iRec6: [0; 2],
            fRec2: [0.0; 2048],
            fRec0: [0.0; 2],
        }
    }

    pub fn instanceClear(&mut self) {
        for l0 in 0..2 {
            self.iRec10[l0 as usize] = 0;
        }
        for l1 in 0..2 {
            self.fRec22[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.fRec25[l2 as usize] = 0.0;
        }
        for l3 in 0..4 {
            self.fRec27[l3 as usize] = 0.0;
        }
        self.IOTA = 0;
        for l4 in 0..2048 {
            self.fRec28[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fVec0[l5 as usize] = 0.0;
        }
        for l6 in 0..2 {
            self.iRec30[l6 as usize] = 0;
        }
        for l7 in 0..3 {
            self.fRec29[l7 as usize] = 0.0;
        }
        for l8 in 0..2 {
            self.fVec1[l8 as usize] = 0.0;
        }
        for l9 in 0..2 {
            self.fVec2[l9 as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec31[l10 as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fVec3[l11 as usize] = 0.0;
        }
        for l12 in 0..2048 {
            self.fRec26[l12 as usize] = 0.0;
        }
        for l13 in 0..2 {
            self.fRec19[l13 as usize] = 0.0;
        }
        for l14 in 0..2048 {
            self.fRec16[l14 as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec18[l15 as usize] = 0.0;
        }
        for l16 in 0..4 {
            self.fRec15[l16 as usize] = 0.0;
        }
        for l17 in 0..2 {
            self.iRec6[l17 as usize] = 0;
        }
        for l18 in 0..2048 {
            self.fRec2[l18 as usize] = 0.0;
        }
        for l19 in 0..2 {
            self.fRec0[l19 as usize] = 0.0;
        }
    }

    pub fn instanceConstants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = f32::min(192000.0, f32::max(1.0, self.fSampleRate as f32));
        self.fConst1 = 0.00147058826 * self.fConst0;
        self.fConst2 = 0.00882352982 * self.fConst0;
        self.fConst3 = 5340.70752 / self.fConst0;
        self.fConst4 = 0.00400000019 * self.fConst0;
        self.fConst5 = 0.00200000009 * self.fConst0;
        self.fConst6 = 500.0 / self.fConst0;
    }

    pub fn instanceInit(&mut self, sample_rate: i32) {
        self.instanceConstants(sample_rate);
        self.instanceClear();
    }

    pub fn setControlVariables(
        &mut self,
        length: f32,
        pluck_position: f32,
        gain: f32,
        trigger: u32,
    ) {
        self.fHslider0 = length;
        self.fHslider1 = pluck_position;
        self.fHslider2 = gain;
        self.fButton0 = trigger as f32;
    }

    pub fn new(length: f32, pluck_position: f32, gain: f32, trigger: u32) -> Guitar {
        let mut modu = Guitar::init();
        modu.instanceInit(44_100);
        modu.setControlVariables(length, pluck_position, gain, trigger);
        modu
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Guitar {
        let length = node_infos.more["length"]
            .parse()
            .expect("length must be a float");
        let pluck_position = node_infos.more["pluck_position"]
            .parse()
            .expect("pluck_position must be a float");
        let modu = Guitar::new(length, pluck_position, 0.9, 1);
        modu.check_io_node_infos(node_infos);
        modu
    }
}

impl fmt::Display for Guitar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "guitar({}, {})", self.fHslider0, self.fHslider1)
    }
}

impl AudioEffect for Guitar {
    fn nb_inputs(&self) -> usize {
        return 0;
    }
    fn nb_outputs(&self) -> usize {
        return 1;
    }

    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        let actual_samplerate = outputs[0].samplerate as i32;
        let output = outputs[0].buffer_mut();
        let count = output.len();

        //Constants have to be changed if we change the samplerate...
        // We should smooth it actually...
        if self.fSampleRate != actual_samplerate {
            self.instanceInit(actual_samplerate);
        }
        let fSlow0: f32 = self.fHslider0 as f32;
        let fSlow1: f32 = fSlow0 + -0.100000001;
        let fSlow2: f32 = self.fHslider1 as f32;
        let fSlow3: f32 = self.fConst1 * (fSlow1 * (1.0 - fSlow2));
        let fSlow4: f32 = fSlow3 + -1.49999499;
        let fSlow5: f32 = f32::floor(fSlow4);
        let fSlow6: f32 = fSlow3 + (-1.0 - fSlow5);
        let fSlow7: f32 = fSlow3 + (-2.0 - fSlow5);
        let fSlow8: f32 = fSlow3 + (-3.0 - fSlow5);
        let fSlow9: f32 = fSlow3 + (-4.0 - fSlow5);
        let fSlow10: f32 = (((0.0 - fSlow6) * (0.0 - (0.5 * fSlow7)))
            * (0.0 - (0.333333343 * fSlow8)))
            * (0.0 - (0.25 * fSlow9));
        let iSlow11: i32 = fSlow4 as i32;
        let iSlow12: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow11) as f32) as i32;
        let iSlow13: i32 = iSlow12 + 1;
        let fSlow14: f32 = fSlow3 - fSlow5;
        let fSlow15: f32 =
            ((0.0 - fSlow7) * (0.0 - (0.5 * fSlow8))) * (0.0 - (0.333333343 * fSlow9));
        let iSlow16: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow11 + 1) as f32) as i32;
        let iSlow17: i32 = iSlow16 + 1;
        let fSlow18: f32 = 0.5 * ((fSlow6 * (0.0 - fSlow8)) * (0.0 - (0.5 * fSlow9)));
        let iSlow19: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow11 + 2) as f32) as i32;
        let iSlow20: i32 = iSlow19 + 1;
        let fSlow21: f32 = fSlow6 * fSlow7;
        let fSlow22: f32 = 0.166666672 * (fSlow21 * (0.0 - fSlow9));
        let iSlow23: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow11 + 3) as f32) as i32;
        let iSlow24: i32 = iSlow23 + 1;
        let fSlow25: f32 = 0.0416666679 * (fSlow21 * fSlow8);
        let iSlow26: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow11 + 4) as f32) as i32;
        let iSlow27: i32 = iSlow26 + 1;
        let fSlow28: f32 = self.fConst1 * (fSlow2 * fSlow1);
        let fSlow29: f32 = fSlow28 + -1.49999499;
        let fSlow30: f32 = f32::floor(fSlow29);
        let fSlow31: f32 = fSlow28 + (-1.0 - fSlow30);
        let fSlow32: f32 = fSlow28 + (-2.0 - fSlow30);
        let fSlow33: f32 = fSlow28 + (-3.0 - fSlow30);
        let fSlow34: f32 = fSlow28 + (-4.0 - fSlow30);
        let fSlow35: f32 = (((0.0 - fSlow31) * (0.0 - (0.5 * fSlow32)))
            * (0.0 - (0.333333343 * fSlow33)))
            * (0.0 - (0.25 * fSlow34));
        let iSlow36: i32 = fSlow29 as i32;
        let iSlow37: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow36) as f32) as i32;
        let iSlow38: i32 = iSlow37 + 2;
        let fSlow39: f32 = fSlow28 - fSlow30;
        let fSlow40: f32 =
            ((0.0 - fSlow32) * (0.0 - (0.5 * fSlow33))) * (0.0 - (0.333333343 * fSlow34));
        let iSlow41: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow36 + 1) as f32) as i32;
        let iSlow42: i32 = iSlow41 + 2;
        let fSlow43: f32 = 0.5 * ((fSlow31 * (0.0 - fSlow33)) * (0.0 - (0.5 * fSlow34)));
        let iSlow44: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow36 + 2) as f32) as i32;
        let iSlow45: i32 = iSlow44 + 2;
        let fSlow46: f32 = fSlow31 * fSlow32;
        let fSlow47: f32 = 0.166666672 * (fSlow46 * (0.0 - fSlow34));
        let iSlow48: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow36 + 3) as f32) as i32;
        let iSlow49: i32 = iSlow48 + 2;
        let fSlow50: f32 = 0.0416666679 * (fSlow46 * fSlow33);
        let iSlow51: i32 = f32::min(self.fConst2, std::cmp::max(0, iSlow36 + 4) as f32) as i32;
        let iSlow52: i32 = iSlow51 + 2;
        let fSlow53: f32 = f32::tan(self.fConst3 / fSlow0);
        let fSlow54: f32 = 1.0 / fSlow53;
        let fSlow55: f32 = ((fSlow54 + 1.41421354) / fSlow53) + 1.0;
        let fSlow56: f32 = (self.fHslider2 as f32) / fSlow55;
        let fSlow57: f32 = 1.0 / fSlow55;
        let fSlow58: f32 = ((fSlow54 + -1.41421354) / fSlow53) + 1.0;
        let fSlow59: f32 = 2.0 * (1.0 - (1.0 / faustpower2_f(fSlow53)));
        let fSlow60: f32 = self.fButton0 as f32;
        let fSlow61: f32 = faustpower2_f(1.0 - (0.113333337 / fSlow0));
        let fSlow62: f32 = self.fConst4 * fSlow61;
        let fSlow63: f32 = self.fConst5 * fSlow61;
        let fSlow64: f32 = self.fConst6 / fSlow61;
        let iSlow65: i32 = iSlow37 + 1;
        let iSlow66: i32 = iSlow41 + 1;
        let iSlow67: i32 = iSlow44 + 1;
        let iSlow68: i32 = iSlow48 + 1;
        let iSlow69: i32 = iSlow51 + 1;
        for i in 0..count {
            self.iRec10[0] = 0;
            let mut iRec11: i32 = self.iRec10[1];
            let mut fRec14: f32 = (self.iRec6[1] as f32)
                - (0.997843683
                    * ((0.699999988 * self.fRec15[2])
                        + (0.150000006 * (self.fRec15[1] + self.fRec15[3]))));
            self.fRec22[0] = (fSlow10 * self.fRec2[((self.IOTA - iSlow13) & 2047) as usize])
                + (fSlow14
                    * ((((fSlow15 * self.fRec2[((self.IOTA - iSlow17) & 2047) as usize])
                        + (fSlow18 * self.fRec2[((self.IOTA - iSlow20) & 2047) as usize]))
                        + (fSlow22 * self.fRec2[((self.IOTA - iSlow24) & 2047) as usize]))
                        + (fSlow25 * self.fRec2[((self.IOTA - iSlow27) & 2047) as usize])));
            self.fRec25[0] = (0.0500000007 * self.fRec25[1]) + (0.949999988 * self.fRec22[1]);
            let mut fRec23: f32 = self.fRec25[0];
            self.fRec27[0] = self.fRec0[1];
            self.fRec28[(self.IOTA & 2047) as usize] = -1.0
                * (0.997843683
                    * ((0.699999988 * self.fRec27[2])
                        + (0.150000006 * (self.fRec27[1] + self.fRec27[3]))));
            self.fVec0[0] = (fSlow35 * self.fRec28[((self.IOTA - iSlow38) & 2047) as usize])
                + (fSlow39
                    * ((((fSlow40 * self.fRec28[((self.IOTA - iSlow42) & 2047) as usize])
                        + (fSlow43 * self.fRec28[((self.IOTA - iSlow45) & 2047) as usize]))
                        + (fSlow47 * self.fRec28[((self.IOTA - iSlow49) & 2047) as usize]))
                        + (fSlow50 * self.fRec28[((self.IOTA - iSlow52) & 2047) as usize])));
            self.iRec30[0] = (1103515245 * self.iRec30[1]) + 12345;
            self.fRec29[0] = (4.65661287e-10 * (self.iRec30[0] as f32))
                - (fSlow57 * ((fSlow58 * self.fRec29[2]) + (fSlow59 * self.fRec29[1])));
            self.fVec1[0] = fSlow60;
            self.fVec2[0] = fSlow61;
            self.fRec31[0] =
                if (((((fSlow60 - self.fVec1[1]) > 0.0) as i32) > 0) as i32) as i32 == 1 {
                    0.0
                } else {
                    f32::min(
                        fSlow62,
                        (self.fRec31[1] + (self.fConst4 * (fSlow61 - self.fVec2[1]))) + 1.0,
                    )
                };
            let mut iTemp0: i32 = (self.fRec31[0] < fSlow63) as i32;
            let mut fTemp1: f32 = fSlow56
                * ((self.fRec29[2] + (self.fRec29[0] + (2.0 * self.fRec29[1])))
                    * if iTemp0 as i32 == 1 {
                        if ((self.fRec31[0] < 0.0) as i32) as i32 == 1 {
                            0.0
                        } else {
                            if iTemp0 as i32 == 1 {
                                (fSlow64 * self.fRec31[0])
                            } else {
                                1.0
                            }
                        }
                    } else {
                        if ((self.fRec31[0] < fSlow62) as i32) as i32 == 1 {
                            ((fSlow64 * (0.0 - (self.fRec31[0] - fSlow63))) + 1.0)
                        } else {
                            0.0
                        }
                    });
            self.fVec3[0] = self.fVec0[1] + fTemp1;
            self.fRec26[(self.IOTA & 2047) as usize] = (0.0500000007
                * self.fRec26[((self.IOTA - 1) & 2047) as usize])
                + (0.949999988 * self.fVec3[1]);
            let mut fRec24: f32 = (fSlow10 * self.fRec26[((self.IOTA - iSlow12) & 2047) as usize])
                + (fSlow14
                    * ((((fSlow15 * self.fRec26[((self.IOTA - iSlow16) & 2047) as usize])
                        + (fSlow18 * self.fRec26[((self.IOTA - iSlow19) & 2047) as usize]))
                        + (fSlow22 * self.fRec26[((self.IOTA - iSlow23) & 2047) as usize]))
                        + (fSlow25 * self.fRec26[((self.IOTA - iSlow26) & 2047) as usize])));
            self.fRec19[0] = fRec23;
            let mut fRec20: f32 = fTemp1 + self.fRec19[1];
            let mut fRec21: f32 = fRec24;
            self.fRec16[(self.IOTA & 2047) as usize] = fRec20;
            let mut fRec17: f32 = (fSlow35 * self.fRec16[((self.IOTA - iSlow65) & 2047) as usize])
                + (fSlow39
                    * ((((fSlow40 * self.fRec16[((self.IOTA - iSlow66) & 2047) as usize])
                        + (fSlow43 * self.fRec16[((self.IOTA - iSlow67) & 2047) as usize]))
                        + (fSlow47 * self.fRec16[((self.IOTA - iSlow68) & 2047) as usize]))
                        + (fSlow50 * self.fRec16[((self.IOTA - iSlow69) & 2047) as usize])));
            self.fRec18[0] = fRec21;
            self.fRec15[0] = self.fRec18[1];
            let mut fRec12: f32 = self.fRec15[1];
            let mut fRec13: f32 = self.fRec15[1];
            self.iRec6[0] = iRec11;
            let mut fRec7: f32 = fRec14;
            let mut fRec8: f32 = fRec12;
            let mut fRec9: f32 = fRec13;
            self.fRec2[(self.IOTA & 2047) as usize] = fRec7;
            let mut fRec3: f32 = fRec17;
            let mut _fRec4: f32 = fRec8;
            let mut fRec5: f32 = fRec9;
            self.fRec0[0] = fRec3;
            let mut fRec1: f32 = fRec5;
            output[i as usize] = fRec1 as f32;
            self.iRec10[1] = self.iRec10[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec25[1] = self.fRec25[0];
            for j0 in 4..0 {
                self.fRec27[j0] = self.fRec27[j0 - 1];
            }
            self.IOTA = self.IOTA + 1;
            self.fVec0[1] = self.fVec0[0];
            self.iRec30[1] = self.iRec30[0];
            self.fRec29[2] = self.fRec29[1];
            self.fRec29[1] = self.fRec29[0];
            self.fVec1[1] = self.fVec1[0];
            self.fVec2[1] = self.fVec2[0];
            self.fRec31[1] = self.fRec31[0];
            self.fVec3[1] = self.fVec3[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            for j1 in 4..0 {
                self.fRec15[j1] = self.fRec15[j1 - 1];
            }
            self.iRec6[1] = self.iRec6[0];
            self.fRec0[1] = self.fRec0[0];
        }
    }
}

/****************************************
** Transposer
****************************************/

pub struct Transposer {
    fDummy: f32,
    IOTA: i32,
    fVec0: [f32; 131072],
    fHslider0: u32,
    fHslider1: u32,
    fRec0: [f32; 2],
    fHslider2: i32,
}

impl Transposer {
    pub fn init() -> Transposer {
        Transposer {
            fDummy: 0 as f32,
            IOTA: 0,
            fVec0: [0.0; 131072],
            fHslider0: 128,
            fHslider1: 32,
            fRec0: [0.0; 2],
            fHslider2: 0,
        }
    }

    pub fn instanceClear(&mut self) {
        self.IOTA = 0;
        for l0 in 0..131072 {
            self.fVec0[l0 as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec0[l1 as usize] = 0.0;
        }
    }

    pub fn new(semitones: i32) -> Transposer {
        let mut transposer = Transposer::init();
        transposer.fHslider2 = semitones;
        transposer
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Transposer {
        let semitones: i32 = node_infos.more["semitones"]
            .parse()
            .expect("semitones must be a positive or negative integer");
        let transposer = Transposer::new(semitones);
        transposer.check_io_node_infos(node_infos);
        transposer
    }
}

impl fmt::Display for Transposer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "transposer({})", self.fHslider2)
    }
}

impl AudioEffect for Transposer {
    fn nb_inputs(&self) -> usize {
        1
    }
    fn nb_outputs(&self) -> usize {
        1
    }
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());

        let input = inputs[0].buffer();
        let output = outputs[0].buffer_mut();
        let count = output.len();

        let fSlow0: f32 = self.fHslider0 as f32;
        let fSlow1: f32 = f32::powf(2.0, 0.0833333358 * (self.fHslider1 as f32));
        let fSlow2: f32 = 1.0 / (self.fHslider2 as f32);
        for i in 0..count {
            let mut fTemp0: f32 = input[i as usize] as f32;
            self.fVec0[(self.IOTA & 131071) as usize] = fTemp0;
            self.fRec0[0] = amath::fmod(fSlow0 + ((self.fRec0[1] + 1.0) - fSlow1), fSlow0);
            let mut iTemp1: i32 = self.fRec0[0] as i32;
            let mut fTemp2: f32 = f32::floor(self.fRec0[0]);
            let mut fTemp3: f32 = 1.0 - self.fRec0[0];
            let mut fTemp4: f32 = f32::min(fSlow2 * self.fRec0[0], 1.0);
            let mut fTemp5: f32 = fSlow0 + self.fRec0[0];
            let mut iTemp6: i32 = fTemp5 as i32;
            let mut fTemp7: f32 = f32::floor(fTemp5);
            output[i as usize] = ((((self.fVec0[((self.IOTA
                - std::cmp::min(65537, std::cmp::max(0, iTemp1)))
                & 131071) as usize]
                * (fTemp2 + fTemp3))
                + ((self.fRec0[0] - fTemp2)
                    * self.fVec0[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, iTemp1 + 1)))
                        & 131071) as usize]))
                * fTemp4)
                + (((self.fVec0[((self.IOTA - std::cmp::min(65537, std::cmp::max(0, iTemp6)))
                    & 131071) as usize]
                    * ((fTemp7 + fTemp3) - fSlow0))
                    + ((fSlow0 + (self.fRec0[0] - fTemp7))
                        * self.fVec0[((self.IOTA
                            - std::cmp::min(65537, std::cmp::max(0, iTemp6 + 1)))
                            & 131071) as usize]))
                    * (1.0 - fTemp4))) as f32;
            self.IOTA = self.IOTA + 1;
            self.fRec0[1] = self.fRec0[0];
        }
    }
}

/****************************************
** Zita reverb stereo
****************************************/

pub struct ZitaReverb {
    fDummy: f32,
    fSampleRate: i32,
    fConst0: f32,
    fConst1: f32,
    fConst2: f32,
    fHslider0: f32,
    fConst3: f32,
    fHslider1: f32,
    fHslider2: f32,
    fConst4: f32,
    fHslider3: f32,
    fRec11: [f32; 2],
    fRec10: [f32; 2],
    IOTA: i32,
    fVec0: [f32; 65536],
    fHslider4: f32,
    fConst5: f32,
    fConst6: f32,
    fVec1: [f32; 16384],
    fConst7: f32,
    fHslider5: f32,
    fVec2: [f32; 2048],
    fConst8: f32,
    fRec8: [f32; 2],
    fConst9: f32,
    fConst10: f32,
    fRec15: [f32; 2],
    fRec14: [f32; 2],
    fVec3: [f32; 65536],
    fConst11: f32,
    fConst12: f32,
    fVec4: [f32; 4096],
    fConst13: f32,
    fRec12: [f32; 2],
    fConst14: f32,
    fConst15: f32,
    fRec19: [f32; 2],
    fRec18: [f32; 2],
    fVec5: [f32; 65536],
    fConst16: f32,
    fConst17: f32,
    fVec6: [f32; 4096],
    fConst18: f32,
    fRec16: [f32; 2],
    fConst19: f32,
    fConst20: f32,
    fRec23: [f32; 2],
    fRec22: [f32; 2],
    fVec7: [f32; 65536],
    fConst21: f32,
    fConst22: f32,
    fVec8: [f32; 4096],
    fConst23: f32,
    fRec20: [f32; 2],
    fConst24: f32,
    fConst25: f32,
    fRec27: [f32; 2],
    fRec26: [f32; 2],
    fVec9: [f32; 32768],
    fConst26: f32,
    fConst27: f32,
    fVec10: [f32; 16384],
    fVec11: [f32; 2048],
    fConst28: f32,
    fRec24: [f32; 2],
    fConst29: f32,
    fConst30: f32,
    fRec31: [f32; 2],
    fRec30: [f32; 2],
    fVec12: [f32; 32768],
    fConst31: f32,
    fConst32: f32,
    fVec13: [f32; 4096],
    fConst33: f32,
    fRec28: [f32; 2],
    fConst34: f32,
    fConst35: f32,
    fRec35: [f32; 2],
    fRec34: [f32; 2],
    fVec14: [f32; 65536],
    fConst36: f32,
    fConst37: f32,
    fVec15: [f32; 4096],
    fConst38: f32,
    fRec32: [f32; 2],
    fConst39: f32,
    fConst40: f32,
    fRec39: [f32; 2],
    fRec38: [f32; 2],
    fVec16: [f32; 32768],
    fConst41: f32,
    fConst42: f32,
    fVec17: [f32; 2048],
    fConst43: f32,
    fRec36: [f32; 2],
    fRec0: [f32; 3],
    fRec1: [f32; 3],
    fRec2: [f32; 3],
    fRec3: [f32; 3],
    fRec4: [f32; 3],
    fRec5: [f32; 3],
    fRec6: [f32; 3],
    fRec7: [f32; 3],
}

impl ZitaReverb {
    pub fn init() -> ZitaReverb {
        ZitaReverb {
            fDummy: 0 as f32,
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fConst2: 0.0,
            fHslider0: 0.0,
            fConst3: 0.0,
            fHslider1: 0.0,
            fHslider2: 0.0,
            fConst4: 0.0,
            fHslider3: 0.0,
            fRec11: [0.0; 2],
            fRec10: [0.0; 2],
            IOTA: 0,
            fVec0: [0.0; 65536],
            fHslider4: 0.0,
            fConst5: 0.0,
            fConst6: 0.0,
            fVec1: [0.0; 16384],
            fConst7: 0.0,
            fHslider5: 0.0,
            fVec2: [0.0; 2048],
            fConst8: 0.0,
            fRec8: [0.0; 2],
            fConst9: 0.0,
            fConst10: 0.0,
            fRec15: [0.0; 2],
            fRec14: [0.0; 2],
            fVec3: [0.0; 65536],
            fConst11: 0.0,
            fConst12: 0.0,
            fVec4: [0.0; 4096],
            fConst13: 0.0,
            fRec12: [0.0; 2],
            fConst14: 0.0,
            fConst15: 0.0,
            fRec19: [0.0; 2],
            fRec18: [0.0; 2],
            fVec5: [0.0; 65536],
            fConst16: 0.0,
            fConst17: 0.0,
            fVec6: [0.0; 4096],
            fConst18: 0.0,
            fRec16: [0.0; 2],
            fConst19: 0.0,
            fConst20: 0.0,
            fRec23: [0.0; 2],
            fRec22: [0.0; 2],
            fVec7: [0.0; 65536],
            fConst21: 0.0,
            fConst22: 0.0,
            fVec8: [0.0; 4096],
            fConst23: 0.0,
            fRec20: [0.0; 2],
            fConst24: 0.0,
            fConst25: 0.0,
            fRec27: [0.0; 2],
            fRec26: [0.0; 2],
            fVec9: [0.0; 32768],
            fConst26: 0.0,
            fConst27: 0.0,
            fVec10: [0.0; 16384],
            fVec11: [0.0; 2048],
            fConst28: 0.0,
            fRec24: [0.0; 2],
            fConst29: 0.0,
            fConst30: 0.0,
            fRec31: [0.0; 2],
            fRec30: [0.0; 2],
            fVec12: [0.0; 32768],
            fConst31: 0.0,
            fConst32: 0.0,
            fVec13: [0.0; 4096],
            fConst33: 0.0,
            fRec28: [0.0; 2],
            fConst34: 0.0,
            fConst35: 0.0,
            fRec35: [0.0; 2],
            fRec34: [0.0; 2],
            fVec14: [0.0; 65536],
            fConst36: 0.0,
            fConst37: 0.0,
            fVec15: [0.0; 4096],
            fConst38: 0.0,
            fRec32: [0.0; 2],
            fConst39: 0.0,
            fConst40: 0.0,
            fRec39: [0.0; 2],
            fRec38: [0.0; 2],
            fVec16: [0.0; 32768],
            fConst41: 0.0,
            fConst42: 0.0,
            fVec17: [0.0; 2048],
            fConst43: 0.0,
            fRec36: [0.0; 2],
            fRec0: [0.0; 3],
            fRec1: [0.0; 3],
            fRec2: [0.0; 3],
            fRec3: [0.0; 3],
            fRec4: [0.0; 3],
            fRec5: [0.0; 3],
            fRec6: [0.0; 3],
            fRec7: [0.0; 3],
        }
    }

    pub fn instanceClear(&mut self) {
        for l0 in 0..2 {
            self.fRec11[l0 as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec10[l1 as usize] = 0.0;
        }
        self.IOTA = 0;
        for l2 in 0..65536 {
            self.fVec0[l2 as usize] = 0.0;
        }
        for l3 in 0..16384 {
            self.fVec1[l3 as usize] = 0.0;
        }
        for l4 in 0..2048 {
            self.fVec2[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fRec8[l5 as usize] = 0.0;
        }
        for l6 in 0..2 {
            self.fRec15[l6 as usize] = 0.0;
        }
        for l7 in 0..2 {
            self.fRec14[l7 as usize] = 0.0;
        }
        for l8 in 0..65536 {
            self.fVec3[l8 as usize] = 0.0;
        }
        for l9 in 0..4096 {
            self.fVec4[l9 as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec12[l10 as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fRec19[l11 as usize] = 0.0;
        }
        for l12 in 0..2 {
            self.fRec18[l12 as usize] = 0.0;
        }
        for l13 in 0..65536 {
            self.fVec5[l13 as usize] = 0.0;
        }
        for l14 in 0..4096 {
            self.fVec6[l14 as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec16[l15 as usize] = 0.0;
        }
        for l16 in 0..2 {
            self.fRec23[l16 as usize] = 0.0;
        }
        for l17 in 0..2 {
            self.fRec22[l17 as usize] = 0.0;
        }
        for l18 in 0..65536 {
            self.fVec7[l18 as usize] = 0.0;
        }
        for l19 in 0..4096 {
            self.fVec8[l19 as usize] = 0.0;
        }
        for l20 in 0..2 {
            self.fRec20[l20 as usize] = 0.0;
        }
        for l21 in 0..2 {
            self.fRec27[l21 as usize] = 0.0;
        }
        for l22 in 0..2 {
            self.fRec26[l22 as usize] = 0.0;
        }
        for l23 in 0..32768 {
            self.fVec9[l23 as usize] = 0.0;
        }
        for l24 in 0..16384 {
            self.fVec10[l24 as usize] = 0.0;
        }
        for l25 in 0..2048 {
            self.fVec11[l25 as usize] = 0.0;
        }
        for l26 in 0..2 {
            self.fRec24[l26 as usize] = 0.0;
        }
        for l27 in 0..2 {
            self.fRec31[l27 as usize] = 0.0;
        }
        for l28 in 0..2 {
            self.fRec30[l28 as usize] = 0.0;
        }
        for l29 in 0..32768 {
            self.fVec12[l29 as usize] = 0.0;
        }
        for l30 in 0..4096 {
            self.fVec13[l30 as usize] = 0.0;
        }
        for l31 in 0..2 {
            self.fRec28[l31 as usize] = 0.0;
        }
        for l32 in 0..2 {
            self.fRec35[l32 as usize] = 0.0;
        }
        for l33 in 0..2 {
            self.fRec34[l33 as usize] = 0.0;
        }
        for l34 in 0..65536 {
            self.fVec14[l34 as usize] = 0.0;
        }
        for l35 in 0..4096 {
            self.fVec15[l35 as usize] = 0.0;
        }
        for l36 in 0..2 {
            self.fRec32[l36 as usize] = 0.0;
        }
        for l37 in 0..2 {
            self.fRec39[l37 as usize] = 0.0;
        }
        for l38 in 0..2 {
            self.fRec38[l38 as usize] = 0.0;
        }
        for l39 in 0..32768 {
            self.fVec16[l39 as usize] = 0.0;
        }
        for l40 in 0..2048 {
            self.fVec17[l40 as usize] = 0.0;
        }
        for l41 in 0..2 {
            self.fRec36[l41 as usize] = 0.0;
        }
        for l42 in 0..3 {
            self.fRec0[l42 as usize] = 0.0;
        }
        for l43 in 0..3 {
            self.fRec1[l43 as usize] = 0.0;
        }
        for l44 in 0..3 {
            self.fRec2[l44 as usize] = 0.0;
        }
        for l45 in 0..3 {
            self.fRec3[l45 as usize] = 0.0;
        }
        for l46 in 0..3 {
            self.fRec4[l46 as usize] = 0.0;
        }
        for l47 in 0..3 {
            self.fRec5[l47 as usize] = 0.0;
        }
        for l48 in 0..3 {
            self.fRec6[l48 as usize] = 0.0;
        }
        for l49 in 0..3 {
            self.fRec7[l49 as usize] = 0.0;
        }
    }

    pub fn instanceConstants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = f32::min(192000.0, f32::max(1.0, self.fSampleRate as f32));
        self.fConst1 = f32::floor((0.219990999 * self.fConst0) + 0.5);
        self.fConst2 = (0.0 - (6.90775537 * self.fConst1)) / self.fConst0;
        self.fConst3 = 6.28318548 / self.fConst0;
        self.fConst4 = 3.14159274 / self.fConst0;
        self.fConst5 = f32::floor((0.0191229992 * self.fConst0) + 0.5);
        self.fConst6 = f32::max(0.0, self.fConst1 - self.fConst5);
        self.fConst7 = 0.00100000005 * self.fConst0;
        self.fConst8 = f32::max(0.0, self.fConst5 + -1.0);
        self.fConst9 = f32::floor((0.256891012 * self.fConst0) + 0.5);
        self.fConst10 = (0.0 - (6.90775537 * self.fConst9)) / self.fConst0;
        self.fConst11 = f32::floor((0.0273330007 * self.fConst0) + 0.5);
        self.fConst12 = f32::max(0.0, self.fConst9 - self.fConst11);
        self.fConst13 = f32::max(0.0, self.fConst11 + -1.0);
        self.fConst14 = f32::floor((0.192303002 * self.fConst0) + 0.5);
        self.fConst15 = (0.0 - (6.90775537 * self.fConst14)) / self.fConst0;
        self.fConst16 = f32::floor((0.0292910002 * self.fConst0) + 0.5);
        self.fConst17 = f32::max(0.0, self.fConst14 - self.fConst16);
        self.fConst18 = f32::max(0.0, self.fConst16 + -1.0);
        self.fConst19 = f32::floor((0.210389003 * self.fConst0) + 0.5);
        self.fConst20 = (0.0 - (6.90775537 * self.fConst19)) / self.fConst0;
        self.fConst21 = f32::floor((0.0244210009 * self.fConst0) + 0.5);
        self.fConst22 = f32::max(0.0, self.fConst19 - self.fConst21);
        self.fConst23 = f32::max(0.0, self.fConst21 + -1.0);
        self.fConst24 = f32::floor((0.125 * self.fConst0) + 0.5);
        self.fConst25 = (0.0 - (6.90775537 * self.fConst24)) / self.fConst0;
        self.fConst26 = f32::floor((0.0134579996 * self.fConst0) + 0.5);
        self.fConst27 = f32::max(0.0, self.fConst24 - self.fConst26);
        self.fConst28 = f32::max(0.0, self.fConst26 + -1.0);
        self.fConst29 = f32::floor((0.127837002 * self.fConst0) + 0.5);
        self.fConst30 = (0.0 - (6.90775537 * self.fConst29)) / self.fConst0;
        self.fConst31 = f32::floor((0.0316039994 * self.fConst0) + 0.5);
        self.fConst32 = f32::max(0.0, self.fConst29 - self.fConst31);
        self.fConst33 = f32::max(0.0, self.fConst31 + -1.0);
        self.fConst34 = f32::floor((0.174713001 * self.fConst0) + 0.5);
        self.fConst35 = (0.0 - (6.90775537 * self.fConst34)) / self.fConst0;
        self.fConst36 = f32::floor((0.0229039993 * self.fConst0) + 0.5);
        self.fConst37 = f32::max(0.0, self.fConst34 - self.fConst36);
        self.fConst38 = f32::max(0.0, self.fConst36 + -1.0);
        self.fConst39 = f32::floor((0.153128996 * self.fConst0) + 0.5);
        self.fConst40 = (0.0 - (6.90775537 * self.fConst39)) / self.fConst0;
        self.fConst41 = f32::floor((0.0203460008 * self.fConst0) + 0.5);
        self.fConst42 = f32::max(0.0, self.fConst39 - self.fConst41);
        self.fConst43 = f32::max(0.0, self.fConst41 + -1.0);
    }

    pub fn instanceInit(&mut self, sample_rate: i32) {
        self.instanceConstants(sample_rate);
        self.instanceClear();
    }

    pub fn setControlVariables(
        &mut self,
        rdel: f32,
        f1: u32,
        f2: u32,
        t60dc: f32,
        t60m: f32,
        fsmax: u32,
    ) {
        self.fHslider0 = t60m;
        self.fHslider1 = f2 as f32;
        self.fHslider2 = t60dc;
        self.fHslider3 = f1 as f32;
        self.fHslider4 = fsmax as f32;
        self.fHslider5 = rdel;
    }

    pub fn new(rdel: f32, f1: u32, f2: u32, t60dc: f32, t60m: f32, fsmax: u32) -> ZitaReverb {
        let mut zita_reverb = ZitaReverb::init();
        zita_reverb.instanceInit(44_100);
        zita_reverb.setControlVariables(rdel, f1, f2, t60dc, t60m, fsmax);
        zita_reverb
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> ZitaReverb {
        let rdel = node_infos.more["rdel"]
            .parse()
            .expect("rdel must be a float");
        let f1 = node_infos.more["f1"]
            .parse()
            .expect("f1 must be an integer");
        let f2 = node_infos.more["f2"]
            .parse()
            .expect("f2 must be an integer");
        let t60dc = node_infos.more["t60dc"]
            .parse()
            .expect("t60dc must be a float");
        let t60m = node_infos.more["t60m"]
            .parse()
            .expect("t60m must be a float");
        let zita_reverb = ZitaReverb::new(rdel, f1, f2, t60dc, t60m, 96_200);
        zita_reverb.check_io_node_infos(node_infos);
        zita_reverb
    }
}

impl fmt::Display for ZitaReverb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "zita_reverb({}, {}, {}, {}, {})",
            self.fHslider5, self.fHslider3, self.fHslider1, self.fHslider2, self.fHslider0
        )
    }
}

impl AudioEffect for ZitaReverb {
    fn nb_inputs(&self) -> usize {
        return 2;
    }
    fn nb_outputs(&self) -> usize {
        return 2;
    }

    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        let actual_samplerate = outputs[0].samplerate as i32;
        let input0 = inputs[0].buffer();
        let input1 = inputs[1].buffer();
        debug_assert_eq!(input0.len(), input1.len());

        // let output0 = outputs[0].buffer_mut();
        // let output1 = outputs[1].buffer_mut();
        debug_assert_eq!(outputs[0].buffer().len(), outputs[1].buffer().len());

        debug_assert_eq!(input0.len(), outputs[0].buffer().len());

        let count = input1.len();

        //Constants have to be changed if we change the samplerate...
        // We should smooth it actually...
        if self.fSampleRate != actual_samplerate {
            self.instanceInit(actual_samplerate);
        }
        let fSlow0: f32 = self.fHslider0 as f32;
        let fSlow1: f32 = f32::exp(self.fConst2 / fSlow0);
        let fSlow2: f32 = faustpower2_f(fSlow1);
        let fSlow3: f32 = f32::cos(self.fConst3 * (self.fHslider1 as f32));
        let fSlow4: f32 = 1.0 - (fSlow2 * fSlow3);
        let fSlow5: f32 = 1.0 - fSlow2;
        let fSlow6: f32 = fSlow4 / fSlow5;
        let fSlow7: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow4) / faustpower2_f(fSlow5)) + -1.0,
        ));
        let fSlow8: f32 = fSlow6 - fSlow7;
        let fSlow9: f32 = fSlow1 * (fSlow7 + (1.0 - fSlow6));
        let fSlow10: f32 = self.fHslider2 as f32;
        let fSlow11: f32 = (f32::exp(self.fConst2 / fSlow10) / fSlow1) + -1.0;
        let fSlow12: f32 = 1.0 / f32::tan(self.fConst4 * (self.fHslider3 as f32));
        let fSlow13: f32 = 1.0 / (fSlow12 + 1.0);
        let fSlow14: f32 = 1.0 - fSlow12;
        let fSlow15: f32 = self.fHslider4 as f32;
        let fSlow16: f32 = f32::floor((0.0191229992 * fSlow15) + 0.5);
        let iSlow17: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.219990999 * fSlow15) + 0.5) - fSlow16),
                    ),
                ),
            ) as i32) as f32,
            self.fConst6,
        ) as i32;
        let iSlow18: i32 = f32::min(
            8192.0,
            f32::max(0.0, self.fConst7 * (self.fHslider5 as f32)),
        ) as i32;
        let iSlow19: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow16)))) as i32) as f32,
            self.fConst8,
        ) as i32;
        let fSlow20: f32 = f32::exp(self.fConst10 / fSlow0);
        let fSlow21: f32 = faustpower2_f(fSlow20);
        let fSlow22: f32 = 1.0 - (fSlow21 * fSlow3);
        let fSlow23: f32 = 1.0 - fSlow21;
        let fSlow24: f32 = fSlow22 / fSlow23;
        let fSlow25: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow22) / faustpower2_f(fSlow23)) + -1.0,
        ));
        let fSlow26: f32 = fSlow24 - fSlow25;
        let fSlow27: f32 = fSlow20 * (fSlow25 + (1.0 - fSlow24));
        let fSlow28: f32 = (f32::exp(self.fConst10 / fSlow10) / fSlow20) + -1.0;
        let fSlow29: f32 = f32::floor((0.0273330007 * fSlow15) + 0.5);
        let iSlow30: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.256891012 * fSlow15) + 0.5) - fSlow29),
                    ),
                ),
            ) as i32) as f32,
            self.fConst12,
        ) as i32;
        let iSlow31: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow29)))) as i32) as f32,
            self.fConst13,
        ) as i32;
        let fSlow32: f32 = f32::exp(self.fConst15 / fSlow0);
        let fSlow33: f32 = faustpower2_f(fSlow32);
        let fSlow34: f32 = 1.0 - (fSlow33 * fSlow3);
        let fSlow35: f32 = 1.0 - fSlow33;
        let fSlow36: f32 = fSlow34 / fSlow35;
        let fSlow37: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow34) / faustpower2_f(fSlow35)) + -1.0,
        ));
        let fSlow38: f32 = fSlow36 - fSlow37;
        let fSlow39: f32 = fSlow32 * (fSlow37 + (1.0 - fSlow36));
        let fSlow40: f32 = (f32::exp(self.fConst15 / fSlow10) / fSlow32) + -1.0;
        let fSlow41: f32 = f32::floor((0.0292910002 * fSlow15) + 0.5);
        let iSlow42: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.192303002 * fSlow15) + 0.5) - fSlow41),
                    ),
                ),
            ) as i32) as f32,
            self.fConst17,
        ) as i32;
        let iSlow43: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow41)))) as i32) as f32,
            self.fConst18,
        ) as i32;
        let fSlow44: f32 = f32::exp(self.fConst20 / fSlow0);
        let fSlow45: f32 = faustpower2_f(fSlow44);
        let fSlow46: f32 = 1.0 - (fSlow45 * fSlow3);
        let fSlow47: f32 = 1.0 - fSlow45;
        let fSlow48: f32 = fSlow46 / fSlow47;
        let fSlow49: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow46) / faustpower2_f(fSlow47)) + -1.0,
        ));
        let fSlow50: f32 = fSlow48 - fSlow49;
        let fSlow51: f32 = fSlow44 * (fSlow49 + (1.0 - fSlow48));
        let fSlow52: f32 = (f32::exp(self.fConst20 / fSlow10) / fSlow44) + -1.0;
        let fSlow53: f32 = f32::floor((0.0244210009 * fSlow15) + 0.5);
        let iSlow54: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.210389003 * fSlow15) + 0.5) - fSlow53),
                    ),
                ),
            ) as i32) as f32,
            self.fConst22,
        ) as i32;
        let iSlow55: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow53)))) as i32) as f32,
            self.fConst23,
        ) as i32;
        let fSlow56: f32 = f32::exp(self.fConst25 / fSlow0);
        let fSlow57: f32 = faustpower2_f(fSlow56);
        let fSlow58: f32 = 1.0 - (fSlow57 * fSlow3);
        let fSlow59: f32 = 1.0 - fSlow57;
        let fSlow60: f32 = fSlow58 / fSlow59;
        let fSlow61: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow58) / faustpower2_f(fSlow59)) + -1.0,
        ));
        let fSlow62: f32 = fSlow60 - fSlow61;
        let fSlow63: f32 = fSlow56 * (fSlow61 + (1.0 - fSlow60));
        let fSlow64: f32 = (f32::exp(self.fConst25 / fSlow10) / fSlow56) + -1.0;
        let fSlow65: f32 = f32::floor((0.0134579996 * fSlow15) + 0.5);
        let iSlow66: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(1.44269502 * f32::ln(f32::floor((0.125 * fSlow15) + 0.5) - fSlow65)),
                ),
            ) as i32) as f32,
            self.fConst27,
        ) as i32;
        let iSlow67: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow65)))) as i32) as f32,
            self.fConst28,
        ) as i32;
        let fSlow68: f32 = f32::exp(self.fConst30 / fSlow0);
        let fSlow69: f32 = faustpower2_f(fSlow68);
        let fSlow70: f32 = 1.0 - (fSlow69 * fSlow3);
        let fSlow71: f32 = 1.0 - fSlow69;
        let fSlow72: f32 = fSlow70 / fSlow71;
        let fSlow73: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow70) / faustpower2_f(fSlow71)) + -1.0,
        ));
        let fSlow74: f32 = fSlow72 - fSlow73;
        let fSlow75: f32 = fSlow68 * (fSlow73 + (1.0 - fSlow72));
        let fSlow76: f32 = (f32::exp(self.fConst30 / fSlow10) / fSlow68) + -1.0;
        let fSlow77: f32 = f32::floor((0.0316039994 * fSlow15) + 0.5);
        let iSlow78: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.127837002 * fSlow15) + 0.5) - fSlow77),
                    ),
                ),
            ) as i32) as f32,
            self.fConst32,
        ) as i32;
        let iSlow79: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow77)))) as i32) as f32,
            self.fConst33,
        ) as i32;
        let fSlow80: f32 = f32::exp(self.fConst35 / fSlow0);
        let fSlow81: f32 = faustpower2_f(fSlow80);
        let fSlow82: f32 = 1.0 - (fSlow81 * fSlow3);
        let fSlow83: f32 = 1.0 - fSlow81;
        let fSlow84: f32 = fSlow82 / fSlow83;
        let fSlow85: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow82) / faustpower2_f(fSlow83)) + -1.0,
        ));
        let fSlow86: f32 = fSlow84 - fSlow85;
        let fSlow87: f32 = fSlow80 * (fSlow85 + (1.0 - fSlow84));
        let fSlow88: f32 = (f32::exp(self.fConst35 / fSlow10) / fSlow80) + -1.0;
        let fSlow89: f32 = f32::floor((0.0229039993 * fSlow15) + 0.5);
        let iSlow90: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.174713001 * fSlow15) + 0.5) - fSlow89),
                    ),
                ),
            ) as i32) as f32,
            self.fConst37,
        ) as i32;
        let iSlow91: i32 = f32::min(
            (f32::powf(2.0, f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow89)))) as i32) as f32,
            self.fConst38,
        ) as i32;
        let fSlow92: f32 = f32::exp(self.fConst40 / fSlow0);
        let fSlow93: f32 = faustpower2_f(fSlow92);
        let fSlow94: f32 = 1.0 - (fSlow93 * fSlow3);
        let fSlow95: f32 = 1.0 - fSlow93;
        let fSlow96: f32 = fSlow94 / fSlow95;
        let fSlow97: f32 = f32::sqrt(f32::max(
            0.0,
            (faustpower2_f(fSlow94) / faustpower2_f(fSlow95)) + -1.0,
        ));
        let fSlow98: f32 = fSlow96 - fSlow97;
        let fSlow99: f32 = fSlow92 * (fSlow97 + (1.0 - fSlow96));
        let fSlow100: f32 = (f32::exp(self.fConst40 / fSlow10) / fSlow92) + -1.0;
        let fSlow101: f32 = f32::floor((0.0203460008 * fSlow15) + 0.5);
        let iSlow102: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(
                    1.0,
                    f32::ceil(
                        1.44269502 * f32::ln(f32::floor((0.153128996 * fSlow15) + 0.5) - fSlow101),
                    ),
                ),
            ) as i32) as f32,
            self.fConst42,
        ) as i32;
        let iSlow103: i32 = f32::min(
            (f32::powf(
                2.0,
                f32::max(1.0, f32::ceil(1.44269502 * f32::ln(fSlow101))),
            ) as i32) as f32,
            self.fConst43,
        ) as i32;
        for i in 0..count {
            self.fRec11[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec11[1]) - (self.fRec7[1] + self.fRec7[2])));
            self.fRec10[0] =
                (fSlow8 * self.fRec10[1]) + (fSlow9 * (self.fRec7[1] + (fSlow11 * self.fRec11[0])));
            self.fVec0[(self.IOTA & 65535) as usize] =
                (0.353553385 * self.fRec10[0]) + 9.99999968e-21;
            self.fVec1[(self.IOTA & 16383) as usize] = input1[i as usize] as f32;
            let mut fTemp0: f32 =
                0.300000012 * self.fVec1[((self.IOTA - iSlow18) & 16383) as usize];
            let mut fTemp1: f32 = ((0.600000024 * self.fRec8[1])
                + self.fVec0[((self.IOTA - iSlow17) & 65535) as usize])
                - fTemp0;
            self.fVec2[(self.IOTA & 2047) as usize] = fTemp1;
            self.fRec8[0] = self.fVec2[((self.IOTA - iSlow19) & 2047) as usize];
            let mut fRec9: f32 = 0.0 - (0.600000024 * fTemp1);
            self.fRec15[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec15[1]) - (self.fRec3[1] + self.fRec3[2])));
            self.fRec14[0] = (fSlow26 * self.fRec14[1])
                + (fSlow27 * (self.fRec3[1] + (fSlow28 * self.fRec15[0])));
            self.fVec3[(self.IOTA & 65535) as usize] =
                (0.353553385 * self.fRec14[0]) + 9.99999968e-21;
            let mut fTemp2: f32 = ((0.600000024 * self.fRec12[1])
                + self.fVec3[((self.IOTA - iSlow30) & 65535) as usize])
                - fTemp0;
            self.fVec4[(self.IOTA & 4095) as usize] = fTemp2;
            self.fRec12[0] = self.fVec4[((self.IOTA - iSlow31) & 4095) as usize];
            let mut fRec13: f32 = 0.0 - (0.600000024 * fTemp2);
            self.fRec19[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec19[1]) - (self.fRec5[1] + self.fRec5[2])));
            self.fRec18[0] = (fSlow38 * self.fRec18[1])
                + (fSlow39 * (self.fRec5[1] + (fSlow40 * self.fRec19[0])));
            self.fVec5[(self.IOTA & 65535) as usize] =
                (0.353553385 * self.fRec18[0]) + 9.99999968e-21;
            let mut fTemp3: f32 = (0.600000024 * self.fRec16[1])
                + (fTemp0 + self.fVec5[((self.IOTA - iSlow42) & 65535) as usize]);
            self.fVec6[(self.IOTA & 4095) as usize] = fTemp3;
            self.fRec16[0] = self.fVec6[((self.IOTA - iSlow43) & 4095) as usize];
            let mut fRec17: f32 = 0.0 - (0.600000024 * fTemp3);
            self.fRec23[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec23[1]) - (self.fRec1[1] + self.fRec1[2])));
            self.fRec22[0] = (fSlow50 * self.fRec22[1])
                + (fSlow51 * (self.fRec1[1] + (fSlow52 * self.fRec23[0])));
            self.fVec7[(self.IOTA & 65535) as usize] =
                (0.353553385 * self.fRec22[0]) + 9.99999968e-21;
            let mut fTemp4: f32 = fTemp0
                + ((0.600000024 * self.fRec20[1])
                    + self.fVec7[((self.IOTA - iSlow54) & 65535) as usize]);
            self.fVec8[(self.IOTA & 4095) as usize] = fTemp4;
            self.fRec20[0] = self.fVec8[((self.IOTA - iSlow55) & 4095) as usize];
            let mut fRec21: f32 = 0.0 - (0.600000024 * fTemp4);
            self.fRec27[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec27[1]) - (self.fRec6[1] + self.fRec6[2])));
            self.fRec26[0] = (fSlow62 * self.fRec26[1])
                + (fSlow63 * (self.fRec6[1] + (fSlow64 * self.fRec27[0])));
            self.fVec9[(self.IOTA & 32767) as usize] =
                (0.353553385 * self.fRec26[0]) + 9.99999968e-21;
            self.fVec10[(self.IOTA & 16383) as usize] = input0[i as usize] as f32;
            let mut fTemp5: f32 =
                0.300000012 * self.fVec10[((self.IOTA - iSlow18) & 16383) as usize];
            let mut fTemp6: f32 = self.fVec9[((self.IOTA - iSlow66) & 32767) as usize]
                - (fTemp5 + (0.600000024 * self.fRec24[1]));
            self.fVec11[(self.IOTA & 2047) as usize] = fTemp6;
            self.fRec24[0] = self.fVec11[((self.IOTA - iSlow67) & 2047) as usize];
            let mut fRec25: f32 = 0.600000024 * fTemp6;
            self.fRec31[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec31[1]) - (self.fRec2[1] + self.fRec2[2])));
            self.fRec30[0] = (fSlow74 * self.fRec30[1])
                + (fSlow75 * (self.fRec2[1] + (fSlow76 * self.fRec31[0])));
            self.fVec12[(self.IOTA & 32767) as usize] =
                (0.353553385 * self.fRec30[0]) + 9.99999968e-21;
            let mut fTemp7: f32 = self.fVec12[((self.IOTA - iSlow78) & 32767) as usize]
                - (fTemp5 + (0.600000024 * self.fRec28[1]));
            self.fVec13[(self.IOTA & 4095) as usize] = fTemp7;
            self.fRec28[0] = self.fVec13[((self.IOTA - iSlow79) & 4095) as usize];
            let mut fRec29: f32 = 0.600000024 * fTemp7;
            self.fRec35[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec35[1]) - (self.fRec4[1] + self.fRec4[2])));
            self.fRec34[0] = (fSlow86 * self.fRec34[1])
                + (fSlow87 * (self.fRec4[1] + (fSlow88 * self.fRec35[0])));
            self.fVec14[(self.IOTA & 65535) as usize] =
                (0.353553385 * self.fRec34[0]) + 9.99999968e-21;
            let mut fTemp8: f32 = (fTemp5 + self.fVec14[((self.IOTA - iSlow90) & 65535) as usize])
                - (0.600000024 * self.fRec32[1]);
            self.fVec15[(self.IOTA & 4095) as usize] = fTemp8;
            self.fRec32[0] = self.fVec15[((self.IOTA - iSlow91) & 4095) as usize];
            let mut fRec33: f32 = 0.600000024 * fTemp8;
            self.fRec39[0] =
                0.0 - (fSlow13 * ((fSlow14 * self.fRec39[1]) - (self.fRec0[1] + self.fRec0[2])));
            self.fRec38[0] = (fSlow98 * self.fRec38[1])
                + (fSlow99 * (self.fRec0[1] + (fSlow100 * self.fRec39[0])));
            self.fVec16[(self.IOTA & 32767) as usize] =
                (0.353553385 * self.fRec38[0]) + 9.99999968e-21;
            let mut fTemp9: f32 = (self.fVec16[((self.IOTA - iSlow102) & 32767) as usize] + fTemp5)
                - (0.600000024 * self.fRec36[1]);
            self.fVec17[(self.IOTA & 2047) as usize] = fTemp9;
            self.fRec36[0] = self.fVec17[((self.IOTA - iSlow103) & 2047) as usize];
            let mut fRec37: f32 = 0.600000024 * fTemp9;
            let mut fTemp10: f32 = self.fRec12[1] + self.fRec8[1];
            let mut fTemp11: f32 = self.fRec32[1] + (self.fRec36[1] + fTemp10);
            self.fRec0[0] = fRec9
                + (fRec13
                    + (fRec17
                        + (fRec21
                            + (fRec25
                                + (fRec29
                                    + (fRec33
                                        + (fRec37
                                            + (self.fRec16[1]
                                                + (self.fRec20[1]
                                                    + (self.fRec24[1]
                                                        + (self.fRec28[1] + fTemp11)))))))))));
            let mut fTemp12: f32 = self.fRec36[1] + self.fRec32[1];
            self.fRec1[0] = (fRec25
                + (fRec29 + (fRec33 + (fRec37 + (self.fRec24[1] + (fTemp12 + self.fRec28[1]))))))
                - (fRec9
                    + (fRec13
                        + (fRec17 + (fRec21 + (self.fRec16[1] + (self.fRec20[1] + fTemp10))))));
            self.fRec2[0] = (fRec17
                + (fRec21 + (fRec33 + (fRec37 + (self.fRec16[1] + (fTemp12 + self.fRec20[1]))))))
                - (fRec9
                    + (fRec13
                        + (fRec25 + (fRec29 + (self.fRec24[1] + (self.fRec28[1] + fTemp10))))));
            self.fRec3[0] = (fRec9 + (fRec13 + (fRec33 + (fRec37 + fTemp11))))
                - (fRec17
                    + (fRec21
                        + (fRec25
                            + (fRec29
                                + (self.fRec16[1]
                                    + ((self.fRec28[1] + self.fRec24[1]) + self.fRec20[1]))))));
            let mut fTemp13: f32 = self.fRec36[1] + self.fRec12[1];
            let mut fTemp14: f32 = self.fRec32[1] + self.fRec8[1];
            self.fRec4[0] = (fRec13
                + (fRec21 + (fRec29 + (fRec37 + (self.fRec20[1] + (self.fRec28[1] + fTemp13))))))
                - (fRec9
                    + (fRec17
                        + (fRec25 + (fRec33 + (self.fRec16[1] + (self.fRec24[1] + fTemp14))))));
            let mut fTemp15: f32 = self.fRec36[1] + self.fRec8[1];
            let mut fTemp16: f32 = self.fRec32[1] + self.fRec12[1];
            self.fRec5[0] = (fRec9
                + (fRec17 + (fRec29 + (fRec37 + (self.fRec16[1] + (self.fRec28[1] + fTemp15))))))
                - (fRec13
                    + (fRec21
                        + (fRec25 + (fRec33 + (self.fRec20[1] + (self.fRec24[1] + fTemp16))))));
            self.fRec6[0] = (fRec9
                + (fRec21 + (fRec25 + (fRec37 + (self.fRec20[1] + (self.fRec24[1] + fTemp15))))))
                - (fRec13
                    + (fRec17
                        + (fRec29 + (fRec33 + (self.fRec16[1] + (self.fRec28[1] + fTemp16))))));
            self.fRec7[0] = (fRec13
                + (fRec17 + (fRec25 + (fRec37 + (self.fRec16[1] + (self.fRec24[1] + fTemp13))))))
                - (fRec9
                    + (fRec21
                        + (fRec29 + (fRec33 + (self.fRec20[1] + (self.fRec28[1] + fTemp14))))));
            outputs[0].buffer_mut()[i as usize] =
                (0.370000005 * (self.fRec1[0] + self.fRec2[0])) as f32;
            outputs[1].buffer_mut()[i as usize] =
                (0.370000005 * (self.fRec1[0] - self.fRec2[0])) as f32;
            self.fRec11[1] = self.fRec11[0];
            self.fRec10[1] = self.fRec10[0];
            self.IOTA = self.IOTA + 1;
            self.fRec8[1] = self.fRec8[0];
            self.fRec15[1] = self.fRec15[0];
            self.fRec14[1] = self.fRec14[0];
            self.fRec12[1] = self.fRec12[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            self.fRec16[1] = self.fRec16[0];
            self.fRec23[1] = self.fRec23[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec20[1] = self.fRec20[0];
            self.fRec27[1] = self.fRec27[0];
            self.fRec26[1] = self.fRec26[0];
            self.fRec24[1] = self.fRec24[0];
            self.fRec31[1] = self.fRec31[0];
            self.fRec30[1] = self.fRec30[0];
            self.fRec28[1] = self.fRec28[0];
            self.fRec35[1] = self.fRec35[0];
            self.fRec34[1] = self.fRec34[0];
            self.fRec32[1] = self.fRec32[0];
            self.fRec39[1] = self.fRec39[0];
            self.fRec38[1] = self.fRec38[0];
            self.fRec36[1] = self.fRec36[0];
            self.fRec0[2] = self.fRec0[1];
            self.fRec0[1] = self.fRec0[0];
            self.fRec1[2] = self.fRec1[1];
            self.fRec1[1] = self.fRec1[0];
            self.fRec2[2] = self.fRec2[1];
            self.fRec2[1] = self.fRec2[0];
            self.fRec3[2] = self.fRec3[1];
            self.fRec3[1] = self.fRec3[0];
            self.fRec4[2] = self.fRec4[1];
            self.fRec4[1] = self.fRec4[0];
            self.fRec5[2] = self.fRec5[1];
            self.fRec5[1] = self.fRec5[0];
            self.fRec6[2] = self.fRec6[1];
            self.fRec6[1] = self.fRec6[0];
            self.fRec7[2] = self.fRec7[1];
            self.fRec7[1] = self.fRec7[0];
        }
    }
}

/****************************************
** Mono freeverb
****************************************/

pub struct MonoFreeverb {
    fDummy: f32,
    fHslider0: f32,
    fHslider1: f32,
    fRec9: [f32; 2],
    IOTA: i32,
    fVec0: [f32; 8192],
    fSampleRate: i32,
    fConst0: f32,
    fConst1: f32,
    fHslider2: f32,
    fRec8: [f32; 2],
    fRec11: [f32; 2],
    fVec1: [f32; 8192],
    fConst2: f32,
    fRec10: [f32; 2],
    fRec13: [f32; 2],
    fVec2: [f32; 8192],
    fConst3: f32,
    fRec12: [f32; 2],
    fRec15: [f32; 2],
    fVec3: [f32; 8192],
    fConst4: f32,
    fRec14: [f32; 2],
    fRec17: [f32; 2],
    fVec4: [f32; 8192],
    fConst5: f32,
    fRec16: [f32; 2],
    fRec19: [f32; 2],
    fVec5: [f32; 8192],
    fConst6: f32,
    fRec18: [f32; 2],
    fRec21: [f32; 2],
    fVec6: [f32; 8192],
    fConst7: f32,
    fRec20: [f32; 2],
    fRec23: [f32; 2],
    fVec7: [f32; 8192],
    fConst8: f32,
    fRec22: [f32; 2],
    fHslider3: f32,
    fVec8: [f32; 2048],
    fConst9: f32,
    fRec6: [f32; 2],
    fVec9: [f32; 2048],
    fConst10: f32,
    fRec4: [f32; 2],
    fVec10: [f32; 2048],
    fConst11: f32,
    fRec2: [f32; 2],
    fVec11: [f32; 1024],
    fConst12: f32,
    fRec0: [f32; 2],
}

impl MonoFreeverb {
    pub fn init() -> MonoFreeverb {
        MonoFreeverb {
            fDummy: 0 as f32,
            fHslider0: 0.0,
            fHslider1: 0.0,
            fRec9: [0.0; 2],
            IOTA: 0,
            fVec0: [0.0; 8192],
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fHslider2: 0.0,
            fRec8: [0.0; 2],
            fRec11: [0.0; 2],
            fVec1: [0.0; 8192],
            fConst2: 0.0,
            fRec10: [0.0; 2],
            fRec13: [0.0; 2],
            fVec2: [0.0; 8192],
            fConst3: 0.0,
            fRec12: [0.0; 2],
            fRec15: [0.0; 2],
            fVec3: [0.0; 8192],
            fConst4: 0.0,
            fRec14: [0.0; 2],
            fRec17: [0.0; 2],
            fVec4: [0.0; 8192],
            fConst5: 0.0,
            fRec16: [0.0; 2],
            fRec19: [0.0; 2],
            fVec5: [0.0; 8192],
            fConst6: 0.0,
            fRec18: [0.0; 2],
            fRec21: [0.0; 2],
            fVec6: [0.0; 8192],
            fConst7: 0.0,
            fRec20: [0.0; 2],
            fRec23: [0.0; 2],
            fVec7: [0.0; 8192],
            fConst8: 0.0,
            fRec22: [0.0; 2],
            fHslider3: 0.0,
            fVec8: [0.0; 2048],
            fConst9: 0.0,
            fRec6: [0.0; 2],
            fVec9: [0.0; 2048],
            fConst10: 0.0,
            fRec4: [0.0; 2],
            fVec10: [0.0; 2048],
            fConst11: 0.0,
            fRec2: [0.0; 2],
            fVec11: [0.0; 1024],
            fConst12: 0.0,
            fRec0: [0.0; 2],
        }
    }

    pub fn instanceResetUserInterface(&mut self) {
        self.fHslider0 = 0.5;
        self.fHslider1 = 0.5;
        self.fHslider2 = 0.5;
        self.fHslider3 = 0.5;
    }

    pub fn instanceClear(&mut self) {
        for l0 in 0..2 {
            self.fRec9[l0 as usize] = 0.0;
        }
        self.IOTA = 0;
        for l1 in 0..8192 {
            self.fVec0[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.fRec8[l2 as usize] = 0.0;
        }
        for l3 in 0..2 {
            self.fRec11[l3 as usize] = 0.0;
        }
        for l4 in 0..8192 {
            self.fVec1[l4 as usize] = 0.0;
        }
        for l5 in 0..2 {
            self.fRec10[l5 as usize] = 0.0;
        }
        for l6 in 0..2 {
            self.fRec13[l6 as usize] = 0.0;
        }
        for l7 in 0..8192 {
            self.fVec2[l7 as usize] = 0.0;
        }
        for l8 in 0..2 {
            self.fRec12[l8 as usize] = 0.0;
        }
        for l9 in 0..2 {
            self.fRec15[l9 as usize] = 0.0;
        }
        for l10 in 0..8192 {
            self.fVec3[l10 as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fRec14[l11 as usize] = 0.0;
        }
        for l12 in 0..2 {
            self.fRec17[l12 as usize] = 0.0;
        }
        for l13 in 0..8192 {
            self.fVec4[l13 as usize] = 0.0;
        }
        for l14 in 0..2 {
            self.fRec16[l14 as usize] = 0.0;
        }
        for l15 in 0..2 {
            self.fRec19[l15 as usize] = 0.0;
        }
        for l16 in 0..8192 {
            self.fVec5[l16 as usize] = 0.0;
        }
        for l17 in 0..2 {
            self.fRec18[l17 as usize] = 0.0;
        }
        for l18 in 0..2 {
            self.fRec21[l18 as usize] = 0.0;
        }
        for l19 in 0..8192 {
            self.fVec6[l19 as usize] = 0.0;
        }
        for l20 in 0..2 {
            self.fRec20[l20 as usize] = 0.0;
        }
        for l21 in 0..2 {
            self.fRec23[l21 as usize] = 0.0;
        }
        for l22 in 0..8192 {
            self.fVec7[l22 as usize] = 0.0;
        }
        for l23 in 0..2 {
            self.fRec22[l23 as usize] = 0.0;
        }
        for l24 in 0..2048 {
            self.fVec8[l24 as usize] = 0.0;
        }
        for l25 in 0..2 {
            self.fRec6[l25 as usize] = 0.0;
        }
        for l26 in 0..2048 {
            self.fVec9[l26 as usize] = 0.0;
        }
        for l27 in 0..2 {
            self.fRec4[l27 as usize] = 0.0;
        }
        for l28 in 0..2048 {
            self.fVec10[l28 as usize] = 0.0;
        }
        for l29 in 0..2 {
            self.fRec2[l29 as usize] = 0.0;
        }
        for l30 in 0..1024 {
            self.fVec11[l30 as usize] = 0.0;
        }
        for l31 in 0..2 {
            self.fRec0[l31 as usize] = 0.0;
        }
    }

    pub fn instanceConstants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = f32::min(192000.0, f32::max(1.0, self.fSampleRate as f32));
        self.fConst1 = ((0.0253061224 * self.fConst0) as i32) as f32;
        self.fConst2 = ((0.0269387756 * self.fConst0) as i32) as f32;
        self.fConst3 = ((0.0289569162 * self.fConst0) as i32) as f32;
        self.fConst4 = ((0.0307482984 * self.fConst0) as i32) as f32;
        self.fConst5 = ((0.0322448984 * self.fConst0) as i32) as f32;
        self.fConst6 = ((0.033809524 * self.fConst0) as i32) as f32;
        self.fConst7 = ((0.0353061222 * self.fConst0) as i32) as f32;
        self.fConst8 = ((0.0366666652 * self.fConst0) as i32) as f32;
        self.fConst9 = ((0.0126077095 * self.fConst0) as i32) as f32;
        self.fConst10 = ((0.00999999978 * self.fConst0) as i32) as f32;
        self.fConst11 = ((0.00773242628 * self.fConst0) as i32) as f32;
        self.fConst12 = ((0.00510204071 * self.fConst0) as i32) as f32;
    }

    pub fn instanceInit(&mut self, sample_rate: i32) {
        self.instanceConstants(sample_rate);
        self.instanceResetUserInterface();
        self.instanceClear();
    }

    pub fn new(fb1: f32, fb2: f32, damp: f32, spread: f32) -> MonoFreeverb {
        let mut mono_freeverb = MonoFreeverb::init();
        mono_freeverb.instanceInit(44_100);
        mono_freeverb.setControlVariables(fb1, fb2, damp, spread);
        mono_freeverb
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> MonoFreeverb {
        let fb1 = node_infos.more["fb1"]
            .parse()
            .expect("fb1 must be a float in [0,1]");
        let fb2 = node_infos.more["fb2"]
            .parse()
            .expect("fb2 must be a float in [0,1]");
        let damp = node_infos.more["damp"]
            .parse()
            .expect("damp must be a float in [0,1]");
        let spread = node_infos.more["spread"]
            .parse()
            .expect("spread must be a float in [0,1]");
        let mono_freeverb = MonoFreeverb::new(fb1, fb2, damp, spread);
        mono_freeverb.check_io_node_infos(node_infos);
        mono_freeverb
    }

    pub fn setControlVariables(&mut self, fb1: f32, fb2: f32, damp: f32, spread: f32) {
        self.fHslider0 = fb1;
        self.fHslider1 = damp;
        self.fHslider2 = spread;
        self.fHslider3 = fb2;
    }
}

impl fmt::Display for MonoFreeverb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "freeverb({}, {}, {}, {})",
            self.fHslider0, self.fHslider3, self.fHslider1, self.fHslider2
        )
    }
}

impl AudioEffect for MonoFreeverb {
    fn nb_inputs(&self) -> usize {
        return 1;
    }
    fn nb_outputs(&self) -> usize {
        return 1;
    }

    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        let actual_samplerate = outputs[0].samplerate as i32;

        let input = inputs[0].buffer();
        let output = outputs[0].buffer_mut();
        let count = output.len();

        if self.fSampleRate != actual_samplerate {
            self.instanceInit(actual_samplerate);
        }

        let fSlow0: f32 = self.fHslider0 as f32;
        let fSlow1: f32 = self.fHslider1 as f32;
        let fSlow2: f32 = 1.0 - fSlow1;
        let fSlow3: f32 = self.fHslider2 as f32;
        let iSlow4: i32 = (self.fConst1 + fSlow3) as i32;
        let iSlow5: i32 = (self.fConst2 + fSlow3) as i32;
        let iSlow6: i32 = (self.fConst3 + fSlow3) as i32;
        let iSlow7: i32 = (self.fConst4 + fSlow3) as i32;
        let iSlow8: i32 = (self.fConst5 + fSlow3) as i32;
        let iSlow9: i32 = (self.fConst6 + fSlow3) as i32;
        let iSlow10: i32 = (self.fConst7 + fSlow3) as i32;
        let iSlow11: i32 = (self.fConst8 + fSlow3) as i32;
        let fSlow12: f32 = self.fHslider3 as f32;
        let fSlow13: f32 = fSlow3 + -1.0;
        let iSlow14: i32 = f32::min(1024.0, f32::max(0.0, self.fConst9 + fSlow13)) as i32;
        let iSlow15: i32 = f32::min(1024.0, f32::max(0.0, self.fConst10 + fSlow13)) as i32;
        let iSlow16: i32 = f32::min(1024.0, f32::max(0.0, self.fConst11 + fSlow13)) as i32;
        let iSlow17: i32 = f32::min(1024.0, f32::max(0.0, self.fConst12 + fSlow13)) as i32;
        for i in 0..count {
            let mut fTemp0: f32 = input[i as usize] as f32;
            self.fRec9[0] = (fSlow1 * self.fRec9[1]) + (fSlow2 * self.fRec8[1]);
            self.fVec0[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec9[0]);
            self.fRec8[0] = self.fVec0[((self.IOTA - iSlow4) & 8191) as usize];
            self.fRec11[0] = (fSlow1 * self.fRec11[1]) + (fSlow2 * self.fRec10[1]);
            self.fVec1[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec11[0]);
            self.fRec10[0] = self.fVec1[((self.IOTA - iSlow5) & 8191) as usize];
            self.fRec13[0] = (fSlow1 * self.fRec13[1]) + (fSlow2 * self.fRec12[1]);
            self.fVec2[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec13[0]);
            self.fRec12[0] = self.fVec2[((self.IOTA - iSlow6) & 8191) as usize];
            self.fRec15[0] = (fSlow1 * self.fRec15[1]) + (fSlow2 * self.fRec14[1]);
            self.fVec3[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec15[0]);
            self.fRec14[0] = self.fVec3[((self.IOTA - iSlow7) & 8191) as usize];
            self.fRec17[0] = (fSlow1 * self.fRec17[1]) + (fSlow2 * self.fRec16[1]);
            self.fVec4[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec17[0]);
            self.fRec16[0] = self.fVec4[((self.IOTA - iSlow8) & 8191) as usize];
            self.fRec19[0] = (fSlow1 * self.fRec19[1]) + (fSlow2 * self.fRec18[1]);
            self.fVec5[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec19[0]);
            self.fRec18[0] = self.fVec5[((self.IOTA - iSlow9) & 8191) as usize];
            self.fRec21[0] = (fSlow1 * self.fRec21[1]) + (fSlow2 * self.fRec20[1]);
            self.fVec6[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec21[0]);
            self.fRec20[0] = self.fVec6[((self.IOTA - iSlow10) & 8191) as usize];
            self.fRec23[0] = (fSlow1 * self.fRec23[1]) + (fSlow2 * self.fRec22[1]);
            self.fVec7[(self.IOTA & 8191) as usize] = fTemp0 + (fSlow0 * self.fRec23[0]);
            self.fRec22[0] = self.fVec7[((self.IOTA - iSlow11) & 8191) as usize];
            let mut fTemp1: f32 = (((((((self.fRec8[0] + self.fRec10[0]) + self.fRec12[0])
                + self.fRec14[0])
                + self.fRec16[0])
                + self.fRec18[0])
                + self.fRec20[0])
                + self.fRec22[0])
                + (fSlow12 * self.fRec6[1]);
            self.fVec8[(self.IOTA & 2047) as usize] = fTemp1;
            self.fRec6[0] = self.fVec8[((self.IOTA - iSlow14) & 2047) as usize];
            let mut fRec7: f32 = 0.0 - (fSlow12 * fTemp1);
            let mut fTemp2: f32 = self.fRec6[1] + (fRec7 + (fSlow12 * self.fRec4[1]));
            self.fVec9[(self.IOTA & 2047) as usize] = fTemp2;
            self.fRec4[0] = self.fVec9[((self.IOTA - iSlow15) & 2047) as usize];
            let mut fRec5: f32 = 0.0 - (fSlow12 * fTemp2);
            let mut fTemp3: f32 = self.fRec4[1] + (fRec5 + (fSlow12 * self.fRec2[1]));
            self.fVec10[(self.IOTA & 2047) as usize] = fTemp3;
            self.fRec2[0] = self.fVec10[((self.IOTA - iSlow16) & 2047) as usize];
            let mut fRec3: f32 = 0.0 - (fSlow12 * fTemp3);
            let mut fTemp4: f32 = self.fRec2[1] + (fRec3 + (fSlow12 * self.fRec0[1]));
            self.fVec11[(self.IOTA & 1023) as usize] = fTemp4;
            self.fRec0[0] = self.fVec11[((self.IOTA - iSlow17) & 1023) as usize];
            let mut fRec1: f32 = 0.0 - (fSlow12 * fTemp4);
            output[i as usize] = (fRec1 + self.fRec0[1]) as f32;
            self.fRec9[1] = self.fRec9[0];
            self.IOTA = self.IOTA + 1;
            self.fRec8[1] = self.fRec8[0];
            self.fRec11[1] = self.fRec11[0];
            self.fRec10[1] = self.fRec10[0];
            self.fRec13[1] = self.fRec13[0];
            self.fRec12[1] = self.fRec12[0];
            self.fRec15[1] = self.fRec15[0];
            self.fRec14[1] = self.fRec14[0];
            self.fRec17[1] = self.fRec17[0];
            self.fRec16[1] = self.fRec16[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            self.fRec21[1] = self.fRec21[0];
            self.fRec20[1] = self.fRec20[0];
            self.fRec23[1] = self.fRec23[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec6[1] = self.fRec6[0];
            self.fRec4[1] = self.fRec4[0];
            self.fRec2[1] = self.fRec2[0];
            self.fRec0[1] = self.fRec0[0];
        }
    }
}

/****************************************
** Compressor
****************************************/

pub struct Compressor {
    fDummy: f32,
    fSampleRate: i32,
    fConst0: f32,
    fConst1: f32,
    fHslider0: f32,
    fHslider1: f32,
    fConst2: f32,
    fHslider2: f32,
    fRec2: [f32; 2],
    fRec1: [f32; 2],
    fHslider3: f32,
    fRec0: [f32; 2],
}

impl Compressor {
    pub fn init() -> Compressor {
        Compressor {
            fDummy: 0 as f32,
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fHslider0: 0.0,
            fHslider1: 0.0,
            fConst2: 0.0,
            fHslider2: 0.0,
            fRec2: [0.0; 2],
            fRec1: [0.0; 2],
            fHslider3: 0.0,
            fRec0: [0.0; 2],
        }
    }

    pub fn instanceResetUserInterface(&mut self) {
        self.fHslider0 = 0.5;
        self.fHslider1 = 2.0;
        self.fHslider2 = 1.0;
        self.fHslider3 = 20.0;
    }

    pub fn instanceClear(&mut self) {
        for l0 in 0..2 {
            self.fRec2[l0 as usize] = 0.0;
        }
        for l1 in 0..2 {
            self.fRec1[l1 as usize] = 0.0;
        }
        for l2 in 0..2 {
            self.fRec0[l2 as usize] = 0.0;
        }
    }

    pub fn instanceConstants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = f32::min(192000.0, f32::max(1.0, self.fSampleRate as f32));
        self.fConst1 = 2.0 / self.fConst0;
        self.fConst2 = 1.0 / self.fConst0;
    }

    pub fn instanceInit(&mut self, sample_rate: i32) {
        self.instanceConstants(sample_rate);
        self.instanceResetUserInterface();
        self.instanceClear();
    }

    pub fn new(ratio: f32, thresh: f32, att: f32, rel: f32) -> Compressor {
        let mut compressor = Compressor::init();
        compressor.instanceInit(44_100);
        compressor.setControlVariables(ratio, thresh, att, rel);
        compressor
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Compressor {
        let ratio = node_infos.more["ratio"]
            .parse()
            .expect("ratio must be a float");
        let thresh = node_infos.more["thresh"]
            .parse()
            .expect("thresh must be a float");
        let att = node_infos.more["att"].parse().expect("att must be a float");
        let rel = node_infos.more["rel"].parse().expect("rel must be a float");
        let compressor = Compressor::new(ratio, thresh, att, rel);
        compressor.check_io_node_infos(node_infos);
        compressor
    }

    pub fn setControlVariables(&mut self, ratio: f32, thresh: f32, att: f32, rel: f32) {
        self.fHslider0 = att;
        self.fHslider1 = ratio;
        self.fHslider2 = rel;
        self.fHslider3 = thresh;
    }
}

impl fmt::Display for Compressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "compressor({}, {}, {}, {})",
            self.fHslider1, self.fHslider3, self.fHslider0, self.fHslider2
        )
    }
}

impl AudioEffect for Compressor {
    fn nb_inputs(&self) -> usize {
        return 1;
    }
    fn nb_outputs(&self) -> usize {
        return 1;
    }
    fn process(&mut self, inputs: &[DspEdge], outputs: &mut [DspEdge]) {
        debug_assert_eq!(inputs.len(), self.nb_inputs());
        debug_assert_eq!(outputs.len(), self.nb_outputs());
        let actual_samplerate = outputs[0].samplerate as i32;

        let input = inputs[0].buffer();
        let output = outputs[0].buffer_mut();
        let count = output.len();

        if self.fSampleRate != actual_samplerate {
            self.instanceInit(actual_samplerate);
        }

        let fSlow0: f32 = self.fHslider0 as f32;
        let fSlow1: f32 = f32::exp(0.0 - (self.fConst1 / fSlow0));
        let fSlow2: f32 = ((1.0 / (self.fHslider1 as f32)) + -1.0) * (1.0 - fSlow1);
        let fSlow3: f32 = f32::exp(0.0 - (self.fConst2 / fSlow0));
        let fSlow4: f32 = f32::exp(0.0 - (self.fConst2 / (self.fHslider2 as f32)));
        let fSlow5: f32 = self.fHslider3 as f32;
        for i in 0..count {
            let mut fTemp0: f32 = input[i as usize] as f32;
            let mut fTemp1: f32 = f32::abs(fTemp0);
            let mut fTemp2: f32 = if ((self.fRec1[1] > fTemp1) as i32) as i32 == 1 {
                fSlow4
            } else {
                fSlow3
            };
            self.fRec2[0] = (self.fRec2[1] * fTemp2) + (fTemp1 * (1.0 - fTemp2));
            self.fRec1[0] = self.fRec2[0];
            self.fRec0[0] = (fSlow1 * self.fRec0[1])
                + (fSlow2 * f32::max((20.0 * f32::log10(self.fRec1[0])) - fSlow5, 0.0));
            output[i as usize] = (fTemp0 * f32::powf(10.0, 0.0500000007 * self.fRec0[0])) as f32;
            self.fRec2[1] = self.fRec2[0];
            self.fRec1[1] = self.fRec1[0];
            self.fRec0[1] = self.fRec0[0];
        }
    }
}
