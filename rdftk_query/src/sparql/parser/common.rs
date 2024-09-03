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

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public (Crate) Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn prologue<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    many(choice((base_decl(), prefix_decl())))
        .with(value(()))
        .expected("Prologue")
}

pub(crate) fn iri<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((iri_ref(), prefixed_name()))
        .with(value(()))
        .expected("iri")
}

pub(crate) fn prefixed_name<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((prefixed_name_local_name(), prefixed_name_namespace()))
        .with(value(()))
        .expected("PrefixedName")
}

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

fn select_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("select"),
        optional(choice((keyword("distinct"), keyword("reduced")))),
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

fn expression_as_var<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    ().with(value(())).expected("expression_as_var")
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
    (keyword("order"), keyword("by"))
        .with(value(()))
        .expected("OrderClause")
}

fn limit_offset_clause<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        (keyword("offset"), integer()),
        (keyword("limit"), integer()),
    ))
    .with(value(()))
    .expected("LimitOffsetClause")
}

///
/// Spec:
///
/// ```abnf
/// [134]  	BooleanLiteral	  ::=  	'true' | 'false'
/// ```
fn boolean_literal<'a, I>() -> impl Parser<I, Output = bool>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    attempt(choice((
        string("false").map(|_| false),
        string("true").map(|_| true),
    )))
}

///
/// Spec:
///
/// ```abnf
/// [5]  	BaseDecl	  ::=  	'BASE' IRIREF
/// ```
fn base_decl<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("base").skip(many1::<Vec<_>, _, _>(space())),
        iri_ref().skip(spaces()),
    )
        .map(|(_, iri): ((), String)| iri)
        .expected("BaseDecl")
}

///
/// Spec:
///
/// ```abnf
/// [6]  	PrefixDecl	  ::=  	'PREFIX' PNAME_NS IRIREF
/// ```
fn prefix_decl<'a, I>() -> impl Parser<I, Output = (Option<String>, String)>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        keyword("prefix").skip(spaces()),
        prefixed_name_namespace(),
        iri_ref().skip(spaces()),
    )
        .map(|(_, prefix, iri): (_, Option<String>, String)| (prefix, iri))
        .expected("PrefixDecl")
}

// fn integer_literal<'a, I>() -> impl Parser<I, Output = i64>
// where
//     I: RangeStream<Token = char, Range = &'a str>,
//     I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
//     (optional(choice((char('+'), char('-')))), many1(digit()))
//         .map(|(s, num): (Option<char>, String)| format!("{}{}", s.unwrap_or_default(), num))
//         .map(|s| s.parse::<i64>().map_err(|_| UnexpectedParse::Unexpected))
//         .expected("integer_literal")
// }

// fn decimal_literal<'a, I>() -> impl Parser<I, Output = &'a str>
// where
//     I: RangeStream<Token = char, Range = &'a str>,
// 30M    I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
//     (
//         optional(choice((char('+'), char('-')))),
//         many(digit()),
//         char('.'),
//         many1(digit()),
//     )
// }

///
/// Spec:
///
/// ```abnf
/// [108]  	Var	      ::=  	VAR1 | VAR2
/// ...
/// [143]  	VAR1	  ::=  	'?' VARNAME
/// [144]  	VAR2	  ::=  	'$' VARNAME
/// ```
fn variable<'a, I>() -> impl Parser<I, Output = Variable>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(one_of("?$".chars())), var_name())
        .map(|(_, name)| name)
        .expected("Var")
}

///
/// Spec:
///
/// ```abnf
/// [166]  	VARNAME	  ::=  	( PN_CHARS_U | [0-9] )
///                         ( PN_CHARS_U | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040] )*
/// ```
fn var_name<'a, I>() -> impl Parser<I, Output = Variable>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (satisfy(is_var_name_head), many(satisfy(is_var_name_tail)))
        .map(|(c, name): (char, String)| Variable::new_unchecked(format!("{}{}", c, name)))
        .expected("VARNAME")
}

