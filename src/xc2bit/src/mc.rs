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

//! Contains functions pertaining to macrocells

use core::fmt;

use jedec::*;

use crate::*;
use crate::fusemap_physical::{mc_block_loc};
use crate::util::{LinebreakSet};
use crate::zia::{zia_get_row_width};

/// Clock source for the register in a macrocell
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCRegClkSrc {
    #[bits("x00")]
    GCK0,
    #[bits("x10")]
    GCK1,
    #[bits("x01")]
    GCK2,
    #[bits("011")]
    PTC,
    #[bits("111")]
    CTC,
}

/// Reset source for the register in a macrocell
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCRegResetSrc {
    #[bits("11")]
    Disabled,
    #[bits("00")]
    PTA,
    #[bits("01")]
    GSR,
    #[bits("10")]
    CTR,
}

/// Set source for the register in a macrocell
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCRegSetSrc {
    #[bits("11")]
    Disabled,
    #[bits("00")]
    PTA,
    #[bits("01")]
    GSR,
    #[bits("10")]
    CTS,
}

/// Mode of the register in a macrocell.
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCRegMode {
    /// D-type flip-flop
    #[bits("00")]
    DFF,
    /// Transparent latch
    #[bits("01")]
    LATCH,
    /// Toggle flip-flop
    #[bits("10")]
    TFF,
    /// D-type flip-flop with clock-enable pin
    #[bits("11")]
    DFFCE,
}

/// Mux selection for the ZIA input from this macrocell. The ZIA input can be chosen to come from either the XOR gate
/// or from the output of the register.
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCFeedbackMode {
    #[bits("X1")]
    Disabled,
    #[bits("00")]
    COMB,
    #[bits("10")]
    REG,
}

/// Mux selection for the "not from OR gate" input to the XOR gate. The XOR gate in a macrocell contains two inputs,
/// the output of the corresponding OR term from the PLA and a specific dedicated AND term from the PLA.
#[bitpattern]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum XC2MCXorMode {
    /// A constant zero which results in this XOR outputting the value of the OR term
    #[bits("00")]
    ZERO,
    /// A constant one which results in this XOR outputting the complement of the OR term
    #[bits("11")]
    ONE,
    /// XOR the OR term with the special product term C
    #[bits("10")]
    PTC,
    /// XNOR the OR term with the special product term C
    #[bits("01")]
    PTCB,
}

pub enum JedSmall {}
pub enum JedLarge {}
pub enum JedLargeUnburied {}
pub enum JedLargeBuried {}
pub enum Crbit32 {}
pub enum Crbit64 {}
pub enum Crbit256 {}
pub enum CrbitLarge {}

