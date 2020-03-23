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

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::punctuated::*;

use crate::args::*;

#[derive(Debug)]
enum BitFragmentSetting {
    ErrType(ArgWithType),
    Variant(ArgWithType),
}

impl Parse for BitFragmentSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::errtype) {
            input.parse().map(BitFragmentSetting::ErrType)
        } else if lookahead.peek(kw::variant) {
            input.parse().map(BitFragmentSetting::Variant)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct BitFragmentSettings(Punctuated<BitFragmentSetting, token::Comma>);

impl Parse for BitFragmentSettings {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(BitFragmentSettings(input.parse_terminated(BitFragmentSetting::parse)?))
    }
}

pub fn bitfragment(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Item);
    let input_copy = input.to_token_stream();
    let args = parse_macro_input!(args as BitFragmentSettings);

    let mut errtype = None;
    let mut encode_variant = None;

    // Tracks if errors (that we can recover from) occurred. If so, we bail
    // before doing final codegen
    let mut errors_occurred = false;

    // process args
    for arg in args.0 {
        match arg {
            BitFragmentSetting::ErrType(bpet) => {
                errtype = Some(bpet.ty);
            },
            BitFragmentSetting::Variant(bpev) => {
                encode_variant = Some(bpev.ty);
            },
        }
    }

    let obj_id;
    if let Item::Enum(enum_) = input {
        obj_id = enum_.ident.clone();

    } else if let Item::Struct(struct_) = input {
        obj_id = struct_.ident.clone();

    } else {
        abort!(input, "#[bitfragment] can only be used on a struct or enum");
    }

    // Can start generating code now
    let encode_variant = if let Some(x) = encode_variant {
        x.into_token_stream()
    } else {
        quote!(())
    };

    let errtype = if errtype.is_none() {
        quote!{()}
    } else {
        quote!{#errtype}
    };

    let output = quote!{
        #input_copy

        impl ::bittwiddler::BitFragment<#encode_variant> for #obj_id {
            const IDX_DIMS: usize = 1;
            type IndexingType = usize;
            type OffsettingType = [usize; 1];
            type MirroringType = [bool; 1];

            type ErrType = #errtype;

            const FIELD_COUNT: usize = 0;

            fn encode<F>(&self, fuses: &mut F, offset: Self::OffsettingType, mirror: Self::MirroringType)
                where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

                // fuses[if mirror[0] {offset[0] - 0} else {offset[0] + 0}] = self.field1;
                // fuses[if mirror[0] {offset[0] - 1} else {offset[0] + 1}] = self.field1;
            }
            fn decode<F>(fuses: &F, offset: Self::OffsettingType, mirror: Self::MirroringType) -> Result<Self, Self::ErrType>
                where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

                // Ok(Self{
                //     field1: fuses[if mirror[0] {offset[0] - 0} else {offset[0] + 0}],
                //     field2: fuses[if mirror[0] {offset[0] - 1} else {offset[0] + 1}],
                // })
                Err(())
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
    };

    TokenStream::from(output)
}
