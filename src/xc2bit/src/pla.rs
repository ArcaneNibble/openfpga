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

//! Contains functions pertaining to the PLA

use crate::*;

/// Represents one single AND term in the PLA. Each AND term can perform an AND function on any subset of its inputs
/// and the complement of those inputs. The index for each input is the corresponding ZIA row.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2PLAAndTerm {
    /// Indicates whether a particular ZIA row output is a part of this AND term.
    ///
    /// `true` = part of and, `false` = not part of and
    input: [u8; INPUTS_PER_ANDTERM / 8],
    /// Indicates whether the complement of a particular ZIA row output is a part of this AND term.
    ///
    /// `true` = part of and, `false` = not part of and
    input_b: [u8; INPUTS_PER_ANDTERM / 8],
}

impl Default for XC2PLAAndTerm {
    /// Returns a "default" AND term. The default state is for none of the inputs to be selected.
    fn default() -> Self {
        XC2PLAAndTerm {
            input: [0u8; INPUTS_PER_ANDTERM / 8],
            input_b: [0u8; INPUTS_PER_ANDTERM / 8],
        }
    }
}

pub enum Jed {}
pub enum CrbitCentralOrBlock {}
pub enum CrbitSideOrBlock {}

impl BitFragment<Jed> for XC2PLAAndTerm {
    const IDX_DIMS: usize = 1;
    type IndexingType = usize;
    type OffsettingType = [isize; 1];
    type MirroringType = [bool; 1];

    type ErrType = ();

    type EncodeExtraType = ();
    type DecodeExtraType = ();

    const FIELD_COUNT: usize = 2;

    fn encode<F>(&self, fuses: &mut F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ())
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        for i in 0..INPUTS_PER_ANDTERM {
            fuses[((offset[0] as isize) +
                (0 + 2 * i as isize) * (if mirror[0] {-1} else {1})) as usize] =

                !self.get(i);

            fuses[((offset[0] as isize) +
                (1 + 2 * i as isize) * (if mirror[0] {-1} else {1})) as usize] =

                !self.get_b(i);
        }
    }
    fn decode<F>(fuses: &F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ()) -> Result<Self, ()>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        let mut ret = Self::default();

        for i in 0..INPUTS_PER_ANDTERM {
            ret.set(i, !fuses[((offset[0] as isize) +
                (0 + 2 * i as isize) * (if mirror[0] {-1} else {1})) as usize]);
            ret.set_b(i, !fuses[((offset[0] as isize) +
                (1 + 2 * i as isize) * (if mirror[0] {-1} else {1})) as usize]);
        }

        Ok(ret)
    }

    fn fieldname(field_i: usize) -> &'static str {
        ["input", "input_b"][field_i]
    }
    fn fielddesc(field_i: usize) -> &'static str {
        ["true inputs", "complement inputs"][field_i]
    }
    fn fieldtype(_: usize) -> BitFragmentFieldType {
        BitFragmentFieldType::PatternArray(INPUTS_PER_ANDTERM)
    }
    fn field_offset(_: usize, _: usize) -> Self::OffsettingType {[0]}
    fn field_mirror(_: usize, _: usize) -> Self::MirroringType {[false]}
    fn field_bits(_: usize) -> usize {0}
    fn field_bit_base_pos(_: usize, _bit_i: usize) -> Self::OffsettingType {[0]}
}

// this one has a gap in the middle
impl BitFragment<CrbitCentralOrBlock> for XC2PLAAndTerm {
    const IDX_DIMS: usize = 2;
    type IndexingType = [usize; 2];
    type OffsettingType = [isize; 2];
    type MirroringType = [bool; 2];

    type ErrType = ();

    type EncodeExtraType = ();
    type DecodeExtraType = ();

    const FIELD_COUNT: usize = 2;

