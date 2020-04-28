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

//! Contains functions pertaining to function blocks

use std::io;
use std::io::Write;

use jedec::*;

use crate::*;
use crate::fusemap_physical::{zia_block_loc, and_block_loc, or_block_loc, mc_block_loc};
use crate::util::{LinebreakSet};
use crate::zia::{zia_get_row_width};

pub enum JedXC2C32 {}
pub enum JedXC2C64 {}
pub enum JedXC2C128 {}
pub enum JedXC2C256 {}
pub enum JedXC2C384 {}
pub enum JedXC2C512 {}

pub enum CrbitXC2C32 {}
pub enum CrbitXC2C64 {}

fn large_get_macrocell_offset(device: XC2Device, fb_i: usize, mc_i: usize) -> usize {
    let mut current_fuse_offset = 0;

    for i in 0..mc_i {
        let iob = fb_mc_num_to_iob_num(device, fb_i as u32, i as u32);

        if iob.is_some() {
            current_fuse_offset += 29;
        } else {
            current_fuse_offset += 16;
        }
    }

    current_fuse_offset
}

#[bitfragment(variant = JedXC2C32, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = JedXC2C64, dimensions = 1, errtype = XC2BitError)]
#[bitfragment(variant = JedXC2C128, dimensions = 1, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]
#[bitfragment(variant = JedXC2C256, dimensions = 1, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]
#[bitfragment(variant = JedXC2C384, dimensions = 1, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]
#[bitfragment(variant = JedXC2C512, dimensions = 1, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]

#[bitfragment(variant = CrbitXC2C32, dimensions = 2, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]
#[bitfragment(variant = CrbitXC2C64, dimensions = 2, errtype = XC2BitError, encode_extra_type = usize, decode_extra_type = usize)]

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
/// Represents a collection of all the parts that make up one function block
pub struct XC2BitstreamFB {
    /// The AND terms of the PLA part of the function block
    #[offset(variant = JedXC2C32, [zia_get_row_width(XC2Device::XC2C32) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C32, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C32, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C64, [zia_get_row_width(XC2Device::XC2C64) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C64, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C64, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C128, [zia_get_row_width(XC2Device::XC2C128) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C128, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C128, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C256, [zia_get_row_width(XC2Device::XC2C256) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C256, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C256, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C384, [zia_get_row_width(XC2Device::XC2C384) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C384, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C384, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C512, [zia_get_row_width(XC2Device::XC2C512) * INPUTS_PER_ANDTERM ])]
    #[arr_off(variant = JedXC2C512, |i| [i * INPUTS_PER_ANDTERM * 2])]
    #[frag(outer_frag_variant = JedXC2C512, inner_frag_variant = pla::Jed)]