/// Represents a macrocell.
#[bitfragment(variant = JedSmall, dimensions = 1)]
#[bitfragment(variant = JedLargeUnburied, dimensions = 1)]
#[bitfragment(variant = JedLargeBuried, dimensions = 1)]
#[bitfragment(variant = Crbit32, dimensions = 2)]
#[bitfragment(variant = Crbit64, dimensions = 2)]
#[bitfragment(variant = Crbit256, dimensions = 2)]
#[bitfragment(variant = CrbitLarge, dimensions = 2)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2Macrocell {
    /// Clock source for the register
    #[pat_pict(frag_variant = JedSmall, "0  .   1 2     .   . .     . .     . .     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, "0  1 2     .   .   .   . .     . .     .   . .     . . . .     . .     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, "0    1 2     .   .   . .     . .     .   . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, "0  .  1  2  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  1  2  .  0
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  1  2  0
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  0  1  2  .  .  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub clk_src: XC2MCRegClkSrc,


    /// Specifies the clock polarity for the register
    ///
    /// `false` = rising edge triggered flip-flop, transparent-when-high latch
    ///
    /// `true` = falling edge triggered flip-flop, transparent-when-low latch
    #[pat_pict(frag_variant = JedSmall, ".  0   . .     .   . .     . .     . .     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".   . .    .   0   .   . .     . .     .   . .     . . . .     . .     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   0   . .     . .     .   . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  0  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  0  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  0  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  0  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub clk_invert_pol: bool,


    /// Specifies whether flip-flop are triggered on both clock edges
    ///
    /// It is currently unknown what happens when this is used on a transparent latch
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     0   . .     . .     . .     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".   . .     0  .   .   . .     . .     .   . .     . . . .     . .     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     0   .   . .     . .     .   . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  0  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  0  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  0  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  0  .  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub is_ddr: bool,


    /// Reset source for the register
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   0 1     . .     . .     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".   . .    .   .   .   . .     . .     .   . .     . . . .     . .     .   .   . .     0 1     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   . .     . .     .   . .     0 1     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  0  1  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  0  1  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  0  1  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  0  1  .  .")]
    pub r_src: XC2MCRegResetSrc,


    /// Set source for the register
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     0 1     . .     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".   . .    .   .   .   . .     . .     .   . .     . . . .    0 1     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   . .     0 1     .   . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  0  1
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, "0  1  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         .  0  1  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  0  1")]
    pub s_src: XC2MCRegSetSrc,


    /// Power-up state of the register
    ///
    /// `false` = init to 0, `true` = init to 1
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     . .     . .     . .     . .     .   .   . .     .   . . . .     .   .   !0")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".  . .     .   .   .   . .     . .     .   . .     . . . .     . .     !0  .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   . .     . .     !0  . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  !0")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        !0 .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         !0 .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  .  !0
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub init_state: bool,


    /// Register mode
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     . .     0 1     . .     . .     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".  . .     .   .   .   . .     . .     .   . .     . . . .     . .     .   .   0 1     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   . .     . .     .   0 1     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  .  .
                                        0  1  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  0  1
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  0  1  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  .  .
                                           .  .  .  .  .  .  .  .  .  0  1  .  .  .  .")]
    pub reg_mode: XC2MCRegMode,


    /// ZIA input mode for feedback from this macrocell
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     . .     . .     . .     0 1     .   .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".  . .     .   .   .   0 1     . .     .   . .     . . . .     . .     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   0 1     . .     .   . .     . .     . .")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  0  1  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  .  .
                                        .  .  .  0  1  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  0  1  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  0  1  .  .  .  .  .  .  .  .  .  .  .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub fb_mode: XC2MCFeedbackMode,


    /// Controls the input for the register
    ///
    /// `false` = use the output of the XOR gate (combinatorial path), `true` = use IOB direct path
    /// (`true` is illegal for buried macrocells in the larger devices)
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     . .     . .     . .     . .     !0  .   . .     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".  . .     .   .   .   . .     . .     !0  . .     . . . .     . .     .   .   . .     . .     .   .   . .")]

    #[pat_pict(frag_variant = JedLargeBuried, "\n0=false")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  !0 .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  .  .
                                        .  .  !0 .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  !0
                                         .  .  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  !0 .
                                           .  .  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub ff_in_ibuf: bool,


    /// Controls the "other" (not from the OR term) input to the XOR gate
    #[pat_pict(frag_variant = JedSmall, ".  .   . .     .   . .     . .     . .     . .     . .     .   .   0 1     .   . . . .     .   .   .")]

    #[pat_pict(frag_variant = JedLargeUnburied, ".  . .     .   .   .   . .     . .     .   . .     . . . .     . .     .   .   . .     . .     .   .   0 1")]

    #[pat_pict(frag_variant = JedLargeBuried, ".    . .     .   .   . .     . .     .   . .     . .     0 1")]

    #[pat_pict(frag_variant = Crbit32, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  0
                                        1  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = Crbit64, ".  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  .  .
                                        .  .  .  .  .  .  .  0  1")]

    #[pat_pict(frag_variant = Crbit256, ".  .  .  .  .  .  .  .  .  .
                                         .  .  .  .  .  .  .  .  .  .
                                         0  1  .  .  .  .  .  .  .  .")]

    #[pat_pict(frag_variant = CrbitLarge, ".  .  .  .  .  .  .  .  .  .  .  .  .  .  .
                                           0  1  .  .  .  .  .  .  .  .  .  .  .  .  .")]
    pub xor_mode: XC2MCXorMode,
}

impl BitFragment<JedLarge> for XC2Macrocell {
    const IDX_DIMS: usize = 1;

    type IndexingType = usize;
    type OffsettingType = [isize; 1];
    type MirroringType = [bool; 1];

    type ErrType = ();

    type EncodeExtraType = bool;
    type DecodeExtraType = bool;

    const FIELD_COUNT: usize = 1;

