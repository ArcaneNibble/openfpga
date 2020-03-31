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
    type OffsettingType: AsRef<[isize]>;
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