///
/// Spec:
///
/// ```abnf
/// [145]  	LANGTAG	  ::=  	'@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*
/// ```
fn lang_tag<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        char('@'),
        many1(satisfy(|c: char| c.is_ascii_alphabetic())),
        many(
            (
                char('-'),
                many1(satisfy(|c: char| c.is_ascii_alphanumeric())),
            )
                .map(|(c, part): (char, String)| format!("{}{}", c, part)),
        ),
    )
        .map(|(_, main, parts): (_, String, Vec<String>)| format!("{}{}", main, parts.join("")))
        .expected("LANGTAG")
}

///
pub(crate) fn keyword<'a, I>(name: &'static str) -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    attempt(string_cmp(name, |l, r| l.eq_ignore_ascii_case(&r)).skip(not_followed_by(alpha_num())))
        .with(value(()))
        .expected(name)
}

///
/// Spec:
///
/// ```abnf
/// [139]  	IRIREF	  ::=  	'<' ([^<>"{}|^`\]-[#x00-#x20])* '>'
/// ```
fn iri_ref<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        char('<'),
        many(satisfy(|c| !is_iri_reserved(c))),
        char('>').skip(spaces()),
    )
        .map(|(_, iri, _): (_, String, _)| iri)
        .expected("IRIREF")
}

///
/// Spec:
///
/// ```abnf
/// [140]  	PNAME_NS	  ::=  	PN_PREFIX? ':'
/// ```
pub fn prefixed_name_namespace<'a, I>() -> impl Parser<I, Output = Option<String>>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(prefixed_name_prefix()), char(':').skip(spaces()))
        .map(|(prefix, _): (Option<String>, char)| prefix)
        .expected("PNAME_NS")
}

///
/// Spec:
///
/// ```abnf
/// [141]  	PNAME_LN	  ::=  	PNAME_NS PN_LOCAL
/// ```
pub fn prefixed_name_local_name<'a, I>() -> impl Parser<I, Output = (Option<String>, String)>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (prefixed_name_namespace(), prefixed_name_local())
        .map(|(prefix, local): (Option<String>, String)| (prefix, local))
        .expected("PNAME_LN")
}

///
/// Spec:
///
/// ```abnf
/// [168]  	PN_PREFIX	  ::=  	PN_CHARS_BASE ((PN_CHARS|'.')* PN_CHARS)?
/// ```
fn prefixed_name_prefix<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        satisfy(is_pn_chars_base),
        optional(many(choice((satisfy(is_pn_chars), char('.'))))),
    )
        .map(|(head, tail): (char, Option<String>)| format!("{}{}", head, tail.unwrap_or_default()))
        .expected("PNAME_PREFIX")
    // TODO: fix trailing '.' possibility
}

///
/// Spec:
///
/// ```abnf
/// [169]  	PN_LOCAL	  ::=  	(PN_CHARS_U | ':' | [0-9] | PLX )
///                             ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX) )?
/// ```
fn prefixed_name_local<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        satisfy(|c| is_pn_chars_u(c) || matches!(c, ':' | '0'..='9')).or(prefixed_local_escape()),
        optional(prefixed_name_local_tail()),
    )
        .map(|(head, tail): (char, Option<String>)| format!("{}{}", head, tail.unwrap_or_default()))
        .skip(whitespace())
        .expected("PNAME_LOCAL")
}

///
/// Spec:
///
/// ```abnf
/// [169]  	PN_LOCAL	  ::=  	(PN_CHARS_U | ':' | [0-9] | PLX )
///                             ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX) )?
/// ```
fn prefixed_name_local_tail<'a, I>() -> impl Parser<I, Output = String>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        many(satisfy(|c| is_pn_chars_u(c) || matches!(c, '.' | ':')).or(prefixed_local_escape())),
        satisfy(|c| is_pn_chars_u(c) || c == ':').or(prefixed_local_escape()),
    )
        .map(|(s, c): (String, char)| format!("{}{}", s, c))
        .expected("PNAME_LOCAL(tail)")
}

