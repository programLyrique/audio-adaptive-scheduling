//! Audio effects
//! Generated using the rust backend from Faust code

use audiograph::*;
use audiograph_parser;

use std::fmt;

pub fn Guitar_faustpower2_f(value: f32) -> f32 {
    return (value * value);
}

pub struct Guitar {
    fDummy: f32,
    iRec10: [i32; 2],
    iVec0: [i32; 2],
    fSampleRate: i32,
    fConst0: f32,
    fVec1: [f32; 2],
    fConst1: f32,
    fConst2: f32,

    fConst3: f32,
    fConst4: f32,
    fConst5: f32,
    fConst6: f32,
    fConst7: f32,
    fConst8: f32,
    fConst9: f32,
    iConst10: i32,
    iConst11: i32,
    iConst12: i32,
    fConst13: f32,
    fConst14: f32,
    iConst15: i32,
    iConst16: i32,
    fConst17: f32,
    iConst18: i32,
    iConst19: i32,
    fConst20: f32,
    fConst21: f32,
    iConst22: i32,
    iConst23: i32,
    fConst24: f32,
    iConst25: i32,
    iConst26: i32,
    fRec22: [f32; 2],
    fRec25: [f32; 2],
    fConst27: f32,
    fConst28: f32,
    fConst29: f32,
    fConst30: f32,
    fConst31: f32,
    fConst32: f32,
    fConst33: f32,
    fConst34: f32,
    fRec27: [f32; 4],
    IOTA: i32,
    fRec28: [f32; 256],
    iConst35: i32,
    iConst36: i32,
    iConst37: i32,
    fConst38: f32,
    fConst39: f32,
    iConst40: i32,
    iConst41: i32,
    fConst42: f32,
    iConst43: i32,
    iConst44: i32,
    fConst45: f32,
    fConst46: f32,
    iConst47: i32,
    iConst48: i32,
    fConst49: f32,
    iConst50: i32,
    iConst51: i32,
    fVec2: [f32; 2],
    fConst52: f32,
    fConst53: f32,
    fConst54: f32,
    fConst55: f32,
    iRec30: [i32; 2],
    fConst56: f32,
    fConst57: f32,
    fConst58: f32,
    fRec29: [f32; 3],
    fConst59: f32,
    fRec31: [f32; 2],
    fConst60: f32,
    fConst61: f32,
    fVec3: [f32; 2],
    fRec26: [f32; 512],
    fRec19: [f32; 2],
    fRec16: [f32; 256],
    iConst62: i32,
    iConst63: i32,
    iConst64: i32,
    iConst65: i32,
    iConst66: i32,
    fRec18: [f32; 2],
    fRec15: [f32; 4],
    iRec6: [i32; 2],
    fRec2: [f32; 512],
    fRec0: [f32; 2],
}

