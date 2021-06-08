use pest::iterators::Pair;
use pest::RuleType;
use rdftk_core::error::{Error as CoreError, ErrorKind};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ParserErrorFactory {
    pub(crate) repr: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct ParserError {
    repr: String,
    fn_name: String,
    rule: Option<String>,
    expecting: Option<String>,
    unreachable: bool,
    context: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

macro_rules! unexpected {
    ($fn_name:expr, $pair:expr) => {{
        error!("ParserError::unexpected({}, {:?})", $fn_name, $pair);
        return Err(ERROR.error($fn_name).unexpected(&$pair).clone().into());
    }};
}

#[allow(unused_macros)]
macro_rules! unreachable {
    ($fn_name:expr) => {{
        error!("ParserError::unreachable({)", $fn_name);
        return ERROR.error($fn_name).unreachable().into();
    }};
}

#[allow(unused_macros)]
macro_rules! expecting {
    ($fn_name:expr, $rule:expr) => {{
        error!("ParserError::new({}, {:?})", $fn_name, $rule);
        return ERROR
            .error($fn_name)
            .expecting(stringify!($rule.to_string()))
            .into();
    }};
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ParserErrorFactory {
    pub(crate) fn error(&self, fn_name: &str) -> ParserError {
        ParserError {
            repr: self.repr.to_string(),
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: false,
            context: None,
        }
    }
    pub(crate) fn parser<R: 'static + Copy + Debug + Hash + Ord + Send>(
        &self,
        e: ::pest::error::Error<R>,
    ) -> CoreError {
        CoreError::with_chain(e, ErrorKind::ReadWrite(self.repr.to_string()))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            format!(
                "{}{}{}{}",
                &self.fn_name,
                match &self.rule {
                    None => String::new(),
                    Some(s) => format!(", rule: {}", s),
                },
                match &self.expecting {
                    None => String::new(),
                    Some(s) => format!(", expecting: {}", s),
                },
                if self.unreachable {
                    ", should have been unreachable".to_string()
                } else {
                    String::new()
                },
            ),
            match &self.context {
                None => String::new(),
                Some(s) => format!(", context: '{}'", s),
            }
        )
    }
}

impl std::error::Error for ParserError {}

impl Into<CoreError> for ParserError {
    fn into(self) -> CoreError {
        CoreError::with_chain(self.clone(), ErrorKind::ReadWrite(self.repr))
    }
}

#[allow(dead_code)]
impl ParserError {
    pub(crate) fn unexpected<T: RuleType>(&mut self, pair: &Pair<'_, T>) -> &mut Self {
        self.context = Some(format!("{:?}: {:?}", pair.as_rule(), pair.as_str()));
        self
    }

    pub(crate) fn unreachable(&mut self) -> &mut Self {
        self.unreachable = true;
        self
    }

    pub(crate) fn in_rule(&mut self, rule: &str) -> &mut Self {
        self.rule = Some(rule.to_string());
        self
    }

    pub(crate) fn expecting(&mut self, expecting: &str) -> &mut Self {
        self.expecting = Some(expecting.to_string());
        self
    }

    pub(crate) fn unreachable_rule(&mut self) -> &mut Self {
        self.unreachable = true;
        self
    }

    pub(crate) fn context(&mut self, context: &dyn Display) -> &mut Self {
        self.context = Some(format!("{}", context));
        self
    }

    pub(crate) fn debug_context(&mut self, context: &dyn Debug) -> &mut Self {
        self.context = Some(format!("{:?}", context));
        self
    }
}
