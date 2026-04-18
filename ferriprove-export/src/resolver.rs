//! Resolver module - converts parsed entities to ferriprove-types
//!
//! This module handles:
//! - Resolving integer references to actual values
//! - Converting parsed AST to ferriprove-types types
//! - Handling forward references by building fully resolved structures

use ferriprove_types::{BinderInfo, Expr, Level, Literal, Name};
use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ParseError, Result};
use crate::parser::ParserState;

/// Resolves parsed names to ferriprove-types Names
pub struct NameResolver<'a> {
    state: &'a ParserState,
    cache: HashMap<u32, Name>,
    resolving: HashMap<u32, bool>, // Track in-progress resolutions for cycle detection
}

impl<'a> NameResolver<'a> {
    pub fn new(state: &'a ParserState) -> Self {
        Self {
            state,
            cache: HashMap::new(),
            resolving: HashMap::new(),
        }
    }

    /// Resolve a name by its ID
    pub fn resolve(&mut self, id: u32) -> Result<Name> {
        // Check cache first
        if let Some(name) = self.cache.get(&id) {
            return Ok(name.clone());
        }

        // Special case: 0 is the anonymous name
        if id == 0 {
            return Ok(Name::new("[anonymous]"));
        }

        // Check for cycles
        if self.resolving.get(&id) == Some(&true) {
            return Err(ParseError::ResolutionCycle {
                kind: "name".to_string(),
                id,
            });
        }

        // Mark as in-progress
        self.resolving.insert(id, true);

        // Look up the parsed name
        let parsed = self
            .state
            .names
            .get(&id)
            .ok_or(ParseError::MissingName { id })?
            .clone();

        let name = match parsed {
            ParsedName::Anon => Name::new("[anonymous]"),
            ParsedName::Str { pre, str } => {
                let prefix = if pre == 0 {
                    String::new()
                } else {
                    format!("{}.", self.resolve(pre)?.as_str())
                };
                Name::new(format!("{}{}", prefix, str))
            }
            ParsedName::Num { pre, i } => {
                let prefix = if pre == 0 {
                    String::new()
                } else {
                    format!("{}.", self.resolve(pre)?.as_str())
                };
                Name::new(format!("{}{}", prefix, i))
            }
        };

        // Cache and clear in-progress flag
        self.resolving.insert(id, false);
        self.cache.insert(id, name.clone());
        Ok(name)
    }
}

/// Resolves parsed levels to ferriprove-types Levels
pub struct LevelResolver<'a> {
    state: &'a ParserState,
    name_resolver: NameResolver<'a>,
    cache: HashMap<u32, Level>,
    resolving: HashMap<u32, bool>, // Track in-progress resolutions for cycle detection
}

impl<'a> LevelResolver<'a> {
    pub fn new(state: &'a ParserState) -> Self {
        Self {
            state,
            name_resolver: NameResolver::new(state),
            cache: HashMap::new(),
            resolving: HashMap::new(),
        }
    }

    /// Resolve a level by its ID
    pub fn resolve(&mut self, id: u32) -> Result<Level> {
        // Check cache first
        if let Some(level) = self.cache.get(&id) {
            return Ok(level.clone());
        }

        // Check for cycles
        if self.resolving.get(&id) == Some(&true) {
            return Err(ParseError::ResolutionCycle {
                kind: "level".to_string(),
                id,
            });
        }

        // Special case: 0 is the zero level
        if id == 0 {
            return Ok(Level::Zero);
        }

        // Mark as in-progress
        self.resolving.insert(id, true);

        // Look up the parsed level
        let parsed = self
            .state
            .levels
            .get(&id)
            .ok_or(ParseError::MissingLevel { id })?
            .clone();

        let level = match parsed {
            ParsedLevel::Zero => Level::Zero,
            ParsedLevel::Succ(inner_id) => {
                let inner = self.resolve(inner_id)?;
                Level::Succ(Box::new(inner))
            }
            ParsedLevel::Max(id1, id2) => {
                let l1 = self.resolve(id1)?;
                let l2 = self.resolve(id2)?;
                Level::Max(Box::new(l1), Box::new(l2))
            }
            ParsedLevel::IMax(id1, id2) => {
                let l1 = self.resolve(id1)?;
                let l2 = self.resolve(id2)?;
                Level::IMax(Box::new(l1), Box::new(l2))
            }
            ParsedLevel::Param(name_id) => {
                let name = self.name_resolver.resolve(name_id)?;
                Level::Param(name)
            }
        };

        // Cache and clear in-progress flag
        self.resolving.insert(id, false);
        self.cache.insert(id, level.clone());
        Ok(level)
    }

