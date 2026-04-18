//! NDJSON Parser for lean4export format
//!
//! Parses the Lean 4 export format (version 3.1.0) as specified in:
//! https://github.com/leanprover/lean4export/blob/master/format_ndjson.md

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::ast::*;
use crate::error::{ParseError, Result};

/// Meta information at the start of an export file
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct MetaInfo {
    #[serde(rename = "exporter")]
    pub exporter: ExporterInfo,
    #[serde(rename = "lean")]
    pub lean: LeanInfo,
    #[serde(rename = "format")]
    pub format: FormatInfo,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ExporterInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LeanInfo {
    pub githash: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct FormatInfo {
    pub version: String,
}

/// A single line in the NDJSON export file
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
#[allow(non_snake_case)]
pub enum ExportLine {
    /// Initial metadata
    Meta { meta: MetaInfo },
    /// Name (str variant)
    NameStr {
        str: NameStrData,
        #[serde(rename = "in")]
        id: u32,
    },
    /// Name (num variant)
    NameNum {
        num: NameNumData,
        #[serde(rename = "in")]
        id: u32,
    },
    /// Level (succ variant)
    LevelSucc {
        succ: u32,
        #[serde(rename = "il")]
        id: u32,
    },
    /// Level (max variant)
    LevelMax {
        max: [u32; 2],
        #[serde(rename = "il")]
        id: u32,
    },
    /// Level (imax variant)
    LevelIMax {
        imax: [u32; 2],
        #[serde(rename = "il")]
        id: u32,
    },
    /// Level (param variant)
    LevelParam {
        param: u32,
        #[serde(rename = "il")]
        id: u32,
    },
    /// Expression (bound variable)
    ExprBVar {
        bvar: u32,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (sort)
    ExprSort {
        sort: u32,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (constant)
    ExprConst {
        #[serde(rename = "const")]
        data: ConstData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (application)
    ExprApp {
        app: AppData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (lambda)
    ExprLam {
        lam: LamData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (forall/pi)
    ExprForallE {
        forallE: ForallEData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (let)
    ExprLetE {
        letE: LetEData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (projection)
    ExprProj {
        proj: ProjData,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (natural literal)
    ExprNatVal {
        #[serde(rename = "natVal")]
        value: String,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Expression (string literal)
    ExprStrVal {
        #[serde(rename = "strVal")]
        value: String,
        #[serde(rename = "ie")]
        id: u32,
    },
    /// Axiom declaration
    Axiom { axiom: AxiomData },
    /// Definition declaration
    Def {
        #[serde(rename = "def")]
        data: DefData,
    },
    /// Opaque declaration
    Opaque { opaque: OpaqueData },
    /// Theorem declaration
    Thm { thm: ThmData },
    /// Quotient declaration
    Quot { quot: QuotData },
    /// Inductive declaration
    Inductive { inductive: InductiveData },
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct NameStrData {
    pub pre: u32,
    pub str: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct NameNumData {
    pub pre: u32,
    pub i: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ConstData {
    pub name: u32,
    pub us: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AppData {
    #[serde(rename = "fn")]
    pub fun: u32,
    pub arg: u32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct LamData {
    pub name: u32,
    #[serde(rename = "type")]
    pub ty: u32,
    pub body: u32,
    pub binderInfo: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct ForallEData {
    pub name: u32,
    #[serde(rename = "type")]
    pub ty: u32,
    pub body: u32,
    pub binderInfo: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct LetEData {
    pub name: u32,
    #[serde(rename = "type")]
    pub ty: u32,
    pub value: u32,
    pub body: u32,
    pub nondep: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct ProjData {
    #[serde(rename = "typeName")]
    pub type_name: u32,
    pub idx: u32,
    pub struct_expr: u32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct AxiomData {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub isUnsafe: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct DefData {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub value: u32,
    pub hints: serde_json::Value,
    pub safety: String,
    pub all: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct OpaqueData {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub value: u32,
    pub isUnsafe: bool,
    pub all: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct ThmData {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub value: u32,
    pub all: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct QuotData {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub kind: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct InductiveData {
    pub types: Vec<InductiveVal>,
    pub ctors: Vec<ConstructorVal>,
    pub recs: Vec<RecursorVal>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct InductiveVal {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub numParams: u32,
    pub numIndices: u32,
    pub all: Vec<u32>,
    pub ctors: Vec<u32>,
    pub numNested: u32,
    pub isRec: bool,
    pub isUnsafe: bool,
    pub isReflexive: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct ConstructorVal {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub induct: u32,
    pub cidx: u32,
    pub numParams: u32,
    pub numFields: u32,
    pub isUnsafe: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct RecursorVal {
    pub name: u32,
    pub levelParams: Vec<u32>,
    #[serde(rename = "type")]
    pub ty: u32,
    pub all: Vec<u32>,
    pub numParams: u32,
    pub numIndices: u32,
    pub numMotives: u32,
    pub numMinors: u32,
    pub rules: Vec<RecursorRule>,
    pub k: bool,
    pub isUnsafe: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct RecursorRule {
    pub ctor: u32,
    pub nfields: u32,
    pub rhs: u32,
}

/// Parses an NDJSON export file and returns the parsed lines
///
/// The meta information is extracted separately and not included in the lines vector
/// to avoid double-processing.
pub fn parse_ndjson_file(path: impl AsRef<Path>) -> Result<(Option<MetaInfo>, Vec<ExportLine>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut meta: Option<MetaInfo> = None;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parsed: ExportLine =
            serde_json::from_str(trimmed).map_err(|e| ParseError::JsonParse {
                line: line_num + 1,
                message: e.to_string(),
            })?;

        // Extract meta separately and don't add Meta lines to the lines vector
        // to avoid double-processing in ParserState::process_line
        if let ExportLine::Meta { meta: m } = &parsed {
            meta = Some(m.clone());
            continue; // Skip adding to lines
        }

        lines.push(parsed);
    }

    Ok((meta, lines))
}

/// Parser state that accumulates parsed entities and resolves references
pub struct ParserState {
    /// Parsed names by their ID
    pub names: HashMap<u32, ParsedName>,
    /// Parsed levels by their ID
    pub levels: HashMap<u32, ParsedLevel>,
    /// Parsed expressions by their ID
    pub expressions: HashMap<u32, ParsedExpr>,
    /// Parsed declarations
    pub declarations: Vec<Declaration>,
    /// Meta information
    pub meta: Option<MetaInfo>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            levels: HashMap::new(),
            expressions: HashMap::new(),
            declarations: Vec::new(),
            meta: None,
        }
    }

    /// Process a single export line and update the state
    pub fn process_line(&mut self, line: ExportLine) -> Result<()> {
        match line {
            ExportLine::Meta { meta } => {
                self.meta = Some(meta);
            }
            ExportLine::NameStr { str, id } => {
                let name = ParsedName::Str {
                    pre: str.pre,
                    str: str.str,
                };
                self.names.insert(id, name);
            }
            ExportLine::NameNum { num, id } => {
                let name = ParsedName::Num {
                    pre: num.pre,
                    i: num.i,
                };
                self.names.insert(id, name);
            }
            ExportLine::LevelSucc { succ, id } => {
                let level = ParsedLevel::Succ(succ);
                self.levels.insert(id, level);
            }
            ExportLine::LevelMax { max, id } => {
                let level = ParsedLevel::Max(max[0], max[1]);
                self.levels.insert(id, level);
            }
            ExportLine::LevelIMax { imax, id } => {
                let level = ParsedLevel::IMax(imax[0], imax[1]);
                self.levels.insert(id, level);
            }
            ExportLine::LevelParam { param, id } => {
                let level = ParsedLevel::Param(param);
                self.levels.insert(id, level);
            }
            ExportLine::ExprBVar { bvar, id } => {
                let expr = ParsedExpr::BVar(bvar as usize);
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprSort { sort, id } => {
                let expr = ParsedExpr::Sort(sort);
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprConst { data, id } => {
                let expr = ParsedExpr::Const {
                    name_id: data.name,
                    universe_params: data.us,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprApp { app, id } => {
                let expr = ParsedExpr::App {
                    fun_id: app.fun,
                    arg_id: app.arg,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprLam { lam, id } => {
                let expr = ParsedExpr::Lam {
                    name_id: lam.name,
                    type_id: lam.ty,
                    body_id: lam.body,
                    binder_info: lam.binderInfo,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprForallE { forallE, id } => {
                let expr = ParsedExpr::ForallE {
                    name_id: forallE.name,
                    type_id: forallE.ty,
                    body_id: forallE.body,
                    binder_info: forallE.binderInfo,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprLetE { letE, id } => {
                let expr = ParsedExpr::LetE {
                    name_id: letE.name,
                    type_id: letE.ty,
                    value_id: letE.value,
                    body_id: letE.body,
                    nondep: letE.nondep,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprProj { proj, id } => {
                let expr = ParsedExpr::Proj {
                    type_name_id: proj.type_name,
                    idx: proj.idx,
                    struct_id: proj.struct_expr,
                };
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprNatVal { value, id } => {
                let expr = ParsedExpr::NatLit(value);
                self.expressions.insert(id, expr);
            }
            ExportLine::ExprStrVal { value, id } => {
                let expr = ParsedExpr::StrLit(value);
                self.expressions.insert(id, expr);
            }
            ExportLine::Axiom { axiom: data } => {
                self.declarations.push(Declaration::Axiom(data));
            }
            ExportLine::Def { data } => {
                self.declarations.push(Declaration::Def(data));
            }
            ExportLine::Opaque { opaque: data } => {
                self.declarations.push(Declaration::Opaque(data));
            }
            ExportLine::Thm { thm: data } => {
                self.declarations.push(Declaration::Thm(data));
            }
            ExportLine::Quot { quot: data } => {
                self.declarations.push(Declaration::Quot(data));
            }
            ExportLine::Inductive { inductive: data } => {
                self.declarations.push(Declaration::Inductive(data));
            }
        }
        Ok(())
    }

    /// Parse an entire NDJSON file and populate the state
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self> {
        let (meta, lines) = parse_ndjson_file(path)?;
        let mut state = Self::new();
        state.meta = meta;

        for line in lines {
            state.process_line(line)?;
        }

        Ok(state)
    }
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
}
