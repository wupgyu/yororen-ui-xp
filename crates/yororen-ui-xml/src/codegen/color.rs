use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::AstAttribute;
use crate::codegen::errors::invalid_attr;
use crate::error::XmlError;

/// Parse a hex colour literal (`#rrggbb` or `#rrggbbaa`) and
/// emit the corresponding gpui constructor. Rejects other
/// literal forms and points the user toward a brace expression.
pub(crate) fn parse_hex_color(raw: &str, attr: &AstAttribute) -> Result<TokenStream, XmlError> {
    let hex = raw.strip_prefix('#').ok_or_else(|| {
        invalid_attr(
            attr,
            format!(
                "attribute `{}` expects a hex colour (`#rrggbb` or `#rrggbbaa`) or a brace expression like `{{gpui::hsla(...)}}`; got `{raw}`",
                attr.name
            ),
        )
    })?;

    let value = match hex.len() {
        6 | 8 => u32::from_str_radix(hex, 16),
        _ => {
            return Err(invalid_attr(
                attr,
                format!(
                    "attribute `{}` expects `#rrggbb` or `#rrggbbaa`, got `{raw}`",
                    attr.name
                ),
            ));
        }
    }
    .map_err(|_| {
        invalid_attr(
            attr,
            format!(
                "attribute `{}` expects a valid hex colour, got `{raw}`",
                attr.name
            ),
        )
    })?;

    match hex.len() {
        6 => Ok(quote! { ::gpui::rgb(#value) }),
        8 => Ok(quote! { ::gpui::rgba(#value) }),
        _ => unreachable!("non-6/8 lengths are rejected above"),
    }
}
