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
use proc_macro2::{Ident, Span};
use quote::*;
use syn::*;

pub fn bitpattern(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);

    // Ignore enums with no variants
    if input.variants.len() == 0 {
        println!("Warning: BitPattern used on enum with no variants");
        return TokenStream::from(quote!{#input});
    }

    let enum_id = input.ident.clone();

    let mut var_data = Vec::new();
    for var in &mut input.variants {
        let var_id = var.ident.clone();

        if var.fields != Fields::Unit {
            panic!("Variant {} must be a unit variant", var_id.to_string());
        }

        // Find the #[bits] attribute
        let mut bits_attrib = None;
        let mut bits_docs = String::new();
        for (i, attr) in var.attrs.iter().enumerate() {
            if attr.path.is_ident("bits") {
                if bits_attrib.is_some() {
                    panic!("Only one #[bits] attribute allowed on {}", var_id.to_string());
                }

                let bits_arg = attr.parse_args::<LitStr>();
                if let Err(e) = bits_arg {
                    return e.to_compile_error().into();
                }
                let bits_arg = bits_arg.unwrap();

                bits_attrib = Some((i, bits_arg.value()));
            }

            if attr.path.is_ident("doc") {
                let doc_meta = attr.parse_meta();
                if let Err(e) = doc_meta {
                    return e.to_compile_error().into();
                }
                let doc_meta = doc_meta.unwrap();

                if let Meta::NameValue(nv) = doc_meta {
                    if let Lit::Str(s) = nv.lit {
                        if bits_docs.len() != 0 {
                            bits_docs.push_str(" ");
                        }
                        bits_docs.push_str(s.value().trim());
                    }
                }
            }
        }
        if bits_attrib.is_none() {
            panic!("Variant {} must have a #[bits] attribute", var_id.to_string());
        }

        let (bits_attrib_i, bits_string) = bits_attrib.unwrap();
        var.attrs.remove(bits_attrib_i);

        var_data.push((var_id, bits_string, bits_docs));
    }

    println!("{:?}", var_data);

    // Gathered all variants and their settings, do some checks to make sure
    // things are valid
    let num_bits = var_data[0].1.len();
    for (_, bits_string, _) in &var_data {
        if bits_string.len() != num_bits {
            panic!("All bits need to be the same length");
        }

        for c in bits_string.chars() {
            if c != '0'  && c != '1' && c != 'x' && c != 'X' {
                panic!("Illegal character in bits attribute");
            }
        }
    }

    // Can start generating code now
    // Dummy for now
    let bit_names = (0..num_bits).map(|x| LitStr::new(&x.to_string(), Span::call_site()));
    let bit_names2 = (0..num_bits).map(|x| LitStr::new(&x.to_string(), Span::call_site()));
    let bit_nums = 0..num_bits;

    // For encode function
    let encode_values = var_data.iter().map(|x|
        x.1.chars().map(|c|
            match c {
                '0'|'x' => quote! {false},
                '1'|'X' => quote! {true},
                _ => unreachable!(),
            }
        ).collect::<Vec<_>>()
    );
    let encode_var_id = var_data.iter().map(|x| x.0.clone());

    let variant_names = var_data.iter().map(|x| LitStr::new(&x.0.to_string(), Span::call_site()));
    let variant_docs = var_data.iter().map(|x| LitStr::new(&x.2.to_string(), Span::call_site()));
    let variant_bits = var_data.iter().map(|x| LitStr::new(&x.1.to_string(), Span::call_site()));

    let output = quote!{
        #input

        impl ::bittwiddler::BitPattern for #enum_id {
            type BitsArrType = [bool; #num_bits];
            const BITS_COUNT: usize = #num_bits;
            type ErrType = ();  // TODO

            type VarNamesIterType = ::std::slice::Iter<'static, &'static str>;
            type VarDescsIterType = ::std::slice::Iter<'static, &'static str>;
            type VarBitsIterType = ::std::slice::Iter<'static, &'static str>;

            fn encode(&self) -> Self::BitsArrType {
                match self {
                    #(Self::#encode_var_id => [#(#encode_values),*]),*
                }
            }

            fn decode(bits: Self::BitsArrType) -> Result<Self, Self::ErrType> {
                unimplemented!()
            }

            fn _pos_to_name(pos: usize) -> &'static str {
                [#(#bit_names),*][pos]
            }

            fn _name_to_pos(name: &'static str) -> usize {
                match name {
                    #(#bit_names2 => #bit_nums),*
                    ,_ => unreachable!()
                }
            }

            fn variantnames() -> Self::VarNamesIterType {
                [#(#variant_names),*].iter()
            }


            fn variantdescs() -> Self::VarDescsIterType {
                [#(#variant_docs),*].iter()
            }

            fn variantbits() -> Self::VarBitsIterType {
                [#(#variant_bits),*].iter()
            }
        }
    };

    TokenStream::from(output)
}