impl Guitar {
    pub fn init() -> Guitar {
        Guitar {
            fDummy: 0 as f32,
            iRec10: [0; 2],
            iVec0: [0; 2],
            fSampleRate: 0,
            fConst0: 0.0,
            fVec1: [0.0; 2],
            fConst1: 0.0,
            fConst2: 0.0,
            fConst3: 0.0,
            fConst4: 0.0,
            fConst5: 0.0,
            fConst6: 0.0,
            fConst7: 0.0,
            fConst8: 0.0,
            fConst9: 0.0,
            iConst10: 0,
            iConst11: 0,
            iConst12: 0,
            fConst13: 0.0,
            fConst14: 0.0,
            iConst15: 0,
            iConst16: 0,
            fConst17: 0.0,
            iConst18: 0,
            iConst19: 0,
            fConst20: 0.0,
            fConst21: 0.0,
            iConst22: 0,
            iConst23: 0,
            fConst24: 0.0,
            iConst25: 0,
            iConst26: 0,
            fRec22: [0.0; 2],
            fRec25: [0.0; 2],
            fConst27: 0.0,
            fConst28: 0.0,
            fConst29: 0.0,
            fConst30: 0.0,
            fConst31: 0.0,
            fConst32: 0.0,
            fConst33: 0.0,
            fConst34: 0.0,
            fRec27: [0.0; 4],
            IOTA: 0,
            fRec28: [0.0; 256],
            iConst35: 0,
            iConst36: 0,
            iConst37: 0,
            fConst38: 0.0,
            fConst39: 0.0,
            iConst40: 0,
            iConst41: 0,
            fConst42: 0.0,
            iConst43: 0,
            iConst44: 0,
            fConst45: 0.0,
            fConst46: 0.0,
            iConst47: 0,
            iConst48: 0,
            fConst49: 0.0,
            iConst50: 0,
            iConst51: 0,
            fVec2: [0.0; 2],
            fConst52: 0.0,
            fConst53: 0.0,
            fConst54: 0.0,
            fConst55: 0.0,
            iRec30: [0; 2],
            fConst56: 0.0,
            fConst57: 0.0,
            fConst58: 0.0,
            fRec29: [0.0; 3],
            fConst59: 0.0,
            fRec31: [0.0; 2],
            fConst60: 0.0,
            fConst61: 0.0,
            fVec3: [0.0; 2],
            fRec26: [0.0; 512],
            fRec19: [0.0; 2],
            fRec16: [0.0; 256],
            iConst62: 0,
            iConst63: 0,
            iConst64: 0,
            iConst65: 0,
            iConst66: 0,
            fRec18: [0.0; 2],
            fRec15: [0.0; 4],
            iRec6: [0; 2],
            fRec2: [0.0; 512],
            fRec0: [0.0; 2],
        }
    }