    #[offset(variant = CrbitXC2C32, {
        let (x, y, _mirror) = and_block_loc(XC2Device::XC2C32, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C32, {
        let (_x, _y, mirror) = and_block_loc(XC2Device::XC2C32, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C32, |i| {
        // FIXME WTF
        let (_x, _y, mirror) = and_block_loc(XC2Device::XC2C32, extra_data as u32);
        [(i as isize) * 2 * (if !mirror {1} else {-1}), 0]
    })]
    #[frag(outer_frag_variant = CrbitXC2C32, inner_frag_variant = pla::CrbitCentralOrBlock)]

    #[offset(variant = CrbitXC2C64, {
        let (x, y, _mirror) = and_block_loc(XC2Device::XC2C64, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C64, {
        let (_x, _y, mirror) = and_block_loc(XC2Device::XC2C64, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C64, |i| {
        // FIXME WTF
        let (_x, _y, mirror) = and_block_loc(XC2Device::XC2C64, extra_data as u32);
        [(i as isize) * 2 * (if !mirror {1} else {-1}), 0]
    })]
    #[frag(outer_frag_variant = CrbitXC2C64, inner_frag_variant = pla::CrbitCentralOrBlock)]

    and_terms: [[XC2PLAAndTerm; ANDTERMS_PER_FB / 2]; 2],


    /// The OR terms of the PLA part of the function block
    #[offset(variant = JedXC2C32, [zia_get_row_width(XC2Device::XC2C32) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C32, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C32, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C64, [zia_get_row_width(XC2Device::XC2C64) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C64, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C64, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C128, [zia_get_row_width(XC2Device::XC2C128) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C128, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C128, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C256, [zia_get_row_width(XC2Device::XC2C256) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C256, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C256, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C384, [zia_get_row_width(XC2Device::XC2C384) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C384, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C384, inner_frag_variant = pla::Jed)]

    #[offset(variant = JedXC2C512, [zia_get_row_width(XC2Device::XC2C512) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB])]
    #[arr_off(variant = JedXC2C512, |i| [i])]
    #[frag(outer_frag_variant = JedXC2C512, inner_frag_variant = pla::Jed)]

    #[offset(variant = CrbitXC2C32, {
        let (x, y, _mirror) = or_block_loc(XC2Device::XC2C32, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C32, {
        let (_x, _y, mirror) = or_block_loc(XC2Device::XC2C32, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C32, |i| {
        // FIXME WTF
        let (_x, _y, mirror) = or_block_loc(XC2Device::XC2C32, extra_data as u32);
        [((i % 2) as isize) * (if !mirror {1} else {-1}), (i as isize) / 2]
    })]
    #[frag(outer_frag_variant = CrbitXC2C32, inner_frag_variant = pla::CrbitCentralOrBlock)]

    #[offset(variant = CrbitXC2C64, {
        let (x, y, _mirror) = or_block_loc(XC2Device::XC2C64, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C64, {
        let (_x, _y, mirror) = or_block_loc(XC2Device::XC2C64, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C64, |i| {
        // FIXME WTF
        let (_x, _y, mirror) = or_block_loc(XC2Device::XC2C64, extra_data as u32);
        [((i % 2) as isize) * (if !mirror {1} else {-1}), (i as isize) / 2]
    })]
    #[frag(outer_frag_variant = CrbitXC2C64, inner_frag_variant = pla::CrbitCentralOrBlock)]

    pub or_terms: [XC2PLAOrTerm; MCS_PER_FB],


    /// The inputs to the function block from the ZIA
    #[arr_off(variant = JedXC2C32, |i| [i * zia_get_row_width(XC2Device::XC2C32)])]
    #[frag(outer_frag_variant = JedXC2C32, inner_frag_variant = zia::JedXC2C32)]
    #[encode_sub_extra_data(variant = JedXC2C32, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C32, arr_elem_i)]

    #[arr_off(variant = JedXC2C64, |i| [i * zia_get_row_width(XC2Device::XC2C64)])]
    #[frag(outer_frag_variant = JedXC2C64, inner_frag_variant = zia::JedXC2C64)]
    #[encode_sub_extra_data(variant = JedXC2C64, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C64, arr_elem_i)]

    #[arr_off(variant = JedXC2C128, |i| [i * zia_get_row_width(XC2Device::XC2C128)])]
    #[frag(outer_frag_variant = JedXC2C128, inner_frag_variant = zia::JedXC2C128)]
    #[encode_sub_extra_data(variant = JedXC2C128, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C128, arr_elem_i)]

    #[arr_off(variant = JedXC2C256, |i| [i * zia_get_row_width(XC2Device::XC2C256)])]
    #[frag(outer_frag_variant = JedXC2C256, inner_frag_variant = zia::JedXC2C256)]
    #[encode_sub_extra_data(variant = JedXC2C256, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C256, arr_elem_i)]

    #[arr_off(variant = JedXC2C384, |i| [i * zia_get_row_width(XC2Device::XC2C384)])]
    #[frag(outer_frag_variant = JedXC2C384, inner_frag_variant = zia::JedXC2C384)]
    #[encode_sub_extra_data(variant = JedXC2C384, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C384, arr_elem_i)]

    #[arr_off(variant = JedXC2C512, |i| [i * zia_get_row_width(XC2Device::XC2C512)])]
    #[frag(outer_frag_variant = JedXC2C512, inner_frag_variant = zia::JedXC2C512)]
    #[encode_sub_extra_data(variant = JedXC2C512, arr_elem_i)]
    #[decode_sub_extra_data(variant = JedXC2C512, arr_elem_i)]

    #[offset(variant = CrbitXC2C32, {
        let (x, y) = zia_block_loc(XC2Device::XC2C32, extra_data as u32);
        [x, y]
    })]
    #[arr_off(variant = CrbitXC2C32, |i| {
        if i >= 20 {
            // There is an OR array in the middle, 8 rows high
            [0, i + 8]
        } else {
            [0, i]
        }
    })]
    #[frag(outer_frag_variant = CrbitXC2C32, inner_frag_variant = zia::CrbitXC2C32)]
    #[encode_sub_extra_data(variant = CrbitXC2C32, arr_elem_i)]
    #[decode_sub_extra_data(variant = CrbitXC2C32, arr_elem_i)]

    #[offset(variant = CrbitXC2C64, {
        let (x, y) = zia_block_loc(XC2Device::XC2C64, extra_data as u32);
        [x, y]
    })]
    #[arr_off(variant = CrbitXC2C64, |i| {
        if i >= 20 {
            // There is an OR array in the middle, 8 rows high
            [0, i + 8]
        } else {
            [0, i]
        }
    })]
    #[frag(outer_frag_variant = CrbitXC2C64, inner_frag_variant = zia::CrbitXC2C64)]
    #[encode_sub_extra_data(variant = CrbitXC2C64, arr_elem_i)]
    #[decode_sub_extra_data(variant = CrbitXC2C64, arr_elem_i)]

    zia_bits: [[XC2ZIAInput; INPUTS_PER_ANDTERM / 2]; 2],


    /// The macrocells of the function block
    #[offset(variant = JedXC2C32, [zia_get_row_width(XC2Device::XC2C32) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C32, |i| [i * 27])]
    #[frag(outer_frag_variant = JedXC2C32, inner_frag_variant = mc::JedSmall)]

    #[offset(variant = JedXC2C64, [zia_get_row_width(XC2Device::XC2C64) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C64, |i| [i * 27])]
    #[frag(outer_frag_variant = JedXC2C64, inner_frag_variant = mc::JedSmall)]

    #[offset(variant = JedXC2C128, [zia_get_row_width(XC2Device::XC2C128) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C128, |i| [large_get_macrocell_offset(XC2Device::XC2C128, extra_data, i)])]
    #[frag(outer_frag_variant = JedXC2C128, inner_frag_variant = mc::JedLarge)]
    #[encode_sub_extra_data(variant = JedXC2C128, fb_mc_num_to_iob_num(XC2Device::XC2C128, extra_data as u32, arr_elem_i as u32).is_none())]
    #[decode_sub_extra_data(variant = JedXC2C128, fb_mc_num_to_iob_num(XC2Device::XC2C128, extra_data as u32, arr_elem_i as u32).is_none())]

    #[offset(variant = JedXC2C256, [zia_get_row_width(XC2Device::XC2C256) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C256, |i| [large_get_macrocell_offset(XC2Device::XC2C256, extra_data, i)])]
    #[frag(outer_frag_variant = JedXC2C256, inner_frag_variant = mc::JedLarge)]
    #[encode_sub_extra_data(variant = JedXC2C256, fb_mc_num_to_iob_num(XC2Device::XC2C256, extra_data as u32, arr_elem_i as u32).is_none())]
    #[decode_sub_extra_data(variant = JedXC2C256, fb_mc_num_to_iob_num(XC2Device::XC2C256, extra_data as u32, arr_elem_i as u32).is_none())]

    #[offset(variant = JedXC2C384, [zia_get_row_width(XC2Device::XC2C384) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C384, |i| [large_get_macrocell_offset(XC2Device::XC2C384, extra_data, i)])]
    #[frag(outer_frag_variant = JedXC2C384, inner_frag_variant = mc::JedLarge)]
    #[encode_sub_extra_data(variant = JedXC2C384, fb_mc_num_to_iob_num(XC2Device::XC2C384, extra_data as u32, arr_elem_i as u32).is_none())]
    #[decode_sub_extra_data(variant = JedXC2C384, fb_mc_num_to_iob_num(XC2Device::XC2C384, extra_data as u32, arr_elem_i as u32).is_none())]

    #[offset(variant = JedXC2C512, [zia_get_row_width(XC2Device::XC2C512) * INPUTS_PER_ANDTERM + INPUTS_PER_ANDTERM * 2 * ANDTERMS_PER_FB + ANDTERMS_PER_FB * MCS_PER_FB])]
    #[arr_off(variant = JedXC2C512, |i| [large_get_macrocell_offset(XC2Device::XC2C512, extra_data, i)])]
    #[frag(outer_frag_variant = JedXC2C512, inner_frag_variant = mc::JedLarge)]
    #[encode_sub_extra_data(variant = JedXC2C512, fb_mc_num_to_iob_num(XC2Device::XC2C512, extra_data as u32, arr_elem_i as u32).is_none())]
    #[decode_sub_extra_data(variant = JedXC2C512, fb_mc_num_to_iob_num(XC2Device::XC2C512, extra_data as u32, arr_elem_i as u32).is_none())]

    #[offset(variant = CrbitXC2C32, {
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C32, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C32, {
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C32, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C32, |i| [0, 3 * i])]
    #[frag(outer_frag_variant = CrbitXC2C32, inner_frag_variant = mc::Crbit32)]

    #[offset(variant = CrbitXC2C64, {
        let (x, y, _mirror) = mc_block_loc(XC2Device::XC2C64, extra_data as u32);
        [x, y]
    })]
    #[mirror(variant = CrbitXC2C64, {
        let (_x, _y, mirror) = mc_block_loc(XC2Device::XC2C64, extra_data as u32);
        [mirror, false]
    })]
    #[arr_off(variant = CrbitXC2C64, |i| [0, 3 * i])]
    #[frag(outer_frag_variant = CrbitXC2C64, inner_frag_variant = mc::Crbit64)]

    pub mcs: [XC2Macrocell; MCS_PER_FB],
}

