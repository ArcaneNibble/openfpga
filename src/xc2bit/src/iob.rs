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

//! Contains functions pertaining to the I/O pins

use core::fmt;

use jedec::*;

use crate::*;
use crate::fusemap_physical::{mc_block_loc};
use crate::mc::{MC_TO_ROW_MAP_LARGE};
use crate::zia::{zia_get_row_width};

/// Mux selection for the ZIA input from this I/O pin's input. The ZIA input can be chosen to come from either the
/// input pin directly or from the output of the register in the macrocell corresponding to this I/O pin. The latter
/// is used to allow for buried combinatorial feedback in a macrocell without "wasting" the register.
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2IOBZIAMode {
    #[bits("X1")]
    Disabled,
    #[bits("00")]
    PAD,
    #[bits("10")]
    REG,
}

impl fmt::Display for XC2IOBZIAMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &XC2IOBZIAMode::Disabled => "disabled",
            &XC2IOBZIAMode::PAD => "input pad",
            &XC2IOBZIAMode::REG => "register",
        })
    }
}

/// Mode selection for the I/O pin's output buffer. See the Xilinx Coolrunner-II documentation for more information.
#[bitpattern(default = XC2BitError::UnsupportedOeConfiguration(bits), errtype = XC2BitError)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2IOBOBufMode {
    #[bits("1111")]
    Disabled,
    #[bits("0000")]
    PushPull,
    #[bits("0001")]
    OpenDrain,
    #[bits("1100")]
    TriStateGTS0,
    #[bits("0010")]
    TriStateGTS1,
    #[bits("1010")]
    TriStateGTS2,
    #[bits("0110")]
    TriStateGTS3,
    #[bits("0100")]
    TriStatePTB,
    #[bits("1000")]
    TriStateCTE,
    #[bits("1110")]
    CGND,
}

impl fmt::Display for XC2IOBOBufMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &XC2IOBOBufMode::Disabled => "disabled",
            &XC2IOBOBufMode::PushPull => "push-pull",
            &XC2IOBOBufMode::OpenDrain => "open-drain",
            &XC2IOBOBufMode::TriStateGTS0 => "GTS0-controlled tri-state",
            &XC2IOBOBufMode::TriStateGTS1 => "GTS1-controlled tri-state",
            &XC2IOBOBufMode::TriStateGTS2 => "GTS2-controlled tri-state",
            &XC2IOBOBufMode::TriStateGTS3 => "GTS3-controlled tri-state",
            &XC2IOBOBufMode::TriStatePTB => "PTB-controlled tri-state",
            &XC2IOBOBufMode::TriStateCTE => "CTE-controlled tri-state",
            &XC2IOBOBufMode::CGND => "CGND",
        })
    }
}

/// Represents an I/O pin on "small" (32 and 64 macrocell) devices.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[derive(BitTwiddler)]
// FIXME: Probably should not be pub
#[bittwiddler = "jed_internal pub err=XC2BitError"]
#[bittwiddler = "crbit32 mirror0 err=XC2BitError"]
#[bittwiddler = "crbit64 mirror0 err=XC2BitError"]
// #[bittwiddler = "crbit_internal"]
pub struct XC2MCSmallIOB {
    /// Mux selection for the ZIA input for this pin
    #[bittwiddler_field = "jed_internal arr 11 12"]
    #[bittwiddler_field = "crbit32 arr 2|1 3|1"]
    #[bittwiddler_field = "crbit64 arr 5|1 6|1"]
    pub zia_mode: XC2IOBZIAMode,
    /// Whether the Schmitt trigger is being used on this pin's input
    #[bittwiddler_field = "jed_internal 16"]
    #[bittwiddler_field = "crbit32 7|1"]
    #[bittwiddler_field = "crbit64 1|1"]
    pub schmitt_trigger: bool,
    /// Selects the source used to drive this pin's output (if the output is enabled).
    /// `false` selects the XOR gate in the macrocell (combinatorial output), and `true` selects the register output
    /// (registered output).
    #[bittwiddler_field = "jed_internal !19"]
    #[bittwiddler_field = "crbit32 !1|2"]
    #[bittwiddler_field = "crbit64 !0|1"]
    pub obuf_uses_ff: bool,
    /// Selects the output mode for this pin
    #[bittwiddler_field = "jed_internal err arr 20 21 22 23"]
    #[bittwiddler_field = "crbit32 err arr 2|2 3|2 4|2 5|2"]
    #[bittwiddler_field = "crbit64 err arr 3|2 4|2 5|2 6|2"]
    pub obuf_mode: XC2IOBOBufMode,
    /// Selects if the global termination (bus hold or pull-up) is enabled on this pin
    #[bittwiddler_field = "jed_internal 24"]
    #[bittwiddler_field = "crbit32 6|2"]
    #[bittwiddler_field = "crbit64 2|2"]
    pub termination_enabled: bool,
    /// Selects if fast slew rate is used on this pin
    #[bittwiddler_field = "jed_internal !25"]
    #[bittwiddler_field = "crbit32 !7|2"]
    #[bittwiddler_field = "crbit64 !1|2"]
    pub slew_is_fast: bool,
}