    fn encode<F>(&self, fuses: &mut F,
        offset: Self::OffsettingType, mirror: Self::MirroringType,
        extra_data: Self::EncodeExtraType)
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        if extra_data {
            // Buried
            <Self as BitFragment<JedLargeBuried>>::encode(self, fuses, offset, mirror, ())
        } else {
            // Not buried
            <Self as BitFragment<JedLargeUnburied>>::encode(self, fuses, offset, mirror, ())
        }
    }

    fn decode<F>(fuses: &F,
        offset: Self::OffsettingType, mirror: Self::MirroringType,
        extra_data: Self::DecodeExtraType) -> Result<Self, Self::ErrType>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        if extra_data {
            // Buried
            <Self as BitFragment<JedLargeBuried>>::decode(fuses, offset, mirror, ())
        } else {
            // Not buried
            <Self as BitFragment<JedLargeUnburied>>::decode(fuses, offset, mirror, ())
        }
    }

    #[inline]
    fn fieldname(_i: usize) -> &'static str {
        unimplemented!();
    }
    #[inline]
    fn fielddesc(_i: usize) -> &'static str {
        unimplemented!();
    }
    #[inline]
    fn fieldtype(_i: usize) -> BitFragmentFieldType {
        unimplemented!();
    }
    #[inline]
    fn field_offset(_field_i: usize, _arr_i: usize) -> Self::OffsettingType {
        unimplemented!();
    }
    #[inline]
    fn field_mirror(_field_i: usize, _arr_i: usize) -> Self::MirroringType {
        unimplemented!();
    }
    #[inline]
    fn field_bits(_field_i: usize) -> usize {
        unimplemented!();
    }
    #[inline]
    fn field_bit_base_pos(_field_i: usize, _bit_i: usize) -> Self::OffsettingType {
        unimplemented!();
    }
}

impl Default for XC2Macrocell {
    /// Returns a "default" macrocell configuration.
    // XXX what should the default state be???
    fn default() -> Self {
        XC2Macrocell {
            clk_src: XC2MCRegClkSrc::GCK0,
            clk_invert_pol: false,
            is_ddr: false,
            r_src: XC2MCRegResetSrc::Disabled,
            s_src: XC2MCRegSetSrc::Disabled,
            init_state: true,
            reg_mode: XC2MCRegMode::DFF,
            fb_mode: XC2MCFeedbackMode::Disabled,
            ff_in_ibuf: false,
            xor_mode: XC2MCXorMode::ZERO,
        }
    }
}

pub static MC_TO_ROW_MAP_LARGE: [usize; MCS_PER_FB] =
    [0, 3, 5, 8, 10, 13, 15, 18, 20, 23, 25, 28, 30, 33, 35, 38];

impl fmt::Display for XC2Macrocell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FF mode: {}\n", match self.reg_mode {
            XC2MCRegMode::DFF => "D flip-flop",
            XC2MCRegMode::LATCH => "transparent latch",
            XC2MCRegMode::TFF => "T flip-flop",
            XC2MCRegMode::DFFCE => "D flip-flop with clock-enable",
        })?;
        write!(f, "initial state: {}\n", if self.init_state {1} else {0})?;
        write!(f, "{}-edge triggered\n", if self.clk_invert_pol {"falling"} else {"rising"})?;
        write!(f, "DDR: {}\n", if self.is_ddr {"yes"} else {"no"})?;
        write!(f, "clock source: {}\n", match self.clk_src {
            XC2MCRegClkSrc::GCK0 => "GCK0",
            XC2MCRegClkSrc::GCK1 => "GCK1",
            XC2MCRegClkSrc::GCK2 => "GCK2",
            XC2MCRegClkSrc::PTC => "PTC",
            XC2MCRegClkSrc::CTC => "CTC",
        })?;
        write!(f, "set source: {}\n", match self.s_src {
            XC2MCRegSetSrc::Disabled => "disabled",
            XC2MCRegSetSrc::PTA => "PTA",
            XC2MCRegSetSrc::GSR => "GSR",
            XC2MCRegSetSrc::CTS => "CTS",
        })?;
        write!(f, "reset source: {}\n", match self.r_src {
            XC2MCRegResetSrc::Disabled => "disabled",
            XC2MCRegResetSrc::PTA => "PTA",
            XC2MCRegResetSrc::GSR => "GSR",
            XC2MCRegResetSrc::CTR => "CTR",
        })?;
        write!(f, "using ibuf direct path: {}\n", if self.ff_in_ibuf {"yes"} else {"no"})?;
        write!(f, "XOR gate input: {}\n", match self.xor_mode {
            XC2MCXorMode::ZERO => "0",
            XC2MCXorMode::ONE => "1",
            XC2MCXorMode::PTC => "PTC",
            XC2MCXorMode::PTCB => "~PTC",
        })?;
        write!(f, "ZIA feedback: {}\n", match self.fb_mode {
            XC2MCFeedbackMode::Disabled => "disabled",
            XC2MCFeedbackMode::COMB => "combinatorial",
            XC2MCFeedbackMode::REG => "registered",
        })?;

        Ok(())
    }
}