///
/// Spec:
///
/// ```abnf
/// [170]  	PLX	  ::=  	PERCENT | PN_LOCAL_ESC
/// ```
fn prefixed_local_escape<'a, I>() -> impl Parser<I, Output = char>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    percent_escape()
        .or(prefixed_name_local_escape())
        .expected("PLX")
}

///
/// Spec:
///
/// ```abnf
/// [171]  	PERCENT	  ::=  	'%' HEX HEX
/// ```
fn percent_escape<'a, I>() -> impl Parser<I, Output = char>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (char('%'), count(2, hex_digit()))
        .map(|(_, hex): (char, String)| char::from_u32(hex.parse::<u32>().unwrap()).unwrap())
        .expected("PERCENT")
}

///
/// Spec:
///
/// ```abnf
/// [173]  	PN_LOCAL_ESC	  ::=  	'\' ( '_' | '~' | '.' | '-' | '!' | '$' | '&' | "'" | '(' | ')'
///                                     | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%' )
/// ```
fn prefixed_name_local_escape<'a, I>() -> impl Parser<I, Output = char>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (char('\\'), satisfy(is_local_escape))
        .map(|(_, escaped): (_, char)| escaped)
        .expected("PN_LOCAL_ESC")
}

#[inline]
fn whitespace<'a, I>() -> impl Parser<I, Output = ()>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    many1::<Vec<_>, _, _>(space())
        .with(value(()))
        .expected("IriRef")
}

// ------------------------------------------------------------------------------------------------

///
/// Spec:
///
/// ```abnf
/// [139]  	IRIREF	  ::=  	'<' ([^<>"{}|^`\]-[#x00-#x20])* '>'
/// ```
#[inline(always)]
fn is_iri_reserved(c: char) -> bool {
    matches!(
        c,
        '<' | '>' | '"' | '{' | '}' | '|' | '^' | '`' | '\\' | '\u{00}'..='\u{20}'
    )
}

///
/// Spec:
///
/// ```abnf
/// [173]  	PN_LOCAL_ESC	  ::=  	'\' ( '_' | '~' | '.' | '-' | '!' | '$' | '&' | "'" | '(' | ')'
///                                     | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%' )
/// ```
#[inline(always)]
fn is_local_escape(c: char) -> bool {
    matches!(
        c,
        '_' | '~'
            | '.'
            | '-'
            | '!'
            | '$'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | ';'
            | '='
            | '/'
            | '?'
            | '#'
            | '@'
            | '%'
    )
}

///
/// Spec:
///
/// ```abnf
/// [164]  	PN_CHARS_BASE	  ::=  	[A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6]
///                                       | [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF]
///                                       | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF]
///                                       | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD]
///                                       | [#x10000-#xEFFFF]
/// ```
#[inline(always)]
fn is_pn_chars_base(c: char) -> bool {
    matches!(c,
        'A'..='Z'
        | 'a'..='z'
        | '\u{C0}'..='\u{D6}'
        | '\u{D8}'..='\u{F6}'
        | '\u{F8}'..='\u{02FF}'
        | '\u{0370}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{010000}'..='\u{0EFFFF}')
}

///
/// Spec:
///
/// ```abnf
/// [165]  	PN_CHARS_U	  ::=  	PN_CHARS_BASE | '_'
/// ```
#[inline(always)]
fn is_pn_chars_u(c: char) -> bool {
    is_pn_chars_base(c) || c == '_'
}

///
/// Spec:
///
/// ```abnf
/// [167]  	PN_CHARS	  ::=  	PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
/// ```
#[inline(always)]
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c)
        || c.is_ascii_digit()
        || matches!(c,
            '-' | '\u{00B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
}

///
/// Spec:
///
/// ```abnf
/// [166]  	VARNAME	  ::=  	( PN_CHARS_U | [0-9] )
///                         ( PN_CHARS_U | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040] )*
/// ```
#[inline(always)]
fn is_var_name_head(c: char) -> bool {
    is_pn_chars_base(c) || c.is_ascii_digit()
}