impl Default for XC2MCSmallIOB {
    /// Returns a "default" I/O pin configuration. The default state is for the output and the input into the ZIA
    /// to be disabled.

    // FIXME: Do the other defaults come from the particular way I invoked the Xilinx tools??
    fn default() -> Self {
        XC2MCSmallIOB {
            zia_mode: XC2IOBZIAMode::Disabled,
            schmitt_trigger: true,
            obuf_uses_ff: false,
            obuf_mode: XC2IOBOBufMode::Disabled,
            termination_enabled: true,
            slew_is_fast: true,
        }
    }
}

impl fmt::Display for XC2MCSmallIOB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "output mode: {}\n", self.obuf_mode)?;
        write!(f, "output comes from {}\n", if self.obuf_uses_ff {"FF"} else {"XOR gate"})?;
        write!(f, "slew rate: {}\n", if self.slew_is_fast {"fast"} else {"slow"})?;
        write!(f, "ZIA driven from: {}\n", self.zia_mode)?;
        write!(f, "Schmitt trigger input: {}\n", if self.schmitt_trigger {"yes"} else {"no"})?;
        write!(f, "termination: {}\n", if self.termination_enabled {"yes"} else {"no"})?;

        Ok(())
    }
}

impl XC2MCSmallIOB {
    /// Write the crbit representation of the settings for this IO pin to the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `iob` must be the index of this IO pin.
    pub fn to_crbit(&self, device: XC2Device, iob: u32, fuse_array: &mut FuseArray) {
        let (fb, mc) = iob_num_to_fb_mc_num(device, iob).unwrap();
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                // The "32" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                self.encode_crbit32(fuse_array, (x, y), mirror);
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                // The "64" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                self.encode_crbit64(fuse_array, (x, y), mirror);
            },
            _ => unreachable!(),
        }
    }

    /// Read the crbit representation of the settings for this IO pin from the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `iob` must be the index of this IO pin.
    pub fn from_crbit(device: XC2Device, iob: u32, fuse_array: &FuseArray) -> Result<Self, XC2BitError> {
        let (fb, mc) = iob_num_to_fb_mc_num(device, iob).unwrap();
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                // The "32" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                Self::decode_crbit32(fuse_array, (x, y), mirror)
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                // The "64" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                Self::decode_crbit64(fuse_array, (x, y), mirror)
            },
            _ => unreachable!(),
        }
    }

    /// Internal function that reads only the IO-related bits from the macrocell configuration
    pub fn from_jed(fuses: &[bool], fuse_idx: usize) -> Result<Self, XC2BitError> {
        Self::decode_jed_internal(fuses, fuse_idx)
    }

    /// Helper that prints the IOB and macrocell configuration on the "small" parts
    pub fn to_jed(&self, jed: &mut JEDECFile, device: XC2Device, fuse_base: usize, i: usize) {
        let zia_row_width = zia_get_row_width(device);
        let mc_fuse_base = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
            ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB + i * 27;
     
        self.encode_jed_internal(&mut jed.f, mc_fuse_base);
    }
}

/// Input mode selection on larger parts with VREF
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2IOBIbufMode {
    /// This input buffer is not using VREF, and it is also not using the Schmitt trigger
    #[bits("00")]
    NoVrefNoSt,
    /// This input buffer is not using VREF, but it is using the Schmitt trigger
    #[bits("11")]
    NoVrefSt,
    /// This input buffer is using VREF (supposedly it always has the Schmitt trigger?)
    #[bits("10")]
    UsesVref,
    /// This input pin is serving as VREF
    #[bits("01")]
    IsVref,
}

