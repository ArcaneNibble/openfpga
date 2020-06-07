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

//! Contains functions pertaining to global control bits (e.g. clocks)

use core::fmt;

use crate::*;

pub enum JedCommon {}
pub enum JedXC2C32 {}
pub enum JedXC2C64 {}
pub enum JedXC2C128 {}
pub enum JedXC2C256 {}
pub enum JedXC2C384 {}
pub enum JedXC2C512 {}

pub enum CrbitXC2C32 {}
pub enum CrbitXC2C64 {}
pub enum CrbitXC2C128 {}
pub enum CrbitXC2C256 {}
pub enum CrbitXC2C384 {}
pub enum CrbitXC2C512 {}

/// Represents the configuration of the global nets. Coolrunner-II parts have various global control signals that have
/// dedicated low-skew paths.
#[bitfragment(variant = JedXC2C32, dimensions = 1)]
#[bitfragment(variant = JedXC2C64, dimensions = 1)]
#[bitfragment(variant = JedXC2C128, dimensions = 1)]
#[bitfragment(variant = JedXC2C256, dimensions = 1)]
#[bitfragment(variant = JedXC2C384, dimensions = 1)]
#[bitfragment(variant = JedXC2C512, dimensions = 1)]

#[bitfragment(variant = CrbitXC2C32, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C64, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C128, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C256, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C384, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C512, dimensions = 2)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2GlobalNets {
    /// Controls whether the three global clock nets are enabled or not
    #[offset(variant = JedXC2C32, [12256])]
    #[arr_off(variant = JedXC2C32, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = 0)]
    #[offset(variant = JedXC2C64, [25792])]
    #[arr_off(variant = JedXC2C64, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = 0)]
    #[offset(variant = JedXC2C128, [55316])]
    #[arr_off(variant = JedXC2C128, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = 0)]
    #[offset(variant = JedXC2C256, [123224])]
    #[arr_off(variant = JedXC2C256, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = 0)]
    #[offset(variant = JedXC2C384, [209328])]
    #[arr_off(variant = JedXC2C384, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = 0)]
    #[offset(variant = JedXC2C512, [296374])]
    #[arr_off(variant = JedXC2C512, |i| [i])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = 0)]

    #[arr_off(variant = CrbitXC2C32, |i|
        [[126, 23], [127, 23], [128, 23]][i])]
    #[pat_bits(frag_variant = CrbitXC2C32, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C64, |i|
        [[133, 23], [134, 23], [135, 23]][i])]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C128, |i|
        [[365, 67], [366, 67], [367, 67]][i])]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C256, |i|
        [[519, 23], [520, 23], [521, 23]][i])]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C384, |i|
        [[467, 102], [468, 102], [469, 102]][i])]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C512, |i|
        [[979, 147], [980, 147], [981, 147]][i])]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = (0, 0))]
    pub gck_enable: [bool; 3],

    /// Controls whether the global set/reset net is enabled or not
    #[offset(variant = JedXC2C32, [12259 + 1])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = 0)]
    #[offset(variant = JedXC2C64, [25795 + 1])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = 0)]
    #[offset(variant = JedXC2C128, [55324 + 1])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = 0)]
    #[offset(variant = JedXC2C256, [123232 + 1])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = 0)]
    #[offset(variant = JedXC2C384, [209336 + 1])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = 0)]
    #[offset(variant = JedXC2C512, [296382 + 1])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = 0)]

    #[pat_bits(frag_variant = CrbitXC2C32, "0" = (130, 23))]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = (136, 73))]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = (2, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = (179, 23))]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = (2, 97))]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = (2, 27))]
    pub gsr_enable: bool,

    /// Controls the polarity of the global set/reset signal
    ///
    /// `false` = active low, `true` = active high
    #[offset(variant = JedXC2C32, [12259])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = 0)]
    #[offset(variant = JedXC2C64, [25795])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = 0)]
    #[offset(variant = JedXC2C128, [55324])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = 0)]
    #[offset(variant = JedXC2C256, [123232])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = 0)]
    #[offset(variant = JedXC2C384, [209336])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = 0)]
    #[offset(variant = JedXC2C512, [296382])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = 0)]

    #[pat_bits(frag_variant = CrbitXC2C32, "0" = (129, 23))]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = (135, 73))]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = (1, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = (178, 23))]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = (1, 97))]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = (1, 27))]
    pub gsr_invert: bool,

    /// Controls whether the four global tristate nets are enabled or not
    #[offset(variant = JedXC2C32, [12261 + 1])]
    #[arr_off(variant = JedXC2C32, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = !0)]
    #[offset(variant = JedXC2C64, [25797 + 1])]
    #[arr_off(variant = JedXC2C64, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = !0)]
    #[offset(variant = JedXC2C128, [55326 + 1])]
    #[arr_off(variant = JedXC2C128, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = !0)]
    #[offset(variant = JedXC2C256, [123234 + 1])]
    #[arr_off(variant = JedXC2C256, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = !0)]
    #[offset(variant = JedXC2C384, [209338 + 1])]
    #[arr_off(variant = JedXC2C384, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = !0)]
    #[offset(variant = JedXC2C512, [296384 + 1])]
    #[arr_off(variant = JedXC2C512, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = !0)]

    #[arr_off(variant = CrbitXC2C32, |i|
        [[127, 24], [129, 24], [127, 25], [129, 25]][i])]
    #[pat_bits(frag_variant = CrbitXC2C32, "0" = !(0, 0))]
    #[arr_off(variant = CrbitXC2C64, |i|
        [[134, 24], [136, 24], [138, 73], [138, 24]][i])]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = !(0, 0))]
    #[arr_off(variant = CrbitXC2C128, |i|
        [[5, 27], [7, 27], [5, 67], [7, 67]][i])]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = !(0, 0))]
    #[arr_off(variant = CrbitXC2C256, |i|
        [[182, 23], [177, 24], [179, 24], [182, 24]][i])]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = !(0, 0))]
    #[arr_off(variant = CrbitXC2C384, |i|
        [[463, 107], [464, 107], [465, 107], [466, 107]][i])]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = !(0, 0))]
    #[arr_off(variant = CrbitXC2C512, |i|
        [[4, 27], [481, 27], [6, 27], [8, 27]][i])]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = !(0, 0))]
    pub gts_enable: [bool; 4],

    /// Controls the polarity of the global tristate signal
    ///
    /// `false` = used as T, `true` = used as !T
    #[offset(variant = JedXC2C32, [12261])]
    #[arr_off(variant = JedXC2C32, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = 0)]
    #[offset(variant = JedXC2C64, [25797])]
    #[arr_off(variant = JedXC2C64, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = 0)]
    #[offset(variant = JedXC2C128, [55326])]
    #[arr_off(variant = JedXC2C128, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = 0)]
    #[offset(variant = JedXC2C256, [123234])]
    #[arr_off(variant = JedXC2C256, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = 0)]
    #[offset(variant = JedXC2C384, [209338])]
    #[arr_off(variant = JedXC2C384, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = 0)]
    #[offset(variant = JedXC2C512, [296384])]
    #[arr_off(variant = JedXC2C512, |i| [i * 2])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = 0)]

    #[arr_off(variant = CrbitXC2C32, |i|
        [[126, 24], [128, 24], [126, 25], [128, 25]][i])]
    #[pat_bits(frag_variant = CrbitXC2C32, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C64, |i|
        [[133, 24], [135, 24], [137, 73], [137, 24]][i])]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C128, |i|
        [[4, 27], [6, 27], [4, 67], [6, 67]][i])]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C256, |i|
        [[181, 23], [176, 24], [178, 24], [181, 24]][i])]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C384, |i|
        [[463, 102], [464, 102], [465, 102], [466, 102]][i])]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = (0, 0))]
    #[arr_off(variant = CrbitXC2C512, |i|
        [[3, 27], [480, 27], [5, 27], [7, 27]][i])]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = (0, 0))]
    pub gts_invert: [bool; 4],

    /// Controls the mode of the global termination
    ///
    /// `false` = keeper, `true` = pull-up
    #[offset(variant = JedXC2C32, [12269])]
    #[pat_bits(frag_variant = JedXC2C32, "0" = 0)]
    #[offset(variant = JedXC2C64, [25805])]
    #[pat_bits(frag_variant = JedXC2C64, "0" = 0)]
    #[offset(variant = JedXC2C128, [55334])]
    #[pat_bits(frag_variant = JedXC2C128, "0" = 0)]
    #[offset(variant = JedXC2C256, [123242])]
    #[pat_bits(frag_variant = JedXC2C256, "0" = 0)]
    #[offset(variant = JedXC2C384, [209346])]
    #[pat_bits(frag_variant = JedXC2C384, "0" = 0)]
    #[offset(variant = JedXC2C512, [296392])]
    #[pat_bits(frag_variant = JedXC2C512, "0" = 0)]

    #[pat_bits(frag_variant = CrbitXC2C32, "0" = (131, 23))]
    #[pat_bits(frag_variant = CrbitXC2C64, "0" = (136, 23))]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = (370, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = (517, 23))]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = (931, 17))]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = (983, 147))]
    pub global_pu: bool,
}

