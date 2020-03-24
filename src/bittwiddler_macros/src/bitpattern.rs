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
use proc_macro2::Span;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::punctuated::*;

use crate::args::*;

#[derive(Debug)]
enum BitPatternSetting {
    DefaultExpr(ArgWithExpr),
    ErrType(ArgWithType),
    BitNames(ArgWithLitStr),
    Variant(ArgWithType),
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
        } else if lookahead.peek(kw::variant) {
            input.parse().map(BitPatternSetting::Variant)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for BitPatternSetting {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            BitPatternSetting::DefaultExpr(x) => x.to_tokens(tokens),
            BitPatternSetting::ErrType(x) => x.to_tokens(tokens),
            BitPatternSetting::BitNames(x) => x.to_tokens(tokens),
            BitPatternSetting::Variant(x) => x.to_tokens(tokens),
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

// Args for the #[bits] attribute macro
#[derive(Debug)]
enum BitValueSetting {
    BitValueString(LitStr),
    Variant(ArgWithType),
}

impl Parse for BitValueSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(BitValueSetting::BitValueString)
        } else if lookahead.peek(kw::variant) {
            input.parse().map(BitValueSetting::Variant)
        } else {
            Err(lookahead.error())
        }
    }
}

type BitValueSettings = Punctuated<BitValueSetting, token::Comma>;

