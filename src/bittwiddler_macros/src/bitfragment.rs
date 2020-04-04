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

use std::collections::HashMap;

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
    EncodeExtraType(ArgWithType),
    DecodeExtraType(ArgWithType),
    Variant(ArgWithType),
    Dims(ArgWithLitInt),
}

impl Parse for BitFragmentSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::errtype) {
            input.parse().map(BitFragmentSetting::ErrType)
        } else if lookahead.peek(kw::encode_extra_type) {
            input.parse().map(BitFragmentSetting::EncodeExtraType)
        } else if lookahead.peek(kw::decode_extra_type) {
            input.parse().map(BitFragmentSetting::DecodeExtraType)
        } else if lookahead.peek(kw::variant) {
            input.parse().map(BitFragmentSetting::Variant)
        } else if lookahead.peek(kw::frag_variant) {
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
            BitFragmentSetting::EncodeExtraType(x) => x.to_tokens(tokens),
            BitFragmentSetting::DecodeExtraType(x) => x.to_tokens(tokens),
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

#[derive(Debug)]
enum PatBitPos {
    Loc(Vec<isize>),
    Bool(bool),
}

#[derive(Debug)]
struct PatBitInfo {
    invert: bool,
    pos: PatBitPos,
}

type PatBitsInfo = HashMap<String, PatBitInfo>;

#[derive(Copy, Clone, Debug)]
enum BitFragmentFieldType {
    Pattern,
    Fragment,
    PatternArray,
    FragmentArray,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    patbits: Option<PatBitsInfo>,
    subvar: Option<Type>,
    base_off_expr: Option<Expr>,
    base_mirror_expr: Option<Expr>,
    arr_dim_exprs: Vec<Expr>,
    arr_off_expr: Option<ExprClosure>,
    arr_mirror_expr: Option<ExprClosure>,
    enc_sub_extra_data_expr: Option<Expr>,
    dec_sub_extra_data_expr: Option<Expr>,
}

#[derive(Debug)]
struct ParsedAttrs {
    errors_occurred: bool,
    docs: String,
    patbits: Option<PatBitsInfo>,
    subvar: Option<Type>,
    base_off_expr: Option<Expr>,
    base_mirror_expr: Option<Expr>,
    arr_off_expr: Option<ExprClosure>,
    arr_mirror_expr: Option<ExprClosure>,
    enc_sub_extra_data_expr: Option<Expr>,
    dec_sub_extra_data_expr: Option<Expr>,
}

// Args for the #[pat_bits] attribute macro
#[derive(Debug)]
enum PatBitsSetting {
    FragVariant(ArgWithType),
    PatVariant(ArgWithType),
    Expr(ArgWithExpr),
    StrExpr(StrArgWithExpr),
}

impl Parse for PatBitsSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::frag_variant) {
            input.parse().map(PatBitsSetting::FragVariant)
        } else if lookahead.peek(kw::pat_variant) {
            input.parse().map(PatBitsSetting::PatVariant)
        } else if lookahead.peek(LitStr) {
            input.parse().map(PatBitsSetting::StrExpr)
        } else {
            input.parse().map(PatBitsSetting::Expr)
        }
    }
}

type PatBitsSettings = Punctuated<PatBitsSetting, token::Comma>;

// Args for the #[pat_pict] attribute macro
#[derive(Debug)]
enum PatPictSetting {
    PatString(LitStr),
    FragVariant(ArgWithType),
    PatVariant(ArgWithType),
}

impl Parse for PatPictSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(PatPictSetting::PatString)
        } else if lookahead.peek(kw::frag_variant) {
            input.parse().map(PatPictSetting::FragVariant)
        } else if lookahead.peek(kw::pat_variant) {
            input.parse().map(PatPictSetting::PatVariant)
        } else {
            Err(lookahead.error())
        }
    }
}

type PatPictSettings = Punctuated<PatPictSetting, token::Comma>;

// Args for the #[frag] attribute macro
#[derive(Debug)]
enum FragSetting {
    OuterVariant(ArgWithType),
    InnerVariant(ArgWithType),
}

impl Parse for FragSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::outer_frag_variant) {
            input.parse().map(FragSetting::OuterVariant)
        } else if lookahead.peek(kw::inner_frag_variant) {
            input.parse().map(FragSetting::InnerVariant)
        } else {
            Err(lookahead.error())
        }
    }
}

type FragSettings = Punctuated<FragSetting, token::Comma>;

// Args for the #[offset], #[mirror],
// #[arr_off] and #[arr_mirror] attribute macro
#[derive(Debug)]
enum FragExprSetting {
    FragVariant(ArgWithType),
    Expr(Expr),
}

impl Parse for FragExprSetting {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::variant) {
            input.parse().map(FragExprSetting::FragVariant)
        } else if lookahead.peek(kw::frag_variant) {
            input.parse().map(FragExprSetting::FragVariant)
        } else {
            input.parse().map(FragExprSetting::Expr)
        }
    }
}

type FragExprSettings = Punctuated<FragExprSetting, token::Comma>;