impl XC2BitstreamFB {
    pub fn get_andterm(&self, i: usize) -> &XC2PLAAndTerm {
        &self.and_terms[i / (ANDTERMS_PER_FB / 2)][i % (ANDTERMS_PER_FB / 2)]
    }

    pub fn get_mut_andterm(&mut self, i: usize) -> &mut XC2PLAAndTerm {
        &mut self.and_terms[i / (ANDTERMS_PER_FB / 2)][i % (ANDTERMS_PER_FB / 2)]
    }

    pub fn get_zia(&self, i: usize) -> &XC2ZIAInput {
        &self.zia_bits[i / (INPUTS_PER_ANDTERM / 2)][i % (INPUTS_PER_ANDTERM / 2)]
    }

    pub fn get_mut_zia(&mut self, i: usize) -> &mut XC2ZIAInput {
        &mut self.zia_bits[i / (INPUTS_PER_ANDTERM / 2)][i % (INPUTS_PER_ANDTERM / 2)]
    }
}

impl Default for XC2BitstreamFB {
    fn default() -> Self {
        XC2BitstreamFB {
            and_terms: [[XC2PLAAndTerm::default(); ANDTERMS_PER_FB / 2]; 2],
            or_terms: [XC2PLAOrTerm::default(); MCS_PER_FB],
            zia_bits: [[XC2ZIAInput::default(); INPUTS_PER_ANDTERM / 2]; 2],
            mcs: [XC2Macrocell::default(); MCS_PER_FB],
        }
    }
}

