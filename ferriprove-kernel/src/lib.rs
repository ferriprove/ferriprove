//! Ferriprove Kernel
//!
//! Lean 4-compatible type checker kernel.
//! Implements the trusted core: environment, declarations, and reduction.

use std::collections::HashMap;

pub use ferriprove_types::{Expr, Level, Name};

/// Transparency levels for definition unfolding
/// Controls when definitions can be unfolded during reduction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transparency {
    /// Never unfold (opaque)
    Reducible,
    /// Unfold for instance resolution
    Instances,
    /// Default unfolding behavior
    Default,
    /// Always unfold
    All,
}

impl Transparency {
    /// Check if this transparency level permits unfolding at a given transparency requirement
    pub fn can_unfold_at(&self, required: Transparency) -> bool {
        use Transparency::*;
        match (self, required) {
            // Opaque never unfolds
            (Reducible, _) => false,
            // Instance unfolds for instance resolution and higher
            (Instances, Instances) | (Instances, Default) | (Instances, All) => true,
            (Instances, Reducible) => false,
            // Default unfolds for default and higher
            (Default, Default) | (Default, All) => true,
            (Default, Reducible) | (Default, Instances) => false,
            // All always unfolds
            (All, _) => true,
        }
    }
}

/// A constructor for an inductive type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    /// Constructor name
    pub name: Name,
    /// Constructor type (with parameters/indices abstracted)
    pub typ: Expr,
    /// Number of recursive arguments (for nested inductive check)
    pub num_recursive_args: usize,
}

/// A recursor rule for an inductive type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursorRule {
    /// Constructor this rule handles
    pub ctor: Name,
    /// Number of fields for this constructor
    pub num_fields: usize,
    /// Right-hand side expression (the reduction rule)
    pub rhs: Expr,
}

/// Declaration types in the kernel environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    /// Axiom (postulated constant with no definition)
    Axiom {
        name: Name,
        universe_params: Vec<Name>,
        typ: Expr,
    },
    /// Definition (transparent, can be unfolded)
    Definition {
        name: Name,
        universe_params: Vec<Name>,
        typ: Expr,
        value: Expr,
        transparency: Transparency,
    },
    /// Theorem (opaque definition with proof irrelevance)
    Theorem {
        name: Name,
        universe_params: Vec<Name>,
        typ: Expr,
        value: Expr,
    },
    /// Opaque definition (can be unfolded only in proof terms)
    Opaque {
        name: Name,
        universe_params: Vec<Name>,
        typ: Expr,
        value: Expr,
        transparency: Transparency,
    },
    /// Quotient type declaration
    Quot { name: Name },
    /// Inductive type declaration
    /// Note: Constructors are stored as separate Declaration::Constructor entries
    /// in the environment, not duplicated here (single source of truth)
    Inductive {
        name: Name,
        universe_params: Vec<Name>,
        num_params: usize,
        /// Types of the indices (if any)
        indices: Vec<Expr>,
        /// The type of the inductive family
        typ: Expr,
        /// Number of constructors (lookup their names separately in environment)
        num_constructors: usize,
        /// Is this a recursive inductive type?
        is_recursive: bool,
    },
    /// Constructor (part of an inductive type)
    Constructor {
        /// The inductive type this constructor belongs to
        inductive: Name,
        /// Constructor details (name, type, recursive args)
        ctor: Constructor,
    },
    /// Recursor for an inductive type
    Recursor {
        name: Name,
        /// The inductive type this recursor eliminates
        inductive: Name,
        universe_params: Vec<Name>,
        num_params: usize,
        num_indices: usize,
        motive: Expr,
        num_minor_premises: usize,
        major_premise_type: Expr,
        typ: Expr,
        rules: Vec<RecursorRule>,
        /// Is this a recursor for a recursive inductive type?
        is_recursive: bool,
    },
}

impl Declaration {
    /// Get the name of this declaration
    pub fn name(&self) -> &Name {
        match self {
            Declaration::Axiom { name, .. } => name,
            Declaration::Definition { name, .. } => name,
            Declaration::Theorem { name, .. } => name,
            Declaration::Opaque { name, .. } => name,
            Declaration::Quot { name, .. } => name,
            Declaration::Inductive { name, .. } => name,
            Declaration::Constructor { ctor, .. } => &ctor.name,
            Declaration::Recursor { name, .. } => name,
        }
    }