fn parse_pat_bits_expr(expr: &Expr) -> Result<(bool, PatBitInfo)> {
    let mut errors_occurred = false;
    let ret = match expr {
        // just a true or false
        Expr::Lit(ExprLit{lit: Lit::Bool(b), ..}) => {
            PatBitInfo {
                invert: false,
                pos: PatBitPos::Bool(b.value),
            }
        },
        // an integer position
        Expr::Lit(ExprLit{lit: Lit::Int(i), ..}) => {
            PatBitInfo {
                invert: false,
                pos: PatBitPos::Loc(vec![i.base10_parse::<isize>()?]),
            }
        },
        // a tuple
        Expr::Tuple(t) => {
            let mut offs = Vec::new();
            for t_elem in &t.elems {
                if let Expr::Lit(ExprLit{lit: Lit::Int(i), ..}) = t_elem {
                    offs.push(i.base10_parse::<isize>()?);
                } else {
                    emit_error!(t_elem, "Invalid position expression");
                    errors_occurred = true;
                }
            }
            PatBitInfo {
                invert: false,
                pos: PatBitPos::Loc(offs),
            }
        },
        // an inversion of one of the above
        Expr::Unary(ExprUnary{op: UnOp::Not(..), expr, ..}) => {
            let (inner_errors, inner_expr) = parse_pat_bits_expr(expr)?;
            if inner_errors {
                errors_occurred = true;
            }
            PatBitInfo {
                invert: !inner_expr.invert,
                pos: inner_expr.pos,
            }
        },
        // parense
        Expr::Paren(ExprParen{expr, ..}) => {
            let (inner_errors, inner_expr) = parse_pat_bits_expr(expr)?;
            if inner_errors {
                errors_occurred = true;
            }
            inner_expr
        },
        _ => {
            emit_error!(expr, "Invalid position expression");
            errors_occurred = true;
            // dummy
            PatBitInfo {
                invert: false,
                pos: PatBitPos::Bool(false),
            }
        },
    };

    Ok((errors_occurred, ret))
}

// parsing something like this:
// A B .  C
// D . !E .
// <blank>
// F = true
// G = false
fn parse_pat_picture(pict_lit: &LitStr, idx_dims: usize) -> Option<PatBitsInfo> {
    let mut ret = PatBitsInfo::new();
    let pict_str = pict_lit.value();
    let pict_str_lines = pict_str.lines().map(|x| x.trim()).collect::<Vec<_>>();

    let mut pict_str_halves = pict_str_lines.splitn(2, |x| *x == "");
    // the main pattern
    let pict_str_bits = pict_str_halves.next().unwrap();
    // additional fixed bits
    let pict_str_extra = pict_str_halves.next();

    // decode the main picture
    for (y, l) in pict_str_bits.into_iter().enumerate() {
        for (x, bit) in l.split_whitespace().enumerate() {
            if bit != "." {
                let (invert, bit) = if bit.starts_with("!") {
                    (true, bit.split_at(1).1)
                } else {
                    (false, bit)
                };

                if idx_dims == 1 {
                    ret.insert(bit.to_owned(), PatBitInfo {
                        invert,
                        pos: PatBitPos::Loc(vec![x as isize]),
                    });
                } else {
                    ret.insert(bit.to_owned(), PatBitInfo {
                        invert,
                        pos: PatBitPos::Loc(vec![x as isize, y as isize]),
                    });
                }
            }
        }
    }

    // decode the additional bits
    if let Some(pict_str_extra) = pict_str_extra {
        for l in pict_str_extra {
            if *l != "" {
                let extra_bit_halves = l.splitn(2, '=').collect::<Vec<_>>();
                if extra_bit_halves.len() != 2 {
                    emit_error!(pict_lit, "Extra bit line \"{}\" is malformed", l);
                    return None;
                }

                let bitname = extra_bit_halves[0].trim();
                let bitval = extra_bit_halves[1].trim();
                let bitval = match bitval {
                    "true" => true,
                    "false" => false,
                    _ => {
                        emit_error!(pict_lit, "Extra bit line value \"{}\" is malformed", bitval);
                        return None;
                    }
                };

                ret.insert(bitname.to_owned(), PatBitInfo {
                    invert: false,
                    pos: PatBitPos::Bool(bitval),
                });
            }
        }
    }

    Some(ret)
}