/// Internal helper that writes a ZIA row to the fuse array
fn zia_row_crbit_write_helper(x: usize, y: usize, zia_row: usize, zia_bits: &[bool], has_gap: bool,
    fuse_array: &mut FuseArray) {

    for zia_bit in 0..zia_bits.len() {
        let mut out_y = y + zia_row;
        if has_gap && zia_row >= 20 {
            // There is an OR array in the middle, 8 rows high
            out_y += 8;
        }

        let out_x = x + zia_bit * 2;

        fuse_array.set(out_x, out_y, zia_bits[zia_bits.len() - 1 - zia_bit]);
    }
}

/// Internal helper that reads a ZIA row from the fuse array
fn zia_row_crbit_read_helper(x: usize, y: usize, zia_row: usize, zia_bits: &mut [bool], has_gap: bool,
    fuse_array: &FuseArray) {

    let l = zia_bits.len();

    for zia_bit in 0..l {
        let mut out_y = y + zia_row;
        if has_gap && zia_row >= 20 {
            // There is an OR array in the middle, 8 rows high
            out_y += 8;
        }

        let out_x = x + zia_bit * 2;

        zia_bits[l - 1 - zia_bit] = fuse_array.get(out_x, out_y);
    }
}

// Weird mapping here in (mostly) groups of 3
// TODO: Explain better
static AND_BLOCK_TYPE2_P2L_MAP: [usize; ANDTERMS_PER_FB] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    55, 54, 53,
    11, 12, 13,
    52, 51, 50,
    14, 15, 16,
    49, 48, 47,
    17, 18, 19,
    46, 45, 44,
    20, 21, 22,
    43, 42, 41,
    23, 24, 25,
    40, 39, 38,
    26, 27, 28,
    37, 36, 35,
    29, 30, 31,
    34, 33, 32];

static AND_BLOCK_TYPE2_L2P_MAP: [usize; ANDTERMS_PER_FB] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    14, 15, 16,
    20, 21, 22,
    26, 27, 28,
    32, 33, 34,
    38, 39, 40,
    44, 45, 46,
    50, 51, 52,
    55, 54, 53,
    49, 48, 47,
    43, 42, 41,
    37, 36, 35,
    31, 30, 29,
    25, 24, 23,
    19, 18, 17,
    13, 12, 11];

