/*!
One-line description.

More detailed description, with

# Example

*/

use rdftk_iri::{Iri, IriRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::statement::StatementRef;
use rdftk_iri::IriRef;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub enum Quantification {
    Universal,
    Existential,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    quantification: Quantification,
    id: IriRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Formula {
    variables: BTreeSet<Variable>,
    statements: BTreeSet<StatementRef>,
}

pub type FormulaRef = Rc<Formula>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <{}> .",
            match self.quantification {
                Quantification::Universal => "@forAll",
                Quantification::Existential => "@forSome",
            },
            self.id
        )
    }
}

impl Variable {
    pub fn universal(iri: IriRef) -> Self {
        Self {
            quantification: Quantification::Universal,
            id: iri.clone(),
        }
    }

    pub fn existential(iri: IriRef) -> Self {
        Self {
            quantification: Quantification::Existential,
            id: iri.clone(),
        }
    }

    pub fn for_all(iri: IriRef) -> Self {
        Self::universal(iri)
    }

    pub fn for_some(iri: IriRef) -> Self {
        Self::existential(iri)
    }

    pub fn is_universal(&self) -> bool {
        matches!(self.quantification, Quantification::Universal)
    }

    pub fn is_existential(&self) -> bool {
        matches!(self.quantification, Quantification::Existential)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.statements.is_empty() {
            let vars: Vec<String> = self
                .variables
                .iter()
                .filter_map(|v| {
                    if v.quantification == Quantification::Universal {
                        Some(v.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            if !vars.is_empty() {
                writeln!(f, "{}", vars.join("\n"))?;
            }

            let vars: Vec<String> = self
                .variables
                .iter()
                .filter_map(|v| {
                    if v.quantification == Quantification::Existential {
                        Some(v.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            if !vars.is_empty() {
                writeln!(f, "{}", vars.join("\n"))?;
            }

            writeln!(f, "{{")?;
            for st in &self.statements {
                writeln!(f, "{} .", st)?;
            }
        } else {
            write!(f, "{{}}")?;
        }
        Ok(())
    }
}

impl Formula {
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
