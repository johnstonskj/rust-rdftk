use nom::bytes::complete::{tag, take_while};
use nom::combinator::{complete, opt, verify};
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_prefixed_name<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (Option<String>, Option<String>), E> {
    let (_, (prefix, _, local)): (&str, (Option<String>, &str, Option<&str>)) =
        prefixed_name(input)?;
    Ok(("", (prefix, local.map(|s| s.to_string()))))
}

fn prefixed_name<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (Option<String>, &str, Option<&str>), E> {
    tuple((
        opt(pn_prefix), // complete(pn_chars_base)
        tag(":"),
        opt(complete(pn_chars_base)),
    ))(input)
}

fn pn_prefix<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E> {
    let (remainder, first) = pn_chars_base(input)?;
    let (remainder, rest) = verify(pn_chars_dot, |s: &str| !s.ends_with('.'))(remainder)?;
    Ok((remainder, format!("{}{}", first, rest)))
}

fn pn_local<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E> {
    let (remainder, first) = pn_chars_base(input)?;
    let (remainder, rest) = verify(pn_chars_dot, |s: &str| !s.ends_with('.'))(remainder)?;
    Ok((remainder, format!("{}{}", first, rest)))
}

named!(
    _pn_local(&str) -> &str,
    recognize!(
        do_parse!(
            pn_local_first >>
            opt!(pn_local_rest) >>
            ()
        )
    )
);

named!(
    pn_local_first(&str) -> &str,
    recognize!(
        do_parse!(
            alt!(
                take_while_m_n!(1, 1, is_pn_local_first)
                | percent_escape
                | local_escape) >>
            many0!(
                alt!(
                    take_while_m_n!(1, 1, is_pn_local_rest)
                    | percent_escape
                    | local_escape)) >>
        ())
    )
);

named!(
    pn_local_rest(&str) -> &str,
    recognize!(
        do_parse!(
            opt!(
                alt!(
                    take_while_m_n!(1, 1, is_pn_local_rest)
                    | percent_escape
                    | local_escape)) >>
            ()
        )
    )
);

named!(
    percent_escape(&str) -> &str,
    recognize!(
        do_parse!(
            tag!("%") >>
            take_while_m_n!(2, 2, is_hex) >>
            ()
        )
    )
);

named!(
    local_escape(&str) -> &str,
    recognize!(
        do_parse!(
            tag!("\\") >>
            one_of!("_~.-!$&'()*+,;=/?#@%") >>
            ()
        )
    )
);

#[inline]
fn is_hex(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn pn_chars_base<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    take_while(move |c: char| {
        c.is_ascii_alphabetic()
            || (c >= '\u{00C0}' && c <= '\u{00D6}')
            || (c >= '\u{00D8}' && c <= '\u{00F6}')
            || (c >= '\u{00F8}' && c <= '\u{02FF}')
            || (c >= '\u{0370}' && c <= '\u{037D}')
            || (c >= '\u{037F}' && c <= '\u{01FFF}')
            || (c >= '\u{200C}' && c <= '\u{200D}')
            || (c >= '\u{2070}' && c <= '\u{218F}')
            || (c >= '\u{2C00}' && c <= '\u{2FEF}')
            || (c >= '\u{3001}' && c <= '\u{D7FF}')
            || (c >= '\u{F900}' && c <= '\u{FDCF}')
            || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
            || (c >= '\u{10000}' && c <= '\u{EFFFF}')
    })(input)
}

fn pn_chars_u<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    take_while(move |c: char| {
        c.is_ascii_alphabetic()
            || (c >= '\u{00C0}' && c <= '\u{00D6}')
            || (c >= '\u{00D8}' && c <= '\u{00F6}')
            || (c >= '\u{00F8}' && c <= '\u{02FF}')
            || (c >= '\u{0370}' && c <= '\u{037D}')
            || (c >= '\u{037F}' && c <= '\u{01FFF}')
            || (c >= '\u{200C}' && c <= '\u{200D}')
            || (c >= '\u{2070}' && c <= '\u{218F}')
            || (c >= '\u{2C00}' && c <= '\u{2FEF}')
            || (c >= '\u{3001}' && c <= '\u{D7FF}')
            || (c >= '\u{F900}' && c <= '\u{FDCF}')
            || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
            || (c >= '\u{10000}' && c <= '\u{EFFFF}')
            // additional ---------------------------
            || c == '_'
    })(input)
}