impl fmt::Display for XC2IOBIbufMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &XC2IOBIbufMode::NoVrefNoSt => "no VREF, no Schmitt trigger",
            &XC2IOBIbufMode::NoVrefSt => "no VREF, Schmitt trigger",
            &XC2IOBIbufMode::UsesVref => "uses VREF (HSTL/SSTL)",
            &XC2IOBIbufMode::IsVref => "is a VREF pin",
        })
    }
}

/// Represents an I/O pin on "large" (128 and greater macrocell) devices.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[derive(BitTwiddler)]
// FIXME: Probably should not be pub
#[bittwiddler = "jed_internal pub err=XC2BitError"]
#[bittwiddler = "crbit256 mirror0 err=XC2BitError"]
#[bittwiddler = "crbit_not256 mirror0 err=XC2BitError"]
pub struct XC2MCLargeIOB {
    /// Mux selection for the ZIA input for this pin
    #[bittwiddler_field = "jed_internal arr 11 12"]
    #[bittwiddler_field = "crbit256 arr 7|1 8|1"]
    #[bittwiddler_field = "crbit_not256 arr 0|0 1|0"]
    pub zia_mode: XC2IOBZIAMode,
    /// Selects the input mode for this pin
    #[bittwiddler_field = "jed_internal arr 8 9"]
    #[bittwiddler_field = "crbit256 arr 0|0 1|0"]
    #[bittwiddler_field = "crbit_not256 arr 5|0 6|0"]
    pub ibuf_mode: XC2IOBIbufMode,
    /// Selects the source used to drive this pin's output (if the output is enabled).
    /// `false` selects the XOR gate in the macrocell (combinatorial output), and `true` selects the register output
    /// (registered output).
    #[bittwiddler_field = "jed_internal !20"]
    #[bittwiddler_field = "crbit256 !8|2"]
    #[bittwiddler_field = "crbit_not256 !8|1"]
    pub obuf_uses_ff: bool,
    /// Selects the output mode for this pin
    #[bittwiddler_field = "jed_internal err arr 13 14 15 16"]
    #[bittwiddler_field = "crbit256 err arr 3|1 4|1 5|1 6|1"]
    #[bittwiddler_field = "crbit_not256 err arr 2|1 3|1 4|1 5|1"]
    pub obuf_mode: XC2IOBOBufMode,
    /// Selects if the global termination (bus hold or pull-up) is enabled on this pin
    #[bittwiddler_field = "jed_internal 26"]
    #[bittwiddler_field = "crbit256 2|2"]
    #[bittwiddler_field = "crbit_not256 7|0"]
    pub termination_enabled: bool,
    /// Selects if fast slew rate is used on this pin
    #[bittwiddler_field = "jed_internal !25"]
    #[bittwiddler_field = "crbit256 !3|2"]
    #[bittwiddler_field = "crbit_not256 !6|1"]
    pub slew_is_fast: bool,
    /// Whether this pin is making use of the DataGate feature
    #[bittwiddler_field = "jed_internal 5"]
    #[bittwiddler_field = "crbit256 4|0"]
    #[bittwiddler_field = "crbit_not256 4|0"]
    pub uses_data_gate: bool,
}

impl Default for XC2MCLargeIOB {
    /// Returns a "default" I/O pin configuration. The default state is for the output and the input into the ZIA
    /// to be disabled.

    // FIXME: Do the other defaults come from the particular way I invoked the Xilinx tools??
    fn default() -> Self {
        XC2MCLargeIOB {
            zia_mode: XC2IOBZIAMode::Disabled,
            ibuf_mode: XC2IOBIbufMode::NoVrefSt,
            obuf_uses_ff: false,
            obuf_mode: XC2IOBOBufMode::Disabled,
            termination_enabled: true,
            slew_is_fast: true,
            uses_data_gate: false,
        }
    }
}

impl fmt::Display for XC2MCLargeIOB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "output mode: {}\n", self.obuf_mode)?;
        write!(f, "input mode: {}\n", self.ibuf_mode)?;
        write!(f, "output comes from {}\n", if self.obuf_uses_ff {"FF"} else {"XOR gate"})?;
        write!(f, "slew rate: {}\n", if self.slew_is_fast {"fast"} else {"slow"})?;
        write!(f, "ZIA driven from: {}\n", self.zia_mode)?;
        write!(f, "termination: {}\n", if self.termination_enabled {"yes"} else {"no"})?;
        write!(f, "DataGate used: {}\n", if self.uses_data_gate {"yes"} else {"no"})?;

        Ok(())
    }
}