pub fn bitpattern(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as BitPatternSettings);
    let mut input = parse_macro_input!(input as ItemEnum);

    let mut default_expr = None;
    let mut errtype = None;
    let mut bitnames_strlit = None;
    let mut encode_variant = None;

    // Tracks if errors (that we can recover from) occurred. If so, we bail
    // before doing final codegen
    let mut errors_occurred = false;

    // process args
    for arg in &args.0 {
        match arg {
            BitPatternSetting::DefaultExpr(x) => {
                if default_expr.is_some() {
                    emit_error!(args.0, "Only one default arg allowed");
                    errors_occurred = true;
                }
                default_expr = Some(x.expr.clone());
            },
            BitPatternSetting::ErrType(x) => {
                if errtype.is_some() {
                    emit_error!(args.0, "Only one errtype arg allowed");
                    errors_occurred = true;
                }
                errtype = Some(x.ty.clone());
            },
            BitPatternSetting::BitNames(x) => {
                if bitnames_strlit.is_some() {
                    emit_error!(args.0, "Only one bitnames arg allowed");
                    errors_occurred = true;
                }
                bitnames_strlit = Some(x.litstr.clone());
            },
            BitPatternSetting::Variant(x) => {
                if encode_variant.is_some() {
                    emit_error!(args.0, "Only one variant arg allowed");
                    errors_occurred = true;
                }
                encode_variant = Some(x.ty.clone());
            },
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
            emit_error!(var.fields, "#[bitpattern] enum variant must be a unit variant");
            errors_occurred = true;
        }

        // Find the #[bits] attribute and docstrings
        let mut bits_attrib = None;
        let mut bits_docs = String::new();
        for (i, attr) in var.attrs.iter().enumerate() {
            if attr.path.is_ident("bits") {
                let parser = BitValueSettings::parse_separated_nonempty;
                let bits_args = attr.parse_args_with(parser);
                if let Err(e) = bits_args {
                    return e.to_compile_error().into();
                }
                let bits_args = bits_args.unwrap();

                // Possibly filter by bit encoding variant
                let mut maybe_bitstr = None;
                let mut maybe_var_ty = None;
                for bits_arg in bits_args {
                    match bits_arg {
                        BitValueSetting::BitValueString(s) => {
                            if maybe_bitstr.is_some() {
                                emit_error!(s, "Only one string literal allowed");
                                errors_occurred = true;
                            }
                            maybe_bitstr = Some(s);
                        },
                        BitValueSetting::Variant(v) => {
                            if maybe_var_ty.is_some() {
                                emit_error!(v, "Only one variant arg allowed");
                                errors_occurred = true;
                            }
                            maybe_var_ty = Some(v.ty);
                        },
                    }
                }

                if maybe_var_ty.is_none() && encode_variant.is_none() ||
                    (maybe_var_ty.is_some() && encode_variant.is_some() &&
                        maybe_var_ty.as_ref().unwrap() == encode_variant.as_ref().unwrap()) {
                    if bits_attrib.is_some() {
                        errors_occurred = true;
                        if let Some(bitvar) = encode_variant.as_ref() {
                            emit_error!(attr, "Only one #[bits] attribute allowed for bit variant {}", quote!{#bitvar}.to_string());
                        } else {
                            emit_error!(attr, "Only one #[bits] attribute allowed");
                        }
                    }

                    if let Some(bitstr) = maybe_bitstr {
                        bits_attrib = Some((i, bitstr));
                    } else if maybe_bitstr.is_none() {
                        emit_error!(attr, "Missing bit pattern string literal");
                        errors_occurred = true;
                    }
                }
            }

            if attr.path.is_ident("doc") {
                let doc_meta = attr.parse_meta();
                if let Err(e) = doc_meta {
                    return e.to_compile_error().into();
                }
                let doc_meta = doc_meta.unwrap();

                if let Meta::NameValue(MetaNameValue{lit: Lit::Str(s), ..}) = doc_meta {
                    if bits_docs.len() != 0 {
                        bits_docs.push_str(" ");
                    }
                    bits_docs.push_str(s.value().trim());
                }
            }
        }

        if let Some((bits_attrib_i, bits_string_lit)) = bits_attrib {
            var.attrs.remove(bits_attrib_i);
            var_data.push((var_id, bits_string_lit.value(), bits_docs, bits_string_lit));
        } else {
            errors_occurred = true;
            if let Some(bitvar) = encode_variant.as_ref() {
                emit_error!(var, "Enum variant must have a #[bits] attribute for bit variant {}", quote!{#bitvar}.to_string());
            } else {
                emit_error!(var, "Enum variant must have a #[bits] attribute");
            }
        }
    }

    // If there are no variants here, then we need to bail because there were
    // too many errors earlier
    if var_data.len() == 0 {
        return TokenStream::from(quote!{#input});
    }

    // Gathered all variants and their settings, do some checks to make sure
    // things are valid
    let num_bits = var_data[0].1.len();
    for (_, bits_string, _, bits_string_lit) in &var_data {
        if bits_string.len() != num_bits {
            emit_error!(bits_string_lit, "All bit pattern strings need to be the same length (expected {})", num_bits);
            errors_occurred = true;
        }

        for c in bits_string.chars() {
            if c != '0'  && c != '1' && c != 'x' && c != 'X' {
                emit_error!(bits_string_lit, "Illegal character '{}' in bit pattern string", c);
                errors_occurred = true;
            }
        }
    }

    // Can start generating code now
    let encode_variant = if let Some(x) = encode_variant {
        x.into_token_stream()
    } else {
        quote!(())
    };

    let bit_names: Vec<LitStr>;
    if let Some(bitnames_strlit) = bitnames_strlit {
        let bitnames_str = bitnames_strlit.value();
        let bitnames_vec = bitnames_str.split_whitespace().collect::<Vec<_>>();
        if bitnames_vec.len() == 1 {
            // No spaces; each char is a bit name
            let bit_names_chars = bitnames_vec[0].chars().collect::<Vec<_>>();
            if bit_names_chars.len() != num_bits {
                emit_error!(bitnames_strlit, "Wrong number of names (expected {})", num_bits);
                errors_occurred = true;
            }
            bit_names = bit_names_chars.iter().map(|x| LitStr::new(&x.to_string(), Span::call_site())).collect();
        } else {
            // Has spaces; each word is a bit name
            if bitnames_vec.len() != num_bits {
                emit_error!(bitnames_strlit, "Wrong number of names (expected {})", num_bits);
                errors_occurred = true;
            }
            bit_names = bitnames_vec.iter().map(|x| LitStr::new(x, Span::call_site())).collect();
        }
    } else {
        // Each bit is just named by the bit index
        bit_names = (0..num_bits).map(|x| LitStr::new(&x.to_string(), Span::call_site())).collect();

    }
    let bit_names2 = bit_names.clone();
    let bit_nums = 0..num_bits;

    // Finally, we have to bail if things are broken
    if errors_occurred {
        return TokenStream::from(quote!{#input});
    }

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
    let variant_docs = var_data.iter().map(|x| LitStr::new(&x.2, Span::call_site()));
    let variant_bits = var_data.iter().map(|x| LitStr::new(&x.1, Span::call_site()));

    let output = quote!{
        #input

        impl ::bittwiddler::BitPattern<#encode_variant> for #enum_id {
            type BitsArrType = [bool; #num_bits];
            const BITS_COUNT: usize = #num_bits;

            type ErrType = #errtype;

            const VARIANT_COUNT: usize = #num_variants;

            #[inline]
            fn encode(&self) -> Self::BitsArrType {
                match self {
                    #(Self::#encode_var_id => [#(#encode_values),*]),*
                }
            }

            #[inline]
            fn decode(bits: &Self::BitsArrType) -> Result<Self, Self::ErrType> {
                match bits {
                    #([#(#decode_values),*] => Ok(Self::#decode_var_id)),*
                    #default_expr
                }
            }

            #[inline]
            fn _pos_to_name(pos: usize) -> &'static str {
                [#(#bit_names),*][pos]
            }

            #[inline]
            fn _name_to_pos(name: &'static str) -> usize {
                match name {
                    #(#bit_names2 => #bit_nums),*
                    ,_ => unreachable!()
                }
            }

            #[inline]
            fn variantname(var: usize) -> &'static str {
                [#(#variant_names),*][var]
            }

            #[inline]
            fn variantdesc(var: usize) -> &'static str {
                [#(#variant_docs),*][var]
            }

            #[inline]
            fn variantbits(var: usize) -> &'static str {
                [#(#variant_bits),*][var]
            }
        }
    };

    TokenStream::from(output)
}
