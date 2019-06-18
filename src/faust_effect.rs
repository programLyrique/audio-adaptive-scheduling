//! Audio effects
//! Generated using the rust backend from Faust code

use audiograph::*;
use audiograph_parser;

use std::fmt;

use amath;

pub fn Guitar_faustpower2_f(value: f32) -> f32 {
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
        let fSlow59: f32 = 2.0 * (1.0 - (1.0 / Guitar_faustpower2_f(fSlow53)));
        let fSlow60: f32 = self.fButton0 as f32;
        let fSlow61: f32 = Guitar_faustpower2_f(1.0 - (0.113333337 / fSlow0));
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
            for j0 in (4..0) {
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
            for j1 in (4..0) {
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
                    * self.fVec0[((self.IOTA
                        - std::cmp::min(65537, std::cmp::max(0, iTemp1 + 1)))
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