///
/// Spec:
///
/// ```abnf
/// [166]  	VARNAME	  ::=  	( PN_CHARS_U | [0-9] )
///                         ( PN_CHARS_U | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040] )*
/// ```
#[inline(always)]
fn is_var_name_tail(c: char) -> bool {
    is_pn_chars_base(c)
        || c.is_ascii_digit()
        || matches!(c,
            '\u{00B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_trailing_ws() {
        let result = keyword("select").parse("select ").unwrap();
        assert_eq!(result, ((), " "));
    }

    #[test]
    fn test_keyword_lower() {
        let result = keyword("select").parse("select").unwrap();
        assert_eq!(result, ((), ""));
    }

    #[test]
    fn test_keyword_upper() {
        let result = keyword("select").parse("SELECT").unwrap();
        assert_eq!(result, ((), ""));
    }

    #[test]
    fn test_keyword_mixed() {
        let result = keyword("select").parse("seLEct").unwrap();
        assert_eq!(result, ((), ""));
    }

    #[test]
    fn test_no_keyword_empty() {
        let result = keyword("select").parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_keyword_partial() {
        let result = keyword("select").parse("sel");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_keyword_prefix_ws() {
        let result = keyword("select").parse(" select");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_keyword_more() {
        let result = keyword("select").parse("selectme");
        assert!(result.is_err());
    }

    #[test]
    fn test_variable() {
        let result = variable().parse("?a").unwrap();
        assert_eq!(result, (Variable::new_unchecked("a"), ""));
    }

    #[test]
    fn test_iri_ref_absolute_url() {
        let result = iri_ref().parse("<http://example.com/>").unwrap();
        assert_eq!(result, ("http://example.com/".to_string(), ""));
    }

    #[test]
    fn test_iri_ref_absolute_url_trailing_ws() {
        let result = iri_ref().parse("<http://example.com/>  ").unwrap();
        assert_eq!(result, ("http://example.com/".to_string(), ""));
    }

    #[test]
    fn test_iri_ref_uuid_urn() {
        let result = iri_ref()
            .parse("<urn:uuid:BA52DF7C-4983-4E4F-BF12-5132412D80AE>")
            .unwrap();
        assert_eq!(
            result,
            (
                "urn:uuid:BA52DF7C-4983-4E4F-BF12-5132412D80AE".to_string(),
                ""
            )
        );
    }

    #[test]
    fn test_base_decl() {
        let result = base_decl().parse("base <http://example.com/base>").unwrap();
        println!("{:#?}", result);
        assert_eq!(result, ("http://example.com/base".to_string(), ""));
    }

    #[test]
    fn test_prefix_decl() {
        let result = prefix_decl()
            .parse("prefix base: <http://example.com/base>")
            .unwrap();
        println!("{:#?}", result);
        assert_eq!(
            result,
            (
                (
                    Some("base".to_string()),
                    "http://example.com/base".to_string()
                ),
                ""
            )
        );
    }

    #[test]
    fn test_prefix_decl_empty() {
        let result = prefix_decl()
            .parse("prefix : <http://example.com/base>")
            .unwrap();
        assert_eq!(result, ((None, "http://example.com/base".to_string()), ""));
    }

    #[test]
    fn test_prefixed_name_prefix() {
        let result = prefixed_name_prefix().parse("base ").unwrap();
        assert_eq!(result, ("base".to_string(), " "));
    }

    #[test]
    fn test_prefixed_name_prefix_dotted() {
        let result = prefixed_name_prefix().parse("b.a.s.e").unwrap();
        assert_eq!(result, ("b.a.s.e".to_string(), ""));
    }

    #[test]
    fn test_prefixed_name_namespace() {
        let result = prefixed_name_namespace().parse("base: ").unwrap();
        assert_eq!(result, (Some("base".to_string()), ""));
    }

    #[test]
    fn test_prefixed_name_namespace_empty() {
        let result = prefixed_name_namespace().parse(": ").unwrap();
        assert_eq!(result, (None, ""));
    }

    #[test]
    fn test_iri_ref_relative_url() {
        let result = iri_ref().parse("<../index.htm>").unwrap();
        assert_eq!(result, ("../index.htm".to_string(), ""));
    }
}