    /// Get the universe parameters for this declaration
    pub fn universe_params(&self) -> &[Name] {
        match self {
            Declaration::Axiom {
                universe_params, ..
            } => universe_params,
            Declaration::Definition {
                universe_params, ..
            } => universe_params,
            Declaration::Theorem {
                universe_params, ..
            } => universe_params,
            Declaration::Opaque {
                universe_params, ..
            } => universe_params,
            Declaration::Quot { .. } => &[],
            Declaration::Inductive {
                universe_params, ..
            } => universe_params,
            Declaration::Constructor { .. } => {
                // Constructors inherit universe params from their inductive type
                // Look them up from the environment if needed
                &[]
            }
            Declaration::Recursor {
                universe_params, ..
            } => universe_params,
        }
    }

    /// Check if this declaration is a definition (Definition or Opaque)
    pub fn is_definition(&self) -> bool {
        matches!(
            self,
            Declaration::Definition { .. } | Declaration::Opaque { .. }
        )
    }

    /// Check if this declaration is a theorem
    pub fn is_theorem(&self) -> bool {
        matches!(self, Declaration::Theorem { .. })
    }

    /// Check if this declaration is an axiom
    pub fn is_axiom(&self) -> bool {
        matches!(self, Declaration::Axiom { .. })
    }

    /// Check if this declaration is an inductive type
    pub fn is_inductive(&self) -> bool {
        matches!(self, Declaration::Inductive { .. })
    }

    /// Get the type of this declaration if available
    pub fn typ(&self) -> Option<&Expr> {
        match self {
            Declaration::Axiom { typ, .. } => Some(typ),
            Declaration::Definition { typ, .. } => Some(typ),
            Declaration::Theorem { typ, .. } => Some(typ),
            Declaration::Opaque { typ, .. } => Some(typ),
            Declaration::Quot { .. } => None,
            Declaration::Inductive { typ, .. } => Some(typ),
            Declaration::Constructor { ctor, .. } => Some(&ctor.typ),
            Declaration::Recursor { typ, .. } => Some(typ),
        }
    }

    /// Get the transparency level if this is a definition-like declaration
    pub fn transparency(&self) -> Option<Transparency> {
        match self {
            Declaration::Definition { transparency, .. } => Some(*transparency),
            Declaration::Opaque { transparency, .. } => Some(*transparency),
            _ => None,
        }
    }
}

/// Errors that can occur when working with the environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvError {
    /// A declaration with this name already exists
    DuplicateName(Name),
    /// Declaration not found
    UnknownName(Name),
    /// Invalid operation for this declaration type
    InvalidOperation { name: Name, operation: String },
    /// Inductive extension error (constructors/recursor mismatch)
    InductiveExtensionError { inductive: Name, message: String },
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::DuplicateName(name) => {
                write!(
                    f,
                    "declaration '{}' already exists in environment",
                    name.as_str()
                )
            }
            EnvError::UnknownName(name) => {
                write!(
                    f,
                    "declaration '{}' not found in environment",
                    name.as_str()
                )
            }
            EnvError::InvalidOperation { name, operation } => {
                write!(
                    f,
                    "cannot {}: invalid for declaration '{}'",
                    operation,
                    name.as_str()
                )
            }
            EnvError::InductiveExtensionError { inductive, message } => {
                write!(
                    f,
                    "inductive extension error for '{}': {}",
                    inductive.as_str(),
                    message
                )
            }
        }
    }
}

impl std::error::Error for EnvError {}