    pub fn instanceClear(&mut self) {
        for l0 in 0..2 {
            self.iRec10[l0 as usize] = 0;
        }
        for l1 in 0..2 {
            self.iVec0[l1 as usize] = 0;
        }
        for l2 in 0..2 {
            self.fVec1[l2 as usize] = 0.0;
        }
        for l3 in 0..2 {
            self.fRec22[l3 as usize] = 0.0;
        }
        for l4 in 0..2 {
            self.fRec25[l4 as usize] = 0.0;
        }
        for l5 in 0..4 {
            self.fRec27[l5 as usize] = 0.0;
        }
        self.IOTA = 0;
        for l6 in 0..256 {
            self.fRec28[l6 as usize] = 0.0;
        }
        for l7 in 0..2 {
            self.fVec2[l7 as usize] = 0.0;
        }
        for l8 in 0..2 {
            self.iRec30[l8 as usize] = 0;
        }
        for l9 in 0..3 {
            self.fRec29[l9 as usize] = 0.0;
        }
        for l10 in 0..2 {
            self.fRec31[l10 as usize] = 0.0;
        }
        for l11 in 0..2 {
            self.fVec3[l11 as usize] = 0.0;
        }
        for l12 in 0..512 {
            self.fRec26[l12 as usize] = 0.0;
        }
        for l13 in 0..2 {
            self.fRec19[l13 as usize] = 0.0;
        }
        for l14 in 0..256 {
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
        for l18 in 0..512 {
            self.fRec2[l18 as usize] = 0.0;
        }
        for l19 in 0..2 {
            self.fRec0[l19 as usize] = 0.0;
        }
    }

    pub fn instanceConstants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = f32::min(192000.0, f32::max(1.0, (self.fSampleRate as f32)));
        self.fConst1 = (0.00195588241 * self.fConst0);
        self.fConst2 = (self.fConst1 + -1.49999499);
        self.fConst3 = f32::floor(self.fConst2);
        self.fConst4 = (self.fConst1 + (-1.0 - self.fConst3));
        self.fConst5 = (self.fConst1 + (-2.0 - self.fConst3));
        self.fConst6 = (self.fConst1 + (-3.0 - self.fConst3));
        self.fConst7 = (self.fConst1 + (-4.0 - self.fConst3));
        self.fConst8 = ((((0.0 - self.fConst4) * (0.0 - (0.5 * self.fConst5)))
            * (0.0 - (0.333333343 * self.fConst6)))
            * (0.0 - (0.25 * self.fConst7)));
        self.fConst9 = (0.00882352982 * self.fConst0);
        self.iConst10 = (self.fConst2 as i32);
        self.iConst11 = (f32::min(self.fConst9, (std::cmp::max(0, self.iConst10) as f32)) as i32);
        self.iConst12 = (self.iConst11 + 1);
        self.fConst13 = (self.fConst1 - self.fConst3);
        self.fConst14 = (((0.0 - self.fConst5) * (0.0 - (0.5 * self.fConst6)))
            * (0.0 - (0.333333343 * self.fConst7)));
        self.iConst15 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst10 + 1)) as f32)) as i32);
        self.iConst16 = (self.iConst15 + 1);
        self.fConst17 =
            (0.5 * ((self.fConst4 * (0.0 - self.fConst6)) * (0.0 - (0.5 * self.fConst7))));
        self.iConst18 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst10 + 2)) as f32)) as i32);
        self.iConst19 = (self.iConst18 + 1);
        self.fConst20 = (self.fConst4 * self.fConst5);
        self.fConst21 = (0.166666672 * (self.fConst20 * (0.0 - self.fConst7)));
        self.iConst22 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst10 + 3)) as f32)) as i32);
        self.iConst23 = (self.iConst22 + 1);
        self.fConst24 = (0.0416666679 * (self.fConst20 * self.fConst6));
        self.iConst25 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst10 + 4)) as f32)) as i32);
        self.iConst26 = (self.iConst25 + 1);
        self.fConst27 = (0.000838235312 * self.fConst0);
        self.fConst28 = (self.fConst27 + -1.49999499);
        self.fConst29 = f32::floor(self.fConst28);
        self.fConst30 = (self.fConst27 + (-1.0 - self.fConst29));
        self.fConst31 = (self.fConst27 + (-2.0 - self.fConst29));
        self.fConst32 = (self.fConst27 + (-3.0 - self.fConst29));
        self.fConst33 = (self.fConst27 + (-4.0 - self.fConst29));
        self.fConst34 = ((((0.0 - self.fConst30) * (0.0 - (0.5 * self.fConst31)))
            * (0.0 - (0.333333343 * self.fConst32)))
            * (0.0 - (0.25 * self.fConst33)));
        self.iConst35 = (self.fConst28 as i32);
        self.iConst36 = (f32::min(self.fConst9, (std::cmp::max(0, self.iConst35) as f32)) as i32);
        self.iConst37 = (self.iConst36 + 2);
        self.fConst38 = (self.fConst27 - self.fConst29);
        self.fConst39 = (((0.0 - self.fConst31) * (0.0 - (0.5 * self.fConst32)))
            * (0.0 - (0.333333343 * self.fConst33)));
        self.iConst40 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst35 + 1)) as f32)) as i32);
        self.iConst41 = (self.iConst40 + 2);
        self.fConst42 =
            (0.5 * ((self.fConst30 * (0.0 - self.fConst32)) * (0.0 - (0.5 * self.fConst33))));
        self.iConst43 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst35 + 2)) as f32)) as i32);
        self.iConst44 = (self.iConst43 + 2);
        self.fConst45 = (self.fConst30 * self.fConst31);
        self.fConst46 = (0.166666672 * (self.fConst45 * (0.0 - self.fConst33)));
        self.iConst47 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst35 + 3)) as f32)) as i32);
        self.iConst48 = (self.iConst47 + 2);
        self.fConst49 = (0.0416666679 * (self.fConst45 * self.fConst32));
        self.iConst50 =
            (f32::min(self.fConst9, (std::cmp::max(0, (self.iConst35 + 4)) as f32)) as i32);
        self.iConst51 = (self.iConst50 + 2);
        self.fConst52 = f32::tan((2670.35376 / self.fConst0));
        self.fConst53 = (1.0 / self.fConst52);
        self.fConst54 = (((self.fConst53 + 1.41421354) / self.fConst52) + 1.0);
        self.fConst55 = (0.800000012 / self.fConst54);
        self.fConst56 = (1.0 / self.fConst54);
        self.fConst57 = (((self.fConst53 + -1.41421354) / self.fConst52) + 1.0);
        self.fConst58 = (2.0 * (1.0 - (1.0 / Guitar_faustpower2_f(self.fConst52))));
        self.fConst59 = (0.00355951115 * self.fConst0);
        self.fConst60 = (0.00177975558 * self.fConst0);
        self.fConst61 = (561.874939 / self.fConst0);
        self.iConst62 = (self.iConst36 + 1);
        self.iConst63 = (self.iConst40 + 1);
        self.iConst64 = (self.iConst43 + 1);
        self.iConst65 = (self.iConst47 + 1);
        self.iConst66 = (self.iConst50 + 1);
    }

    pub fn instanceInit(&mut self, sample_rate: i32) {
        self.instanceConstants(sample_rate);
        self.instanceClear();
    }

    pub fn new() -> Guitar {
        let mut modu = Guitar::init();
        modu.instanceInit(44_100);
        modu
    }

    pub fn from_node_infos(node_infos: &audiograph_parser::Node) -> Guitar {
        let modu = Guitar::new();
        modu.check_io_node_infos(node_infos);
        modu
    }
}

