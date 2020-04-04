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

use quote::*;
use syn::*;
use syn::parse::*;

pub mod kw {
    syn::custom_keyword!(default);
    syn::custom_keyword!(errtype);
    syn::custom_keyword!(bitnames);
    syn::custom_keyword!(variant);
    syn::custom_keyword!(frag_variant);
    syn::custom_keyword!(pat_variant);
    syn::custom_keyword!(dimensions);
    syn::custom_keyword!(outer_frag_variant);
    syn::custom_keyword!(inner_frag_variant);
    syn::custom_keyword!(encode_extra_type);
    syn::custom_keyword!(decode_extra_type);
}

#[derive(Debug)]
pub struct ArgWithExpr {
    pub ident: Ident,
    _eq: syn::token::Eq,
    pub expr: Expr,
}

impl Parse for ArgWithExpr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(ArgWithExpr {
            ident: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for ArgWithExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens);
        self._eq.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct StrArgWithExpr {
    pub litstr: LitStr,
    _eq: syn::token::Eq,
    pub expr: Expr,
}

impl Parse for StrArgWithExpr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(StrArgWithExpr {
            litstr: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for StrArgWithExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.litstr.to_tokens(tokens);
        self._eq.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct ArgWithType {
    _ident: Ident,
    _eq: syn::token::Eq,
    pub ty: Type,
}

impl Parse for ArgWithType {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(ArgWithType {
            _ident: input.parse()?,
            _eq: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl ToTokens for ArgWithType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._ident.to_tokens(tokens);
        self._eq.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct ArgWithLitStr {
    _ident: Ident,
    _eq: syn::token::Eq,
    pub litstr: LitStr,
}

impl Parse for ArgWithLitStr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(ArgWithLitStr {
            _ident: input.parse()?,
            _eq: input.parse()?,
            litstr: input.parse()?,
        })
    }
}

impl ToTokens for ArgWithLitStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._ident.to_tokens(tokens);
        self._eq.to_tokens(tokens);
        self.litstr.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct ArgWithLitInt {
    _ident: Ident,
    _eq: syn::token::Eq,
    pub litint: LitInt,
}

impl Parse for ArgWithLitInt {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(ArgWithLitInt {
            _ident: input.parse()?,
            _eq: input.parse()?,
            litint: input.parse()?,
        })
    }
}

impl ToTokens for ArgWithLitInt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._ident.to_tokens(tokens);
        self._eq.to_tokens(tokens);
        self.litint.to_tokens(tokens);
    }
}