    /// Resolve multiple levels by their IDs
    pub fn resolve_many(&mut self, ids: &[u32]) -> Result<Vec<Level>> {
        ids.iter().map(|&id| self.resolve(id)).collect()
    }
}

/// Resolves parsed expressions to ferriprove-types Expressions
pub struct ExprResolver<'a> {
    state: &'a ParserState,
    level_resolver: LevelResolver<'a>,
    cache: HashMap<u32, Expr>,
    resolving: HashMap<u32, bool>, // Track in-progress resolutions for cycle detection
}

impl<'a> ExprResolver<'a> {
    pub fn new(state: &'a ParserState) -> Self {
        Self {
            state,
            level_resolver: LevelResolver::new(state),
            cache: HashMap::new(),
            resolving: HashMap::new(),
        }
    }

    /// Parse binder info string to BinderInfo enum
    fn parse_binder_info(&self, s: &str) -> Result<BinderInfo> {
        match s {
            "default" => Ok(BinderInfo::Default),
            "implicit" => Ok(BinderInfo::Implicit),
            "strictImplicit" => Ok(BinderInfo::StrictImplicit),
            "instImplicit" => Ok(BinderInfo::InstImplicit),
            _ => Err(ParseError::InvalidBinderInfo(s.to_string())),
        }
    }

    /// Resolve an expression by its ID
    pub fn resolve(&mut self, id: u32) -> Result<Expr> {
        // Check cache first
        if let Some(expr) = self.cache.get(&id) {
            return Ok(expr.clone());
        }

        // Check for cycles
        if self.resolving.get(&id) == Some(&true) {
            return Err(ParseError::ResolutionCycle {
                kind: "expression".to_string(),
                id,
            });
        }

        // Mark as in-progress
        self.resolving.insert(id, true);

        // Look up the parsed expression
        let parsed = self
            .state
            .expressions
            .get(&id)
            .ok_or(ParseError::MissingExpr { id })?
            .clone();

        let expr = match parsed {
            ParsedExpr::BVar(idx) => Expr::Var(idx),
            ParsedExpr::Sort(level_id) => {
                let level = self.level_resolver.resolve(level_id)?;
                Expr::Sort(level)
            }
            ParsedExpr::Const {
                name_id,
                universe_params,
            } => {
                let name = self.level_resolver.name_resolver.resolve(name_id)?;
                let levels = self.level_resolver.resolve_many(&universe_params)?;
                Expr::Const(name, levels)
            }
            ParsedExpr::App { fun_id, arg_id } => {
                let fun_expr = self.resolve(fun_id)?;
                let arg_expr = self.resolve(arg_id)?;
                Expr::app(fun_expr, arg_expr)
            }
            ParsedExpr::Lam {
                name_id: _,
                type_id,
                body_id,
                binder_info,
            } => {
                let binder = self.parse_binder_info(&binder_info)?;
                let domain = self.resolve(type_id)?;
                let body = self.resolve(body_id)?;
                Expr::lam(binder, domain, body)
            }
            ParsedExpr::ForallE {
                name_id: _,
                type_id,
                body_id,
                binder_info,
            } => {
                let binder = self.parse_binder_info(&binder_info)?;
                let domain = self.resolve(type_id)?;
                let codomain = self.resolve(body_id)?;
                Expr::pi(binder, domain, codomain)
            }
            ParsedExpr::LetE {
                name_id: _,
                type_id,
                value_id,
                body_id,
                nondep: _,
            } => {
                let binder = BinderInfo::Default;
                let type_expr = self.resolve(type_id)?;
                let value = self.resolve(value_id)?;
                let body = self.resolve(body_id)?;
                Expr::let_(binder, type_expr, value, body)
            }
            ParsedExpr::Proj {
                type_name_id,
                idx,
                struct_id,
            } => {
                // Projections are not directly supported in ferriprove-types yet
                // For now, we encode them as a constant application
                let type_name = self.level_resolver.name_resolver.resolve(type_name_id)?;
                let struct_expr = self.resolve(struct_id)?;
                // Proj is encoded as: Proj type_name idx struct
                // We'll need to add this to ferriprove-types Expr enum
                // For now, treat as application of a projection constant
                let proj_const =
                    Expr::const_(Name::new(format!("{}.proj.{}", type_name.as_str(), idx)));
                Expr::app(proj_const, struct_expr)
            }
            ParsedExpr::NatLit(value_str) => {
                let value: u64 = value_str.parse().map_err(|_| {
                    ParseError::InvalidBinderInfo(format!(
                        "nat literal out of u64 range or invalid: {}",
                        value_str
                    ))
                })?;
                Expr::lit(Literal::Nat(value))
            }
            ParsedExpr::StrLit(value) => Expr::lit(Literal::string(value)),
        };

        // Cache and clear in-progress flag
        self.resolving.insert(id, false);
        self.cache.insert(id, expr.clone());
        Ok(expr)
    }