impl fmt::Display for Guitar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "guitar()")
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
        if self.fSampleRate  != actual_samplerate {
            self.instanceInit(actual_samplerate);
        }

        for i in 0..count {
            self.iRec10[0] = 0;
            self.iVec0[0] = 1;
            let mut iRec11: i32 = self.iRec10[1];
            let mut fRec14: f32 = ((self.iRec6[1] as f32)
                - (0.997843683
                    * ((0.699999988 * self.fRec15[2])
                        + (0.150000006 * (self.fRec15[1] + self.fRec15[3])))));
            self.fVec1[0] = self.fConst0;
            self.fRec22[0] = ((self.fConst8
                * self.fRec2[((self.IOTA - self.iConst12) & 511) as usize])
                + (self.fConst13
                    * ((((self.fConst14
                        * self.fRec2[((self.IOTA - self.iConst16) & 511) as usize])
                        + (self.fConst17
                            * self.fRec2[((self.IOTA - self.iConst19) & 511) as usize]))
                        + (self.fConst21
                            * self.fRec2[((self.IOTA - self.iConst23) & 511) as usize]))
                        + (self.fConst24
                            * self.fRec2[((self.IOTA - self.iConst26) & 511) as usize]))));
            self.fRec25[0] = ((0.0500000007 * self.fRec25[1]) + (0.949999988 * self.fRec22[1]));
            let mut fRec23: f32 = self.fRec25[0];
            self.fRec27[0] = self.fRec0[1];
            self.fRec28[(self.IOTA & 255) as usize] = (-1.0
                * (0.997843683
                    * ((0.699999988 * self.fRec27[2])
                        + (0.150000006 * (self.fRec27[1] + self.fRec27[3])))));
            self.fVec2[0] = ((self.fConst34
                * self.fRec28[((self.IOTA - self.iConst37) & 255) as usize])
                + (self.fConst38
                    * ((((self.fConst39
                        * self.fRec28[((self.IOTA - self.iConst41) & 255) as usize])
                        + (self.fConst42
                            * self.fRec28[((self.IOTA - self.iConst44) & 255) as usize]))
                        + (self.fConst46
                            * self.fRec28[((self.IOTA - self.iConst48) & 255) as usize]))
                        + (self.fConst49
                            * self.fRec28[((self.IOTA - self.iConst51) & 255) as usize]))));
            self.iRec30[0] = ((1103515245 * self.iRec30[1]) + 12345);
            self.fRec29[0] = ((4.65661287e-10 * (self.iRec30[0] as f32))
                - (self.fConst56
                    * ((self.fConst57 * self.fRec29[2]) + (self.fConst58 * self.fRec29[1]))));
            self.fRec31[0] = if ((((((1 - self.iVec0[1]) > 0) as i32) > 0) as i32) as i32 == 1) {
                0.0
            } else {
                f32::min(
                    self.fConst59,
                    ((self.fRec31[1] + (0.00355951115 * (self.fConst0 - self.fVec1[1]))) + 1.0),
                )
            };
            let mut iTemp0: i32 = ((self.fRec31[0] < self.fConst60) as i32);
            let mut fTemp1: f32 = (self.fConst55
                * ((self.fRec29[2] + (self.fRec29[0] + (2.0 * self.fRec29[1])))
                    * if (iTemp0 as i32 == 1) {
                        if (((self.fRec31[0] < 0.0) as i32) as i32 == 1) {
                            0.0
                        } else {
                            if (iTemp0 as i32 == 1) {
                                (self.fConst61 * self.fRec31[0])
                            } else {
                                1.0
                            }
                        }
                    } else {
                        if (((self.fRec31[0] < self.fConst59) as i32) as i32 == 1) {
                            ((self.fConst61 * (0.0 - (self.fRec31[0] - self.fConst60))) + 1.0)
                        } else {
                            0.0
                        }
                    }));
            self.fVec3[0] = (self.fVec2[1] + fTemp1);
            self.fRec26[(self.IOTA & 511) as usize] = ((0.0500000007
                * self.fRec26[((self.IOTA - 1) & 511) as usize])
                + (0.949999988 * self.fVec3[1]));
            let mut fRec24: f32 = ((self.fConst8
                * self.fRec26[((self.IOTA - self.iConst11) & 511) as usize])
                + (self.fConst13
                    * ((((self.fConst14
                        * self.fRec26[((self.IOTA - self.iConst15) & 511) as usize])
                        + (self.fConst17
                            * self.fRec26[((self.IOTA - self.iConst18) & 511) as usize]))
                        + (self.fConst21
                            * self.fRec26[((self.IOTA - self.iConst22) & 511) as usize]))
                        + (self.fConst24
                            * self.fRec26[((self.IOTA - self.iConst25) & 511) as usize]))));
            self.fRec19[0] = fRec23;
            let mut fRec20: f32 = (fTemp1 + self.fRec19[1]);
            let mut fRec21: f32 = fRec24;
            self.fRec16[(self.IOTA & 255) as usize] = fRec20;
            let mut fRec17: f32 = ((self.fConst34
                * self.fRec16[((self.IOTA - self.iConst62) & 255) as usize])
                + (self.fConst38
                    * ((((self.fConst39
                        * self.fRec16[((self.IOTA - self.iConst63) & 255) as usize])
                        + (self.fConst42
                            * self.fRec16[((self.IOTA - self.iConst64) & 255) as usize]))
                        + (self.fConst46
                            * self.fRec16[((self.IOTA - self.iConst65) & 255) as usize]))
                        + (self.fConst49
                            * self.fRec16[((self.IOTA - self.iConst66) & 255) as usize]))));
            self.fRec18[0] = fRec21;
            self.fRec15[0] = self.fRec18[1];
            let mut fRec12: f32 = self.fRec15[1];
            let mut fRec13: f32 = self.fRec15[1];
            self.iRec6[0] = iRec11;
            let mut fRec7: f32 = fRec14;
            let mut fRec8: f32 = fRec12;
            let mut fRec9: f32 = fRec13;
            self.fRec2[(self.IOTA & 511) as usize] = fRec7;
            let mut fRec3: f32 = fRec17;
            let mut fRec4: f32 = fRec8;
            let mut fRec5: f32 = fRec9;
            self.fRec0[0] = fRec3;
            let mut fRec1: f32 = fRec5;
            output[i as usize] = (fRec1 as f32);
            self.iRec10[1] = self.iRec10[0];
            self.iVec0[1] = self.iVec0[0];
            self.fVec1[1] = self.fVec1[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec25[1] = self.fRec25[0];
            for j0 in (0..4).rev() {
                self.fRec27[j0 as usize] = self.fRec27[(j0 - 1) as usize];
            }
            self.IOTA = (self.IOTA + 1);
            self.fVec2[1] = self.fVec2[0];
            self.iRec30[1] = self.iRec30[0];
            self.fRec29[2] = self.fRec29[1];
            self.fRec29[1] = self.fRec29[0];
            self.fRec31[1] = self.fRec31[0];
            self.fVec3[1] = self.fVec3[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            for j1 in (0..4).rev() {
                self.fRec15[j1 as usize] = self.fRec15[(j1 - 1) as usize];
            }
            self.iRec6[1] = self.iRec6[0];
            self.fRec0[1] = self.fRec0[0];
        }
    }
}
