/*!
One-line description.

More detailed description, with

# Example

 */

use crate::sparql::Variable;
use combine::parser::char::{alpha_num, char, hex_digit, space, spaces, string, string_cmp};
use combine::{
    attempt, choice, count, look_ahead, many, many1, not_followed_by, one_of, optional, satisfy,
    value, ParseError, Parser, RangeStream,
};

use super::common::{keyword, prologue};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

//
// Spec:
//
// ```abnf
// [1]  	QueryUnit	  ::=  	Query
// [2]  	Query	      ::=  	Prologue
//                             ( SelectQuery | ConstructQuery | DescribeQuery | AskQuery )
//                             ValuesClause
// ```
pub fn query<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        prologue(),
        choice((
            (look_ahead(keyword("select")), select_query()),
            (look_ahead(keyword("construct")), construct_query()),
        )),
        values_clause(),
    )
        .with(value(()))
        .expected("Query")
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn select_query<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        select_clause(),
        many(dataset_clause()),
        where_clause(),
        solution_modifier(),
    )
        .with(value(()))
        .expected("SelectQuery")
}

fn construct_query<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice().with(value(())).expected("Query")
}

fn select_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("select").skip(spaces()),
        optional(choice((keyword("distinct"), keyword("reduced")))).skip(spaces()),
        choice((
            many1(choice((
                variable(),
                (char('('), expression_as_var(), char(')')),
            ))),
            char('*'),
        )),
    )
        .with(value(()))
        .expected("SelectClause")
}

fn dataset_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("from").skip(spaces()),
        optional(keyword("named")).skip(spaces()),
        iri_ref().skip(spaces()),
    )
        .with(value(()))
        .expected("DatasetClause")
}

fn where_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(keyword("where").skip(spaces())),
        group_graph_pattern().skip(spaces()),
    )
        .with(value(()))
        .expected("WhereClause")
}

fn expression_as_var<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    ().with(value(())).expected("expression_as_var")
}

fn group_graph_pattern<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    ().with(value(())).expected("GroupGraphPattern")
}

fn solution_modifier<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(group_clause()),
        optional(having_clause()),
        optional(order_clause()),
        optional(limit_offset_clause()),
    )
        .with(value(()))
        .expected("SolutionModifier")
}

fn group_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (keyword("group"), keyword("by"))
        .with(value(()))
        .expected("GroupClause")
}

fn having_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (keyword("having")).with(value(())).expected("HavingClause")
}

fn order_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("order").skip(spaces()),
        keyword("by").skip(spaces()),
    )
        .with(value(()))
        .expected("OrderClause")
}

fn limit_offset_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        (keyword("offset").skip(spaces()), integer()),
        (keyword("limit").skip(spaces()), integer()),
    ))
    .with(value(()))
    .expected("LimitOffsetClause")
}

fn values_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (keyword("offsetvalues").skip(spaces()),)
        .with(value(()))
        .expected("ValuesClause")
}
