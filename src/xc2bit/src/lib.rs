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

//! # xc2bit: A library for working with Xilinx Coolrunner-II bitstreams
//!
//! xc2bit is a library for reading and writing bitstreams for the Xilinx Coolrunner-II family of CPLD devices.
//!
//! This project is the result of a reverse-engineering effort involving a combination of [imaging physical
//! CPLD devices](http://siliconexposed.blogspot.com/2014/03/getting-my-feet-wet-with-invasive.html) and black-box
//! reverse-engineering of generated .jed files. It is not an official project of Xilinx, Inc. and is not
//! affiliated or endorsed by Xilinx, Inc.
//!
//! Logically, a Coolrunner-II CPLD contains the following major blocks: function blocks (occasionally abbreviated
//! to FBs), a global interconnect (occasionally referred to as the ZIA or the AIM), and input/output blocks
//! (occasionally abbreviated to IOBs). Function blocks are further divided into the PLA (programmable logic array,
//! a matrix of AND and OR gates) and macrocells. In the Coolrunner-II architecture, macrocells also contain
//! an XOR gate and a register. The global interconnect accepts inputs from IOBs and function blocks and connects these
//! inputs into the PLA of each function block. IOBs also have direct connections to a corresponding macrocell in
//! a function block. (The reverse is not always true - on larger devices, there are macrocells that are not connected
//! to IOBs.) As a special exception, the smallest 32-macrocell devices also have one single input-only pin that is
//! connected directly into the global interconnect and does not have a corresponding macrocell.

use bittwiddler::*;
use serde_derive::{Deserialize, Serialize};

/// The number of inputs from the ZIA interconnect into the AND gate section of each PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const INPUTS_PER_ANDTERM: usize = 40;
/// The number of AND gates in each PLA. This is also the number of inputs into each OR gate in the PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const ANDTERMS_PER_FB: usize = 56;
/// The number of macrocells in each function block. This is also the number of OR gates in each PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const MCS_PER_FB: usize = 16;

/// The number of BUFG sites for clock signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_CLK: usize = 3;
/// The number of BUFG sites for tristate signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_GTS: usize = 4;
/// The number of BUFG sites for set/reset signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_GSR: usize = 1;

mod bitstream;
pub use crate::bitstream::{XC2Bitstream, XC2BitstreamBits,
    XC2BitsXC2C32, XC2BitsXC2C32A,
    XC2BitsXC2C64, XC2BitsXC2C64A,
    XC2BitsXC2C128,
    XC2BitsXC2C256,
    XC2BitsXC2C384,
    XC2BitsXC2C512};

mod crbit;
pub use crate::crbit::{FuseArray};

mod errors;
pub use crate::errors::{XC2BitError};

mod fb;
pub use crate::fb::{XC2BitstreamFB, CTC, CTR, CTS, CTE, get_pta, get_ptb, get_ptc};

mod fusemap_logical;
mod fusemap_physical;

mod globalbits;
pub use crate::globalbits::{XC2GlobalNets, XC2ClockDivRatio, XC2ClockDiv};

mod iob;
pub use crate::iob::{XC2MCSmallIOB, XC2IOBZIAMode, XC2IOBOBufMode, XC2ExtraIBuf, XC2IOBIbufMode, XC2MCLargeIOB,
                     iob_num_to_fb_mc_num, fb_mc_num_to_iob_num};

mod mc;
pub use crate::mc::{XC2Macrocell, XC2MCRegClkSrc, XC2MCRegResetSrc, XC2MCRegSetSrc, XC2MCRegMode, XC2MCFeedbackMode,
                    XC2MCXorMode};

mod partdb;
pub use crate::partdb::{XC2Device, XC2Speed, XC2Package, XC2DeviceSpeedPackage};

mod pla;
pub use crate::pla::{XC2PLAAndTerm, XC2PLAOrTerm};

mod structure;
pub use crate::structure::{get_gck, get_gts, get_gsr, get_cdrst, get_dge, get_device_structure};

mod zia;
pub use crate::zia::{XC2ZIAInput, zia_table_get_row, ZIA_MAP_32, ZIA_MAP_64, ZIA_MAP_128, ZIA_MAP_256,
                     ZIA_MAP_384, ZIA_MAP_512};

mod util;

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;

    use jedec::*;

    fn run_one_reftest(jed_filename: &'static str) {
        let jed_path = std::path::Path::new(jed_filename);
        let mut txt_path = jed_path.to_path_buf();
        txt_path.set_extension("txt");

        let mut jed_data = Vec::new();
        let mut txt_data = Vec::new();

        File::open(&jed_path).expect("failed to open jed file")
            .read_to_end(&mut jed_data).expect("failed to read jed file");
        File::open(&txt_path).expect("failed to open txt file")
            .read_to_end(&mut txt_data).expect("failed to read txt file");

        // Read original JED
        let jed = JEDECFile::from_bytes(&jed_data).expect("failed to read jed");
        let parsed_bitstream_data = XC2Bitstream::from_jed(&jed).expect("failed to process jed");

        // Write to crbit
        let mut crbit = Vec::new();
        let write_fuse_array = parsed_bitstream_data.to_crbit();
        write_fuse_array.write_to_writer(&mut crbit).expect("failed to write crbit");

        // Read back from crbit
        let read_fuse_array = FuseArray::from_file_contents(&crbit).expect("failed to read crbit");
        let parsed_bitstream_data = XC2Bitstream::from_crbit(&read_fuse_array).expect("failed to process crbit");

        // FIXME: This is quite hacky
        let mut new_jed = Vec::new();
        parsed_bitstream_data.to_jed(&mut new_jed).expect("failed to write jed");
        assert_eq!(jed_data, new_jed);

        let mut human_readable_data = Vec::new();
        parsed_bitstream_data.dump_human_readable(&mut human_readable_data)
            .expect("failed to get human readable");
        assert_eq!(txt_data, human_readable_data);
    }

    // Include list of actual tests to run
    include!(concat!(env!("OUT_DIR"), "/reftests.rs"));
}