    /// Resolve multiple expressions by their IDs
    pub fn resolve_many(&mut self, ids: &[u32]) -> Result<Vec<Expr>> {
        ids.iter().map(|&id| self.resolve(id)).collect()
    }
}

/// Fully resolved declaration ready for use in the kernel
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedDeclaration {
    Axiom {
        name: Name,
        level_params: Vec<Name>,
        ty: Expr,
        is_unsafe: bool,
    },
    Definition {
        name: Name,
        level_params: Vec<Name>,
        ty: Expr,
        value: Expr,
        hints: DefinitionHints,
        safety: Safety,
    },
    Opaque {
        name: Name,
        level_params: Vec<Name>,
        ty: Expr,
        value: Expr,
        is_unsafe: bool,
    },
    Theorem {
        name: Name,
        level_params: Vec<Name>,
        ty: Expr,
        value: Expr,
    },
    Quot {
        name: Name,
        level_params: Vec<Name>,
        ty: Expr,
        kind: QuotKind,
    },
    Inductive {
        types: Vec<ResolvedInductiveType>,
        constructors: Vec<ResolvedConstructor>,
        recursors: Vec<ResolvedRecursor>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionHints {
    Opaque,
    Abbrev,
    Regular(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Safety {
    Unsafe,
    Safe,
    Partial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotKind {
    Type,
    Ctor,
    Lift,
    Ind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedInductiveType {
    pub name: Name,
    pub level_params: Vec<Name>,
    pub ty: Expr,
    pub num_params: u32,
    pub num_indices: u32,
    pub constructors: Vec<Name>,
    pub is_recursive: bool,
    pub is_unsafe: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedConstructor {
    pub name: Name,
    pub level_params: Vec<Name>,
    pub ty: Expr,
    pub inductive: Name,
    pub index: u32,
    pub num_params: u32,
    pub num_fields: u32,
    pub is_unsafe: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedRecursor {
    pub name: Name,
    pub level_params: Vec<Name>,
    pub ty: Expr,
    pub num_params: u32,
    pub num_indices: u32,
    pub num_motives: u32,
    pub num_minors: u32,
    pub rules: Vec<RecursorRule>,
    pub is_k: bool,
    pub is_unsafe: bool,
}

/// Resolves declarations to fully resolved form
pub struct DeclarationResolver<'a> {
    expr_resolver: ExprResolver<'a>,
}

impl<'a> DeclarationResolver<'a> {
    pub fn new(state: &'a ParserState) -> Self {
        Self {
            expr_resolver: ExprResolver::new(state),
        }
    }

    fn resolve_level_params(&mut self, ids: &[u32]) -> Result<Vec<Name>> {
        ids.iter()
            .map(|&id| self.expr_resolver.level_resolver.name_resolver.resolve(id))
            .collect()
    }

    fn parse_hints(&self, hints: &serde_json::Value) -> Result<DefinitionHints> {
        match hints {
            serde_json::Value::String(s) => match s.as_str() {
                "opaque" => Ok(DefinitionHints::Opaque),
                "abbrev" => Ok(DefinitionHints::Abbrev),
                _ => Err(ParseError::InvalidHint(s.clone())),
            },
            serde_json::Value::Object(obj) => {
                if let Some(regular) = obj.get("regular") {
                    if let Some(n) = regular.as_u64() {
                        Ok(DefinitionHints::Regular(n as u32))
                    } else {
                        Err(ParseError::InvalidHint(format!("{:?}", hints)))
                    }
                } else {
                    Err(ParseError::InvalidHint(format!("{:?}", hints)))
                }
            }
            _ => Err(ParseError::InvalidHint(format!("{:?}", hints))),
        }
    }

    fn parse_safety(&self, safety: &str) -> Result<Safety> {
        match safety {
            "unsafe" => Ok(Safety::Unsafe),
            "safe" => Ok(Safety::Safe),
            "partial" => Ok(Safety::Partial),
            _ => Err(ParseError::InvalidSafety(safety.to_string())),
        }
    }

    fn parse_quot_kind(&self, kind: &str) -> Result<QuotKind> {
        match kind {
            "type" => Ok(QuotKind::Type),
            "ctor" => Ok(QuotKind::Ctor),
            "lift" => Ok(QuotKind::Lift),
            "ind" => Ok(QuotKind::Ind),
            _ => Err(ParseError::InvalidQuotKind(kind.to_string())),
        }
    }

    /// Resolve a declaration
    pub fn resolve_declaration(&mut self, decl: &Declaration) -> Result<ResolvedDeclaration> {
        match decl {
            Declaration::Axiom(data) => {
                let name = self
                    .expr_resolver
                    .level_resolver
                    .name_resolver
                    .resolve(data.name)?;
                let level_params = self.resolve_level_params(&data.levelParams)?;
                let ty = self.expr_resolver.resolve(data.ty)?;
                Ok(ResolvedDeclaration::Axiom {
                    name,
                    level_params,
                    ty,
                    is_unsafe: data.isUnsafe,
                })
            }
            Declaration::Def(data) => {
                let name = self
                    .expr_resolver
                    .level_resolver
                    .name_resolver
                    .resolve(data.name)?;
                let level_params = self.resolve_level_params(&data.levelParams)?;
                let ty = self.expr_resolver.resolve(data.ty)?;
                let value = self.expr_resolver.resolve(data.value)?;
                let hints = self.parse_hints(&data.hints)?;
                let safety = self.parse_safety(&data.safety)?;
                Ok(ResolvedDeclaration::Definition {
                    name,
                    level_params,
                    ty,
                    value,
                    hints,
                    safety,
                })
            }
            Declaration::Opaque(data) => {
                let name = self
                    .expr_resolver
                    .level_resolver
                    .name_resolver
                    .resolve(data.name)?;
                let level_params = self.resolve_level_params(&data.levelParams)?;
                let ty = self.expr_resolver.resolve(data.ty)?;
                let value = self.expr_resolver.resolve(data.value)?;
                Ok(ResolvedDeclaration::Opaque {
                    name,
                    level_params,
                    ty,
                    value,
                    is_unsafe: data.isUnsafe,
                })
            }
            Declaration::Thm(data) => {
                let name = self
                    .expr_resolver
                    .level_resolver
                    .name_resolver
                    .resolve(data.name)?;
                let level_params = self.resolve_level_params(&data.levelParams)?;
                let ty = self.expr_resolver.resolve(data.ty)?;
                let value = self.expr_resolver.resolve(data.value)?;
                Ok(ResolvedDeclaration::Theorem {
                    name,
                    level_params,
                    ty,
                    value,
                })
            }
            Declaration::Quot(data) => {
                let name = self
                    .expr_resolver
                    .level_resolver
                    .name_resolver
                    .resolve(data.name)?;
                let level_params = self.resolve_level_params(&data.levelParams)?;
                let ty = self.expr_resolver.resolve(data.ty)?;
                let kind = self.parse_quot_kind(&data.kind)?;
                Ok(ResolvedDeclaration::Quot {
                    name,
                    level_params,
                    ty,
                    kind,
                })
            }
            Declaration::Inductive(data) => {
                let mut types = Vec::new();
                let mut constructors = Vec::new();
                let mut recursors = Vec::new();

                for type_val in &data.types {
                    let name = self
                        .expr_resolver
                        .level_resolver
                        .name_resolver
                        .resolve(type_val.name)?;
                    let level_params = self.resolve_level_params(&type_val.levelParams)?;
                    let ty = self.expr_resolver.resolve(type_val.ty)?;
                    let ctor_names: Vec<Name> = type_val
                        .ctors
                        .iter()
                        .map(|&id| self.expr_resolver.level_resolver.name_resolver.resolve(id))
                        .collect::<Result<_>>()?;

                    types.push(ResolvedInductiveType {
                        name,
                        level_params,
                        ty,
                        num_params: type_val.numParams,
                        num_indices: type_val.numIndices,
                        constructors: ctor_names,
                        is_recursive: type_val.isRec,
                        is_unsafe: type_val.isUnsafe,
                    });
                }

                for ctor_val in &data.ctors {
                    let name = self
                        .expr_resolver
                        .level_resolver
                        .name_resolver
                        .resolve(ctor_val.name)?;
                    let level_params = self.resolve_level_params(&ctor_val.levelParams)?;
                    let ty = self.expr_resolver.resolve(ctor_val.ty)?;
                    let inductive = self
                        .expr_resolver
                        .level_resolver
                        .name_resolver
                        .resolve(ctor_val.induct)?;

                    constructors.push(ResolvedConstructor {
                        name,
                        level_params,
                        ty,
                        inductive,
                        index: ctor_val.cidx,
                        num_params: ctor_val.numParams,
                        num_fields: ctor_val.numFields,
                        is_unsafe: ctor_val.isUnsafe,
                    });
                }

                for rec_val in &data.recs {
                    let name = self
                        .expr_resolver
                        .level_resolver
                        .name_resolver
                        .resolve(rec_val.name)?;
                    let level_params = self.resolve_level_params(&rec_val.levelParams)?;
                    let ty = self.expr_resolver.resolve(rec_val.ty)?;

                    let rules: Vec<RecursorRule> = rec_val
                        .rules
                        .iter()
                        .map(|rule| {
                            let ctor = self
                                .expr_resolver
                                .level_resolver
                                .name_resolver
                                .resolve(rule.ctor)?;
                            let rhs = self.expr_resolver.resolve(rule.rhs)?;
                            Ok(RecursorRule {
                                ctor,
                                num_fields: rule.nfields,
                                rhs,
                            })
                        })
                        .collect::<Result<_>>()?;

                    recursors.push(ResolvedRecursor {
                        name,
                        level_params,
                        ty,
                        num_params: rec_val.numParams,
                        num_indices: rec_val.numIndices,
                        num_motives: rec_val.numMotives,
                        num_minors: rec_val.numMinors,
                        rules,
                        is_k: rec_val.k,
                        is_unsafe: rec_val.isUnsafe,
                    });
                }

                Ok(ResolvedDeclaration::Inductive {
                    types,
                    constructors,
                    recursors,
                })
            }
        }
    }

    /// Resolve all declarations in the parser state
    pub fn resolve_all(&mut self, state: &'a ParserState) -> Result<Vec<ResolvedDeclaration>> {
        state
            .declarations
            .iter()
            .map(|decl| self.resolve_declaration(decl))
            .collect()
    }
}

/// A fully resolved recursor rule
#[derive(Debug, Clone, PartialEq)]
pub struct RecursorRule {
    pub ctor: Name,
    pub num_fields: u32,
    pub rhs: Expr,
}
