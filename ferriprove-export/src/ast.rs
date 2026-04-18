//! AST types for parsed lean4export format
//!
//! These types represent the raw parsed data with integer references.
//! They are later resolved to ferriprove-types.

/// Parsed name with integer references
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedName {
    /// Anonymous name (referenced as ID 0 implicitly)
    Anon,
    /// String component: pre points to previous name, str is the component
    Str { pre: u32, str: String },
    /// Numeric component: pre points to previous name, i is the numeric index
    Num { pre: u32, i: i64 },
}

/// Parsed level with integer references
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedLevel {
    /// Zero level (not explicitly in export, but needed for resolution)
    Zero,
    /// Successor: argument is the ID of the underlying level
    Succ(u32),
    /// Maximum of two levels
    Max(u32, u32),
    /// Impredicative maximum
    IMax(u32, u32),
    /// Universe parameter: argument is the name ID
    Param(u32),
}

/// Parsed expression with integer references
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedExpr {
    /// Bound variable with de Bruijn index
    BVar(usize),
    /// Sort/Type universe: argument is the level ID
    Sort(u32),
    /// Constant reference with universe parameters
    Const {
        name_id: u32,
        universe_params: Vec<u32>,
    },
    /// Function application
    App { fun_id: u32, arg_id: u32 },
    /// Lambda abstraction
    Lam {
        name_id: u32,
        type_id: u32,
        body_id: u32,
        binder_info: String,
    },
    /// Forall/Pi type
    ForallE {
        name_id: u32,
        type_id: u32,
        body_id: u32,
        binder_info: String,
    },
    /// Let binding
    LetE {
        name_id: u32,
        type_id: u32,
        value_id: u32,
        body_id: u32,
        nondep: bool,
    },
    /// Projection
    Proj {
        type_name_id: u32,
        idx: u32,
        struct_id: u32,
    },
    /// Natural number literal
    NatLit(String),
    /// String literal
    StrLit(String),
}

/// Parsed declaration (raw with integer references)
#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Axiom(crate::parser::AxiomData),
    Def(crate::parser::DefData),
    Opaque(crate::parser::OpaqueData),
    Thm(crate::parser::ThmData),
    Quot(crate::parser::QuotData),
    Inductive(crate::parser::InductiveData),
}
