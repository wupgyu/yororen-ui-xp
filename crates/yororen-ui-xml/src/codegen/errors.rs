//! Shared helpers for building [`XmlError`]s during codegen.
//!
//! Centralizing the "parse failed + span + byte offset" dance
//! keeps the individual codegen modules focused on the happy
//! path and makes error message formatting consistent.

use crate::ast::AstAttribute;
use crate::error::{XmlError, XmlErrorKind};

/// Build an `InvalidExpression` error for `attr` with a
/// fully custom message. The error is annotated with the
/// attribute's span and byte offset.
pub(crate) fn invalid_attr(attr: &AstAttribute, message: impl Into<String>) -> XmlError {
    XmlError::new(XmlErrorKind::InvalidExpression, attr.span, message).at(attr.byte_offset)
}

/// Build an `InvalidExpression` error for `attr` when the
/// literal value is not one of the accepted forms.
pub(crate) fn invalid_attr_expr(attr: &AstAttribute, expected: &str, got: &str) -> XmlError {
    invalid_attr(
        attr,
        format!("attribute `{}` expects {expected}, got `{got}`", attr.name),
    )
}

/// Parse `attr.raw` into `T` or return a formatted
/// `InvalidExpression` error.
pub(crate) fn parse_attr<T: std::str::FromStr>(
    attr: &AstAttribute,
    expected: &str,
) -> Result<T, XmlError> {
    attr.raw
        .parse::<T>()
        .map_err(|_| invalid_attr_expr(attr, expected, attr.raw.as_str()))
}

/// Look up `raw` in a static table of `(input, variant)`
/// pairs and return the matching variant name. The table
/// can map multiple accepted spellings (e.g. `"H1"` and
/// `"h1"`) to the same canonical variant.
pub(crate) fn parse_enum_variant<'a>(
    attr: &AstAttribute,
    raw: &str,
    variants: &[(&str, &'a str)],
    expected: &str,
) -> Result<&'a str, XmlError> {
    variants
        .iter()
        .find(|(input, _)| *input == raw)
        .map(|(_, output)| *output)
        .ok_or_else(|| invalid_attr_expr(attr, expected, raw))
}
