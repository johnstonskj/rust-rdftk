/*!
One-line description.

More detailed description, with

# Example

 */

use super::common::{iri_ref, prologue};
use combine::parser::char::char;
use combine::{choice, optional, value, ParseError, Parser, RangeStream};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub fn update<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        prologue(),
        optional((update_1(), optional((char(';'), update())))),
    )
        .with(value(()))
        .expected("Update")
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn update_1<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice().with(value(())).expected("Update1")
}

pub fn graph_ref<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (keyword("graph").skip(spaces()), iri_ref())
        .with(value(()))
        .expected("GraphRef")
}

pub fn graph_ref_all<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        graph_ref(),
        keyword("default"),
        keyword("named"),
        keyword("all"),
    ))
    .with(value(()))
    .expected("GraphRef")
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