impl XC2MCLargeIOB {
    /// Write the crbit representation of the settings for this IO pin to the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `iob` must be the index of this IO pin.
    pub fn to_crbit(&self, device: XC2Device, iob: u32, fuse_array: &mut FuseArray) {
        let (fb, mc) = iob_num_to_fb_mc_num(device, iob).unwrap();
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C256 => {
                // The "256" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                self.encode_crbit256(fuse_array, (x, y), mirror);
            },
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                // The "common large macrocell" variant
                // we need this funny lookup table, but otherwise macrocells are 2x15
                let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
                self.encode_crbit_not256(fuse_array, (x, y), mirror);
            },
            _ => unreachable!(),
        }
    }

    /// Read the crbit representation of the settings for this IO pin from the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `iob` must be the index of this IO pin.
    pub fn from_crbit(device: XC2Device, iob: u32, fuse_array: &FuseArray) -> Result<Self, XC2BitError> {
        let (fb, mc) = iob_num_to_fb_mc_num(device, iob).unwrap();
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C256 => {
                // The "256" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                Self::decode_crbit256(fuse_array, (x, y), mirror)
            },
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                // The "common large macrocell" variant
                // we need this funny lookup table, but otherwise macrocells are 2x15
                let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
                Self::decode_crbit_not256(fuse_array, (x, y), mirror)
            },
            _ => unreachable!(),
        }
    }

    /// Internal function that reads only the IO-related bits from the macrocell configuration
    pub fn from_jed(fuses: &[bool], fuse_idx: usize) -> Result<Self, XC2BitError> {
        Self::decode_jed_internal(fuses, fuse_idx)
    }

    /// Helper that prints the IOB configuration on the "large" parts
    pub fn to_jed(&self, jed: &mut JEDECFile, fuse_base: usize) {
        self.encode_jed_internal(&mut jed.f, fuse_base);
    }
}

/// Represents the one additional special input-only pin on 32-macrocell devices.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[derive(BitTwiddler)]
#[bittwiddler = "jed pub abs"]
#[bittwiddler = "crbit pub abs"]
pub struct XC2ExtraIBuf {
    #[bittwiddler_field = "jed 12272"]
    #[bittwiddler_field = "crbit 131|24"]
    pub schmitt_trigger: bool,
    #[bittwiddler_field = "jed 12273"]
    #[bittwiddler_field = "crbit 132|24"]
    pub termination_enabled: bool,
}

impl Default for XC2ExtraIBuf {
    /// Returns a "default" pin configuration.

    // FIXME: Do the other defaults come from the particular way I invoked the Xilinx tools??
    fn default() -> Self {
        XC2ExtraIBuf {
            schmitt_trigger: true,
            termination_enabled: true,
        }
    }
}

impl fmt::Display for XC2ExtraIBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Schmitt trigger input: {}\n", if self.schmitt_trigger {"yes"} else {"no"})?;
        write!(f, "termination: {}\n", if self.termination_enabled {"yes"} else {"no"})?;

        Ok(())
    }
}

