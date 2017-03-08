// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// at http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Format {
    Csv,
    Json,
    JsonApi,
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match *self {
            Format::Csv => ".csv",
            Format::Json | Format::JsonApi => ".json",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Analysis {
    kind: Format,
    prelude: Option<CratePreludeData>,
    imports: Vec<Import>,
    defs: Vec<Def>,
    refs: Vec<Ref>,
    macro_refs: Vec<MacroRef>,
    relations: Vec<Relation>,
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {
            kind: Format::Json,
            prelude: None,
            imports: vec![],
            defs: vec![],
            refs: vec![],
            macro_refs: vec![],
            relations: vec![],
        }
    }
}

// DefId::index is a newtype and so the JSON serialisation is ugly. Therefore
// we use our own Id which is the same, but without the newtype.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Id {
    krate: u32,
    index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpanData {
    pub file_name: String,
    pub byte_start: u32,
    pub byte_end: u32,
    /// 1-based.
    pub line_start: usize,
    pub line_end: usize,
    /// 1-based, character offset.
    pub column_start: usize,
    pub column_end: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CratePreludeData {
    pub crate_name: String,
    pub crate_root: String,
    pub external_crates: Vec<ExternalCrateData>,
    pub span: SpanData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalCrateData {
    pub name: String,
    pub num: u32,
    pub file_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Import {
    kind: ImportKind,
    ref_id: Option<Id>,
    span: SpanData,
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ImportKind {
    ExternCrate,
    Use,
    GlobUse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Def {
    kind: DefKind,
    id: Id,
    span: SpanData,
    name: String,
    qualname: String,
    value: String,
    children: Vec<Id>,
    decl_id: Option<Id>,
    docs: String,
    sig: Option<Signature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DefKind {
    // value = variant names
    Enum,
    // value = enum name + variant name + types
    Tuple,
    // value = [enum name +] name + fields
    Struct,
    // value = signature
    Trait,
    // value = type + generics
    Function,
    // value = type + generics
    Method,
    // No id, no value.
    Macro,
    // value = file_name
    Mod,
    // value = aliased type
    Type,
    // value = type and init expression (for all variable kinds).
    Local,
    Static,
    Const,
    Field,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ref {
    kind: RefKind,
    span: SpanData,
    ref_id: Id,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RefKind {
    Function,
    Mod,
    Type,
    Variable,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MacroRef {
    span: SpanData,
    qualname: String,
    callee_span: SpanData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Relation {
    span: SpanData,
    kind: RelationKind,
    from: Id,
    to: Id,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RelationKind {
    Impl,
    SuperTrait,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    span: SpanData,
    text: String,
    ident_start: usize,
    ident_end: usize,
    defs: Vec<SigElement>,
    refs: Vec<SigElement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SigElement {
    id: Id,
    start: usize,
    end: usize,
}
