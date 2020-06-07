/*
Copyright (c) 2016-2017, Robert Ou <rqou@robertou.com> and contributors
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

// Toplevel bitstrem stuff

use std::io;
use std::io::Write;

use jedec::*;

use crate::*;
use crate::fb::{MC_TO_ROW_MAP_LARGE, large_get_macrocell_offset};
use crate::fusemap_logical::{fb_fuse_idx, gck_fuse_idx, gsr_fuse_idx, gts_fuse_idx, global_term_fuse_idx,
                             total_logical_fuse_count, clock_div_fuse_idx};
use crate::fusemap_physical::{fuse_array_dims, mc_block_loc};
use crate::util::{LinebreakSet};
use crate::zia::{zia_get_row_width};

/// Toplevel struct representing an entire Coolrunner-II bitstream
#[derive(Serialize)]
pub struct XC2Bitstream {
    pub speed_grade: XC2Speed,
    pub package: XC2Package,
    pub bits: XC2BitstreamBits,
}

impl XC2Bitstream {
    /// Dump a human-readable explanation of the bitstream to the given `writer` object.
    pub fn dump_human_readable<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        write!(writer, "xc2bit dump\n")?;
        write!(writer, "device speed grade: {}\n", self.speed_grade)?;
        write!(writer, "device package: {}\n", self.package)?;
        self.bits.dump_human_readable(&mut writer)?;

        Ok(())
    }

    /// Write a .jed representation of the bitstream to the given `writer` object.
    pub fn to_jed<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        write!(writer, ".JED fuse map written by xc2bit\n")?;
        write!(writer, "https://github.com/azonenberg/openfpga\n\n")?;

        let mut linebreaks = LinebreakSet::new();
        let mut jed = JEDECFile::new(total_logical_fuse_count(self.bits.device_type()));
        jed.dev_name_str = Some(format!("{}-{}-{}", self.bits.device_type(), self.speed_grade, self.package));
        self.bits.to_jed(&mut jed, &mut linebreaks);

        jed.write_custom_linebreaks(&mut writer, linebreaks.iter())?;

        Ok(())
    }

    /// Converts the bitstream into a FuseArray object so that it can be written to the native "crbit" format
    pub fn to_crbit(&self) -> FuseArray {
        let (w, h) = fuse_array_dims(self.bits.device_type());
        let mut fuse_array = FuseArray::from_dim(w, h);

        fuse_array.dev_name_str = Some(format!("{}-{}-{}", self.bits.device_type(), self.speed_grade, self.package));

        self.bits.to_crbit(&mut fuse_array);

        fuse_array
    }

    /// Processes a fuse array into a bitstream object
    pub fn from_jed(jed: &JEDECFile) -> Result<Self, XC2BitError> {
        if jed.dev_name_str.is_none() {
            return Err(XC2BitError::BadDeviceName(String::new()));
        }

        let device = jed.dev_name_str.as_ref().unwrap();

        let device_combination = XC2DeviceSpeedPackage::from_str(device);
        if device_combination.is_none() {
            return Err(XC2BitError::BadDeviceName(device.to_owned()));
        }

        let XC2DeviceSpeedPackage {
            dev, spd, pkg
        } = device_combination.unwrap();

        let fuses = &jed.f;

        if fuses.len() != total_logical_fuse_count(dev) {
            return Err(XC2BitError::WrongFuseCount);
        }

        match dev {
            XC2Device::XC2C32 => {
                let bits = <XC2BitsXC2C32 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C32(bits),
                })
            },
            XC2Device::XC2C32A => {
                let bits = <XC2BitsXC2C32A as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C32A(bits),
                })
            },
            XC2Device::XC2C64 => {
                let bits = <XC2BitsXC2C64 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C64(bits),
                })
            },
            XC2Device::XC2C64A => {
                let bits = <XC2BitsXC2C64A as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C64A(bits),
                })
            },
            XC2Device::XC2C128 => {
                let bits = <XC2BitsXC2C128 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C128(bits),
                })
            },
            XC2Device::XC2C256 => {
                let bits = <XC2BitsXC2C256 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C256(bits),
                })
            },
            XC2Device::XC2C384 => {
                let bits = <XC2BitsXC2C384 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C384(bits),
                })
            },
            XC2Device::XC2C512 => {
                let bits = <XC2BitsXC2C512 as BitFragment<Jed>>::decode(
                    fuses, [0], [false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C512(bits),
                })
            },
        }
    }

    /// Processes a fuse array (in physical addressing) into a bitstream object
    pub fn from_crbit(fuse_array: &FuseArray) -> Result<Self, XC2BitError> {
        // FIXME: Can we guess the device type from the dimensions?
        if fuse_array.dev_name_str.is_none() {
            return Err(XC2BitError::BadDeviceName(String::from("")));
        }

        let device_combination = XC2DeviceSpeedPackage::from_str(fuse_array.dev_name_str.as_ref().unwrap());
        if device_combination.is_none() {
            return Err(XC2BitError::BadDeviceName(fuse_array.dev_name_str.as_ref().unwrap().to_owned()));
        }

        let XC2DeviceSpeedPackage {
            dev, spd, pkg
        } = device_combination.unwrap();

        if fuse_array.dim() != fuse_array_dims(dev) {
            return Err(XC2BitError::WrongFuseCount);
        }


        match dev {
            XC2Device::XC2C32 => {
                let bits = <XC2BitsXC2C32 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C32(bits),
                })
            },
            XC2Device::XC2C32A => {
                let bits = <XC2BitsXC2C32A as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C32A(bits),
                })
            },
            XC2Device::XC2C64 => {
                let bits = <XC2BitsXC2C64 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C64(bits),
                })
            },
            XC2Device::XC2C64A => {
                let bits = <XC2BitsXC2C64A as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C64A(bits),
                })
            },
            XC2Device::XC2C128 => {
                let bits = <XC2BitsXC2C128 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C128(bits),
                })
            },
            XC2Device::XC2C256 => {
                let bits = <XC2BitsXC2C256 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C256(bits),
                })
            },
            XC2Device::XC2C384 => {
                let bits = <XC2BitsXC2C384 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C384(bits),
                })
            },
            XC2Device::XC2C512 => {
                let bits = <XC2BitsXC2C512 as BitFragment<Crbit>>::decode(
                    fuse_array, [0, 0], [false, false], ())?;
                Ok(XC2Bitstream {
                    speed_grade: spd,
                    package: pkg,
                    bits: XC2BitstreamBits::XC2C512(bits),
                })
            },
        }
    }

    /// Construct a new blank bitstream of the given part
    pub fn blank_bitstream(part_combination: XC2DeviceSpeedPackage) -> Self {
        let XC2DeviceSpeedPackage {
            dev: device, spd: speed_grade, pkg: package
        } = part_combination;

        match device {
            XC2Device::XC2C32 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C32(XC2BitsXC2C32 {
                        fb: [XC2BitstreamFB::default(); 2],
                        iobs: [XC2MCSmallIOB::default(); 32],
                        inpin: XC2ExtraIBuf::default(),
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: false,
                        ovoltage: false,
                    })
                }
            },
            XC2Device::XC2C32A => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A {
                        fb: [XC2BitstreamFB::default(); 2],
                        iobs: [XC2MCSmallIOB::default(); 32],
                        inpin: XC2ExtraIBuf::default(),
                        global_nets: XC2GlobalNets::default(),
                        legacy_ivoltage: false,
                        legacy_ovoltage: false,
                        ivoltage: [false, false],
                        ovoltage: [false, false],
                    })
                }
            },
            XC2Device::XC2C64 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C64(XC2BitsXC2C64 {
                        fb: [XC2BitstreamFB::default(); 4],
                        iobs: [[XC2MCSmallIOB::default(); 32]; 2],
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: false,
                        ovoltage: false,
                    })
                }
            },
            XC2Device::XC2C64A => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A {
                        fb: [XC2BitstreamFB::default(); 4],
                        iobs: [[XC2MCSmallIOB::default(); 32]; 2],
                        global_nets: XC2GlobalNets::default(),
                        legacy_ivoltage: false,
                        legacy_ovoltage: false,
                        ivoltage: [false, false],
                        ovoltage: [false, false],
                    })
                }
            },
            XC2Device::XC2C128 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C128(XC2BitsXC2C128 {
                        fb: [XC2BitstreamFB::default(); 8],
                        iobs: [[XC2MCLargeIOB::default(); 25]; 4],
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: [false, false],
                        ovoltage: [false, false],
                        data_gate: false,
                        use_vref: false,
                        clock_div: XC2ClockDiv::default(),
                    })
                }
            },
            XC2Device::XC2C256 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C256(XC2BitsXC2C256 {
                        fb: [XC2BitstreamFB::default(); 16],
                        iobs: [[XC2MCLargeIOB::default(); 23]; 8],
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: [false, false],
                        ovoltage: [false, false],
                        data_gate: false,
                        use_vref: false,
                        clock_div: XC2ClockDiv::default(),
                    })
                }
            },
            XC2Device::XC2C384 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C384(XC2BitsXC2C384 {
                        fb: [XC2BitstreamFB::default(); 24],
                        iobs: [[XC2MCLargeIOB::default(); 24]; 10],
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: [false, false, false, false],
                        ovoltage: [false, false, false, false],
                        data_gate: false,
                        use_vref: false,
                        clock_div: XC2ClockDiv::default(),
                    })
                }
            },
            XC2Device::XC2C512 => {
                XC2Bitstream {
                    speed_grade,
                    package,
                    bits: XC2BitstreamBits::XC2C512(XC2BitsXC2C512 {
                        fb: [XC2BitstreamFB::default(); 32],
                        iobs: [[XC2MCLargeIOB::default(); 27]; 10],
                        global_nets: XC2GlobalNets::default(),
                        ivoltage: [false, false, false, false],
                        ovoltage: [false, false, false, false],
                        data_gate: false,
                        use_vref: false,
                        clock_div: XC2ClockDiv::default(),
                    })
                }
            }
        }
    }
}

pub enum Jed {}
pub enum Crbit{}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C32 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C32, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C32)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C32)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    pub fb: [XC2BitstreamFB; 2],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C32, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C32) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        [(fboff as isize) + (everythingelseoff as isize) + (mc as isize) * 27]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C32, fb);
        // The "32" variant
        // each macrocell is 3 rows high
        let y = y + (mc as usize) * 3;
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C32, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit32)]
    pub iobs: [XC2MCSmallIOB; 32],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit)]
    pub inpin: XC2ExtraIBuf,

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C32)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C32)]
    pub global_nets: XC2GlobalNets,

    /// Voltage level control
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !12271)]
    #[pat_bits(frag_variant = Crbit, "0" = !(130, 24))]
    pub ivoltage: bool,

    /// Voltage level control
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !12270)]
    #[pat_bits(frag_variant = Crbit, "0" = !(130, 25))]
    pub ovoltage: bool,
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C32A {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C32A, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C32)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C32)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    pub fb: [XC2BitstreamFB; 2],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32A, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C32A, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C32A) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        [(fboff as isize) + (everythingelseoff as isize) + (mc as isize) * 27]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32A, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C32A, fb);
        // The "32" variant
        // each macrocell is 3 rows high
        let y = y + (mc as usize) * 3;
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C32A, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C32A, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit32)]
    pub iobs: [XC2MCSmallIOB; 32],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit)]
    pub inpin: XC2ExtraIBuf,

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C32)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C32)]
    pub global_nets: XC2GlobalNets,

    /// Legacy voltage level control, should almost always be set to `false`
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !12271)]
    #[pat_bits(frag_variant = Crbit, "0" = !(130, 24))]
    legacy_ivoltage: bool,

    /// Legacy voltage level control, should almost always be set to `false`
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !12270)]
    #[pat_bits(frag_variant = Crbit, "0" = !(130, 25))]
    legacy_ovoltage: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[12274], [12276]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[131, 25], [133, 25]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ivoltage: [bool; 2],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[12275], [12277]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[132, 25], [134, 25]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ovoltage: [bool; 2],
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C64 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C64, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C64)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C64)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 4],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C64, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C64) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        [(fboff as isize) + (everythingelseoff as isize) + (mc as isize) * 27]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C64, fb);
        // The "64" variant
        // each macrocell is 3 rows high
        let y = y + (mc as usize) * 3;
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C64, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit64)]
    iobs: [[XC2MCSmallIOB; 32]; 2],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C64)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C64)]
    global_nets: XC2GlobalNets,

    /// Voltage level control
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !25807)]
    #[pat_bits(frag_variant = Crbit, "0" = !(138, 23))]
    ivoltage: bool,

    /// Voltage level control
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !25806)]
    #[pat_bits(frag_variant = Crbit, "0" = !(137, 23))]
    ovoltage: bool,
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C64A {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C64A, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C64)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C64)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 4],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64A, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C64A, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C64A) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        [(fboff as isize) + (everythingelseoff as isize) + (mc as isize) * 27]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64A, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C64A, fb);
        // The "64" variant
        // each macrocell is 3 rows high
        let y = y + (mc as usize) * 3;
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C64A, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C64A, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit64)]
    iobs: [[XC2MCSmallIOB; 32]; 2],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C64)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C64)]
    global_nets: XC2GlobalNets,

    /// Legacy voltage level control, should almost always be set to `false`
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !25807)]
    #[pat_bits(frag_variant = Crbit, "0" = !(138, 23))]
    legacy_ivoltage: bool,

    /// Legacy voltage level control, should almost always be set to `false`
    ///
    /// `false` = low, `true` = high
    #[pat_bits(frag_variant = Jed, "0" = !25806)]
    #[pat_bits(frag_variant = Crbit, "0" = !(137, 23))]
    legacy_ovoltage: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[25808], [25810]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[139, 23], [141, 23]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ivoltage: [bool; 2],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[25809], [25811]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[140, 23], [142, 23]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ovoltage: [bool; 2],
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C128 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C128, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C128)]
    #[encode_sub_extra_data(variant = Jed, arr_elem_i)]
    #[decode_sub_extra_data(variant = Jed, arr_elem_i)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C128)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 8],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C128, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C128, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C128) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        let mcoff = large_get_macrocell_offset(XC2Device::XC2C128, fb as usize, mc as usize);
        [(fboff as isize) + (everythingelseoff as isize) + mcoff as isize]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C128, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C128, fb);
        // The "common large macrocell" variant
        // we need this funny lookup table, but otherwise macrocells are 2x15
        let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C128, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C128, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::CrbitLarge)]
    iobs: [[XC2MCLargeIOB; 25]; 4],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C128)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C128)]
    global_nets: XC2GlobalNets,

    #[offset(variant = Jed, [clock_div_fuse_idx(XC2Device::XC2C128) as isize])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedCommon)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C128)]
    clock_div: XC2ClockDiv,

    /// Whether the DataGate feature is used
    #[pat_bits(frag_variant = Jed, "0" = !55335)]
    #[pat_bits(frag_variant = Crbit, "0" = !(371, 67))]
    data_gate: bool,

    /// Whether I/O standards with VREF are used
    #[pat_bits(frag_variant = Jed, "0" = !55340)]
    #[pat_bits(frag_variant = Crbit, "0" = !(10, 67))]
    use_vref: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[55336], [55337]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[8, 67], [368, 67]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ivoltage: [bool; 2],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[55338], [55339]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[9, 67], [369, 67]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ovoltage: [bool; 2],
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C256 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C256, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C256)]
    #[encode_sub_extra_data(variant = Jed, arr_elem_i)]
    #[decode_sub_extra_data(variant = Jed, arr_elem_i)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C256)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 16],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C256, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C256, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C256) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        let mcoff = large_get_macrocell_offset(XC2Device::XC2C256, fb as usize, mc as usize);
        [(fboff as isize) + (everythingelseoff as isize) + mcoff as isize]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C256, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C256, fb);
        // The "256" variant
        // each macrocell is 3 rows high
        let y = y + (mc as usize) * 3;
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C256, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C256, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::Crbit256)]
    iobs: [[XC2MCLargeIOB; 23]; 8],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C256)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C256)]
    global_nets: XC2GlobalNets,

    #[offset(variant = Jed, [clock_div_fuse_idx(XC2Device::XC2C256) as isize])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedCommon)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C256)]
    clock_div: XC2ClockDiv,

    /// Whether the DataGate feature is used
    #[pat_bits(frag_variant = Jed, "0" = !123243)]
    #[pat_bits(frag_variant = Crbit, "0" = !(518, 23))]
    data_gate: bool,

    /// Whether I/O standards with VREF are used
    #[pat_bits(frag_variant = Jed, "0" = !123248)]
    #[pat_bits(frag_variant = Crbit, "0" = !(177, 23))]
    use_vref: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[123244], [123245]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[175, 23], [515, 23]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ivoltage: [bool; 2],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[123246], [123247]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[176, 23], [516, 23]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ovoltage: [bool; 2],
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C384 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C384, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C384)]
    #[encode_sub_extra_data(variant = Jed, arr_elem_i)]
    #[decode_sub_extra_data(variant = Jed, arr_elem_i)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C384)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 24],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C384, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C384, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C384) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        let mcoff = large_get_macrocell_offset(XC2Device::XC2C384, fb as usize, mc as usize);
        [(fboff as isize) + (everythingelseoff as isize) + mcoff as isize]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C384, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C384, fb);
        // The "common large macrocell" variant
        // we need this funny lookup table, but otherwise macrocells are 2x15
        let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C384, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C384, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::CrbitLarge)]
    iobs: [[XC2MCLargeIOB; 24]; 10],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C384)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C384)]
    global_nets: XC2GlobalNets,

    #[offset(variant = Jed, [clock_div_fuse_idx(XC2Device::XC2C384) as isize])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedCommon)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C384)]
    clock_div: XC2ClockDiv,

    /// Whether the DataGate feature is used
    #[pat_bits(frag_variant = Jed, "0" = !209347)]
    #[pat_bits(frag_variant = Crbit, "0" = !(932, 17))]
    data_gate: bool,

    /// Whether I/O standards with VREF are used
    #[pat_bits(frag_variant = Jed, "0" = !209356)]
    #[pat_bits(frag_variant = Crbit, "0" = !(3, 17))]
    use_vref: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[209348], [209349], [209350], [209351]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[936, 17], [1864, 17], [1, 17], [929, 17]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ivoltage: [bool; 4],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[209352], [209353], [209354], [209355]][i])]
    #[pat_bits(frag_variant = Jed, "0" = !0)]
    #[arr_off(variant = Crbit, |i| [[937, 17], [1865, 17], [2, 17], [930, 17]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = !(0, 0))]
    ovoltage: [bool; 4],
}

#[bitfragment(variant = Jed, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = Crbit, dimensions = 2, errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2BitsXC2C512 {
    #[arr_off(variant = Jed, |i| [fb_fuse_idx(XC2Device::XC2C512, i as u32)])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = fb::JedXC2C512)]
    #[encode_sub_extra_data(variant = Jed, arr_elem_i)]
    #[decode_sub_extra_data(variant = Jed, arr_elem_i)]

    #[arr_off(variant = Crbit, |_| [0, 0])]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = fb::CrbitXC2C512)]
    #[encode_sub_extra_data(variant = Crbit, arr_elem_i)]
    #[decode_sub_extra_data(variant = Crbit, arr_elem_i)]
    fb: [XC2BitstreamFB; 32],

    // XXX this offset is here whereas the fb offset is automagic
    #[arr_off(variant = Jed, |iob| {
        // XXX wtf
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C512, iob as u32).unwrap();
        let fboff = fb_fuse_idx(XC2Device::XC2C512, fb);
        let everythingelseoff = zia_get_row_width(XC2Device::XC2C512) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB;
        let mcoff = large_get_macrocell_offset(XC2Device::XC2C512, fb as usize, mc as usize);
        [(fboff as isize) + (everythingelseoff as isize) + mcoff as isize]
    })]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = iob::Jed)]

    #[arr_off(variant = Crbit, |iob| {
        let (fb, mc) = iob_num_to_fb_mc_num(XC2Device::XC2C512, iob as u32).unwrap();
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C512, fb);
        // The "common large macrocell" variant
        // we need this funny lookup table, but otherwise macrocells are 2x15
        let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
        [x, y]
    })]
    #[arr_mirror(variant = Crbit, |iob| {
        let (fb, _mc) = iob_num_to_fb_mc_num(XC2Device::XC2C512, iob as u32).unwrap();
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C512, fb);
        [mirror, false]
    })]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = iob::CrbitLarge)]
    iobs: [[XC2MCLargeIOB; 27]; 10],

    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedXC2C512)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C512)]
    global_nets: XC2GlobalNets,

    #[offset(variant = Jed, [clock_div_fuse_idx(XC2Device::XC2C512) as isize])]
    #[frag(outer_frag_variant = Jed, inner_frag_variant = globalbits::JedCommon)]
    #[frag(outer_frag_variant = Crbit, inner_frag_variant = globalbits::CrbitXC2C512)]
    clock_div: XC2ClockDiv,

    /// Whether the DataGate feature is used
    #[pat_bits(frag_variant = Jed, "0" = !296393)]
    #[pat_bits(frag_variant = Crbit, "0" = !(982, 147))]
    data_gate: bool,

    /// Whether I/O standards with VREF are used
    #[pat_bits(frag_variant = Jed, "0" = !296402)]
    #[pat_bits(frag_variant = Crbit, "0" = !(1, 147))]
    use_vref: bool,

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[296394], [296395], [296396], [296397]][i])]
    #[pat_bits(frag_variant = Jed, "0" = 0)]
    #[arr_off(variant = Crbit, |i| [[992, 147], [1965, 147], [3, 147], [985, 147]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = (0, 0))]
    ivoltage: [bool; 4],

    /// Voltage level control for each I/O bank
    ///
    /// `false` = low, `true` = high
    #[arr_off(variant = Jed, |i| [[296398], [296399], [296400], [296401]][i])]
    #[pat_bits(frag_variant = Jed, "0" = 0)]
    #[arr_off(variant = Crbit, |i| [[991, 147], [1964, 147], [2, 147], [984, 147]][i])]
    #[pat_bits(frag_variant = Crbit, "0" = (0, 0))]
    ovoltage: [bool; 4],
}

/// The actual bitstream bits for each possible Coolrunner-II part
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2BitstreamBits {
    XC2C32(XC2BitsXC2C32),
    XC2C32A(XC2BitsXC2C32A),
    XC2C64(XC2BitsXC2C64),
    XC2C64A(XC2BitsXC2C64A),
    XC2C128(XC2BitsXC2C128),
    XC2C256(XC2BitsXC2C256),
    XC2C384(XC2BitsXC2C384),
    XC2C512(XC2BitsXC2C512),
}

impl XC2BitstreamBits {
    /// Helper to convert ourself into a `XC2Device` enum because an `XC2Device` enum has various useful methods
    pub fn device_type(&self) -> XC2Device {
        match self {
            &XC2BitstreamBits::XC2C32{..} => XC2Device::XC2C32,
            &XC2BitstreamBits::XC2C32A{..} => XC2Device::XC2C32A,
            &XC2BitstreamBits::XC2C64{..} => XC2Device::XC2C64,
            &XC2BitstreamBits::XC2C64A{..} => XC2Device::XC2C64A,
            &XC2BitstreamBits::XC2C128{..} => XC2Device::XC2C128,
            &XC2BitstreamBits::XC2C256{..} => XC2Device::XC2C256,
            &XC2BitstreamBits::XC2C384{..} => XC2Device::XC2C384,
            &XC2BitstreamBits::XC2C512{..} => XC2Device::XC2C512,
        }
    }

    /// Helper to extract only the function block data without having to perform an explicit `match`
    pub fn get_fb(&self) -> &[XC2BitstreamFB] {
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref fb, ..}) => fb,
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref fb, ..}) => fb,
        }
    }

    /// Helper to extract only the function block data without having to perform an explicit `match`
    pub fn get_fb_mut(&mut self) -> &mut [XC2BitstreamFB] {
        match self {
            &mut XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref mut fb, ..}) => fb,
            &mut XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref mut fb, ..}) => fb,
        }
    }

    /// Helper to extract only the I/O data without having to perform an explicit `match`
    pub fn get_small_iob(&self, i: usize) -> Option<&XC2MCSmallIOB> {
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref iobs, ..}) => Some(&iobs[i]),
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref iobs, ..}) => Some(&iobs[i]),
            &XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref iobs, ..}) => Some(&iobs[i / 32][i % 32]),
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref iobs, ..}) => Some(&iobs[i / 32][i % 32]),
            _ => None,
        }
    }

    /// Helper to extract only the I/O data without having to perform an explicit `match`
    pub fn get_mut_small_iob(&mut self, i: usize) -> Option<&mut XC2MCSmallIOB> {
        match self {
            &mut XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref mut iobs, ..}) => Some(&mut iobs[i]),
            &mut XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref mut iobs, ..}) => Some(&mut iobs[i]),
            &mut XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref mut iobs, ..}) => Some(&mut iobs[i / 32][i % 32]),
            &mut XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref mut iobs, ..}) => Some(&mut iobs[i / 32][i % 32]),
            _ => None,
        }
    }

    /// Helper to extract only the I/O data without having to perform an explicit `match`
    pub fn get_large_iob(&self, i: usize) -> Option<&XC2MCLargeIOB> {
        match self {
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref iobs, ..}) => Some(&iobs[i / 25][i % 25]),
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref iobs, ..}) => Some(&iobs[i / 23][i % 23]),
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref iobs, ..}) => Some(&iobs[i / 24][i % 24]),
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref iobs, ..}) => Some(&iobs[i / 27][i % 27]),
            _ => None,
        }
    }

    /// Helper to extract only the I/O data without having to perform an explicit `match`
    pub fn get_mut_large_iob(&mut self, i: usize) -> Option<&mut XC2MCLargeIOB> {
        match self {
            &mut XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref mut iobs, ..}) => Some(&mut iobs[i / 25][i % 25]),
            &mut XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref mut iobs, ..}) => Some(&mut iobs[i / 23][i % 23]),
            &mut XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref mut iobs, ..}) => Some(&mut iobs[i / 24][i % 24]),
            &mut XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref mut iobs, ..}) => Some(&mut iobs[i / 27][i % 27]),
            _ => None,
        }
    }

    /// Helper to extract only the global net data without having to perform an explicit `match`
    pub fn get_global_nets(&self) -> &XC2GlobalNets {
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref global_nets, ..}) => global_nets,
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref global_nets, ..}) => global_nets,
        }
    }

    /// Helper to extract only the global net data without having to perform an explicit `match`
    pub fn get_global_nets_mut(&mut self) -> &mut XC2GlobalNets {
        match self {
            &mut XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref mut global_nets, ..}) => global_nets,
            &mut XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref mut global_nets, ..}) => global_nets,
        }
    }

    pub fn get_clock_div(&self) -> Option<&XC2ClockDiv> {
        match self {
            &XC2BitstreamBits::XC2C32{..} => None,
            &XC2BitstreamBits::XC2C32A{..} => None,
            &XC2BitstreamBits::XC2C64{..} => None,
            &XC2BitstreamBits::XC2C64A{..} => None,
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref clock_div, ..}) => Some(clock_div),
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref clock_div, ..}) => Some(clock_div),
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref clock_div, ..}) => Some(clock_div),
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref clock_div, ..}) => Some(clock_div),
        }
    }

    /// Convert the actual bitstream bits to crbit format
    pub fn to_crbit(&self, fuse_array: &mut FuseArray) {
        match self {
            XC2BitstreamBits::XC2C32(x) => {
                <XC2BitsXC2C32 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C32A(x) => {
                <XC2BitsXC2C32A as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C64(x) => {
                <XC2BitsXC2C64 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C64A(x) => {
                <XC2BitsXC2C64A as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C128(x) => {
                <XC2BitsXC2C128 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C256(x) => {
                <XC2BitsXC2C256 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C384(x) => {
                <XC2BitsXC2C384 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
            XC2BitstreamBits::XC2C512(x) => {
                <XC2BitsXC2C512 as BitFragment<Crbit>>::encode(
                    x, fuse_array, [0, 0], [false, false], ());
            },
        }

        // Initialize security/done/usercode rows to all 1s
        for x in 0..fuse_array.dim().0 {
            fuse_array.set(x, fuse_array.dim().1 - 1, true);
            fuse_array.set(x, fuse_array.dim().1 - 2, true);
        }

        // Set done1 to 0
        match self {
            &XC2BitstreamBits::XC2C32{..} | &XC2BitstreamBits::XC2C32A{..} => {
                fuse_array.set(9, 48, false);
            },
            &XC2BitstreamBits::XC2C64{..} | &XC2BitstreamBits::XC2C64A{..} => {
                fuse_array.set(8, 96, false);
            },
            &XC2BitstreamBits::XC2C128{..} => {
                fuse_array.set(9, 80, false);
            },
            &XC2BitstreamBits::XC2C256{..} => {
                fuse_array.set(9, 96, false);
            },
            &XC2BitstreamBits::XC2C384{..} => {
                fuse_array.set(9, 120, false);
            },
            &XC2BitstreamBits::XC2C512{..} => {
                fuse_array.set(9, 160, false);
            },
        }

        // TODO: Security bits and USERCODE bits
    }

    /// Dump a human-readable explanation of the bitstream to the given `writer` object.
    pub fn dump_human_readable<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        write!(writer, "device type: {}\n", self.device_type())?;

        // Bank voltages
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref ivoltage, ref ovoltage, ..}) |
            &XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref ivoltage, ref ovoltage, ..}) => {
                write!(writer, "output voltage range: {}\n", if *ovoltage {"high"} else {"low"})?;
                write!(writer, "input voltage range: {}\n", if *ivoltage {"high"} else {"low"})?;
            },
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref legacy_ivoltage, ref legacy_ovoltage, ref ivoltage, ref ovoltage, ..}) |
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref legacy_ivoltage, ref legacy_ovoltage, ref ivoltage, ref ovoltage, ..}) => {
                write!(writer, "legacy output voltage range: {}\n", if *legacy_ovoltage {"high"} else {"low"})?;
                write!(writer, "legacy input voltage range: {}\n", if *legacy_ivoltage {"high"} else {"low"})?;
                write!(writer, "bank 0 output voltage range: {}\n", if ovoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 output voltage range: {}\n", if ovoltage[1] {"high"} else {"low"})?;
                write!(writer, "bank 0 input voltage range: {}\n", if ivoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 input voltage range: {}\n", if ivoltage[1] {"high"} else {"low"})?;
            },
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..}) |
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..}) => {
                write!(writer, "bank 0 output voltage range: {}\n", if ovoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 output voltage range: {}\n", if ovoltage[1] {"high"} else {"low"})?;
                write!(writer, "bank 0 input voltage range: {}\n", if ivoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 input voltage range: {}\n", if ivoltage[1] {"high"} else {"low"})?;
                write!(writer, "DataGate used: {}\n", if *data_gate {"yes"} else {"no"})?;
                write!(writer, "VREF used: {}\n", if *use_vref {"yes"} else {"no"})?;
            },
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..}) |
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..}) => {
                write!(writer, "bank 0 output voltage range: {}\n", if ovoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 output voltage range: {}\n", if ovoltage[1] {"high"} else {"low"})?;
                write!(writer, "bank 2 output voltage range: {}\n", if ovoltage[2] {"high"} else {"low"})?;
                write!(writer, "bank 3 output voltage range: {}\n", if ovoltage[3] {"high"} else {"low"})?;
                write!(writer, "bank 0 input voltage range: {}\n", if ivoltage[0] {"high"} else {"low"})?;
                write!(writer, "bank 1 input voltage range: {}\n", if ivoltage[1] {"high"} else {"low"})?;
                write!(writer, "bank 2 input voltage range: {}\n", if ivoltage[2] {"high"} else {"low"})?;
                write!(writer, "bank 3 input voltage range: {}\n", if ivoltage[3] {"high"} else {"low"})?;
                write!(writer, "DataGate used: {}\n", if *data_gate {"yes"} else {"no"})?;
                write!(writer, "VREF used: {}\n", if *use_vref {"yes"} else {"no"})?;
            }
        }

        // Clock divider
        if let Some(clock_div) = self.get_clock_div() {
            write!(writer, "\n{}", clock_div)?;
        }

        // Global net configuration
        write!(writer, "\n{}", self.get_global_nets())?;

        // IOBs
        for i in 0..self.device_type().num_iobs() {
            write!(writer, "\n")?;
            let (fb, mc) = iob_num_to_fb_mc_num(self.device_type(), i as u32).unwrap();
            write!(writer, "I/O configuration for FB{}_{}\n", fb + 1, mc + 1)?;
            if let Some(iob) = self.get_small_iob(i) {
                write!(writer, "{}", iob)?;
            }
            if let Some(iob) = self.get_large_iob(i) {
                write!(writer, "{}", iob)?;
            }
        }

        // Input-only pin
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref inpin, ..}) | &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref inpin, ..}) => {
                write!(writer, "\n")?;
                write!(writer, "I/O configuration for input-only pin\n")?;
                write!(writer, "{}", inpin)?;
            },
            _ => {}
        }

        // FBs
        for i in 0..self.device_type().num_fbs() {
            self.get_fb()[i].dump_human_readable(self.device_type(), i as u32, &mut writer)?;
        }

        Ok(())
    }

    /// Write a .jed representation of the bitstream to the given `jed` object.
    pub fn to_jed(&self, jed: &mut JEDECFile, linebreaks: &mut LinebreakSet) {
        match self {
            XC2BitstreamBits::XC2C32(x) => {
                <XC2BitsXC2C32 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C32A(x) => {
                <XC2BitsXC2C32A as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C64(x) => {
                <XC2BitsXC2C64 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C64A(x) => {
                <XC2BitsXC2C64A as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C128(x) => {
                <XC2BitsXC2C128 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C256(x) => {
                <XC2BitsXC2C256 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C384(x) => {
                <XC2BitsXC2C384 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
            XC2BitstreamBits::XC2C512(x) => {
                <XC2BitsXC2C512 as BitFragment<Jed>>::encode(
                    x, &mut jed.f, [0], [false], ());
            },
        }

        // FBs
        for fb_i in 0..self.device_type().num_fbs() {
            let fuse_base = fb_fuse_idx(self.device_type(), fb_i as u32);
            self.get_fb()[fb_i].to_jed(self.device_type(), fuse_base, jed, linebreaks, fb_i);
        }

        // IOB
        for fb_i in 0..self.device_type().num_fbs() {
            let fuse_base = fb_fuse_idx(self.device_type(), fb_i as u32);
            if self.device_type().is_small_iob() {
                for i in 0..MCS_PER_FB {
                    let iob = fb_mc_num_to_iob_num(self.device_type(), fb_i as u32, i as u32).unwrap() as usize;
                    // self.get_small_iob(iob).unwrap().to_jed(jed, self.device_type(), fuse_base, i);
                }
            }
            if self.device_type().is_large_iob() {
                let zia_row_width = zia_get_row_width(self.device_type());
                let mut current_fuse_offset = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
                    ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB;

                for i in 0..MCS_PER_FB {
                    let iob = fb_mc_num_to_iob_num(self.device_type(), fb_i as u32, i as u32);

                    if iob.is_some() {
                        let iob = iob.unwrap() as usize;

                        // self.get_large_iob(iob).unwrap().to_jed(jed, current_fuse_offset);
                        current_fuse_offset += 29;
                    } else {
                        current_fuse_offset += 16;
                    }
                }
            }
        }

        // GCK
        linebreaks.add(gck_fuse_idx(self.device_type()));
        linebreaks.add(gck_fuse_idx(self.device_type()));

        // Clock divider
        if let Some(clock_div) = self.get_clock_div() {
            let clock_fuse_block = clock_div_fuse_idx(self.device_type());

            linebreaks.add(clock_fuse_block);
            linebreaks.add(clock_fuse_block + 4);

            // <XC2ClockDiv as BitFragment<globalbits::JedCommon>>::encode(
            //     clock_div, &mut jed.f, [clock_fuse_block as isize], [false], ());
        }

        // GSR
        linebreaks.add(gsr_fuse_idx(self.device_type()));

        // GTS
        linebreaks.add(gts_fuse_idx(self.device_type()));

        // Global termination
        linebreaks.add(global_term_fuse_idx(self.device_type()));

        // // Actually write bits
        // match self.device_type() {
        //     XC2Device::XC2C32 | XC2Device::XC2C32A => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C32>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        //     XC2Device::XC2C64 | XC2Device::XC2C64A => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C64>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        //     XC2Device::XC2C128 => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C128>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        //     XC2Device::XC2C256 => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C256>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        //     XC2Device::XC2C384 => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C384>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        //     XC2Device::XC2C512 => {
        //         <XC2GlobalNets as BitFragment<globalbits::JedXC2C512>>::encode(
        //             self.get_global_nets(),
        //             &mut jed.f, [0], [false], ());
        //     },
        // }

        // Bank voltages and miscellaneous
        match self {
            &XC2BitstreamBits::XC2C32(XC2BitsXC2C32{ref inpin, ref ivoltage, ref ovoltage, ..}) |
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref inpin, legacy_ivoltage: ref ivoltage,
                legacy_ovoltage: ref ovoltage, ..}) => {

                linebreaks.add(12270);
                // jed.f[12270] = !ovoltage;
                linebreaks.add(12271);
                // jed.f[12271] = !ivoltage;

                linebreaks.add(12272);

                <XC2ExtraIBuf as BitFragment<iob::Jed>>::encode(&inpin, &mut jed.f, [0], [false], ());
            }
            &XC2BitstreamBits::XC2C64(XC2BitsXC2C64{ref ivoltage, ref ovoltage, ..}) |
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{legacy_ivoltage: ref ivoltage, legacy_ovoltage: ref ovoltage, ..}) => {
                linebreaks.add(25806);
                // jed.f[25806] = !ovoltage;
                linebreaks.add(25807);
                // jed.f[25807] = !ivoltage;
            }
            &XC2BitstreamBits::XC2C128(XC2BitsXC2C128{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..})  => {
                linebreaks.add(55335);
                // jed.f[55335] = !data_gate;

                linebreaks.add(55336);
                // jed.f[55336] = !ivoltage[0];
                // jed.f[55337] = !ivoltage[1];
                linebreaks.add(55338);
                // jed.f[55338] = !ovoltage[0];
                // jed.f[55339] = !ovoltage[1];

                linebreaks.add(55340);
                // jed.f[55340] = !use_vref;
            }
            &XC2BitstreamBits::XC2C256(XC2BitsXC2C256{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..})  => {
                linebreaks.add(123243);
                // jed.f[123243] = !data_gate;

                linebreaks.add(123244);
                // jed.f[123244] = !ivoltage[0];
                // jed.f[123245] = !ivoltage[1];
                linebreaks.add(123246);
                // jed.f[123246] = !ovoltage[0];
                // jed.f[123247] = !ovoltage[1];

                linebreaks.add(123248);
                // jed.f[123248] = !use_vref;
            }
            &XC2BitstreamBits::XC2C384(XC2BitsXC2C384{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..})  => {
                linebreaks.add(209347);
                // jed.f[209347] = !data_gate;

                linebreaks.add(209348);
                // jed.f[209348] = !ivoltage[0];
                // jed.f[209349] = !ivoltage[1];
                // jed.f[209350] = !ivoltage[2];
                // jed.f[209351] = !ivoltage[3];

                linebreaks.add(209352);
                // jed.f[209352] = !ovoltage[0];
                // jed.f[209353] = !ovoltage[1];
                // jed.f[209354] = !ovoltage[2];
                // jed.f[209355] = !ovoltage[3];

                linebreaks.add(209356);
                // jed.f[209356] = !use_vref;
            }
            &XC2BitstreamBits::XC2C512(XC2BitsXC2C512{ref ivoltage, ref ovoltage, ref data_gate, ref use_vref, ..})  => {
                linebreaks.add(296393);
                // jed.f[296393] = !data_gate;

                linebreaks.add(296394);
                // jed.f[296394] = ivoltage[0];
                // jed.f[296395] = ivoltage[1];
                // jed.f[296396] = ivoltage[2];
                // jed.f[296397] = ivoltage[3];

                linebreaks.add(296398);
                // jed.f[296398] = ovoltage[0];
                // jed.f[296399] = ovoltage[1];
                // jed.f[296400] = ovoltage[2];
                // jed.f[296401] = ovoltage[3];

                linebreaks.add(296402);
                // jed.f[296402] = !use_vref;
            }
        }

        // A-variant bank voltages
        match self {
            &XC2BitstreamBits::XC2C32A(XC2BitsXC2C32A{ref ivoltage, ref ovoltage, ..}) => {
                linebreaks.add(12274);
                // jed.f[12274] = !ivoltage[0];
                linebreaks.add(12275);
                // jed.f[12275] = !ovoltage[0];
                linebreaks.add(12276);
                // jed.f[12276] = !ivoltage[1];
                linebreaks.add(12277);
                // jed.f[12277] = !ovoltage[1];
            },
            &XC2BitstreamBits::XC2C64A(XC2BitsXC2C64A{ref ivoltage, ref ovoltage, ..}) => {
                linebreaks.add(25808);
                // jed.f[25808] = !ivoltage[0];
                linebreaks.add(25809);
                // jed.f[25809] = !ovoltage[0];
                linebreaks.add(25810);
                // jed.f[25810] = !ivoltage[1];
                linebreaks.add(25811);
                // jed.f[25811] = !ovoltage[1];
            },
            _ => {}
        }
    }
}
