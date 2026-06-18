#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The preprocessor only accepts Rust source strings, so discard invalid
    // UTF-8. This still exercises the full valid-UTF-8 input space.
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = yororen_ui_xml::parser::fuzzing::normalise_bool_attrs(s);
    }
});