impl XC2Macrocell {
    /// Write the crbit representation of this macrocell to the given `fuse_array`.
    pub fn to_crbit(&self, device: XC2Device, fb: u32, mc: u32, fuse_array: &mut FuseArray) {
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                // The "32" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                BitFragment::<Crbit32>::encode(self, fuse_array, [x as isize, y as isize], [mirror, false], ());
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                // The "64" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                BitFragment::<Crbit64>::encode(self, fuse_array, [x as isize, y as isize], [mirror, false], ());
            },
            XC2Device::XC2C256 => {
                // The "256" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                BitFragment::<Crbit256>::encode(self, fuse_array, [x as isize, y as isize], [mirror, false], ());
            },
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                // The "common large macrocell" variant
                // we need this funny lookup table, but otherwise macrocells are 2x15
                let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
                BitFragment::<CrbitLarge>::encode(self, fuse_array, [x as isize, y as isize], [mirror, false], ());
            }
        }
    }

    /// Reads the crbit representation of this macrocell from the given `fuse_array`.
    pub fn from_crbit(device: XC2Device, fb: u32, mc: u32, fuse_array: &FuseArray) -> Self {
        let (x, y, mirror) = mc_block_loc(device, fb);
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                // The "32" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                <Self as BitFragment::<Crbit32>>::decode(fuse_array, [x as isize, y as isize], [mirror, false], ()).unwrap()
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                // The "64" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                <Self as BitFragment::<Crbit64>>::decode(fuse_array, [x as isize, y as isize], [mirror, false], ()).unwrap()
            },
            XC2Device::XC2C256 => {
                // The "256" variant
                // each macrocell is 3 rows high
                let y = y + (mc as usize) * 3;
                <Self as BitFragment::<Crbit256>>::decode(fuse_array, [x as isize, y as isize], [mirror, false], ()).unwrap()
            },
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                // The "common large macrocell" variant
                // we need this funny lookup table, but otherwise macrocells are 2x15
                let y = y + MC_TO_ROW_MAP_LARGE[mc as usize];
                <Self as BitFragment::<CrbitLarge>>::decode(fuse_array, [x as isize, y as isize], [mirror, false], ()).unwrap()
            }
        }
    }

    ///  Internal function that reads only the macrocell-related bits from the macrcocell configuration
    pub fn from_jed_small(fuses: &[bool], block_idx: usize, mc_idx: usize) -> Self {
        <Self as BitFragment::<JedSmall>>::decode(fuses, [(block_idx + mc_idx * 27) as isize], [false], ()).unwrap()
    }

    ///  Internal function that reads only the macrocell-related bits from the macrcocell configuration
    pub fn from_jed_large(fuses: &[bool], fuse_idx: usize) -> Self {
        <Self as BitFragment::<JedLargeUnburied>>::decode(fuses, [fuse_idx as isize], [false], ()).unwrap()
    }

    ///  Internal function that reads only the macrocell-related bits from the macrcocell configuration
    pub fn from_jed_large_buried(fuses: &[bool], fuse_idx: usize) -> Self {
        <Self as BitFragment::<JedLargeBuried>>::decode(fuses, [fuse_idx as isize], [false], ()).unwrap()
    }

    /// Helper that prints the macrocell configuration on the "small" parts
    pub fn to_jed_small(jed: &mut JEDECFile, linebreaks: &mut LinebreakSet,
        device: XC2Device, fb: &XC2BitstreamFB, fuse_base: usize) {

        let zia_row_width = zia_get_row_width(device);

        for i in 0..MCS_PER_FB {
            let mc_fuse_base = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
                ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB + i * 27;

            linebreaks.add(mc_fuse_base);
            if i == 0 {
                linebreaks.add(mc_fuse_base);
            }

            BitFragment::<JedSmall>::encode(&fb.mcs[i], &mut jed.f, [mc_fuse_base as isize], [false], ());
        }
    }

    /// Helper that prints the macrocell configuration on the "large" parts
    pub fn to_jed_large(jed: &mut JEDECFile, linebreaks: &mut LinebreakSet,
        device: XC2Device, fb: &XC2BitstreamFB, fb_i: usize, fuse_base: usize) {

        let zia_row_width = zia_get_row_width(device);

        let mut current_fuse_offset = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
            ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB;

        linebreaks.add(current_fuse_offset);

        for i in 0..MCS_PER_FB {
            linebreaks.add(current_fuse_offset);

            let iob = fb_mc_num_to_iob_num(device, fb_i as u32, i as u32);

            if iob.is_some() {
                BitFragment::<JedLargeUnburied>::encode(&fb.mcs[i], &mut jed.f, [current_fuse_offset as isize], [false], ());
                current_fuse_offset += 29;
            } else {
                BitFragment::<JedLargeBuried>::encode(&fb.mcs[i], &mut jed.f, [current_fuse_offset as isize], [false], ());
                current_fuse_offset += 16;
            }
        }
    }
}
