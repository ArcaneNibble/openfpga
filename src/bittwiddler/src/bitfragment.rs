/*
Copyright (c) 2020, R. Ou <rqou@robertou.com> and contributors
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BitFragmentFieldType {
    Pattern,
    Fragment,
    PatternArray(usize),
    FragmentArray(usize),
}

pub trait BitFragment<T> where Self: Sized {
    const IDX_DIMS: usize;
    type IndexingType;
    type OffsettingType: AsRef<[usize]>;
    type MirroringType: AsRef<[bool]>;

    type ErrType;

    const FIELD_COUNT: usize;

    fn encode<F>(&self, fuses: &mut F, offset: Self::OffsettingType, mirror: Self::MirroringType)
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized;
    fn decode<F>(fuses: &F, offset: Self::OffsettingType, mirror: Self::MirroringType) -> Result<Self, Self::ErrType>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized;

    fn fieldname(i: usize) -> &'static str;
    fn fielddesc(i: usize) -> &'static str;
    fn fieldtype(i: usize) -> BitFragmentFieldType;
    fn field_offset(field_i: usize, arr_i: usize) -> Self::OffsettingType;
    fn field_mirror(field_i: usize, arr_i: usize) -> Self::MirroringType;
    fn field_bits(field_i: usize) -> usize;
    fn field_bit_base_pos(field_i: usize, bit_i: usize) -> Self::OffsettingType;
}

pub struct TestFragment {
    pub field1: bool,
    pub field2: bool,
}

impl BitFragment<()> for TestFragment {
    const IDX_DIMS: usize = 1;
    type IndexingType = usize;
    type OffsettingType = [usize; 1];
    type MirroringType = [bool; 1];

    type ErrType = ();

    const FIELD_COUNT: usize = 0;

    fn encode<F>(&self, fuses: &mut F, offset: Self::OffsettingType, mirror: Self::MirroringType)
        where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

        fuses[if mirror[0] {offset[0] - 0} else {offset[0] + 0}] = self.field1;
        fuses[if mirror[0] {offset[0] - 1} else {offset[0] + 1}] = self.field1;
    }
    fn decode<F>(fuses: &F, offset: Self::OffsettingType, mirror: Self::MirroringType) -> Result<Self, Self::ErrType>
        where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

        Ok(Self{
            field1: fuses[if mirror[0] {offset[0] - 0} else {offset[0] + 0}],
            field2: fuses[if mirror[0] {offset[0] - 1} else {offset[0] + 1}],
        })
    }

    fn fieldname(_i: usize) -> &'static str {
        ""
    }
    fn fielddesc(_i: usize) -> &'static str {
        ""
    }
    fn fieldtype(_i: usize) -> BitFragmentFieldType {
        BitFragmentFieldType::Fragment
    }
    fn field_offset(_field_i: usize, _arr_i: usize) -> Self::OffsettingType {
        [0]
    }
    fn field_mirror(_field_i: usize, _arr_i: usize) -> Self::MirroringType {
        [false]
    }
    fn field_bits(_field_i: usize) -> usize {
        0
    }
    fn field_bit_base_pos(_field_i: usize, _bit_i: usize) -> Self::OffsettingType {
        [0]
    }
}