impl Default for XC2GlobalNets {
    /// Returns a "default" global net configuration which has everything disabled.
    fn default() -> Self {
        XC2GlobalNets {
            gck_enable: [false; 3],
            gsr_enable: false,
            gsr_invert: false,
            gts_enable: [false; 4],
            gts_invert: [true; 4],
            global_pu: true,
        }
    }
}

impl fmt::Display for XC2GlobalNets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GCK0 {}\n", if self.gck_enable[0] {"enabled"} else {"disabled"})?;
        write!(f, "GCK1 {}\n", if self.gck_enable[1] {"enabled"} else {"disabled"})?;
        write!(f, "GCK2 {}\n", if self.gck_enable[2] {"enabled"} else {"disabled"})?;

        write!(f, "GSR {}, active {}\n",
            if self.gsr_enable {"enabled"} else {"disabled"},
            if self.gsr_invert {"high"} else {"low"})?;

        write!(f, "GTS0 {}, acts as {}\n",
            if self.gts_enable[0] {"enabled"} else {"disabled"},
            if self.gts_invert[0] {"!T"} else {"T"})?;
        write!(f, "GTS1 {}, acts as {}\n",
            if self.gts_enable[1] {"enabled"} else {"disabled"},
            if self.gts_invert[1] {"!T"} else {"T"})?;
        write!(f, "GTS2 {}, acts as {}\n",
            if self.gts_enable[2] {"enabled"} else {"disabled"},
            if self.gts_invert[2] {"!T"} else {"T"})?;
        write!(f, "GTS3 {}, acts as {}\n",
            if self.gts_enable[3] {"enabled"} else {"disabled"},
            if self.gts_invert[3] {"!T"} else {"T"})?;

        write!(f, "global termination is {}\n", if self.global_pu {"pull-up"} else {"bus hold"})?;

        Ok(())
    }
}

