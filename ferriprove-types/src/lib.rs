//! Ferriprove Types
//! 
//! Core types for Lean 4-compatible type checking.
//! 
//! This crate implements the foundational types used throughout the Ferriprove
//! type checker, following Lean 4's kernel language design.

use std::sync::Arc;
use std::hash::{Hash, Hasher};
use rustc_hash::FxHashMap;

/// Unique identifier for interned expressions
pub type ExprId = u32;

/// Unique identifier for free variables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FVarId(pub u32);

/// Unique identifier for metavariables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MVarId(pub u32);

/// Unique identifier for level metavariables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelMVarId(pub u32);

/// Hierarchical dotted names, interned by content
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(Arc<str>);

impl Name {
    pub fn new(s: impl Into<Arc<str>>) -> Self {
        Name(s.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name::new(s)
    }
}

/// Universe levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Level {
    /// Zero level: Type 0
    Zero,
    /// Successor level: l + 1
    Succ(Box<Level>),
    /// Maximum of two levels: max(l1, l2)
    Max(Box<Level>, Box<Level>),
    /// Impredicative maximum: imax(l1, l2) = max(0, l2) if l1 = 0 else l1
    IMax(Box<Level>, Box<Level>),
    /// Universe parameter
    Param(Name),
    /// Metavariable
    MVar(LevelMVarId),
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Nat(u64),
    String(Arc<str>),
}

impl Literal {
    pub fn string(s: impl Into<Arc<str>>) -> Self {
        Literal::String(s.into())
    }
}

/// Binder information for implicit arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinderInfo {
    Default,
    Implicit,
    StrictImplicit,
    InstImplicit,
}

/// Core expression type using locally nameless representation
/// 
/// Variable convention:
/// - De Bruijn indices for bound variables (Var)
/// - Free variables with explicit IDs (FVar)  
/// - Metavariables with explicit IDs (MVar)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// Bound variable (de Bruijn index)
    Var(usize),
    /// Sort/universe
    Sort(Level),
    /// Constant with universe levels
    Const(Name, Vec<Level>),
    /// Application
    App(Box<Expr>, Box<Expr>),
    /// Lambda abstraction
    Lam(BinderInfo, Box<Expr>, Box<Expr>), // (binder_info, domain, body)
    /// Pi type (dependent function type)
    Pi(BinderInfo, Box<Expr>, Box<Expr>), // (binder_info, domain, codomain)
    /// Let binding
    Let(BinderInfo, Box<Expr>, Box<Expr>, Box<Expr>), // (binder_info, type, value, body)
    /// Literal value
    Lit(Literal),
    /// Free variable
    FVar(FVarId),
    /// Metavariable
    MVar(MVarId, Vec<Level>), // (mvar_id, universe_params)
}

impl Expr {
    /// Create a constant expression
    pub fn const_(name: Name) -> Self {
        Expr::Const(name, Vec::new())
    }
    
    /// Create a constant with universe levels
    pub fn const_with_levels(name: Name, levels: Vec<Level>) -> Self {
        Expr::Const(name, levels)
    }
    
    /// Create a sort expression
    pub fn sort(level: Level) -> Self {
        Expr::Sort(level)
    }
    
    /// Create a variable expression
    pub fn var(idx: usize) -> Self {
        Expr::Var(idx)
    }
    
    /// Create a free variable expression
    pub fn fvar(id: FVarId) -> Self {
        Expr::FVar(id)
    }
    
    /// Create an application
    pub fn app(f: Expr, arg: Expr) -> Self {
        Expr::App(Box::new(f), Box::new(arg))
    }
    
    /// Create a lambda abstraction
    pub fn lam(binder: BinderInfo, domain: Expr, body: Expr) -> Self {
        Expr::Lam(binder, Box::new(domain), Box::new(body))
    }
    
    /// Create a pi type
    pub fn pi(binder: BinderInfo, domain: Expr, codomain: Expr) -> Self {
        Expr::Pi(binder, Box::new(domain), Box::new(codomain))
    }
    
    /// Create a let binding
    pub fn let_(binder: BinderInfo, type_: Expr, value: Expr, body: Expr) -> Self {
        Expr::Let(binder, Box::new(type_), Box::new(value), Box::new(body))
    }
    
    /// Create a literal expression
    pub fn lit(lit: Literal) -> Self {
        Expr::Lit(lit)
    }
    
    /// Create a metavariable
    pub fn mvar(id: MVarId, levels: Vec<Level>) -> Self {
        Expr::MVar(id, levels)
    }
    