/// Function to map from the internal numbering scheme for I/O pins to a function block and macrocell number.
pub fn iob_num_to_fb_mc_num(device: XC2Device, iob: u32) -> Option<(u32, u32)> {
    match device {
        XC2Device::XC2C32 | XC2Device::XC2C32A => {
            if iob >= 32 {
                None
            } else {
                Some((iob / MCS_PER_FB as u32, iob % MCS_PER_FB as u32))
            }
        },
        XC2Device::XC2C64 | XC2Device::XC2C64A => {
            if iob >= 64 {
                None
            } else {
                Some((iob / MCS_PER_FB as u32, iob % MCS_PER_FB as u32))
            }
        },
        XC2Device::XC2C128 => {
            match iob {
                // "Missing" 4 IOBs
                 0..= 5 => Some((0, iob -  0 +  0)),
                 6..=11 => Some((0, iob -  6 + 10)),
                12..=17 => Some((1, iob - 12 +  0)),
                18..=23 => Some((1, iob - 18 + 10)),
                // "Missing" 3 IOBs
                24..=30 => Some((2, iob - 24 +  0)),
                31..=36 => Some((2, iob - 31 + 10)),
                37..=43 => Some((3, iob - 37 +  0)),
                44..=49 => Some((3, iob - 44 + 10)),
                50..=56 => Some((4, iob - 50 +  0)),
                57..=62 => Some((4, iob - 57 + 10)),
                // "Missing" 4 IOBs
                63..=68 => Some((5, iob - 63 +  0)),
                69..=74 => Some((5, iob - 69 + 10)),
                // "Missing" 3 IOBs
                75..=81 => Some((6, iob - 75 +  0)),
                82..=87 => Some((6, iob - 82 + 10)),
                // "Missing" 4 IOBs
                88..=93 => Some((7, iob - 88 +  0)),
                94..=99 => Some((7, iob - 94 + 10)),
                _ => None,
            }
        },
        XC2Device::XC2C256 => {
            match iob {
                // "Missing" 5 IOBs
                  0..=  5 => Some(( 0, iob -   0 +  0)),
                  6..= 10 => Some(( 0, iob -   6 + 11)),
                 11..= 16 => Some(( 1, iob -  11 +  0)),
                 17..= 21 => Some(( 1, iob -  17 + 11)),
                 22..= 27 => Some(( 2, iob -  22 +  0)),
                 28..= 32 => Some(( 2, iob -  28 + 11)),
                 33..= 38 => Some(( 3, iob -  33 +  0)),
                 39..= 43 => Some(( 3, iob -  39 + 11)),
                 44..= 49 => Some(( 4, iob -  44 +  0)),
                 50..= 54 => Some(( 4, iob -  50 + 11)),
                 55..= 60 => Some(( 5, iob -  55 +  0)),
                 61..= 65 => Some(( 5, iob -  61 + 11)),
                // "Missing" 4 IOBs
                 66..= 71 => Some(( 6, iob -  66 +  0)),
                 72..= 77 => Some(( 6, iob -  72 + 10)),
                 78..= 83 => Some(( 7, iob -  78 +  0)),
                 84..= 89 => Some(( 7, iob -  84 + 10)),
                 90..= 95 => Some(( 8, iob -  90 +  0)),
                 96..=101 => Some(( 8, iob -  96 + 10)),
                102..=107 => Some(( 9, iob - 102 +  0)),
                108..=113 => Some(( 9, iob - 108 + 10)),
                114..=119 => Some((10, iob - 114 +  0)),
                120..=125 => Some((10, iob - 120 + 10)),
                126..=131 => Some((11, iob - 126 +  0)),
                132..=137 => Some((11, iob - 132 + 10)),
                // "Missing" 5 IOBs
                138..=143 => Some((12, iob - 138 +  0)),
                144..=148 => Some((12, iob - 144 + 11)),
                149..=154 => Some((13, iob - 149 +  0)),
                155..=159 => Some((13, iob - 155 + 11)),
                160..=165 => Some((14, iob - 160 +  0)),
                166..=171 => Some((14, iob - 166 + 10)),
                172..=177 => Some((15, iob - 172 +  0)),
                178..=183 => Some((15, iob - 178 + 10)),
                // "Missing" 4 IOBs
                _ => None,
            }
        },
        XC2Device::XC2C384 => {
            if iob >= 240 {
                None
            } else {
                // every FB is "missing" 6 IOBs
                let fb = iob / 10;
                let mut mc = iob % 10;
                if mc >= 5 {
                    mc = mc + 6;
                }

                Some((fb, mc))
            }
        },
        XC2Device::XC2C512 => {
            match iob {
                // "Missing" 8 IOBs
                  0..=  3 => Some(( 0, iob -   0 +  0)),
                  4..=  7 => Some(( 0, iob -   4 + 12)),
                  8..= 11 => Some(( 1, iob -   8 +  0)),
                 12..= 15 => Some(( 1, iob -  12 + 12)),
                // "Missing" 7 IOBs
                 16..= 20 => Some(( 2, iob -  16 +  0)),
                 21..= 24 => Some(( 2, iob -  21 + 12)),
                // "Missing" 8 IOBs
                 25..= 28 => Some(( 3, iob -  25 +  0)),
                 29..= 32 => Some(( 3, iob -  29 + 12)),
                // "Missing" 7 IOBs
                 33..= 37 => Some(( 4, iob -  33 +  0)),
                 38..= 41 => Some(( 4, iob -  38 + 12)),
                // "Missing" 8 IOBs
                 42..= 45 => Some(( 5, iob -  42 +  0)),
                 46..= 49 => Some(( 5, iob -  46 + 12)),
                // "Missing" 7 IOBs
                 50..= 54 => Some(( 6, iob -  50 +  0)),
                 55..= 58 => Some(( 6, iob -  55 + 12)),
                // "Missing" 8 IOBs
                 59..= 62 => Some(( 7, iob -  59 +  0)),
                 63..= 66 => Some(( 7, iob -  63 + 12)),
                 67..= 70 => Some(( 8, iob -  67 +  0)),
                 71..= 74 => Some(( 8, iob -  71 + 12)),
                 75..= 78 => Some(( 9, iob -  75 +  0)),
                 79..= 82 => Some(( 9, iob -  79 + 12)),
                 83..= 86 => Some((10, iob -  83 +  0)),
                 87..= 90 => Some((10, iob -  87 + 12)),
                // "Missing" 7 IOBs
                 91..= 95 => Some((11, iob -  91 +  0)),
                 96..= 99 => Some((11, iob -  96 + 12)),
                // "Missing" 8 IOBs
                100..=103 => Some((12, iob - 100 +  0)),
                104..=107 => Some((12, iob - 104 + 12)),
                // "Missing" 7 IOBs
                108..=112 => Some((13, iob - 108 +  0)),
                113..=116 => Some((13, iob - 113 + 12)),
                // "Missing" 8 IOBs
                117..=120 => Some((14, iob - 117 +  0)),
                121..=124 => Some((14, iob - 121 + 12)),
                // "Missing" 7 IOBs
                125..=129 => Some((15, iob - 125 +  0)),
                130..=133 => Some((15, iob - 130 + 12)),
                134..=138 => Some((16, iob - 134 +  0)),
                139..=142 => Some((16, iob - 139 + 12)),
                // "Missing" 8 IOBs
                143..=146 => Some((17, iob - 143 +  0)),
                147..=150 => Some((17, iob - 147 + 12)),
                // "Missing" 7 IOBs
                151..=155 => Some((18, iob - 151 +  0)),
                156..=159 => Some((18, iob - 156 + 12)),
                // "Missing" 8 IOBs
                160..=163 => Some((19, iob - 160 +  0)),
                164..=167 => Some((19, iob - 164 + 12)),
                // "Missing" 7 IOBs
                168..=172 => Some((20, iob - 168 +  0)),
                173..=176 => Some((20, iob - 173 + 12)),
                // "Missing" 8 IOBs
                177..=180 => Some((21, iob - 177 +  0)),
                181..=184 => Some((21, iob - 181 + 12)),
                // "Missing" 7 IOBs
                185..=189 => Some((22, iob - 185 +  0)),
                190..=193 => Some((22, iob - 190 + 12)),
                // "Missing" 8 IOBs
                194..=197 => Some((23, iob - 194 +  0)),
                198..=201 => Some((23, iob - 198 + 12)),
                202..=205 => Some((24, iob - 202 +  0)),
                206..=209 => Some((24, iob - 206 + 12)),
                // "Missing" 7 IOBs
                210..=214 => Some((25, iob - 210 +  0)),
                215..=218 => Some((25, iob - 215 + 12)),
                // "Missing" 8 IOBs
                219..=222 => Some((26, iob - 219 +  0)),
                223..=226 => Some((26, iob - 223 + 12)),
                // "Missing" 7 IOBs
                227..=231 => Some((27, iob - 227 +  0)),
                232..=235 => Some((27, iob - 232 + 12)),
                // "Missing" 8 IOBs
                236..=239 => Some((28, iob - 236 +  0)),
                240..=243 => Some((28, iob - 240 + 12)),
                // "Missing" 7 IOBs
                244..=248 => Some((29, iob - 244 +  0)),
                249..=252 => Some((29, iob - 249 + 12)),
                // "Missing" 8 IOBs
                253..=256 => Some((30, iob - 253 +  0)),
                257..=260 => Some((30, iob - 257 + 12)),
                // "Missing" 7 IOBs
                261..=265 => Some((31, iob - 261 +  0)),
                266..=269 => Some((31, iob - 266 + 12)),
                _ => None,
            }
        },
    }
}