impl XC2BitstreamFB {
    /// Dump a human-readable explanation of the settings for this FB to the given `writer` object.
    /// `device` must be the device type this FB was extracted from and is needed to decode I/O pin numbers.
    /// `fb` must be the index of this function block.
    pub fn dump_human_readable<W: Write>(&self, device: XC2Device, fb: u32, mut writer: W) -> Result<(), io::Error> {
        for i in 0..MCS_PER_FB {
            write!(writer, "\n")?;
            write!(writer, "FF configuration for FB{}_{}\n", fb + 1, i + 1)?;
            write!(writer, "{}", self.mcs[i])?;
        }

        write!(writer, "\n")?;
        write!(writer, "ZIA inputs for FB{}\n", fb + 1)?;
        for i in 0..INPUTS_PER_ANDTERM {
            write!(writer, "{:2}: ", i)?;
            match *self.get_zia(i) {
                XC2ZIAInput::Zero => write!(writer, "0\n")?,
                XC2ZIAInput::One => write!(writer, "1\n")?,
                XC2ZIAInput::Macrocell{fb, mc} =>
                    write!(writer, "FB{}_{} FF\n", fb + 1, mc + 1)?,
                XC2ZIAInput::IBuf{ibuf} => {
                    let (fb, mc) = iob_num_to_fb_mc_num(device, ibuf as u32).unwrap();
                    write!(writer, "FB{}_{} pad\n", fb + 1, mc + 1)?;
                },
                XC2ZIAInput::DedicatedInput => write!(writer, "dedicated input\n")?,
            }
        }

        write!(writer, "\n")?;
        write!(writer, "AND terms for FB{}\n", fb + 1)?;
        write!(writer, "   |  0| ~0|  1| ~1|  2| ~2|  3| ~3|  4| ~4|  5| ~5|  6| ~6|  7| ~7|  8| ~8|  9| ~9| 10|~10| \
                                     11|~11| 12|~12| 13|~13| 14|~14| 15|~15| 16|~16| 17|~17| 18|~18| 19|~19| 20|~20| \
                                     21|~21| 22|~22| 23|~23| 24|~24| 25|~25| 26|~26| 27|~27| 28|~28| 29|~29| 30|~30| \
                                     31|~31| 32|~32| 33|~33| 34|~34| 35|~35| 36|~36| 37|~37| 38|~38| 39|~39\
                                     \n")?;
        for i in 0..ANDTERMS_PER_FB {
            write!(writer, "{:2}:", i)?;
            for j in 0..INPUTS_PER_ANDTERM {
                if self.get_andterm(i).get(j) {
                    write!(writer, "|XXX")?;
                } else {
                    write!(writer, "|   ")?;
                }

                if self.get_andterm(i).get_b(j) {
                    write!(writer, "|XXX")?;
                } else {
                    write!(writer, "|   ")?;
                }
            }
            write!(writer, "\n")?;
        }

        write!(writer, "\n")?;
        write!(writer, "OR terms for FB{}\n", fb + 1)?;
        write!(writer, "   | 0| 1| 2| 3| 4| 5| 6| 7| 8| 9|10|11|12|13|14|15|16|17|18|19|20|\
                               21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|36|37|38|39|40|\
                               41|42|43|44|45|46|47|48|49|50|51|52|53|54|55\n")?;
        for i in 0..MCS_PER_FB {
            write!(writer, "{:2}:", i)?;
            for j in 0..ANDTERMS_PER_FB {
                if self.or_terms[i].get(j) {
                    write!(writer, "|XX")?;
                } else {
                    write!(writer, "|  ")?;
                }
            }
            write!(writer, "\n")?;
        }

        Ok(())
    }