fn parse_attrs(attrs: &mut Vec<Attribute>, encode_variant: &Option<Type>, idx_dims: usize) -> Result<ParsedAttrs> {
    let mut errors_occurred = false;
    let mut docs = String::new();
    let mut patbits = None;
    let mut subvar = None;
    let mut to_remove = Vec::new();
    let mut seen_pat = false;
    let mut seen_frag = false;
    let mut base_off_expr = None;
    let mut base_mirror_expr = None;
    let mut arr_off_expr = None;
    let mut arr_mirror_expr = None;
    let mut enc_sub_extra_data_expr = None;
    let mut dec_sub_extra_data_expr = None;

    for (i, attr) in attrs.into_iter().enumerate() {
        if attr.path.is_ident("doc") {
            let doc_meta = attr.parse_meta()?;

            if let Meta::NameValue(MetaNameValue{lit: Lit::Str(s), ..}) = doc_meta {
                if docs.len() != 0 {
                    docs.push_str(" ");
                }
                docs.push_str(s.value().trim());
            }
        }

        if attr.path.is_ident("pat_bits") {
            let parser = PatBitsSettings::parse_separated_nonempty;
            let attr_args = attr.parse_args_with(parser)?;

            // Loop through parsed list
            let mut maybe_frag_var = None;
            let mut maybe_pat_var = None;
            let mut maybe_patbits = HashMap::new();
            for attr_arg in attr_args {
                match attr_arg {
                    PatBitsSetting::FragVariant(x) => {
                        if maybe_frag_var.is_some() {
                            emit_error!(x, "Only one frag_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_frag_var = Some(x.ty);
                    },
                    PatBitsSetting::PatVariant(x) => {
                        if maybe_pat_var.is_some() {
                            emit_error!(x, "Only one pat_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_pat_var = Some(x.ty);
                    },
                    PatBitsSetting::Expr(x) => {
                        let bit_id = x.ident.to_string();
                        if maybe_patbits.contains_key(&bit_id) {
                            emit_error!(x, "Duplicate bit {} position", bit_id);
                            errors_occurred = true;
                        }

                        maybe_patbits.insert(bit_id, x.expr.clone());
                    },
                    PatBitsSetting::StrExpr(x) => {
                        let bit_id = x.litstr.value();
                        if maybe_patbits.contains_key(&bit_id) {
                            emit_error!(x, "Duplicate bit {} position", bit_id);
                            errors_occurred = true;
                        }

                        maybe_patbits.insert(bit_id, x.expr.clone());
                    },
                }
            }

            // Possibly filter by fragment variant
            if maybe_frag_var.is_none() && encode_variant.is_none() ||
                (maybe_frag_var.is_some() && encode_variant.is_some() &&
                    maybe_frag_var.as_ref().unwrap() == encode_variant.as_ref().unwrap()) {

                seen_pat = true;
                if seen_frag {
                    errors_occurred = true;
                    emit_error!(attr, "Cannot mix #[frag] with #[pat_*] attributes");
                }

                if patbits.is_some() {
                    errors_occurred = true;
                    if let Some(bitvar) = encode_variant.as_ref() {
                        emit_error!(attr, "Only one #[pat_*] attribute allowed for variant {}", quote!{#bitvar}.to_string());
                    } else {
                        emit_error!(attr, "Only one #[pat_*] attribute allowed");
                    }
                }

                let mut patbits_ = PatBitsInfo::new();

                for (bit_id, expr) in maybe_patbits {
                    let (bit_info_error, bit_info) = parse_pat_bits_expr(&expr)?;
                    if bit_info_error {
                        errors_occurred = true;
                    }
                    if let PatBitInfo{pos: PatBitPos::Loc(locs), ..} = &bit_info {
                        if locs.len() != idx_dims {
                            emit_error!(expr, "Position doesn't match dimension (expected {})", idx_dims);
                            errors_occurred = true;
                        }
                    }

                    patbits_.insert(bit_id, bit_info);
                }
                patbits = Some(patbits_);
                subvar = maybe_pat_var;
                to_remove.push(i);
            }
        }

        if attr.path.is_ident("pat_pict") {
            let parser = PatPictSettings::parse_separated_nonempty;
            let attr_args = attr.parse_args_with(parser)?;

            // Loop through parsed list
            let mut maybe_frag_var = None;
            let mut maybe_pat_var = None;
            let mut maybe_patstr = None;
            for attr_arg in attr_args {
                match attr_arg {
                    PatPictSetting::FragVariant(x) => {
                        if maybe_frag_var.is_some() {
                            emit_error!(x, "Only one frag_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_frag_var = Some(x.ty);
                    },
                    PatPictSetting::PatVariant(x) => {
                        if maybe_pat_var.is_some() {
                            emit_error!(x, "Only one pat_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_pat_var = Some(x.ty);
                    },
                    PatPictSetting::PatString(x) => {
                        if maybe_patstr.is_some() {
                            emit_error!(x, "Only one string literal allowed");
                            errors_occurred = true;
                        }
                        maybe_patstr = Some(x);
                    },
                }
            }

            // Possibly filter by fragment variant
            if maybe_frag_var.is_none() && encode_variant.is_none() ||
                (maybe_frag_var.is_some() && encode_variant.is_some() &&
                    maybe_frag_var.as_ref().unwrap() == encode_variant.as_ref().unwrap()) {

                seen_pat = true;
                if seen_frag {
                    errors_occurred = true;
                    emit_error!(attr, "Cannot mix #[frag] with #[pat_*] attributes");
                }

                if patbits.is_some() {
                    errors_occurred = true;
                    if let Some(bitvar) = encode_variant.as_ref() {
                        emit_error!(attr, "Only one #[pat_*] attribute allowed for variant {}", quote!{#bitvar}.to_string());
                    } else {
                        emit_error!(attr, "Only one #[pat_*] attribute allowed");
                    }
                }

                if idx_dims != 1 && idx_dims != 2 {
                    emit_error!(attr, "#[pat_pict] can only be used on 1- or 2-dimensional fragments");
                    errors_occurred = true;
                } else {
                    if let Some(patstr) = maybe_patstr {
                        let parsed_patbits = parse_pat_picture(&patstr, idx_dims);
                        if let Some(parsed_patbits) = parsed_patbits {
                            patbits = Some(parsed_patbits)
                        } else {
                            errors_occurred = true;
                        }
                    } else {
                        emit_error!(attr, "Missing bit pattern string literal");
                        errors_occurred = true;
                    }
                    subvar = maybe_pat_var;
                }
                to_remove.push(i);
            }
        }

        if attr.path.is_ident("frag") {
            let parser = FragSettings::parse_separated_nonempty;
            let attr_args = attr.parse_args_with(parser)?;

            // Loop through parsed list
            let mut maybe_outer_var = None;
            let mut maybe_inner_var = None;
            for attr_arg in attr_args {
                match attr_arg {
                    FragSetting::OuterVariant(x) => {
                        if maybe_outer_var.is_some() {
                            emit_error!(x, "Only one outer_frag_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_outer_var = Some(x.ty);
                    },
                    FragSetting::InnerVariant(x) => {
                        if maybe_inner_var.is_some() {
                            emit_error!(x, "Only one inner_frag_variant arg allowed");
                            errors_occurred = true;
                        }
                        maybe_inner_var = Some(x.ty);
                    },
                }
            }

            // Possibly filter by fragment variant
            if maybe_outer_var.is_none() && encode_variant.is_none() ||
                (maybe_outer_var.is_some() && encode_variant.is_some() &&
                    maybe_outer_var.as_ref().unwrap() == encode_variant.as_ref().unwrap()) {

                seen_frag = true;
                if seen_pat {
                    errors_occurred = true;
                    emit_error!(attr, "Cannot mix #[frag] with #[pat_*] attributes");
                }

                if subvar.is_some() && maybe_inner_var.is_some() {
                    errors_occurred = true;
                    if let Some(bitvar) = encode_variant.as_ref() {
                        emit_error!(attr, "Only one #[frag] attribute allowed for variant {}", quote!{#bitvar}.to_string());
                    } else {
                        emit_error!(attr, "Only one #[frag] attribute allowed");
                    }
                }

                if maybe_inner_var.is_some() {
                    subvar = maybe_inner_var;
                }
                to_remove.push(i);
            }
        }

        macro_rules! parse_frag_expr_attr {
            ($attr_name:literal, $missing_expr_err:literal, $dup_check_expr:expr, $do_stuff:expr) => {
                if attr.path.is_ident($attr_name) {
                    let parser = FragExprSettings::parse_separated_nonempty;
                    let attr_args = attr.parse_args_with(parser)?;

                    // Loop through parsed list
                    let mut maybe_frag_var = None;
                    let mut maybe_expr = None;
                    for attr_arg in attr_args {
                        match attr_arg {
                            FragExprSetting::FragVariant(x) => {
                                if maybe_frag_var.is_some() {
                                    emit_error!(x, "Only one variant arg allowed");
                                    errors_occurred = true;
                                }
                                maybe_frag_var = Some(x.ty);
                            },
                            FragExprSetting::Expr(x) => {
                                if maybe_expr.is_some() {
                                    emit_error!(x, "Only one expression arg allowed");
                                    errors_occurred = true;
                                }
                                maybe_expr = Some(x);
                            },
                        }
                    }

                    // Possibly filter by fragment variant
                    if maybe_frag_var.is_none() && encode_variant.is_none() ||
                        (maybe_frag_var.is_some() && encode_variant.is_some() &&
                            maybe_frag_var.as_ref().unwrap() == encode_variant.as_ref().unwrap()) {

                        if $dup_check_expr {
                            errors_occurred = true;
                            if let Some(bitvar) = encode_variant.as_ref() {
                                emit_error!(attr, concat!("Only one #[", $attr_name, "] attribute allowed for variant {}"),
                                    quote!{#bitvar}.to_string());
                            } else {
                                emit_error!(attr, concat!("Only one #[", $attr_name, "] attribute allowed"));
                            }
                        }

                        if let Some(expr) = maybe_expr {
                            $do_stuff(expr);
                        } else {
                            emit_error!(attr, $missing_expr_err);
                            errors_occurred = true;
                        }
                        to_remove.push(i);
                    }
                }
            };
        }

        parse_frag_expr_attr!("arr_off", "Missing offset expression", arr_off_expr.is_some(), |off_expr| {
            if let Expr::Closure(c) = off_expr {

                arr_off_expr = Some(c);
            } else {
                emit_error!(off_expr, "Offset expression must be a closure");
                errors_occurred = true;
            }
        });

        parse_frag_expr_attr!("arr_mirror", "Missing mirror expression", arr_mirror_expr.is_some(), |mirror_expr| {
            if let Expr::Closure(c) = mirror_expr {
                arr_mirror_expr = Some(c);
            } else {
                emit_error!(mirror_expr, "Mirror expression must be a closure");
                errors_occurred = true;
            }
        });

        parse_frag_expr_attr!("offset", "Missing offset expression", base_off_expr.is_some(), |off_expr| {
            base_off_expr = Some(off_expr);
        });

        parse_frag_expr_attr!("mirror", "Missing mirror expression", base_mirror_expr.is_some(), |mirror_expr| {
            base_mirror_expr = Some(mirror_expr);
        });

        parse_frag_expr_attr!("encode_sub_extra_data", "Missing extra data expression",
            enc_sub_extra_data_expr.is_some(), |extra_data_expr| {
                enc_sub_extra_data_expr = Some(extra_data_expr);
            }
        );

        parse_frag_expr_attr!("decode_sub_extra_data", "Missing extra data expression",
            dec_sub_extra_data_expr.is_some(), |extra_data_expr| {
                dec_sub_extra_data_expr = Some(extra_data_expr);
            }
        );
    }

    for i in to_remove.into_iter().rev() {
        attrs.remove(i);
    }

    if patbits.is_some() && enc_sub_extra_data_expr.is_some() {
        emit_error!(enc_sub_extra_data_expr.as_ref().unwrap(),
            "Can only have extra data on a fragment");
        errors_occurred = true;
    }

    if patbits.is_some() && dec_sub_extra_data_expr.is_some() {
        emit_error!(dec_sub_extra_data_expr.as_ref().unwrap(),
            "Can only have extra data on a fragment");
        errors_occurred = true;
    }

    Ok(ParsedAttrs {
        errors_occurred,
        docs,
        patbits,
        subvar,
        base_off_expr,
        base_mirror_expr,
        arr_off_expr,
        arr_mirror_expr,
        enc_sub_extra_data_expr,
        dec_sub_extra_data_expr,
    })
}

pub fn bitfragment(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Item);
    let input_copy = input.to_token_stream();
    let args = parse_macro_input!(args as BitFragmentSettings);

    let mut errtype = None;
    let mut encode_variant = None;
    let mut idx_dims = None;
    let mut encode_extra_type = None;
    let mut decode_extra_type = None;

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
            BitFragmentSetting::EncodeExtraType(x) => {
                if encode_extra_type.is_some() {
                    emit_error!(args.0, "Only one encode_extra_type arg allowed");
                    errors_occurred = true;
                }
                encode_extra_type = Some(x.ty.clone());
            },
            BitFragmentSetting::DecodeExtraType(x) => {
                if decode_extra_type.is_some() {
                    emit_error!(args.0, "Only one decode_extra_type arg allowed");
                    errors_occurred = true;
                }
                decode_extra_type = Some(x.ty.clone());
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

    // We really need dimensions for a bunch of stuff, so parse/unwrap/bail it now
    if idx_dims.is_none() {
        abort!(args.0, "#[bitfragment] requires dimensions to be specified");
    }
    let idx_dims = idx_dims.unwrap().base10_parse::<usize>();
    if let Err(e) = idx_dims {
        return e.to_compile_error().into();
    }
    let idx_dims = idx_dims.unwrap();

    // arg parsing done, walk over data and gather info about fields

    let obj_id;
    let field_mode;
    let mut obj_field_info = Vec::new();

    match &mut input {
        Item::Enum(enum_) => {
            obj_id = enum_.ident.clone();
            field_mode = FieldMode::Enum;

            let parsed_attrs = parse_attrs(&mut enum_.attrs, &encode_variant, idx_dims);
            if let Err(e) = parsed_attrs {
                return e.to_compile_error().into();
            }
            let parsed_attrs = parsed_attrs.unwrap();

            if parsed_attrs.errors_occurred {
                errors_occurred = true;
            }

            obj_field_info.push(FieldInfo {
                name_str: obj_id.to_string(),
                field_id: None,
                docs: parsed_attrs.docs,
                field_type_enum: BitFragmentFieldType::Pattern,
                field_type_ty: None,
                patbits: parsed_attrs.patbits,
                subvar: parsed_attrs.subvar,
                base_off_expr: parsed_attrs.base_off_expr,
                base_mirror_expr: parsed_attrs.base_mirror_expr,
                arr_dim_exprs: vec![],
                arr_off_expr: None,
                arr_mirror_expr: None,
                enc_sub_extra_data_expr: None,
                dec_sub_extra_data_expr: None,
            });
        },
        Item::Struct(struct_) => {
            obj_id = struct_.ident.clone();

            let (mode, fields) = match &mut struct_.fields {
                Fields::Named(fields) => {
                    (FieldMode::NamedStruct, &mut fields.named)
                },
                Fields::Unnamed(fields) => {
                    (FieldMode::UnnamedStruct, &mut fields.unnamed)
                },
                Fields::Unit => {
                    abort!(input, "#[bitfragment] cannot be used on a unit struct");
                }
            };

            field_mode = mode;
            for (field_i, field) in fields.iter_mut().enumerate() {
                let name_str = if let Some(id) = field.ident.as_ref() {
                    id.to_string()
                } else {
                    field_i.to_string()
                };

                let parsed_attrs = parse_attrs(&mut field.attrs, &encode_variant, idx_dims);
                if let Err(e) = parsed_attrs {
                    return e.to_compile_error().into();
                }
                let parsed_attrs = parsed_attrs.unwrap();

                if parsed_attrs.errors_occurred {
                    errors_occurred = true;
                }

                // Do we have an array?
                let mut arr_dim_exprs = Vec::new();
                let mut maybe_ty_arr = &field.ty;
                while let Type::Array(tyarr) = maybe_ty_arr {
                    arr_dim_exprs.push(tyarr.len.clone());
                    maybe_ty_arr = &*tyarr.elem;
                }
                let is_array = arr_dim_exprs.len() > 0;
                // field_type is the innermost non-array type
                let field_type = maybe_ty_arr.clone();

                // figure out what type of field this is supposed to be
                let field_type_enum;
                if is_array {
                    if parsed_attrs.patbits.is_some() {
                        field_type_enum = BitFragmentFieldType::PatternArray;
                    } else {
                        field_type_enum = BitFragmentFieldType::FragmentArray;
                    }
                } else {
                    if parsed_attrs.patbits.is_some() {
                        field_type_enum = BitFragmentFieldType::Pattern;
                    } else {
                        field_type_enum = BitFragmentFieldType::Fragment;
                    }
                }

                if is_array && parsed_attrs.arr_off_expr.is_none() {
                    emit_error!(field, "Array field must have #[arr_off] attribute");
                    errors_occurred = true;
                }

                obj_field_info.push(FieldInfo {
                    name_str,
                    field_id: field.ident.clone(),
                    docs: parsed_attrs.docs,
                    field_type_enum,
                    field_type_ty: Some(field_type),
                    patbits: parsed_attrs.patbits,
                    subvar: parsed_attrs.subvar,
                    base_off_expr: parsed_attrs.base_off_expr,
                    base_mirror_expr: parsed_attrs.base_mirror_expr,
                    arr_dim_exprs,
                    arr_off_expr: parsed_attrs.arr_off_expr,
                    arr_mirror_expr: parsed_attrs.arr_mirror_expr,
                    enc_sub_extra_data_expr: parsed_attrs.enc_sub_extra_data_expr,
                    dec_sub_extra_data_expr: parsed_attrs.dec_sub_extra_data_expr,
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

    let encode_extra_type = if encode_extra_type.is_none() {
        quote!{()}
    } else {
        quote!{#encode_extra_type}
    };

    let decode_extra_type = if decode_extra_type.is_none() {
        quote!{()}
    } else {
        quote!{#decode_extra_type}
    };

    let indexingtype = if idx_dims == 1 {
        quote!{usize}
    } else {
        quote!{[usize; #idx_dims]}
    };

    // common code
    let mut encode_decode_common = Vec::new();
    for field_info in &obj_field_info {
        // Field offset and mirroring configuration
        let base_offset_setup = if let Some(base_offset_expr) = &field_info.base_off_expr {
            quote!{
                let base_offset = (#base_offset_expr);
            }
        } else {
            quote!{
                let base_offset = [0; #idx_dims];
            }
        };

        let base_mirror_setup = if let Some(base_mirror_expr) = &field_info.base_mirror_expr {
            quote!{
                let base_mirror = (#base_mirror_expr);
            }
        } else {
            quote!{
                let base_mirror = [false; #idx_dims];
            }
        };

        let base_off_mirror_setup = quote!{
            #base_offset_setup
            #base_mirror_setup
        };

        // Array setup
        let array_setup = if let Some(arr_off_expr) = &field_info.arr_off_expr {
            // Stupid workaround for arrays that are oversize
            // calc final real index
            let mut arr_index_expr = quote!{arr_layer_0};
            for arr_dim_expr_i in &field_info.arr_dim_exprs[1..] {
                arr_index_expr = quote!{#arr_index_expr * (#arr_dim_expr_i)};
            }
            arr_index_expr = quote!{(#arr_index_expr)};
            // index the array to get that index
            let mut arr_indexing_expr = quote!{[arr_layer_0]};
            for arr_layer_i in 1..field_info.arr_dim_exprs.len() {
                let arr_layer_i_expr = Ident::new(&format!("arr_layer_{}", arr_layer_i), Span::call_site());
                arr_indexing_expr = quote!{#arr_indexing_expr[#arr_layer_i_expr]};

                let mut arr_index_layer_expr = quote!{#arr_layer_i_expr};
                for arr_dim_expr_i in &field_info.arr_dim_exprs[(arr_layer_i + 1)..] {
                    arr_index_layer_expr = quote!{#arr_index_layer_expr * (#arr_dim_expr_i)};
                }
                arr_index_layer_expr = quote!{(#arr_index_layer_expr)};

                arr_index_expr = quote!{#arr_index_expr + #arr_index_layer_expr};
            }

            let arr_elem_mirror_expr = if let Some(arr_mirror_expr) = &field_info.arr_mirror_expr {
                quote!{
                    let arr_elem_mirror = (#arr_mirror_expr)(arr_elem_i);
                }
            } else {
                quote!{
                    let arr_elem_mirror = [false; #idx_dims];
                }
            };

            quote!{
                /* let var = &field OR &mut field */ #arr_indexing_expr;
                let arr_elem_i = #arr_index_expr;
                let arr_elem_off = (#arr_off_expr)(arr_elem_i);
                #arr_elem_mirror_expr
            }
        } else {
            quote!{}
        };

        // Final subblock offset/mirroring
        let elem_off_mirror_setup = if field_info.arr_off_expr.is_some() {
            quote!{
                // offsetting
                let mut subfrag_off = [0isize; #idx_dims];
                for i in 0..#idx_dims {
                    subfrag_off[i] = offset[i];

                    // TODO: WTF IS GOING ON HERE
                    if mirror[i] ^ arr_elem_mirror[i] {
                        subfrag_off[i] -= (base_offset[i] as isize);
                    } else {
                        subfrag_off[i] += (base_offset[i] as isize);
                    }

                    if mirror[i] {
                        subfrag_off[i] -= (arr_elem_off[i] as isize);
                    } else {
                        subfrag_off[i] += (arr_elem_off[i] as isize);
                    }
                }

                // mirroring
                let mut subfrag_mirror = [false; #idx_dims];
                for i in 0..#idx_dims {
                    subfrag_mirror[i] = mirror[i] ^ base_mirror[i] ^ arr_elem_mirror[i];
                }
            }
        } else {
            quote!{
                // offsetting
                let mut subfrag_off = [0isize; #idx_dims];
                for i in 0..#idx_dims {
                    subfrag_off[i] = offset[i];

                    // TODO: WTF IS GOING ON HERE
                    if mirror[i] {
                        subfrag_off[i] -= (base_offset[i] as isize);
                    } else {
                        subfrag_off[i] += (base_offset[i] as isize);
                    }
                }

                // mirroring
                let mut subfrag_mirror = [false; #idx_dims];
                for i in 0..#idx_dims {
                    subfrag_mirror[i] = mirror[i] ^ base_mirror[i];
                }
            }
        };

        encode_decode_common.push((base_off_mirror_setup, array_setup, elem_off_mirror_setup));
    }

    let array_loopgen = |field_info: &FieldInfo, inner_body| {
        let mut for_loop = inner_body;
        for (arr_layer_i, arr_layer_dim_expr) in field_info.arr_dim_exprs.iter().enumerate().rev() {
            let arr_layer_ident = Ident::new(&format!("arr_layer_{}", arr_layer_i), Span::call_site());
            for_loop = quote!{
                for #arr_layer_ident in 0..(#arr_layer_dim_expr) {
                    #for_loop
                }
            }
        }
        for_loop
    };

    // encoding
    let mut encode_fields = Vec::new();
    for (field_i, field_info) in obj_field_info.iter().enumerate() {
        let (base_off_mirror_setup, array_setup, elem_off_mirror_setup) = &encode_decode_common[field_i];

        let get_field_ref = match field_mode {
            FieldMode::Enum => {
                quote!{let field_ref = self;}
            },
            FieldMode::NamedStruct => {
                let field_id = field_info.field_id.as_ref().unwrap();
                quote!{let field_ref = &self.#field_id;}
            },
            FieldMode::UnnamedStruct => {
                let idx = Index::from(field_i);
                quote!{let field_ref = &self.#idx;}
            },
        };

        let field_type = match field_mode {
            FieldMode::Enum => {
                quote!{Self}
            },
            FieldMode::NamedStruct | FieldMode::UnnamedStruct => {
                let field_ty = field_info.field_type_ty.as_ref().unwrap();
                quote!{#field_ty}
            },
        };

        let subvar = if let Some(subvar_ty) = &field_info.subvar {
            quote!{#subvar_ty}
        } else {
            quote!{()}
        };

        let sub_extra_data = if let Some(extra_data_expr) = &field_info.enc_sub_extra_data_expr {
            quote!{
                let sub_extra_data = (#extra_data_expr);
            }
        } else {
            quote!{
                let sub_extra_data = ();
            }
        };

        let encode_pattern = || {
            let mut encode_each_bit = Vec::new();
            for (bitname, bitinfo) in field_info.patbits.as_ref().unwrap() {
                if let PatBitPos::Loc(locs) = &bitinfo.pos {
                    let inv_token = if bitinfo.invert {quote!{!}} else {quote!{}};
                    let bitname_litstr = LitStr::new(bitname, Span::call_site());

                    let mut encode_each_dim = Vec::new();
                    for dim in 0..idx_dims {
                        let loc = locs[dim];
                        encode_each_dim.push(quote!{
                            (subfrag_off[#dim] + (if subfrag_mirror[#dim] {-1} else {1}) * #loc) as usize
                        });
                    }

                    if idx_dims == 1 {
                        let encode_dim0 = &encode_each_dim[0];
                        encode_each_bit.push(quote!{
                            fuses[#encode_dim0] =
                                #inv_token encoded_arr[<#field_type as ::bittwiddler::BitPattern<#subvar>>::_name_to_pos(#bitname_litstr)];
                        });
                    } else {
                        encode_each_bit.push(quote!{
                            fuses[[#(#encode_each_dim),*]] =
                                #inv_token encoded_arr[<#field_type as ::bittwiddler::BitPattern<#subvar>>::_name_to_pos(#bitname_litstr)];
                        });
                    }
                }
            }

            quote!{
                let encoded_arr = <#field_type as ::bittwiddler::BitPattern<#subvar>>::encode(field_ref);
                #(#encode_each_bit)*
            }
        };

        let encode_field_ref = match field_info.field_type_enum {
            BitFragmentFieldType::Pattern => {
                let encode_pattern_core = encode_pattern();
                quote!{
                    #elem_off_mirror_setup
                    #encode_pattern_core
                }
            },
            BitFragmentFieldType::Fragment => {
                quote!{
                    #elem_off_mirror_setup
                    #sub_extra_data
                    <#field_type as ::bittwiddler::BitFragment<#subvar>>::encode(
                        field_ref, fuses, subfrag_off, subfrag_mirror, sub_extra_data);
                }
            },
            BitFragmentFieldType::PatternArray => {
                let encode_pattern_core = encode_pattern();
                let encode_elem = quote!{
                    let field_ref = &field_ref #array_setup
                    #elem_off_mirror_setup
                    #encode_pattern_core
                };
                array_loopgen(field_info, encode_elem)
            },
            BitFragmentFieldType::FragmentArray => {
                let encode_elem = quote!{
                    let arr_elem = &field_ref #array_setup
                    #elem_off_mirror_setup
                    #sub_extra_data
                    <#field_type as ::bittwiddler::BitFragment<#subvar>>::encode(
                        arr_elem, fuses, subfrag_off, subfrag_mirror, sub_extra_data);
                };
                array_loopgen(field_info, encode_elem)
            },
        };

        encode_fields.push(quote!{
            {
                #base_off_mirror_setup
                #get_field_ref
                #encode_field_ref
            }
        });
    }

    // decoding
    let mut decode_field_names = Vec::new();
    let mut decode_field_vals = Vec::new();
    for (field_i, field_info) in obj_field_info.iter().enumerate() {
        let (base_off_mirror_setup, array_setup, elem_off_mirror_setup) = &encode_decode_common[field_i];

        let field_type = match field_mode {
            FieldMode::Enum => {
                quote!{Self}
            },
            FieldMode::NamedStruct | FieldMode::UnnamedStruct => {
                let field_ty = field_info.field_type_ty.as_ref().unwrap();
                quote!{#field_ty}
            },
        };

        let field_name_prefix = match field_mode {
            FieldMode::NamedStruct => {
                let field_id = field_info.field_id.as_ref().unwrap();
                quote!{#field_id: }
            },
            FieldMode::Enum | FieldMode::UnnamedStruct => {
                quote!{}
            },
        };
        decode_field_names.push(field_name_prefix);

        let subvar = if let Some(subvar_ty) = &field_info.subvar {
            quote!{#subvar_ty}
        } else {
            quote!{()}
        };

        let sub_extra_data = if let Some(extra_data_expr) = &field_info.dec_sub_extra_data_expr {
            quote!{
                let sub_extra_data = (#extra_data_expr);
            }
        } else {
            quote!{
                let sub_extra_data = ();
            }
        };

        let decode_pattern = || {
            let bitsinfo = field_info.patbits.as_ref().unwrap();
            let num_bits = bitsinfo.len();

            let mut decode_each_bit = Vec::new();
            for (bitname, bitinfo) in bitsinfo {
                let inv_token = if bitinfo.invert {quote!{!}} else {quote!{}};
                let bitname_litstr = LitStr::new(bitname, Span::call_site());
                let decode_bitval = match &bitinfo.pos {
                    PatBitPos::Loc(locs) => {
                        let mut decode_each_dim = Vec::new();
                        for dim in 0..idx_dims {
                            let loc = locs[dim];
                            decode_each_dim.push(quote!{
                                (subfrag_off[#dim] + (if subfrag_mirror[#dim] {-1} else {1}) * #loc) as usize
                            });
                        }

                        if idx_dims == 1 {
                            let decode_dim0 = &decode_each_dim[0];
                            quote!{
                                #inv_token fuses[#decode_dim0];
                            }
                        } else {
                            quote!{
                                #inv_token fuses[[#(#decode_each_dim),*]];
                            }
                        }
                    },
                    PatBitPos::Bool(b) => {
                        quote!{
                            #inv_token #b;
                        }
                    }
                };

                decode_each_bit.push(quote!{
                    decode_arr[<#field_type as ::bittwiddler::BitPattern<#subvar>>::_name_to_pos(#bitname_litstr)] = #decode_bitval
                });
            }

            quote!{
                {
                    let mut decode_arr = [false; #num_bits];
                    #(#decode_each_bit)*
                    <#field_type as ::bittwiddler::BitPattern<#subvar>>::decode(&decode_arr)?
                }
            }
        };

        let decode_field = match field_info.field_type_enum {
            BitFragmentFieldType::Pattern => {
                let decode_pattern_core = decode_pattern();

                quote!{
                    {
                        #base_off_mirror_setup
                        #elem_off_mirror_setup
                        #decode_pattern_core
                    }
                }
            },
            BitFragmentFieldType::Fragment => {
                quote!{
                    {
                        #base_off_mirror_setup
                        #elem_off_mirror_setup
                        #sub_extra_data
                        <#field_type as ::bittwiddler::BitFragment<#subvar>>::decode(
                            fuses, subfrag_off, subfrag_mirror, sub_extra_data)?
                    }
                }
            },
            BitFragmentFieldType::PatternArray => {
                let decode_pattern_core = decode_pattern();

                // Here we generate the code to loop through the array
                let decode_elem = quote!{
                    let arr_elem = &mut out_arr #array_setup
                    #elem_off_mirror_setup
                    *arr_elem = ::core::mem::MaybeUninit::new(#decode_pattern_core);
                };
                let decode_for_loops = array_loopgen(field_info, decode_elem);

                // Set up uninitialized array
                let arr_dim_expr_n = field_info.arr_dim_exprs.last();
                let mut uninit_array = quote!{
                    [::core::mem::MaybeUninit<#field_type>; #arr_dim_expr_n]
                };
                for i in (0..(field_info.arr_dim_exprs.len() - 1)).rev() {
                    let arr_dim_expr_i = &field_info.arr_dim_exprs[i];
                    uninit_array = quote!{
                        [#uninit_array; #arr_dim_expr_i]
                    }
                }

                quote!{{
                    #base_off_mirror_setup

                    let mut out_arr: #uninit_array = unsafe {
                        ::core::mem::MaybeUninit::uninit().assume_init()
                    };

                    #decode_for_loops

                    unsafe { ::core::mem::transmute::<_, _>(out_arr) }
                }}
            },
            BitFragmentFieldType::FragmentArray => {
                // Here we generate the code to loop through the array
                let decode_elem = quote!{
                    let arr_elem = &mut out_arr #array_setup
                    #elem_off_mirror_setup
                    #sub_extra_data
                    *arr_elem = ::core::mem::MaybeUninit::new(
                        <#field_type as ::bittwiddler::BitFragment<#subvar>>::decode(
                            fuses, subfrag_off, subfrag_mirror, sub_extra_data)?);
                };
                let decode_for_loops = array_loopgen(field_info, decode_elem);

                // Set up uninitialized array
                let arr_dim_expr_n = field_info.arr_dim_exprs.last();
                let mut uninit_array = quote!{
                    [::core::mem::MaybeUninit<#field_type>; #arr_dim_expr_n]
                };
                for i in (0..(field_info.arr_dim_exprs.len() - 1)).rev() {
                    let arr_dim_expr_i = &field_info.arr_dim_exprs[i];
                    uninit_array = quote!{
                        [#uninit_array; #arr_dim_expr_i]
                    }
                }

                quote!{{
                    #base_off_mirror_setup

                    let mut out_arr: #uninit_array = unsafe {
                        ::core::mem::MaybeUninit::uninit().assume_init()
                    };

                    #decode_for_loops

                    unsafe { ::core::mem::transmute::<_, _>(out_arr) }
                }}
            },
        };
        decode_field_vals.push(decode_field);
    }

    let decode_func_body = match field_mode {
        FieldMode::Enum => {
            let field0 = &decode_field_vals[0];
            quote!{#field0}
        },
        FieldMode::NamedStruct => {
            quote!{
                Self {
                    #(#decode_field_names #decode_field_vals),*
                }
            }
        },
        FieldMode::UnnamedStruct => {
            quote!{
                Self (
                    #(#decode_field_vals),*
                )
            }
        }
    };

    // for docs
    let num_fields = obj_field_info.len();
    let field_names = obj_field_info.iter().map(|x| LitStr::new(&x.name_str, Span::call_site()));
    let field_docs = obj_field_info.iter().map(|x| LitStr::new(&x.docs, Span::call_site()));
    let field_types = obj_field_info.iter().map(|x| {
        let arr_dim_total_expr = match x.field_type_enum {
            BitFragmentFieldType::Pattern | BitFragmentFieldType::Fragment => None,
            BitFragmentFieldType::PatternArray | BitFragmentFieldType::FragmentArray => {
                let expr_0 = &x.arr_dim_exprs[0];
                let mut ret = quote!{(#expr_0)};
                for expr_n in &x.arr_dim_exprs[1..] {
                    ret = quote!{(#expr_n) * #ret};
                }

                Some(ret)
            }
        };

        let fieldtype_id = match x.field_type_enum {
            BitFragmentFieldType::Pattern => quote!{Pattern},
            BitFragmentFieldType::Fragment => quote!{Fragment},
            BitFragmentFieldType::PatternArray => {
                let arr_dim_total_expr = arr_dim_total_expr.unwrap();
                quote!{PatternArray(#arr_dim_total_expr)}
            },
            BitFragmentFieldType::FragmentArray => {
                let arr_dim_total_expr = arr_dim_total_expr.unwrap();
                quote!{FragmentArray(#arr_dim_total_expr)}
            },
        };
        quote!{::bittwiddler::BitFragmentFieldType::#fieldtype_id}
    });

    // dummy for now
    let dim_zeros = (0..idx_dims).map(|_| quote!{0}).collect::<Vec<_>>();
    let dim_false = (0..idx_dims).map(|_| quote!{false}).collect::<Vec<_>>();

    let output = quote!{
        #input

        impl ::bittwiddler::BitFragment<#encode_variant> for #obj_id {
            const IDX_DIMS: usize = #idx_dims;
            type IndexingType = #indexingtype;
            type OffsettingType = [isize; #idx_dims];
            type MirroringType = [bool; #idx_dims];

            type ErrType = #errtype;

            type EncodeExtraType = #encode_extra_type;
            type DecodeExtraType = #decode_extra_type;

            const FIELD_COUNT: usize = #num_fields;

            fn encode<F>(&self, fuses: &mut F,
                offset: Self::OffsettingType, mirror: Self::MirroringType,
                extra_data: Self::EncodeExtraType)
                where F: ::core::ops::IndexMut<Self::IndexingType, Output=bool> + ?Sized {

                #(#encode_fields)*
            }
            fn decode<F>(fuses: &F,
                offset: Self::OffsettingType, mirror: Self::MirroringType,
                extra_data: Self::DecodeExtraType) -> Result<Self, Self::ErrType>
                where F: ::core::ops::Index<Self::IndexingType, Output=bool> + ?Sized {

                Ok(#decode_func_body)
            }

            #[inline]
            fn fieldname(i: usize) -> &'static str {
                [#(#field_names),*][i]
            }
            #[inline]
            fn fielddesc(i: usize) -> &'static str {
                [#(#field_docs),*][i]
            }
            #[inline]
            fn fieldtype(i: usize) -> BitFragmentFieldType {
                [#(#field_types),*][i]
            }
            #[inline]
            fn field_offset(_field_i: usize, _arr_i: usize) -> Self::OffsettingType {
                [#(#dim_zeros),*]
            }
            #[inline]
            fn field_mirror(_field_i: usize, _arr_i: usize) -> Self::MirroringType {
                [#(#dim_false),*]
            }
            #[inline]
            fn field_bits(_field_i: usize) -> usize {
                0
            }
            #[inline]
            fn field_bit_base_pos(_field_i: usize, _bit_i: usize) -> Self::OffsettingType {
                [#(#dim_zeros),*]
            }
        }
    };

    TokenStream::from(output)
}