    fn encode<F>(&self, fuses: &mut F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ())
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        for input_idx in 0..INPUTS_PER_ANDTERM {
            let mut out_y_off = input_idx as isize;
            if input_idx >= 20 {
                // There is an OR array in the middle, 8 rows high
                out_y_off += 8;
            }

            let out_x = offset[0] + if !mirror[0] {1} else {-1};
            let out_x_b = offset[0];

            let out_y = offset[1] + out_y_off * (if !mirror[1] {1} else {-1});

            // true input
            fuses[[out_x as usize, out_y as usize]] = !self.get(input_idx);
            // complement input
            fuses[[out_x_b as usize, out_y as usize]] = !self.get_b(input_idx);
        }
    }
    fn decode<F>(fuses: &F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ()) -> Result<Self, ()>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        let mut ret = Self::default();

        for input_idx in 0..INPUTS_PER_ANDTERM {
            let mut out_y_off = input_idx as isize;
            if input_idx >= 20 {
                // There is an OR array in the middle, 8 rows high
                out_y_off += 8;
            }

            let out_x = offset[0] + if !mirror[0] {1} else {-1};
            let out_x_b = offset[0];

            let out_y = offset[1] + out_y_off * (if !mirror[1] {1} else {-1});

            ret.set(input_idx, !fuses[[out_x as usize, out_y as usize]]);
            ret.set_b(input_idx, !fuses[[out_x_b as usize, out_y as usize]]);
        }

        Ok(ret)
    }

    fn fieldname(field_i: usize) -> &'static str {
        ["input", "input_b"][field_i]
    }
    fn fielddesc(field_i: usize) -> &'static str {
        ["true inputs", "complement inputs"][field_i]
    }
    fn fieldtype(_: usize) -> BitFragmentFieldType {
        BitFragmentFieldType::PatternArray(INPUTS_PER_ANDTERM)
    }
    fn field_offset(_: usize, _: usize) -> Self::OffsettingType {[0, 0]}
    fn field_mirror(_: usize, _: usize) -> Self::MirroringType {[false, false]}
    fn field_bits(_: usize) -> usize {0}
    fn field_bit_base_pos(_: usize, _bit_i: usize) -> Self::OffsettingType {[0, 0]}
}

// this one does not have a gap in the middle
impl BitFragment<CrbitSideOrBlock> for XC2PLAAndTerm {
    const IDX_DIMS: usize = 2;
    type IndexingType = [usize; 2];
    type OffsettingType = [isize; 2];
    type MirroringType = [bool; 2];

    type ErrType = ();

    type EncodeExtraType = ();
    type DecodeExtraType = ();

    const FIELD_COUNT: usize = 2;

    fn encode<F>(&self, fuses: &mut F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ())
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        for input_idx in 0..INPUTS_PER_ANDTERM {
            let out_y_off = input_idx as isize;

            let out_x = offset[0] + if !mirror[0] {1} else {-1};
            let out_x_b = offset[0];

            let out_y = offset[1] + out_y_off * (if !mirror[1] {1} else {-1});

            // true input
            fuses[[out_x as usize, out_y as usize]] = !self.get(input_idx);
            // complement input
            fuses[[out_x_b as usize, out_y as usize]] = !self.get_b(input_idx);
        }
    }
    fn decode<F>(fuses: &F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ()) -> Result<Self, ()>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        let mut ret = Self::default();

        for input_idx in 0..INPUTS_PER_ANDTERM {
            let out_y_off = input_idx as isize;

            let out_x = offset[0] + if !mirror[0] {1} else {-1};
            let out_x_b = offset[0];

            let out_y = offset[1] + out_y_off * (if !mirror[1] {1} else {-1});

            ret.set(input_idx, !fuses[[out_x as usize, out_y as usize]]);
            ret.set_b(input_idx, !fuses[[out_x_b as usize, out_y as usize]]);
        }

        Ok(ret)
    }

    fn fieldname(field_i: usize) -> &'static str {
        ["input", "input_b"][field_i]
    }
    fn fielddesc(field_i: usize) -> &'static str {
        ["true inputs", "complement inputs"][field_i]
    }
    fn fieldtype(_: usize) -> BitFragmentFieldType {
        BitFragmentFieldType::PatternArray(INPUTS_PER_ANDTERM)
    }
    fn field_offset(_: usize, _: usize) -> Self::OffsettingType {[0, 0]}
    fn field_mirror(_: usize, _: usize) -> Self::MirroringType {[false, false]}
    fn field_bits(_: usize) -> usize {0}
    fn field_bit_base_pos(_: usize, _bit_i: usize) -> Self::OffsettingType {[0, 0]}
}

impl XC2PLAAndTerm {
    /// Returns `true` if the `i`th input is used in this AND term
    pub fn get(&self, i: usize) -> bool {
        self.input[i / 8] & (1 << (i % 8)) != 0
    }