    /// Check if this expression is a definition
    pub fn is_definition(&self) -> bool {
        matches!(self, Expr::Const(_, _))
    }
    
    /// Check if this expression is a theorem
    pub fn is_theorem(&self) -> bool {
        // In Lean 4, theorems are constants that are not definitions
        // For now, we'll consider all constants as potential theorems
        matches!(self, Expr::Const(_, _))
    }
}

/// Term interning and hash-consing module
pub mod interning {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};
    use dashmap::DashMap;
    use bumpalo::Bump;

    /// Global counter for generating unique ExprIds
    static NEXT_EXPR_ID: AtomicU32 = AtomicU32::new(0);

    /// Generate a new unique ExprId
    fn next_expr_id() -> ExprId {
        NEXT_EXPR_ID.fetch_add(1, Ordering::SeqCst)
    }

    /// Interned expression with hash-consing
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InternedExpr {
        pub id: ExprId,
        // The actual expression is stored in the arena
    }

    impl InternedExpr {
        pub fn new(id: ExprId) -> Self {
            InternedExpr { id }
        }
    }

    /// Expression interner with arena storage and hash-consing
    pub struct ExprInterner {
        /// Arena for storing expressions
        arena: Bump,
        /// Hash table for structural hashing to ExprId
        intern_table: HashMap<u64, ExprId>,
        /// Storage for interned expressions by ExprId
        exprs: HashMap<ExprId, Expr>,
        /// Concurrent read access for parallel elaboration
        concurrent_table: DashMap<u64, ExprId>,
    }

    impl ExprInterner {
        /// Create a new expression interner
        pub fn new() -> Self {
            ExprInterner {
                arena: Bump::new(),
                intern_table: HashMap::new(),
                exprs: HashMap::new(),
                concurrent_table: DashMap::new(),
            }
        }

        /// Intern an expression by structural content
        pub fn intern(&mut self, expr: Expr) -> InternedExpr {
            let hash = self.hash_expr(&expr);
            
            // Check if already interned
            if let Some(&existing_id) = self.intern_table.get(&hash) {
                return InternedExpr::new(existing_id);
            }

            // Allocate new ID and store
            let id = next_expr_id();
            self.intern_table.insert(hash, id);
            self.concurrent_table.insert(hash, id);
            
            // Store expression in arena
            let expr_clone = self.clone_expr_to_arena(&expr);
            self.exprs.insert(id, expr_clone);
            
            InternedExpr::new(id)
        }

        /// Get an expression by ID
        pub fn get(&self, id: ExprId) -> Option<&Expr> {
            self.exprs.get(&id)
        }

        /// Check if two interned expressions are identical by pointer equality
        pub fn ptr_eq(&self, e1: InternedExpr, e2: InternedExpr) -> bool {
            e1.id == e2.id
        }

        /// Hash an expression structurally
        fn hash_expr(&self, expr: &Expr) -> u64 {
            let mut hasher = rustc_hash::FxHasher::default();
            self.hash_expr_recursive(expr, &mut hasher);
            hasher.finish()
        }

        /// Recursive hashing for expressions
        fn hash_expr_recursive(&self, expr: &Expr, hasher: &mut rustc_hash::FxHasher) {
            match expr {
                Expr::Var(idx) => {
                    0u8.hash(hasher);
                    idx.hash(hasher);
                }
                Expr::Sort(level) => {
                    1u8.hash(hasher);
                    self.hash_level_recursive(level, hasher);
                }
                Expr::Const(name, levels) => {
                    2u8.hash(hasher);
                    name.hash(hasher);
                    levels.hash(hasher);
                    for level in levels {
                        self.hash_level_recursive(level, hasher);
                    }
                }
                Expr::App(f, arg) => {
                    3u8.hash(hasher);
                    self.hash_expr_recursive(f, hasher);
                    self.hash_expr_recursive(arg, hasher);
                }
                Expr::Lam(binder, domain, body) => {
                    4u8.hash(hasher);
                    binder.hash(hasher);
                    self.hash_expr_recursive(domain, hasher);
                    self.hash_expr_recursive(body, hasher);
                }
                Expr::Pi(binder, domain, codomain) => {
                    5u8.hash(hasher);
                    binder.hash(hasher);
                    self.hash_expr_recursive(domain, hasher);
                    self.hash_expr_recursive(codomain, hasher);
                }
                Expr::Let(binder, type_, value, body) => {
                    6u8.hash(hasher);
                    binder.hash(hasher);
                    self.hash_expr_recursive(type_, hasher);
                    self.hash_expr_recursive(value, hasher);
                    self.hash_expr_recursive(body, hasher);
                }
                Expr::Lit(lit) => {
                    7u8.hash(hasher);
                    match lit {
                        Literal::Nat(n) => {
                            0u8.hash(hasher);
                            n.hash(hasher);
                        }
                        Literal::String(s) => {
                            1u8.hash(hasher);
                            s.hash(hasher);
                        }
                    }
                }
                Expr::FVar(id) => {
                    8u8.hash(hasher);
                    id.hash(hasher);
                }
                Expr::MVar(id, levels) => {
                    9u8.hash(hasher);
                    id.hash(hasher);
                    levels.hash(hasher);
                    for level in levels {
                        self.hash_level_recursive(level, hasher);
                    }
                }
            }
        }

        /// Hash a level recursively
        fn hash_level_recursive(&self, level: &Level, hasher: &mut rustc_hash::FxHasher) {
            match level {
                Level::Zero => 0u8.hash(hasher),
                Level::Succ(l) => {
                    1u8.hash(hasher);
                    self.hash_level_recursive(l, hasher);
                }
                Level::Max(l1, l2) => {
                    2u8.hash(hasher);
                    self.hash_level_recursive(l1, hasher);
                    self.hash_level_recursive(l2, hasher);
                }
                Level::IMax(l1, l2) => {
                    3u8.hash(hasher);
                    self.hash_level_recursive(l1, hasher);
                    self.hash_level_recursive(l2, hasher);
                }
                Level::Param(name) => {
                    4u8.hash(hasher);
                    name.hash(hasher);
                }
                Level::MVar(id) => {
                    5u8.hash(hasher);
                    id.hash(hasher);
                }
            }
        }

        /// Clone an expression to the arena
        fn clone_expr_to_arena(&self, expr: &Expr) -> Expr {
            // For now, we'll just clone the expression
            // In a real implementation, we'd allocate sub-expressions in the arena
            expr.clone()
        }

        /// Clear the interner (useful for testing)
        pub fn clear(&mut self) {
            self.arena = Bump::new();
            self.intern_table.clear();
            self.exprs.clear();
            self.concurrent_table.clear();
        }

        /// Get statistics about the interner
        pub fn stats(&self) -> InternerStats {
            InternerStats {
                total_exprs: self.exprs.len(),
                table_size: self.intern_table.len(),
                concurrent_table_size: self.concurrent_table.len(),
            }
        }
    }

    /// Statistics about the expression interner
    #[derive(Debug, Clone)]
    pub struct InternerStats {
        pub total_exprs: usize,
        pub table_size: usize,
        pub concurrent_table_size: usize,
    }

    impl Default for ExprInterner {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Expression utility functions
pub mod utils {
    use super::*;

    /// Capture-avoiding substitution
    /// Replace bound variable at given depth with replacement expression
    pub fn subst(expr: &Expr, replacement: &Expr, depth: usize) -> Expr {
        match expr {
            Expr::Var(idx) => {
                if *idx == depth {
                    replacement.clone()
                } else if *idx > depth {
                    Expr::Var(idx - 1)
                } else {
                    expr.clone()
                }
            }
            Expr::App(f, arg) => {
                Expr::app(
                    subst(f, replacement, depth),
                    subst(arg, replacement, depth)
                )
            }
            Expr::Lam(binder, domain, body) => {
                Expr::lam(
                    *binder,
                    subst(domain, replacement, depth),
                    subst(body, replacement, depth + 1)
                )
            }
            Expr::Pi(binder, domain, codomain) => {
                Expr::pi(
                    *binder,
                    subst(domain, replacement, depth),
                    subst(codomain, replacement, depth + 1)
                )
            }
            Expr::Let(binder, type_, value, body) => {
                Expr::let_(
                    *binder,
                    subst(type_, replacement, depth),
                    subst(value, replacement, depth),
                    subst(body, replacement, depth + 1)
                )
            }
            // These don't contain bound variables, so just clone
            Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) | 
            Expr::FVar(_) | Expr::MVar(_, _) => expr.clone(),
        }
    }

    /// Lift de Bruijn indices by n
    pub fn lift_vars(expr: &Expr, n: usize) -> Expr {
        lift_vars_from(expr, n, 0)
    }

    /// Lift de Bruijn indices by n starting from depth d
    fn lift_vars_from(expr: &Expr, n: usize, depth: usize) -> Expr {
        match expr {
            Expr::Var(idx) => {
                if *idx >= depth {
                    Expr::Var(idx + n)
                } else {
                    expr.clone()
                }
            }
            Expr::App(f, arg) => {
                Expr::app(
                    lift_vars_from(f, n, depth),
                    lift_vars_from(arg, n, depth)
                )
            }
            Expr::Lam(binder, domain, body) => {
                Expr::lam(
                    *binder,
                    lift_vars_from(domain, n, depth),
                    lift_vars_from(body, n, depth + 1)
                )
            }
            Expr::Pi(binder, domain, codomain) => {
                Expr::pi(
                    *binder,
                    lift_vars_from(domain, n, depth),
                    lift_vars_from(codomain, n, depth + 1)
                )
            }
            Expr::Let(binder, type_, value, body) => {
                Expr::let_(
                    *binder,
                    lift_vars_from(type_, n, depth),
                    lift_vars_from(value, n, depth),
                    lift_vars_from(body, n, depth + 1)
                )
            }
            // These don't contain bound variables, so just clone
            Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) | 
            Expr::FVar(_) | Expr::MVar(_, _) => expr.clone(),
        }
    }

    /// Check if expression contains a specific free variable
    pub fn has_fvar(expr: &Expr, id: FVarId) -> bool {
        match expr {
            Expr::FVar(fvar_id) => *fvar_id == id,
            Expr::App(f, arg) => has_fvar(f, id) || has_fvar(arg, id),
            Expr::Lam(_, domain, body) => has_fvar(domain, id) || has_fvar(body, id),
            Expr::Pi(_, domain, codomain) => has_fvar(domain, id) || has_fvar(codomain, id),
            Expr::Let(_, type_, value, body) => {
                has_fvar(type_, id) || has_fvar(value, id) || has_fvar(body, id)
            }
            // These don't contain free variables
            Expr::Var(_) | Expr::Sort(_) | Expr::Const(_, _) | 
            Expr::Lit(_) | Expr::MVar(_, _) => false,
        }
    }

    /// Abstract free variables with bound variables
    /// Replace the given free variables with bound variables starting from depth 0
    pub fn abstract_fvars(expr: &Expr, fvars: &[FVarId]) -> Expr {
        let mut result = expr.clone();
        for &fvar_id in fvars.iter() {
            result = abstract_fvar(&result, fvar_id, 0);
        }
        result
    }

    /// Abstract a single free variable with a bound variable at the given depth
    fn abstract_fvar(expr: &Expr, fvar_id: FVarId, depth: usize) -> Expr {
        match expr {
            Expr::FVar(id) if *id == fvar_id => Expr::Var(depth),
            Expr::App(f, arg) => {
                Expr::app(
                    abstract_fvar(f, fvar_id, depth),
                    abstract_fvar(arg, fvar_id, depth)
                )
            }
            Expr::Lam(binder, domain, body) => {
                Expr::lam(
                    *binder,
                    abstract_fvar(domain, fvar_id, depth),
                    abstract_fvar(body, fvar_id, depth + 1)
                )
            }
            Expr::Pi(binder, domain, codomain) => {
                Expr::pi(
                    *binder,
                    abstract_fvar(domain, fvar_id, depth),
                    abstract_fvar(codomain, fvar_id, depth + 1)
                )
            }
            Expr::Let(binder, type_, value, body) => {
                Expr::let_(
                    *binder,
                    abstract_fvar(type_, fvar_id, depth),
                    abstract_fvar(value, fvar_id, depth),
                    abstract_fvar(body, fvar_id, depth + 1)
                )
            }
            // These don't contain the target free variable, so just clone
            Expr::Var(_) | Expr::Sort(_) | Expr::Const(_, _) | 
            Expr::Lit(_) | Expr::FVar(_) | Expr::MVar(_, _) => expr.clone(),
        }
    }

    /// Instantiate bound variables with arguments
    /// Replace bound variables starting from depth 0 with the given arguments
    pub fn instantiate(expr: &Expr, args: &[Expr]) -> Expr {
        let mut result = expr.clone();
        for arg in args.iter() {
            result = subst(&result, arg, 0);
        }
        result
    }

    /// Calculate structural size of an expression
    pub fn expr_size(expr: &Expr) -> usize {
        match expr {
            Expr::Var(_) | Expr::Sort(_) | Expr::Const(_, _) | 
            Expr::Lit(_) | Expr::FVar(_) | Expr::MVar(_, _) => 1,
            Expr::App(f, arg) => 1 + expr_size(f) + expr_size(arg),
            Expr::Lam(_, domain, body) => 1 + expr_size(domain) + expr_size(body),
            Expr::Pi(_, domain, codomain) => 1 + expr_size(domain) + expr_size(codomain),
            Expr::Let(_, type_, value, body) => {
                1 + expr_size(type_) + expr_size(value) + expr_size(body)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::interning::*;
    use super::utils::*;

    #[test]
    fn test_name_creation() {
        let name = Name::from("Nat.add");
        assert_eq!(name.as_str(), "Nat.add");
    }

    #[test]
    fn test_level_construction() {
        let zero = Level::Zero;
        let one = Level::Succ(Box::new(Level::Zero));
        assert_ne!(zero, one);
    }

    #[test]
    fn test_expr_basic() {
        let nat = Name::from("Nat");
        let expr = Expr::const_(nat.clone());
        assert!(expr.is_definition());
        assert!(expr.is_theorem());
    }

    #[test]
    fn test_expr_application() {
        let add = Name::from("Nat.add");
        let nat = Name::from("Nat");
        let app = Expr::app(
            Expr::const_(add),
            Expr::app(Expr::const_(nat), Expr::lit(Literal::Nat(1)))
        );
        assert!(matches!(app, Expr::App(_, _)));
    }

    #[test]
    fn test_expr_interning() {
        let mut interner = ExprInterner::new();
        
        let nat = Name::from("Nat");
        let expr1 = Expr::const_(nat.clone());
        let expr2 = Expr::const_(nat.clone());
        
        let interned1 = interner.intern(expr1.clone());
        let interned2 = interner.intern(expr2.clone());
        
        // Same structural content should get same ID
        assert_eq!(interned1.id, interned2.id);
        assert!(interner.ptr_eq(interned1, interned2));
        
        // Verify we can retrieve the expression
        let retrieved = interner.get(interned1.id);
        assert!(retrieved.is_some());
        assert_eq!(*retrieved.unwrap(), expr1);
    }

    #[test]
    fn test_interner_stats() {
        let mut interner = ExprInterner::new();
        
        let expr = Expr::const_(Name::from("Nat"));
        interner.intern(expr);
        
        let stats = interner.stats();
        assert_eq!(stats.total_exprs, 1);
        assert_eq!(stats.table_size, 1);
        assert_eq!(stats.concurrent_table_size, 1);
    }

    #[test]
    fn test_different_exprs_different_ids() {
        let mut interner = ExprInterner::new();
        
        let expr1 = Expr::const_(Name::from("Nat"));
        let expr2 = Expr::const_(Name::from("String"));
        
        let interned1 = interner.intern(expr1);
        let interned2 = interner.intern(expr2);
        
        // Different expressions should get different IDs
        assert_ne!(interned1.id, interned2.id);
        assert!(!interner.ptr_eq(interned1, interned2));
    }

    #[test]
    fn test_substitution() {
        let x = Expr::Var(0);
        let replacement = Expr::lit(Literal::Nat(42));
        let body = Expr::app(x.clone(), x);
        
        let result = subst(&body, &replacement, 0);
        let expected = Expr::app(replacement.clone(), replacement);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lift_vars() {
        let x = Expr::Var(0);
        let y = Expr::Var(1);
        let body = Expr::app(x, y);
        
        let result = lift_vars(&body, 1);
        let expected = Expr::app(Expr::Var(1), Expr::Var(2));
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_has_fvar() {
        let fvar = FVarId(0);
        let other_fvar = FVarId(1);
        let expr_with_fvar = Expr::FVar(fvar);
        let expr_without_fvar = Expr::FVar(other_fvar);
        let complex_expr = Expr::app(expr_with_fvar.clone(), Expr::const_(Name::from("Nat")));
        
        assert!(has_fvar(&expr_with_fvar, fvar));
        assert!(!has_fvar(&expr_without_fvar, fvar));
        assert!(has_fvar(&complex_expr, fvar));
    }

    #[test]
    fn test_abstract_fvars() {
        let fvar = FVarId(0);
        let expr = Expr::FVar(fvar);
        let fvars = [fvar];
        
        let result = abstract_fvars(&expr, &fvars);
        let expected = Expr::Var(0);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_expr_size() {
        let simple = Expr::Var(0);
        let complex = Expr::app(
            Expr::Var(0),
            Expr::app(Expr::Var(1), Expr::Var(2))
        );
        
        assert_eq!(expr_size(&simple), 1);
        assert_eq!(expr_size(&complex), 5); // 1 (app) + 1 (var) + 1 (app) + 1 (var) + 1 (var)
    }
}
