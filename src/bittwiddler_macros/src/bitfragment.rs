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
enum BitFragmentSetting {
    ErrType(ArgWithType),
    Variant(ArgWithType),
    Dims(ArgWithLitInt),
}

impl Parse for BitFragmentSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::errtype) {
            input.parse().map(BitFragmentSetting::ErrType)
        } else if lookahead.peek(kw::variant) {
            input.parse().map(BitFragmentSetting::Variant)
        } else if lookahead.peek(kw::dimensions) {
            input.parse().map(BitFragmentSetting::Dims)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for BitFragmentSetting {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            BitFragmentSetting::ErrType(x) => x.to_tokens(tokens),
            BitFragmentSetting::Variant(x) => x.to_tokens(tokens),
            BitFragmentSetting::Dims(x) => x.to_tokens(tokens),
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

#[derive(Copy, Clone, Debug)]
enum BitFragmentFieldType {
    Pattern,
    Fragment,
    PatternArray,
    FragmentArray,
}

#[derive(Copy, Clone, Debug)]
enum FieldMode {
    Enum,
    NamedStruct,
    UnnamedStruct,
}

#[derive(Debug)]
struct FieldInfo {
    name_str: String,
    field_id: Option<Ident>,
    docs: String,
    field_type_enum: BitFragmentFieldType,
    field_type_ty: Option<Type>,
}

#[derive(Debug)]
struct ParsedAttrs {
    docs: String,
}

fn parse_attrs(attrs: &[Attribute]) -> ::core::result::Result<ParsedAttrs, syn::Error> {
    let mut docs = String::new();
    for attr in attrs {
        if attr.path.is_ident("doc") {
            let doc_meta = attr.parse_meta()?;

            if let Meta::NameValue(nv) = doc_meta {
                if let Lit::Str(s) = nv.lit {
                    if docs.len() != 0 {
                        docs.push_str(" ");
                    }
                    docs.push_str(s.value().trim());
                }
            }
        }
    }

    Ok(ParsedAttrs {
        docs,
    })
}

pub fn bitfragment(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Item);
    let input_copy = input.to_token_stream();
    let args = parse_macro_input!(args as BitFragmentSettings);

    let mut errtype = None;
    let mut encode_variant = None;
    let mut idx_dims = None;

    // Tracks if errors (that we can recover from) occurred. If so, we bail
    // before doing final codegen
    let mut errors_occurred = false;

    // process args
    for arg in &args.0 {
        match arg {
            BitFragmentSetting::ErrType(x) => {
                if errtype.is_some() {
                    emit_error!(args.0, "Only one errtype arg allowed");
                    errors_occurred = true;
                }
                errtype = Some(x.ty.clone());
            },
            BitFragmentSetting::Variant(x) => {
                if encode_variant.is_some() {
                    emit_error!(args.0, "Only one variant arg allowed");
                    errors_occurred = true;
                }
                encode_variant = Some(x.ty.clone());
            },
            BitFragmentSetting::Dims(x) => {
                if idx_dims.is_some() {
                    emit_error!(args.0, "Only one dimensions arg allowed");
                    errors_occurred = true;
                }
                idx_dims = Some(x.litint.clone());
            }
        }
    }

    if idx_dims.is_none() {
        emit_error!(args.0, "#[bitfragment] requires dimensions to be specified");
        errors_occurred = true;
    }

    // arg parsing done, walk over data and gather info about fields

    let obj_id;
    let field_mode;
    let mut obj_field_info = Vec::new();

    match &input {
        Item::Enum(enum_) => {
            obj_id = enum_.ident.clone();
            field_mode = FieldMode::Enum;

            let parsed_attrs = parse_attrs(&enum_.attrs);
            if let Err(e) = parsed_attrs {
                return e.to_compile_error().into();
            }
            let parsed_attrs = parsed_attrs.unwrap();

            obj_field_info.push(FieldInfo {
                name_str: obj_id.to_string(),
                field_id: Some(obj_id.clone()),
                docs: parsed_attrs.docs,
                field_type_enum: BitFragmentFieldType::Pattern,
                field_type_ty: None,
            });
        },
        Item::Struct(struct_) => {
            obj_id = struct_.ident.clone();

            let (mode, fields) = match &struct_.fields {
                Fields::Named(fields) => {
                    (FieldMode::NamedStruct, &fields.named)
                },
                Fields::Unnamed(fields) => {
                    (FieldMode::UnnamedStruct, &fields.unnamed)
                },
                Fields::Unit => {
                    abort!(input, "#[bitfragment] cannot be used on a unit struct");
                }
            };

            field_mode = mode;
            for (field_i, field) in fields.iter().enumerate() {
                let name_str = if let Some(id) = field.ident.as_ref() {
                    id.to_string()
                } else {
                    field_i.to_string()
                };

                let parsed_attrs = parse_attrs(&field.attrs);
                if let Err(e) = parsed_attrs {
                    return e.to_compile_error().into();
                }
                let parsed_attrs = parsed_attrs.unwrap();

                obj_field_info.push(FieldInfo {
                    name_str,
                    field_id: field.ident.clone(),
                    docs: parsed_attrs.docs,
                    field_type_enum: BitFragmentFieldType::Pattern,  // TODO
                    field_type_ty: Some(field.ty.clone())
                });
            }
        },
        _ => {
            abort!(input, "#[bitfragment] can only be used on a struct or enum");
        }
    }

    // Can start generating code now
    if errors_occurred {
        return TokenStream::from(quote!{#input_copy});
    }

    // basic settings
    let idx_dims = idx_dims.unwrap();
    let idx_dims = idx_dims.base10_parse::<usize>();
    if let Err(e) = idx_dims {
        return e.to_compile_error().into();
    }
    let idx_dims = idx_dims.unwrap();

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

    let indexingtype = if idx_dims == 1 {
        quote!{usize}
    } else {
        quote!{[usize; #idx_dims]}
    };

    // for docs
    let num_fields = obj_field_info.len();
    let field_names = obj_field_info.iter().map(|x| LitStr::new(&x.name_str, Span::call_site()));
    let field_docs = obj_field_info.iter().map(|x| LitStr::new(&x.docs, Span::call_site()));
    let field_types = obj_field_info.iter().map(|x| {
        let fieldtype_id = match x.field_type_enum {
            BitFragmentFieldType::Pattern => quote!{Pattern},
            BitFragmentFieldType::Fragment => quote!{Fragment},
            BitFragmentFieldType::PatternArray => quote!{PatternArray},
            BitFragmentFieldType::FragmentArray => quote!{FragmentArray},
        };
        quote!{::bittwiddler::BitFragmentFieldType::#fieldtype_id}
    });
    
    let output = quote!{
        #input_copy

        impl ::bittwiddler::BitFragment<#encode_variant> for #obj_id {
            const IDX_DIMS: usize = #idx_dims;
            type IndexingType = #indexingtype;
            type OffsettingType = [usize; #idx_dims];
            type MirroringType = [bool; #idx_dims];

            type ErrType = #errtype;

            const FIELD_COUNT: usize = #num_fields;

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

            fn fieldname(i: usize) -> &'static str {
                [#(#field_names),*][i]
            }
            fn fielddesc(i: usize) -> &'static str {
                [#(#field_docs),*][i]
            }
            fn fieldtype(i: usize) -> BitFragmentFieldType {
                [#(#field_types),*][i]
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