/// Environment storing all declarations
#[derive(Debug, Clone)]
pub struct Environment {
    /// Map from declaration names to declarations
    declarations: HashMap<Name, Declaration>,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Environment {
            declarations: HashMap::new(),
        }
    }

    /// Add a declaration to the environment
    /// Returns Err if a declaration with the same name already exists
    pub fn add_declaration(&mut self, decl: Declaration) -> Result<(), EnvError> {
        let name = decl.name().clone();
        if self.declarations.contains_key(&name) {
            return Err(EnvError::DuplicateName(name));
        }
        self.declarations.insert(name, decl);
        Ok(())
    }

    /// Get a declaration by name
    pub fn get_declaration(&self, name: &Name) -> Option<&Declaration> {
        self.declarations.get(name)
    }

    /// Get a declaration by name with transparency check
    /// Returns None if the declaration exists but doesn't meet the transparency requirement
    pub fn get_declaration_with_transparency(
        &self,
        name: &Name,
        transparency: Transparency,
    ) -> Option<&Declaration> {
        let decl = self.declarations.get(name)?;

        // Check if this declaration can be unfolded at the required transparency
        match decl {
            Declaration::Definition {
                transparency: decl_transparency,
                ..
            } => {
                if decl_transparency.can_unfold_at(transparency) {
                    Some(decl)
                } else {
                    None
                }
            }
            Declaration::Opaque {
                transparency: decl_transparency,
                ..
            } => {
                if decl_transparency.can_unfold_at(transparency) {
                    Some(decl)
                } else {
                    None
                }
            }
            // Theorems are visible for type checking but opaque (never unfold)
            // Their type signature is needed, only the proof body is hidden
            Declaration::Theorem { .. } => Some(decl),
            // Everything else (Axiom, Quot, Inductive, Constructor, Recursor) is always visible
            _ => Some(decl),
        }
    }

    /// Check if a declaration exists
    pub fn contains(&self, name: &Name) -> bool {
        self.declarations.contains_key(name)
    }

    /// Get the number of declarations in the environment
    pub fn len(&self) -> usize {
        self.declarations.len()
    }

    /// Check if the environment is empty
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// Extend the environment with an inductive type, its constructors, and recursor atomically
    /// Either all are added successfully, or none are added
    pub fn extend_inductive(
        &mut self,
        inductive: Declaration,
        constructors: Vec<Declaration>,
        recursor: Declaration,
    ) -> Result<(), EnvError> {
        // Validate that we have an inductive type
        let inductive_name = match &inductive {
            Declaration::Inductive { name, .. } => name.clone(),
            _ => {
                return Err(EnvError::InvalidOperation {
                    name: inductive.name().clone(),
                    operation: "extend_inductive".to_string(),
                });
            }
        };

        // Check that all constructors belong to this inductive
        for ctor in &constructors {
            match ctor {
                Declaration::Constructor {
                    inductive: ctor_ind,
                    ..
                } => {
                    if ctor_ind != &inductive_name {
                        return Err(EnvError::InductiveExtensionError {
                            inductive: inductive_name.clone(),
                            message: format!(
                                "constructor '{}' belongs to different inductive '{}'",
                                ctor.name().as_str(),
                                ctor_ind.as_str()
                            ),
                        });
                    }
                }
                _ => {
                    return Err(EnvError::InductiveExtensionError {
                        inductive: inductive_name.clone(),
                        message: format!("expected Constructor, got '{}'", ctor.name().as_str()),
                    });
                }
            }
        }

        // Check that recursor belongs to this inductive
        match &recursor {
            Declaration::Recursor {
                inductive: rec_ind, ..
            } => {
                if rec_ind != &inductive_name {
                    return Err(EnvError::InductiveExtensionError {
                        inductive: inductive_name.clone(),
                        message: format!(
                            "recursor '{}' belongs to different inductive '{}'",
                            recursor.name().as_str(),
                            rec_ind.as_str()
                        ),
                    });
                }
            }
            _ => {
                return Err(EnvError::InductiveExtensionError {
                    inductive: inductive_name.clone(),
                    message: format!("expected Recursor, got '{}'", recursor.name().as_str()),
                });
            }
        }

        // Check for name conflicts before adding anything
        let all_names: Vec<_> = std::iter::once(inductive.name().clone())
            .chain(constructors.iter().map(|c| c.name().clone()))
            .chain(std::iter::once(recursor.name().clone()))
            .collect();

        for name in &all_names {
            if self.declarations.contains_key(name) {
                return Err(EnvError::DuplicateName(name.clone()));
            }
        }

        // All checks passed - add everything
        self.declarations.insert(inductive_name.clone(), inductive);
        for ctor in constructors {
            self.declarations.insert(ctor.name().clone(), ctor);
        }
        self.declarations.insert(recursor.name().clone(), recursor);

        Ok(())
    }

    /// Iterate over all declarations
    pub fn iter(&self) -> impl Iterator<Item = (&Name, &Declaration)> {
        self.declarations.iter()
    }

    /// Get all declarations of a specific type
    pub fn get_declarations_by_type<F>(&self, predicate: F) -> Vec<&Declaration>
    where
        F: Fn(&Declaration) -> bool,
    {
        self.declarations
            .values()
            .filter(|d| predicate(d))
            .collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transparency_levels() {
        use Transparency::*;

        // Reducible never unfolds
        assert!(!Reducible.can_unfold_at(Reducible));
        assert!(!Reducible.can_unfold_at(Instances));
        assert!(!Reducible.can_unfold_at(Default));
        assert!(!Reducible.can_unfold_at(All));

        // Instances unfolds at Instances and above
        assert!(!Instances.can_unfold_at(Reducible));
        assert!(Instances.can_unfold_at(Instances));
        assert!(Instances.can_unfold_at(Default));
        assert!(Instances.can_unfold_at(All));

        // Default unfolds at Default and above
        assert!(!Default.can_unfold_at(Reducible));
        assert!(!Default.can_unfold_at(Instances));
        assert!(Default.can_unfold_at(Default));
        assert!(Default.can_unfold_at(All));

        // All always unfolds
        assert!(All.can_unfold_at(Reducible));
        assert!(All.can_unfold_at(Instances));
        assert!(All.can_unfold_at(Default));
        assert!(All.can_unfold_at(All));
    }

    #[test]
    fn test_declaration_helpers() {
        let axiom = Declaration::Axiom {
            name: Name::from("test_axiom"),
            universe_params: vec![Name::from("u")],
            typ: Expr::const_(Name::from("Type")),
        };

        assert!(axiom.is_axiom());
        assert!(!axiom.is_definition());
        assert!(!axiom.is_theorem());
        assert_eq!(axiom.name().as_str(), "test_axiom");
        assert_eq!(axiom.universe_params().len(), 1);
        assert!(axiom.typ().is_some());
        assert!(axiom.transparency().is_none());
    }

    #[test]
    fn test_environment_add_and_get() {
        let mut env = Environment::new();

        let axiom = Declaration::Axiom {
            name: Name::from("Nat"),
            universe_params: vec![],
            typ: Expr::sort(ferriprove_types::Level::Zero),
        };

        // Add declaration
        assert!(env.add_declaration(axiom.clone()).is_ok());
        assert_eq!(env.len(), 1);

        // Get declaration
        let retrieved = env.get_declaration(&Name::from("Nat"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name().as_str(), "Nat");

        // Duplicate add should fail
        let result = env.add_declaration(axiom);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EnvError::DuplicateName(_)));
    }

    #[test]
    fn test_environment_transparency_filter() {
        let mut env = Environment::new();

        let def = Declaration::Definition {
            name: Name::from("add"),
            universe_params: vec![],
            typ: Expr::const_(Name::from("Nat")),
            value: Expr::var(0),
            transparency: Transparency::Default,
        };

        env.add_declaration(def).unwrap();

        // Should be visible at Default and All
        assert!(
            env.get_declaration_with_transparency(&Name::from("add"), Transparency::Default)
                .is_some()
        );
        assert!(
            env.get_declaration_with_transparency(&Name::from("add"), Transparency::All)
                .is_some()
        );

        // Should not be visible at lower transparency
        assert!(
            env.get_declaration_with_transparency(&Name::from("add"), Transparency::Instances)
                .is_none()
        );
        assert!(
            env.get_declaration_with_transparency(&Name::from("add"), Transparency::Reducible)
                .is_none()
        );
    }

    #[test]
    fn test_inductive_extension() {
        let mut env = Environment::new();

        let inductive = Declaration::Inductive {
            name: Name::from("Nat"),
            universe_params: vec![],
            num_params: 0,
            indices: vec![],
            typ: Expr::sort(ferriprove_types::Level::Zero),
            num_constructors: 1,
            is_recursive: true,
        };

        let ctor = Declaration::Constructor {
            inductive: Name::from("Nat"),
            ctor: Constructor {
                name: Name::from("Nat.zero"),
                typ: Expr::const_(Name::from("Nat")),
                num_recursive_args: 0,
            },
        };

        let recursor = Declaration::Recursor {
            name: Name::from("Nat.rec"),
            inductive: Name::from("Nat"),
            universe_params: vec![],
            num_params: 0,
            num_indices: 0,
            motive: Expr::const_(Name::from("Nat")),
            num_minor_premises: 2,
            major_premise_type: Expr::const_(Name::from("Nat")),
            typ: Expr::const_(Name::from("Nat")),
            rules: vec![],
            is_recursive: true,
        };

        // Successful extension
        assert!(
            env.extend_inductive(inductive, vec![ctor], recursor)
                .is_ok()
        );
        assert_eq!(env.len(), 3);

        // Verify all three are accessible
        assert!(env.get_declaration(&Name::from("Nat")).is_some());
        assert!(env.get_declaration(&Name::from("Nat.zero")).is_some());
        assert!(env.get_declaration(&Name::from("Nat.rec")).is_some());
    }

    #[test]
    fn test_inductive_extension_wrong_inductive() {
        let mut env = Environment::new();

        let inductive = Declaration::Inductive {
            name: Name::from("Nat"),
            universe_params: vec![],
            num_params: 0,
            indices: vec![],
            typ: Expr::sort(ferriprove_types::Level::Zero),
            num_constructors: 0,
            is_recursive: true,
        };

        // Constructor belongs to wrong inductive
        let wrong_ctor = Declaration::Constructor {
            inductive: Name::from("List"), // Wrong!
            ctor: Constructor {
                name: Name::from("List.nil"),
                typ: Expr::const_(Name::from("List")),
                num_recursive_args: 0,
            },
        };

        let recursor = Declaration::Recursor {
            name: Name::from("Nat.rec"),
            inductive: Name::from("Nat"),
            universe_params: vec![],
            num_params: 0,
            num_indices: 0,
            motive: Expr::const_(Name::from("Nat")),
            num_minor_premises: 1,
            major_premise_type: Expr::const_(Name::from("Nat")),
            typ: Expr::const_(Name::from("Nat")),
            rules: vec![],
            is_recursive: true,
        };

        // Should fail because constructor belongs to different inductive
        let result = env.extend_inductive(inductive, vec![wrong_ctor], recursor);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EnvError::InductiveExtensionError { .. }
        ));
        assert!(env.is_empty()); // Nothing was added
    }

    #[test]
    fn test_inductive_extension_duplicate_name() {
        let mut env = Environment::new();

        // Pre-populate with a name that will conflict
        let existing = Declaration::Axiom {
            name: Name::from("Nat"),
            universe_params: vec![],
            typ: Expr::const_(Name::from("Type")),
        };
        env.add_declaration(existing).unwrap();

        let inductive = Declaration::Inductive {
            name: Name::from("Nat"), // Duplicate!
            universe_params: vec![],
            num_params: 0,
            indices: vec![],
            typ: Expr::sort(ferriprove_types::Level::Zero),
            num_constructors: 0,
            is_recursive: true,
        };

        let ctor = Declaration::Constructor {
            inductive: Name::from("Nat"),
            ctor: Constructor {
                name: Name::from("Nat.zero"),
                typ: Expr::const_(Name::from("Nat")),
                num_recursive_args: 0,
            },
        };

        let recursor = Declaration::Recursor {
            name: Name::from("Nat.rec"),
            inductive: Name::from("Nat"),
            universe_params: vec![],
            num_params: 0,
            num_indices: 0,
            motive: Expr::const_(Name::from("Nat")),
            num_minor_premises: 2,
            major_premise_type: Expr::const_(Name::from("Nat")),
            typ: Expr::const_(Name::from("Nat")),
            rules: vec![],
            is_recursive: true,
        };

        // Should fail because "Nat" already exists
        let result = env.extend_inductive(inductive, vec![ctor], recursor);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EnvError::DuplicateName(_)));
        assert_eq!(env.len(), 1); // Only the original axiom
    }
}