impl XC2GlobalNets {
    /// Internal function to read the global nets
    pub fn from_crbit(device: XC2Device, fuse_array: &FuseArray) -> Self {
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                <Self as BitFragment<CrbitXC2C32>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                <Self as BitFragment<CrbitXC2C64>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
            XC2Device::XC2C128 => {
                <Self as BitFragment<CrbitXC2C128>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
            XC2Device::XC2C256 => {
                <Self as BitFragment<CrbitXC2C256>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
            XC2Device::XC2C384 => {
                <Self as BitFragment<CrbitXC2C384>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
            XC2Device::XC2C512 => {
                <Self as BitFragment<CrbitXC2C512>>::decode(
                    fuse_array, [0, 0], [false, false], ()).unwrap()
            },
        }
    }
}

/// Possible clock divide ratios for the programmable clock divider
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2ClockDivRatio {
    #[bits("000")]
    Div2,
    #[bits("001")]
    Div4,
    #[bits("010")]
    Div6,
    #[bits("011")]
    Div8,
    #[bits("100")]
    Div10,
    #[bits("101")]
    Div12,
    #[bits("110")]
    Div14,
    #[bits("111")]
    Div16,
}

/// Represents the configuration of the programmable clock divider in devices with 128 macrocells or more. This is
/// hard-wired onto the GCK2 clock pin.
#[bitfragment(variant = JedCommon, dimensions = 1)]
#[bitfragment(variant = CrbitXC2C128, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C256, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C384, dimensions = 2)]
#[bitfragment(variant = CrbitXC2C512, dimensions = 2)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2ClockDiv {
    /// Ratio that input clock is divided by
    #[pat_pict(frag_variant = JedCommon, ". 0 1 2 .")]
    #[pat_bits(frag_variant = CrbitXC2C128,
        "0" = (363, 67),
        "1" = (362, 67),
        "2" = (361, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256,
        "0" = (518, 24),
        "1" = (517, 24),
        "2" = (516, 24))]
    #[pat_bits(frag_variant = CrbitXC2C384,
        "0" = (470, 107),
        "1" = (469, 107),
        "2" = (468, 107))]
    #[pat_bits(frag_variant = CrbitXC2C512,
        "0" = (977, 147),
        "1" = (976, 147),
        "2" = (975, 147))]
    pub div_ratio: XC2ClockDivRatio,
    /// Whether the "delay" feature is enabled
    #[pat_pict(frag_variant = JedCommon, ". . . . !0")]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = !(360, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = !(515, 24))]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = !(467, 107))]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = !(974, 147))]
    pub delay: bool,
    /// Whether the clock divider is enabled (other settings are ignored if not)
    #[pat_pict(frag_variant = JedCommon, "!0 . . . .")]
    #[pat_bits(frag_variant = CrbitXC2C128, "0" = !(364, 67))]
    #[pat_bits(frag_variant = CrbitXC2C256, "0" = !(519, 24))]
    #[pat_bits(frag_variant = CrbitXC2C384, "0" = !(471, 107))]
    #[pat_bits(frag_variant = CrbitXC2C512, "0" = !(978, 147))]
    pub enabled: bool,
}

impl fmt::Display for XC2ClockDiv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GCK2 clock divider {}\n", if self.enabled {"enabled"} else {"disabled"})?;
        write!(f, "clock divider delay {}\n", if self.delay {"enabled"} else {"disabled"})?;

        write!(f, "clock division ratio: {}\n", match self.div_ratio {
            XC2ClockDivRatio::Div2 => "2",
            XC2ClockDivRatio::Div4 => "4",
            XC2ClockDivRatio::Div6 => "6",
            XC2ClockDivRatio::Div8 => "8",
            XC2ClockDivRatio::Div10 => "10",
            XC2ClockDivRatio::Div12 => "12",
            XC2ClockDivRatio::Div14 => "14",
            XC2ClockDivRatio::Div16 => "16",
        })?;

        Ok(())
    }
}

impl Default for XC2ClockDiv {
    /// Returns a "default" clock divider configuration, which is one that is not used
    fn default() -> Self {
        XC2ClockDiv {
            div_ratio: XC2ClockDivRatio::Div16,
            delay: false,
            enabled: false,
        }
    }
}