fn pn_chars<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    take_while(move |c: char| {
        c.is_ascii_alphabetic()
            || (c >= '\u{00C0}' && c <= '\u{00D6}')
            || (c >= '\u{00D8}' && c <= '\u{00F6}')
            || (c >= '\u{00F8}' && c <= '\u{02FF}')
            || (c >= '\u{0370}' && c <= '\u{037D}')
            || (c >= '\u{037F}' && c <= '\u{01FFF}')
            || (c >= '\u{200C}' && c <= '\u{200D}')
            || (c >= '\u{2070}' && c <= '\u{218F}')
            || (c >= '\u{2C00}' && c <= '\u{2FEF}')
            || (c >= '\u{3001}' && c <= '\u{D7FF}')
            || (c >= '\u{F900}' && c <= '\u{FDCF}')
            || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
            || (c >= '\u{10000}' && c <= '\u{EFFFF}')
            || c == '_'
            // additional ---------------------------
            || c == '-'
            || c.is_ascii_digit()
            || c == '\u{00B7}'
            || (c >= '\u{0300}' && c <= '\u{036F}')
            || (c >= '\u{203F}' && c <= '\u{2040}')
    })(input)
}

fn pn_chars_dot<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    take_while(move |c: char| {
        c.is_ascii_alphabetic()
            || (c >= '\u{00C0}' && c <= '\u{00D6}')
            || (c >= '\u{00D8}' && c <= '\u{00F6}')
            || (c >= '\u{00F8}' && c <= '\u{02FF}')
            || (c >= '\u{0370}' && c <= '\u{037D}')
            || (c >= '\u{037F}' && c <= '\u{01FFF}')
            || (c >= '\u{200C}' && c <= '\u{200D}')
            || (c >= '\u{2070}' && c <= '\u{218F}')
            || (c >= '\u{2C00}' && c <= '\u{2FEF}')
            || (c >= '\u{3001}' && c <= '\u{D7FF}')
            || (c >= '\u{F900}' && c <= '\u{FDCF}')
            || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
            || (c >= '\u{10000}' && c <= '\u{EFFFF}')
            || c == '_'
            || c == '-'
            || c.is_ascii_digit()
            || c == '\u{00B7}'
            || (c >= '\u{0300}' && c <= '\u{036F}')
            || (c >= '\u{203F}' && c <= '\u{2040}')
            // additional ---------------------------
            || c == '.'
    })(input)
}

#[inline]
fn is_pn_chars_base(c: char) -> bool {
    c.is_ascii_alphabetic()
        || (c >= '\u{00C0}' && c <= '\u{00D6}')
        || (c >= '\u{00D8}' && c <= '\u{00F6}')
        || (c >= '\u{00F8}' && c <= '\u{02FF}')
        || (c >= '\u{0370}' && c <= '\u{037D}')
        || (c >= '\u{037F}' && c <= '\u{01FFF}')
        || (c >= '\u{200C}' && c <= '\u{200D}')
        || (c >= '\u{2070}' && c <= '\u{218F}')
        || (c >= '\u{2C00}' && c <= '\u{2FEF}')
        || (c >= '\u{3001}' && c <= '\u{D7FF}')
        || (c >= '\u{F900}' && c <= '\u{FDCF}')
        || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
        || (c >= '\u{10000}' && c <= '\u{EFFFF}')
}

#[inline]
fn is_pn_chars_u(c: char) -> bool {
    is_pn_chars_base(c) || c == '_'
}

#[inline]
fn is_pn_prefix_1(c: char) -> bool {
    is_pn_chars(c) || c == '.'
}

#[inline]
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c)
        || c == '-'
        || c.is_ascii_digit()
        || c == '\u{00B7}'
        || (c >= '\u{0300}' && c <= '\u{036F}')
        || (c >= '\u{203F}' && c <= '\u{2040}')
}

#[inline]
fn is_pn_local_first(c: char) -> bool {
    is_pn_chars_u(c) || c == ':' || c.is_ascii_digit()
}

#[inline]
fn is_pn_local_rest(c: char) -> bool {
    is_pn_chars(c) || c == '.' || c == ':'
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::convert_error;
    use nom::Err;

    fn test_this(s: &str) {
        println!("input: {}", s);
        let result = parse_prefixed_name(s);
        match result {
            Ok((rest, (prefix, local))) => println!(
                "success [{:?} : {:?}] (remainder: {:?})",
                prefix, local, rest
            ),
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!(
                    "verbose errors - `parse_prefixed_name::<VerboseError>({})`:\n{}",
                    s,
                    convert_error(s, e)
                );
            }
            Err(Err::Incomplete(needed)) => {
                println!("incomplete input, {:?} more input needed - `parse_prefixed_name::<VerboseError>({})`",
                         needed, s);
            }
        }
    }

    #[test]
    fn test_basic_parser() {
        test_this("foo:bar");
        test_this("foo:");
        test_this(":bar");
        test_this(":");
    }
}
