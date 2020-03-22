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
use proc_macro2::Span;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::punctuated::*;

mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(errtype);
    syn::custom_keyword!(bitnames);
}

#[derive(Debug)]
struct BitPatternDefaultExpr {
    _ident: Ident,
    _eq: syn::token::Eq,
    expr: Expr,
}

impl Parse for BitPatternDefaultExpr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(BitPatternDefaultExpr {
            _ident: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct BitPatternErrType {
    _ident: Ident,
    _eq: syn::token::Eq,
    ty: Type,
}

impl Parse for BitPatternErrType {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(BitPatternErrType {
            _ident: input.parse()?,
            _eq: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct BitPatternBitNames {
    _ident: Ident,
    _eq: syn::token::Eq,
    names: LitStr,
}

impl Parse for BitPatternBitNames {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(BitPatternBitNames {
            _ident: input.parse()?,
            _eq: input.parse()?,
            names: input.parse()?,
        })
    }
}

#[derive(Debug)]
enum BitPatternSetting {
    DefaultExpr(BitPatternDefaultExpr),
    ErrType(BitPatternErrType),
    BitNames(BitPatternBitNames),
}

impl Parse for BitPatternSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::default) {
            input.parse().map(BitPatternSetting::DefaultExpr)
        } else if lookahead.peek(kw::errtype) {
            input.parse().map(BitPatternSetting::ErrType)
        } else if lookahead.peek(kw::bitnames) {
            input.parse().map(BitPatternSetting::BitNames)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct BitPatternSettings(Punctuated<BitPatternSetting, token::Comma>);

impl Parse for BitPatternSettings {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(BitPatternSettings(input.parse_terminated(BitPatternSetting::parse)?))
    }
}

pub fn bitpattern(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as BitPatternSettings);
    let mut input = parse_macro_input!(input as ItemEnum);

    let mut default_expr = None;
    let mut errtype = None;
    let mut bitnames_str = None;

    // process args
    for arg in args.0 {
        match arg {
            BitPatternSetting::DefaultExpr(bpdx) => {
                default_expr = Some(bpdx.expr);
            },
            BitPatternSetting::ErrType(bpet) => {
                errtype = Some(bpet.ty);
            },
            BitPatternSetting::BitNames(bpbn) => {
                bitnames_str = Some(bpbn.names.value());
            }
        }
    }

    // Ignore enums with no variants
    if input.variants.len() == 0 {
        println!("Warning: BitPattern used on enum with no variants");
        return TokenStream::from(quote!{#input});
    }

    let enum_id = input.ident.clone();

    // loop over the variants and grab the necessary data
    let mut var_data = Vec::new();
    for var in &mut input.variants {
        let var_id = var.ident.clone();

        if var.fields != Fields::Unit {
            panic!("Variant {} must be a unit variant", var_id.to_string());
        }

        // Find the #[bits] attribute and docstrings
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
    let bit_names: Vec<LitStr>;
    if let Some(bitnames_str) = bitnames_str {
        let bitnames_vec = bitnames_str.split_whitespace().collect::<Vec<_>>();
        if bitnames_vec.len() == 1 {
            // No spaces; each char is a bit name
            let bit_names_chars = bitnames_vec[0].chars().collect::<Vec<_>>();
            if bit_names_chars.len() != num_bits {
                panic!("Mismatched number of names in bitnames");
            }
            bit_names = bit_names_chars.iter().map(|x| LitStr::new(&x.to_string(), Span::call_site())).collect();
        } else {
            // Has spaces; each word is a bit name
            if bitnames_vec.len() != num_bits {
                panic!("Mismatched number of names in bitnames");
            }
            bit_names = bitnames_vec.iter().map(|x| LitStr::new(x, Span::call_site())).collect();
        }
    } else {
        // Each bit is just named by the bit index
        bit_names = (0..num_bits).map(|x| LitStr::new(&x.to_string(), Span::call_site())).collect();

    }
    let bit_names2 = bit_names.clone();
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

    // For decode function
    let decode_values = var_data.iter().map(|x|
        x.1.chars().map(|c|
            match c {
                '0' => quote! {false},
                '1' => quote! {true},
                'x'|'X' => quote! {_},
                _ => unreachable!(),
            }
        ).collect::<Vec<_>>()
    );
    let decode_var_id = var_data.iter().map(|x| x.0.clone());

    let default_expr = default_expr.iter();
    let default_expr = if errtype.is_none() {
        quote!{
            #(,_ => Ok(#default_expr))*
        }
    } else {
        quote!{
            #(,_ => Err(#default_expr))*
        }
    };

    let errtype = if errtype.is_none() {
        quote!{()}
    } else {
        quote!{#errtype}
    };

    // For docs
    let num_variants = input.variants.len();
    let variant_names = var_data.iter().map(|x| LitStr::new(&x.0.to_string(), Span::call_site()));
    let variant_docs = var_data.iter().map(|x| LitStr::new(&x.2.to_string(), Span::call_site()));
    let variant_bits = var_data.iter().map(|x| LitStr::new(&x.1.to_string(), Span::call_site()));

    let output = quote!{
        #input

        impl ::bittwiddler::BitPattern for #enum_id {
            type BitsArrType = [bool; #num_bits];
            const BITS_COUNT: usize = #num_bits;

            type ErrType = #errtype;

            const VARIANT_COUNT: usize = #num_variants;

            fn encode(&self) -> Self::BitsArrType {
                match self {
                    #(Self::#encode_var_id => [#(#encode_values),*]),*
                }
            }

            fn decode(bits: Self::BitsArrType) -> Result<Self, Self::ErrType> {
                match bits {
                    #([#(#decode_values),*] => Ok(Self::#decode_var_id)),*
                    #default_expr
                }
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

            fn variantname(var: usize) -> &'static str {
                [#(#variant_names),*][var]
            }


            fn variantdesc(var: usize) -> &'static str {
                [#(#variant_docs),*][var]
            }

            fn variantbits(var: usize) -> &'static str {
                [#(#variant_bits),*][var]
            }
        }
    };

    TokenStream::from(output)
}