    /// Returns `true` if the `i`th input complement is used in this AND term
    pub fn get_b(&self, i: usize) -> bool {
        self.input_b[i / 8] & (1 << (i % 8)) != 0
    }

    /// Sets whether the `i`th input is used in this AND term
    pub fn set(&mut self, i: usize, val: bool) {
        if !val {
            self.input[i / 8] &=  !(1 << (i % 8));
        } else {
            self.input[i / 8] |=  1 << (i % 8);
        }
    }

    /// Sets whether the `i`th input complement is used in this AND term
    pub fn set_b(&mut self, i: usize, val: bool) {
        if !val {
            self.input_b[i / 8] &=  !(1 << (i % 8));
        } else {
            self.input_b[i / 8] |=  1 << (i % 8);
        }
    }
}

/// Represents one single OR term in the PLA. Each OR term can perform an OR function on any subset of its inputs.
/// The index for each input is the index of the corresponding AND term in the same PLA.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct XC2PLAOrTerm {
    /// Indicates whether a particular PLA AND term is a part of this OR term.
    ///
    /// `true` = part of or, `false` = not part of or
    input: [u8; ANDTERMS_PER_FB / 8],
}

impl Default for XC2PLAOrTerm {
    /// Returns a "default" OR term. The default state is for none of the inputs to be selected.
    fn default() -> Self {
        XC2PLAOrTerm {
            input: [0u8; ANDTERMS_PER_FB / 8],
        }
    }
}

impl BitFragment<Jed> for XC2PLAOrTerm {
    const IDX_DIMS: usize = 1;
    type IndexingType = usize;
    type OffsettingType = [isize; 1];
    type MirroringType = [bool; 1];

    type ErrType = ();

    type EncodeExtraType = ();
    type DecodeExtraType = ();

    const FIELD_COUNT: usize = 2;

    fn encode<F>(&self, fuses: &mut F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ())
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        for i in 0..ANDTERMS_PER_FB {
            fuses[((offset[0] as isize) +
                ((MCS_PER_FB * i) as isize) * (if mirror[0] {-1} else {1})) as usize] =

                ((self.input[i / 8]) & (1 << (i % 8))) == 0;
        }
    }
    fn decode<F>(fuses: &F,
        offset: Self::OffsettingType, mirror: Self::MirroringType, _: ()) -> Result<Self, ()>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        let mut input = [0u8; ANDTERMS_PER_FB / 8];

        for i in 0..ANDTERMS_PER_FB {
            if !fuses[((offset[0] as isize) +
                ((MCS_PER_FB * i) as isize) * (if mirror[0] {-1} else {1})) as usize] {

                input[i / 8] |= 1 << (i % 8);
            }
        }

        Ok(XC2PLAOrTerm {
            input,
        })
    }

    fn fieldname(_: usize) -> &'static str {"input"}
    fn fielddesc(_: usize) -> &'static str {"inputs"}
    fn fieldtype(_: usize) -> BitFragmentFieldType {
        BitFragmentFieldType::PatternArray(ANDTERMS_PER_FB)
    }
    fn field_offset(_: usize, _: usize) -> Self::OffsettingType {[0]}
    fn field_mirror(_: usize, _: usize) -> Self::MirroringType {[false]}
    fn field_bits(_: usize) -> usize {0}
    fn field_bit_base_pos(_: usize, _bit_i: usize) -> Self::OffsettingType {[0]}
}

impl XC2PLAOrTerm {
    /// Internal function that reads one single OR term from a block of fuses using logical fuse indexing
    pub fn from_jed(fuses: &[bool], block_idx: usize, term_idx: usize) -> XC2PLAOrTerm {
        <Self as BitFragment<Jed>>::decode(fuses, [(block_idx + term_idx) as isize], [false], ()).unwrap()
    }

    /// Returns `true` if the `i`th AND term is used in this OR term
    pub fn get(&self, i: usize) -> bool {
        self.input[i / 8] & (1 << (i % 8)) != 0
    }

    /// Sets whether the `i`th AND term is used in this OR term
    pub fn set(&mut self, i: usize, val: bool) {
        if !val {
            self.input[i / 8] &=  !(1 << (i % 8));
        } else {
            self.input[i / 8] |=  1 << (i % 8);
        }
    }
}
