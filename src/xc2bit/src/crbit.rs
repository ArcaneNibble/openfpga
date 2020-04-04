/*
Copyright (c) 2017, Robert Ou <rqou@robertou.com> and contributors
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

//! Contains routines for dealing with xc2bit's "native" crbit format. TODO: Document this format.

use crate::util::{b2s};

use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::str;

/// Struct representing a 2-dimensional fuse array and handles converting xy-coordinates into a single linear index.
/// The x-axis is horizontal and the y-axis is vertical. The origin is at the top-left corner. (This is the standard
/// "computer graphics" coordinate scheme.)
pub struct FuseArray {
    /// Internal 1-dimensional storage
    v: Vec<bool>,
    /// Width of the array
    w: usize,
    /// Possibly contains a device name
    pub dev_name_str: Option<String>,
}

impl FuseArray {
    /// Get a fuse value at the particular xy coordinate
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.v[y * self.w + x]
    }

    /// Set the fuse value at the particular xy coordinate
    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        self.v[y * self.w + x] = val;
    }

    /// Returns the dimensions of this array as (width, height)
    pub fn dim(&self) -> (usize, usize) {
        (self.w, self.v.len() / self.w)
    }

    /// Processes the given data and converts it into a `FuseArray` struct.
    pub fn from_file_contents(in_bytes: &[u8]) -> Result<Self, &'static str> {
        // This capacity is approximate but close enough
        let mut v = Vec::with_capacity(in_bytes.len());
        let mut w = None;
        let mut dev_name_str = None;

        let in_str = str::from_utf8(in_bytes);
        if in_str.is_err() {
            return Err("invalid characters in crbit");
        }

        for l in in_str.unwrap().split('\n') {
            let l = l.trim_matches(|c| c == ' ' || c == '\r' || c == '\n');
            if l.len() == 0 {
                // ignore empty lines
                continue;
            }

            if l.starts_with("// DEVICE ") {
                dev_name_str = Some(l["// DEVICE ".len()..].to_owned());
            } else if !l.starts_with("//") {
                // not a comment
                if w.is_none() {
                    w = Some(l.len());
                }

                for c in l.chars() {
                    match c {
                        '0' => v.push(false),
                        '1' => v.push(true),
                        _ => return Err("invalid character in crbit"),
                    }
                }
            }
        }

        if w.is_none() {
            return Err("crbit contained no data");
        }

        Ok(FuseArray {
            v,
            w: w.unwrap(),
            dev_name_str
        })
    }

    /// Constructs a new `FuseArray` object with the given dimensions and filled with 0s
    pub fn from_dim(w: usize, h: usize) -> Self {
        FuseArray {
            w,
            v: vec![false; w*h],
            dev_name_str: None,
        }
    }

    /// Writes the fuse array to the internal "crbit" file format, which is an ASCII file containing '1' and '0'.
    /// (This format is intended to be compatible with `$readmemb`.)
    pub fn write_to_writer<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        write!(writer, "// crbit native bitstream file written by xc2bit\n")?;
        write!(writer, "// https://github.com/azonenberg/openfpga\n\n")?;

        if let Some(ref dev_name) = self.dev_name_str {
            write!(writer, "// DEVICE {}\n\n", dev_name)?;
        }

        let (w, h) = self.dim();
        for y in 0..h {
            for x in 0..w {
                write!(writer, "{}", b2s(self.get(x, y)))?;
            }
            write!(writer, "\n")?;
        }
        write!(writer, "\n")?;

        Ok(())
    }
}

impl Index<(usize, usize)> for FuseArray {
    type Output = bool;

    fn index(&self, coords: (usize, usize)) -> &bool {
        &self.v[coords.1 * self.w + coords.0]
    }
}

impl IndexMut<(usize, usize)> for FuseArray {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut bool {
        &mut self.v[coords.1 * self.w + coords.0]
    }
}

impl Index<[usize; 2]> for FuseArray {
    type Output = bool;

    fn index(&self, coords: [usize; 2]) -> &bool {
        &self.v[coords[1] * self.w + coords[0]]
    }
}

impl IndexMut<[usize; 2]> for FuseArray {
    fn index_mut(&mut self, coords: [usize; 2]) -> &mut bool {
        &mut self.v[coords[1] * self.w + coords[0]]
    }
}
