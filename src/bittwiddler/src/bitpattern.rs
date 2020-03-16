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

pub trait BitPattern where Self: Sized {
    type BitsArrType;
    type ErrType;
    const BITS_COUNT: usize;
    type NamesIterType;

    type VarNamesIterType;
    type VarDescsIterType;
    type VarBitsIterType;

    fn encode(&self) -> Self::BitsArrType;
    fn decode(bits: Self::BitsArrType) -> Result<Self, Self::ErrType>;
    fn bitnames() -> Self::NamesIterType;

    fn variantnames() -> Self::VarNamesIterType;
    fn variantdescs() -> Self::VarDescsIterType;
    fn variantbits() -> Self::VarBitsIterType;
}

impl BitPattern for bool {
    type BitsArrType = [bool; 1];
    const BITS_COUNT: usize = 1;
    type ErrType = ();
    type NamesIterType = std::slice::Iter<'static, &'static str>;

    type VarNamesIterType = std::slice::Iter<'static, &'static str>;
    type VarDescsIterType = std::slice::Iter<'static, &'static str>;
    type VarBitsIterType = std::slice::Iter<'static, &'static str>;

    fn encode(&self) -> Self::BitsArrType {
        [*self]
    }

    fn decode(bits: Self::BitsArrType) -> Result<Self, Self::ErrType> {
        Ok(bits[0])
    }

    fn bitnames() -> Self::NamesIterType {
        ["x"].iter()
    }

    fn variantnames() -> Self::VarNamesIterType {
        ["false", "true"].iter()
    }

    fn variantdescs() -> Self::VarDescsIterType {
        ["false", "true"].iter()
    }

    fn variantbits() -> Self::VarBitsIterType {
        ["0", "1"].iter()
    }
}

pub fn docs_as_ascii_table<T>() -> String 
    where T: BitPattern,
        <T as BitPattern>::NamesIterType: std::iter::Iterator<Item=&'static &'static str>,
        <T as BitPattern>::VarNamesIterType: std::iter::Iterator<Item=&'static &'static str>,
        <T as BitPattern>::VarDescsIterType: std::iter::Iterator<Item=&'static &'static str>,
        <T as BitPattern>::VarBitsIterType: std::iter::Iterator<Item=&'static &'static str>,
{
    let mut ret = String::new();

    let variantnames = T::variantnames().collect::<Vec<_>>();
    let variantdescs = T::variantdescs().collect::<Vec<_>>();
    let variantbits = T::variantbits().collect::<Vec<_>>();

    assert_eq!(variantnames.len(), variantdescs.len());
    assert_eq!(variantdescs.len(), variantbits.len());

    let mut max_name_len = 0;
    for varname in &variantnames {
        if varname.len() > max_name_len {
            max_name_len = varname.len()
        }
    }

    let mut max_desc_len = 0;
    for vardesc in &variantdescs {
        if vardesc.len() > max_desc_len {
            max_desc_len = vardesc.len()
        }
    }

    // Header
    for x in T::bitnames() {
        ret.push_str(x);
    }
    ret.push_str(" | ");
    for _ in 0..max_name_len {
        ret.push_str(" ");
    }
    ret.push_str(" |\n");

    // Separator
    for _ in 0..T::BITS_COUNT {
        ret.push_str("-");
    }
    ret.push_str("-+-");
    for _ in 0..max_name_len {
        ret.push_str("-");
    }
    ret.push_str("-+-");
    for _ in 0..max_desc_len {
        ret.push_str("-");
    }
    ret.push_str("\n");

    // Data
    for i in 0..variantnames.len() {
        ret.push_str(variantbits[i]);
        ret.push_str(" | ");
        ret.push_str(variantnames[i]);
        for _ in variantnames[i].len()..max_name_len {
            ret.push_str(" ");
        }
        ret.push_str(" | ");
        ret.push_str(variantdescs[i]);
        ret.push_str("\n");
    }

    ret
}