    /// Write the crbit representation of the settings for this FB to the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `fb` must be the index of this function block.
    pub fn to_crbit(&self, device: XC2Device, fb: u32, fuse_array: &mut FuseArray) {
        if device == XC2Device::XC2C32 || device == XC2Device::XC2C32A {
            <Self as BitFragment<CrbitXC2C32>>::encode(
                &self,
                fuse_array,
                [0, 0],
                [false, false],
                fb as usize);

            return;
        }
        if device == XC2Device::XC2C64 || device == XC2Device::XC2C64A {
            <Self as BitFragment<CrbitXC2C64>>::encode(
                &self,
                fuse_array,
                [0, 0],
                [false, false],
                fb as usize);

            return;
        }

        // FFs
        for i in 0..MCS_PER_FB {
            self.mcs[i].to_crbit(device, fb, i as u32, fuse_array);
        }

        // ZIA
        let (x, y) = zia_block_loc(device, fb);
        for zia_row in 0..INPUTS_PER_ANDTERM {
            match device {
                XC2Device::XC2C32 | XC2Device::XC2C32A => {
                    let zia_choice_bits = XC2ZIAInput::encode_32_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, true, fuse_array);
                },
                XC2Device::XC2C64 | XC2Device::XC2C64A => {
                    let zia_choice_bits = XC2ZIAInput::encode_64_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, true, fuse_array);
                },
                XC2Device::XC2C128 => {
                    let zia_choice_bits = XC2ZIAInput::encode_128_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, false, fuse_array);
                },
                XC2Device::XC2C256 => {
                    let zia_choice_bits = XC2ZIAInput::encode_256_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, true, fuse_array);
                },
                XC2Device::XC2C384 => {
                    let zia_choice_bits = XC2ZIAInput::encode_384_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, false, fuse_array);
                },
                XC2Device::XC2C512 => {
                    let zia_choice_bits = XC2ZIAInput::encode_512_zia_choice(zia_row as u32, *self.get_zia(zia_row))
                        // FIXME: Fold this into the error system??
                        .expect("invalid ZIA input");

                    zia_row_crbit_write_helper(x, y, zia_row, &zia_choice_bits, false, fuse_array);
                },
            };
        }

        // AND block
        let (x, y, mirror) = and_block_loc(device, fb);
        match device {
            // "Type 1" blocks (OR array is in the middle)
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A | XC2Device::XC2C256 => {
                for term_idx in 0..ANDTERMS_PER_FB {
                    let out_x = (x as isize) + ((term_idx as isize * 2) * if !mirror {1} else {-1});

                    <XC2PLAAndTerm as BitFragment<pla::CrbitCentralOrBlock>>::encode(
                        self.get_andterm(term_idx),
                        fuse_array,
                        [out_x, y as isize],
                        [mirror, false],
                        ());
                }
            },
            // "Type 2" blocks (OR array is on the sides)
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                for logi_term_idx in 0..ANDTERMS_PER_FB {
                    let term_idx = AND_BLOCK_TYPE2_L2P_MAP[logi_term_idx];
                    let out_x = (x as isize) + ((term_idx as isize * 2) * if !mirror {1} else {-1});

                    <XC2PLAAndTerm as BitFragment<pla::CrbitSideOrBlock>>::encode(
                        self.get_andterm(logi_term_idx),
                        fuse_array,
                        [out_x, y as isize],
                        [mirror, false],
                        ());
                }
            },
        }

        // OR block
        let (x, y, mirror) = or_block_loc(device, fb);
        match device {
            // "Type 1" blocks (OR array is in the middle)
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A | XC2Device::XC2C256 => {
                for or_term_idx in 0..MCS_PER_FB {
                    let out_y = y + (or_term_idx / 2);
                    let off_x = if !mirror {
                        x + or_term_idx % 2
                    } else {
                        x - or_term_idx % 2
                    };

                    <XC2PLAOrTerm as BitFragment<pla::CrbitCentralOrBlock>>::encode(
                        &self.or_terms[or_term_idx],
                        fuse_array,
                        [off_x as isize, out_y as isize],
                        [mirror, false],
                        ());
                }
            },
            // "Type 2" blocks (OR array is on the sides)
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                for or_term_idx in 0..MCS_PER_FB {
                    let off_x = if !mirror {
                        x + or_term_idx * 2
                    } else {
                        x - or_term_idx * 2
                    };

                    <XC2PLAOrTerm as BitFragment<pla::CrbitSideOrBlock>>::encode(
                        &self.or_terms[or_term_idx],
                        fuse_array,
                        [off_x as isize, y as isize],
                        [mirror, false],
                        ());
                }
            },
        }
    }

    /// Reads the crbit representation of the settings for this FB from the given `fuse_array`.
    /// `device` must be the device type this FB was extracted from.
    /// `fb` must be the index of this function block.
    pub fn from_crbit(device: XC2Device, fb: u32, fuse_array: &FuseArray) -> Result<Self, XC2BitError> {
        let mut ret = Self::default();

        // ZIA
        let (x, y) = zia_block_loc(device, fb);
        for zia_row in 0..INPUTS_PER_ANDTERM {
            *ret.get_mut_zia(zia_row) = match device {
                XC2Device::XC2C32 | XC2Device::XC2C32A => {
                    let mut zia_bits = [false; 8];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, true, fuse_array);
                    XC2ZIAInput::decode_32_zia_choice(zia_row, &zia_bits)?
                },
                XC2Device::XC2C64 | XC2Device::XC2C64A => {
                    let mut zia_bits = [false; 16];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, true, fuse_array);
                    XC2ZIAInput::decode_64_zia_choice(zia_row, &zia_bits)?
                },
                XC2Device::XC2C128 => {
                    let mut zia_bits = [false; 28];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, false, fuse_array);
                    XC2ZIAInput::decode_128_zia_choice(zia_row, &zia_bits)?
                },
                XC2Device::XC2C256 => {
                    let mut zia_bits = [false; 48];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, true, fuse_array);
                    XC2ZIAInput::decode_256_zia_choice(zia_row, &zia_bits)?
                },
                XC2Device::XC2C384 => {
                    let mut zia_bits = [false; 74];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, false, fuse_array);
                    XC2ZIAInput::decode_384_zia_choice(zia_row, &zia_bits)?
                },
                XC2Device::XC2C512 => {
                    let mut zia_bits = [false; 88];
                    zia_row_crbit_read_helper(x, y, zia_row, &mut zia_bits, false, fuse_array);
                    XC2ZIAInput::decode_512_zia_choice(zia_row, &zia_bits)?
                },
            };
        }

        // AND block
        let (x, y, mirror) = and_block_loc(device, fb);
        match device {
            // "Type 1" blocks (OR array is in the middle)
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A | XC2Device::XC2C256 => {
                for term_idx in 0..ANDTERMS_PER_FB {
                    let out_x = (x as isize) + ((term_idx as isize * 2) * if !mirror {1} else {-1});

                    let andterm = <XC2PLAAndTerm as BitFragment<pla::CrbitCentralOrBlock>>::decode(
                        fuse_array,
                        [out_x, y as isize],
                        [mirror, false],
                        ()).unwrap();

                    *ret.get_mut_andterm(term_idx) = andterm;
                }
            },
            // "Type 2" blocks (OR array is on the sides)
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                for term_idx in 0..ANDTERMS_PER_FB {
                    let phys_term_idx = AND_BLOCK_TYPE2_P2L_MAP[term_idx];
                    let out_x = (x as isize) + ((term_idx as isize * 2) * if !mirror {1} else {-1});

                    let andterm = <XC2PLAAndTerm as BitFragment<pla::CrbitSideOrBlock>>::decode(
                        fuse_array,
                        [out_x, y as isize],
                        [mirror, false],
                        ()).unwrap();

                    *ret.get_mut_andterm(phys_term_idx) = andterm;
                }
            },
        }

        // OR block
        let (x, y, mirror) = or_block_loc(device, fb);
        match device {
            // "Type 1" blocks (OR array is in the middle)
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A | XC2Device::XC2C256 => {
                for or_term_idx in 0..MCS_PER_FB {
                    let out_y = y + (or_term_idx / 2);
                    let off_x = if !mirror {
                        x + or_term_idx % 2
                    } else {
                        x - or_term_idx % 2
                    };

                    let orterm = <XC2PLAOrTerm as BitFragment<pla::CrbitCentralOrBlock>>::decode(
                        fuse_array,
                        [off_x as isize, out_y as isize],
                        [mirror, false],
                        ()).unwrap();

                    ret.or_terms[or_term_idx] = orterm;
                }
            },
            // "Type 2" blocks (OR array is on the sides)
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                for or_term_idx in 0..MCS_PER_FB {
                    let off_x = if !mirror {
                        x + or_term_idx * 2
                    } else {
                        x - or_term_idx * 2
                    };

                    let orterm = <XC2PLAOrTerm as BitFragment<pla::CrbitSideOrBlock>>::decode(
                        fuse_array,
                        [off_x as isize, y as isize],
                        [mirror, false],
                        ()).unwrap();

                    ret.or_terms[or_term_idx] = orterm;
                }
            },
        }

        // FFs
        for i in 0..MCS_PER_FB {
            ret.mcs[i] = XC2Macrocell::from_crbit(device, fb, i as u32, fuse_array);
        }

        Ok(ret)
    }

    /// Write the .JED representation of the settings for this FB to the given `jed` object.
    /// `device` must be the device type this FB was extracted from and is needed to encode the ZIA.
    /// `fuse_base` must be the starting fuse number of this function block.
    pub fn to_jed(&self, device: XC2Device, fuse_base: usize, jed: &mut JEDECFile, linebreaks: &mut LinebreakSet, fb_i: usize) {
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                <Self as BitFragment<JedXC2C32>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], ());
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                <Self as BitFragment<JedXC2C64>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], ());
            },
            XC2Device::XC2C128 => {
                <Self as BitFragment<JedXC2C128>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], fb_i);
            },
            XC2Device::XC2C256 => {
                <Self as BitFragment<JedXC2C256>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], fb_i);
            },
            XC2Device::XC2C384 => {
                <Self as BitFragment<JedXC2C384>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], fb_i);
            },
            XC2Device::XC2C512 => {
                <Self as BitFragment<JedXC2C512>>::encode(&self, &mut jed.f, [fuse_base as isize], [false], fb_i);
            },
        }

        // Linebreaks

        // ZIA
        let zia_row_width = zia_get_row_width(device);

        if fuse_base != 0 {
            linebreaks.add(fuse_base);
        }
        for i in 0..INPUTS_PER_ANDTERM {
            let zia_fuse_base = fuse_base + i * zia_row_width;
            if zia_fuse_base != 0 {
                linebreaks.add(zia_fuse_base);
            }
        }

        // AND terms
        linebreaks.add(fuse_base + zia_row_width * INPUTS_PER_ANDTERM);
        for i in 0..ANDTERMS_PER_FB {
            let and_fuse_base = fuse_base + zia_row_width * INPUTS_PER_ANDTERM + i * INPUTS_PER_ANDTERM * 2;
            linebreaks.add(and_fuse_base);
        }

        // OR terms
        linebreaks.add(fuse_base + zia_row_width * INPUTS_PER_ANDTERM + ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2);
        for i in 0..ANDTERMS_PER_FB {
            let or_fuse_base = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
                ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + i * MCS_PER_FB;
            linebreaks.add(or_fuse_base);
        }

        // macrocell line breaks
        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A |
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                for i in 0..MCS_PER_FB {
                    let mc_fuse_base = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
                        ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB + i * 27;

                    linebreaks.add(mc_fuse_base);
                    if i == 0 {
                        linebreaks.add(mc_fuse_base);
                    }
                }
            },
            XC2Device::XC2C128 | XC2Device::XC2C256 |
            XC2Device::XC2C384 | XC2Device::XC2C512 => {
                let mut current_fuse_offset = fuse_base + zia_row_width * INPUTS_PER_ANDTERM +
                    ANDTERMS_PER_FB * INPUTS_PER_ANDTERM * 2 + ANDTERMS_PER_FB * MCS_PER_FB;

                linebreaks.add(current_fuse_offset);

                for i in 0..MCS_PER_FB {
                    linebreaks.add(current_fuse_offset);

                    let iob = fb_mc_num_to_iob_num(device, fb_i as u32, i as u32);

                    if iob.is_some() {
                        current_fuse_offset += 29;
                    } else {
                        current_fuse_offset += 16;
                    }
                }
            },
        }
    }

    /// Internal function that reads a function block
    pub fn from_jed(device: XC2Device, fuses: &[bool], fb: u32, fuse_base: usize)
        -> Result<XC2BitstreamFB, XC2BitError> {

        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                <Self as BitFragment<JedXC2C32>>::decode(fuses, [fuse_base as isize], [false], ())
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                <Self as BitFragment<JedXC2C64>>::decode(fuses, [fuse_base as isize], [false], ())
            },
            XC2Device::XC2C128 => {
                <Self as BitFragment<JedXC2C128>>::decode(fuses, [fuse_base as isize], [false], fb as usize)
            },
            XC2Device::XC2C256 => {
                <Self as BitFragment<JedXC2C256>>::decode(fuses, [fuse_base as isize], [false], fb as usize)
            },
            XC2Device::XC2C384 => {
                <Self as BitFragment<JedXC2C384>>::decode(fuses, [fuse_base as isize], [false], fb as usize)
            },
            XC2Device::XC2C512 => {
                <Self as BitFragment<JedXC2C512>>::decode(fuses, [fuse_base as isize], [false], fb as usize)
            },
        }
    }
}

// TODO: This is the same across all sizes, right?

/// The index of the special CTC product term
pub const CTC: u32 = 4;

/// The index of the special CTR product term
pub const CTR: u32 = 5;

/// The index of the special CTS product term
pub const CTS: u32 = 6;

/// The index of the special CTE product term
pub const CTE: u32 = 7;

/// Returns the special PTA product term given a macrocell index
pub const fn get_pta(mc: u32) -> u32 {
    3 * mc + 8
}

/// Returns the special PTB product term given a macrocell index
pub const fn get_ptb(mc: u32) -> u32 {
    3 * mc + 9
}

/// Returns the special PTC product term given a macrocell index
pub const fn get_ptc(mc: u32) -> u32 {
    3 * mc + 10
}