/// Function to map from a function block and macrocell number to the internal numbering scheme for I/O pins.
pub fn fb_mc_num_to_iob_num(device: XC2Device, fb: u32, mc: u32) -> Option<u32> {
    match device {
        XC2Device::XC2C32 | XC2Device::XC2C32A => {
            if fb >= 2 || mc >= MCS_PER_FB as u32 {
                None
            } else {
                Some(fb * MCS_PER_FB as u32 + mc)
            }
        },
        XC2Device::XC2C64 | XC2Device::XC2C64A => {
            if fb >= 4 || mc >= MCS_PER_FB as u32 {
                None
            } else {
                Some(fb * MCS_PER_FB as u32 + mc)
            }
        },
        XC2Device::XC2C128 => {
            match fb {
                // "Missing" 4 IOBs
                0 => match mc {
                    0..=5   => Some(0 + mc),
                    10..=15 => Some(6 + (mc - 10)),
                    _ => None,
                },
                1 => match mc {
                    0..=5   => Some(12 + mc),
                    10..=15 => Some(18 + (mc - 10)),
                    _ => None,
                },
                // "Missing" 3 IOBs
                2 => match mc {
                    0..=6   => Some(24 + mc),
                    10..=15 => Some(31 + (mc - 10)),
                    _ => None,
                },
                3 => match mc {
                    0..=6   => Some(37 + mc),
                    10..=15 => Some(44 + (mc - 10)),
                    _ => None,
                },
                4 => match mc {
                    0..=6   => Some(50 + mc),
                    10..=15 => Some(57 + (mc - 10)),
                    _ => None,
                },
                // "Missing" 4 IOBs
                5 => match mc {
                    0..=5   => Some(63 + mc),
                    10..=15 => Some(69 + (mc - 10)),
                    _ => None,
                },
                // "Missing" 3 IOBs
                6 => match mc {
                    0..=6   => Some(75 + mc),
                    10..=15 => Some(82 + (mc - 10)),
                    _ => None,
                },
                // "Missing" 4 IOBs
                7 => match mc {
                    0..=5   => Some(88 + mc),
                    10..=15 => Some(94 + (mc - 10)),
                    _ => None,
                },
                _ => None,
            }
        },
        XC2Device::XC2C256 => {
            match fb {
                // "Missing" 5 IOBs
                0 => match mc {
                    0..=5   => Some(0 + mc),
                    11..=15 => Some(6 + (mc - 11)),
                    _ => None,
                },
                1 => match mc {
                    0..=5   => Some(11 + mc),
                    11..=15 => Some(17 + (mc - 11)),
                    _ => None,
                },
                2 => match mc {
                    0..=5   => Some(22 + mc),
                    11..=15 => Some(28 + (mc - 11)),
                    _ => None,
                },
                3 => match mc {
                    0..=5   => Some(33 + mc),
                    11..=15 => Some(39 + (mc - 11)),
                    _ => None,
                },
                4 => match mc {
                    0..=5   => Some(44 + mc),
                    11..=15 => Some(50 + (mc - 11)),
                    _ => None,
                },
                5 => match mc {
                    0..=5   => Some(55 + mc),
                    11..=15 => Some(61 + (mc - 11)),
                    _ => None,
                },
                // "Missing" 4 IOBs
                6 => match mc {
                    0..=5   => Some(66 + mc),
                    10..=15 => Some(72 + (mc - 10)),
                    _ => None,
                },
                7 => match mc {
                    0..=5   => Some(78 + mc),
                    10..=15 => Some(84 + (mc - 10)),
                    _ => None,
                },
                8 => match mc {
                    0..=5   => Some(90 + mc),
                    10..=15 => Some(96 + (mc - 10)),
                    _ => None,
                },
                9 => match mc {
                    0..=5   => Some(102 + mc),
                    10..=15 => Some(108 + (mc - 10)),
                    _ => None,
                },
                10 => match mc {
                    0..=5   => Some(114 + mc),
                    10..=15 => Some(120 + (mc - 10)),
                    _ => None,
                },
                11 => match mc {
                    0..=5   => Some(126 + mc),
                    10..=15 => Some(132 + (mc - 10)),
                    _ => None,
                },
                // "Missing" 5 IOBs
                12 => match mc {
                    0..=5   => Some(138 + mc),
                    11..=15 => Some(144 + (mc - 11)),
                    _ => None,
                },
                13 => match mc {
                    0..=5   => Some(149 + mc),
                    11..=15 => Some(155 + (mc - 11)),
                    _ => None,
                },
                // "Missing" 4 IOBs
                14 => match mc {
                    0..=5   => Some(160 + mc),
                    10..=15 => Some(166 + (mc - 10)),
                    _ => None,
                },
                15 => match mc {
                    0..=5   => Some(172 + mc),
                    10..=15 => Some(178 + (mc - 10)),
                    _ => None,
                },
                _ => None,
            }
        },
        XC2Device::XC2C384 => {
            // every FB is "missing" 6 IOBs
            match mc {
                0..=4 => Some(10 * fb + mc),
                11..=15 => Some(10 * fb + 5 + (mc - 11)),
                _ => None,
            }
        },
        XC2Device::XC2C512 => {
            match fb {
                // "Missing" 8 IOBs
                0 => match mc {
                    0..=3   => Some(0 + mc),
                    12..=15 => Some(4 + (mc - 12)),
                    _ => None,
                },
                1 => match mc {
                    0..=3   => Some(8 + mc),
                    12..=15 => Some(12 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                2 => match mc {
                    0..=4   => Some(16 + mc),
                    12..=15 => Some(21 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                3 => match mc {
                    0..=3   => Some(25 + mc),
                    12..=15 => Some(29 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                4 => match mc {
                    0..=4   => Some(33 + mc),
                    12..=15 => Some(38 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                5 => match mc {
                    0..=3   => Some(42 + mc),
                    12..=15 => Some(46 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                6 => match mc {
                    0..=4   => Some(50 + mc),
                    12..=15 => Some(55 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                7 => match mc {
                    0..=3   => Some(59 + mc),
                    12..=15 => Some(63 + (mc - 12)),
                    _ => None,
                },
                8 => match mc {
                    0..=3   => Some(67 + mc),
                    12..=15 => Some(71 + (mc - 12)),
                    _ => None,
                },
                9 => match mc {
                    0..=3   => Some(75 + mc),
                    12..=15 => Some(79 + (mc - 12)),
                    _ => None,
                },
                10 => match mc {
                    0..=3   => Some(83 + mc),
                    12..=15 => Some(87 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                11 => match mc {
                    0..=4   => Some(91 + mc),
                    12..=15 => Some(96 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                12 => match mc {
                    0..=3   => Some(100 + mc),
                    12..=15 => Some(104 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                13 => match mc {
                    0..=4   => Some(108 + mc),
                    12..=15 => Some(113 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                14 => match mc {
                    0..=3   => Some(117 + mc),
                    12..=15 => Some(121 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                15 => match mc {
                    0..=4   => Some(125 + mc),
                    12..=15 => Some(130 + (mc - 12)),
                    _ => None,
                },
                16 => match mc {
                    0..=4   => Some(134 + mc),
                    12..=15 => Some(139 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                17 => match mc {
                    0..=3   => Some(143 + mc),
                    12..=15 => Some(147 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                18 => match mc {
                    0..=4   => Some(151 + mc),
                    12..=15 => Some(156 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                19 => match mc {
                    0..=3   => Some(160 + mc),
                    12..=15 => Some(164 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                20 => match mc {
                    0..=4   => Some(168 + mc),
                    12..=15 => Some(173 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                21 => match mc {
                    0..=3   => Some(177 + mc),
                    12..=15 => Some(181 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                22 => match mc {
                    0..=4   => Some(185 + mc),
                    12..=15 => Some(190 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                23 => match mc {
                    0..=3   => Some(194 + mc),
                    12..=15 => Some(198 + (mc - 12)),
                    _ => None,
                },
                24 => match mc {
                    0..=3   => Some(202 + mc),
                    12..=15 => Some(206 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                25 => match mc {
                    0..=4   => Some(210 + mc),
                    12..=15 => Some(215 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                26 => match mc {
                    0..=3   => Some(219 + mc),
                    12..=15 => Some(223 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                27 => match mc {
                    0..=4   => Some(227 + mc),
                    12..=15 => Some(232 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                28 => match mc {
                    0..=3   => Some(236 + mc),
                    12..=15 => Some(240 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                29 => match mc {
                    0..=4   => Some(244 + mc),
                    12..=15 => Some(249 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 8 IOBs
                30 => match mc {
                    0..=3   => Some(253 + mc),
                    12..=15 => Some(257 + (mc - 12)),
                    _ => None,
                },
                // "Missing" 7 IOBs
                31 => match mc {
                    0..=4   => Some(261 + mc),
                    12..=15 => Some(266 + (mc - 12)),
                    _ => None,
                },
                _ => None,
            }
        }
    }
}
