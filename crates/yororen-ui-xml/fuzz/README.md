# cargo-fuzz for `yororen-ui-xml`

## Running

```bash
cd crates/yororen-ui-xml
cargo +nightly fuzz run normalise_bool_attrs
```

## Sanitizer note (macOS)

The default `address` sanitizer conflicts with the `ctor`/`dtor` static
initializers used by dependencies such as `gpui-ce` and `inventory`, causing
link errors like:

```
ld: multiple errors: initializer pointer has no target ...
```

On macOS run without a sanitizer to exercise the panic/OOM surface:

```bash
cargo +nightly fuzz run normalise_bool_attrs --sanitizer none
```

## Fuzz target

- `normalise_bool_attrs`: feeds arbitrary valid UTF-8 strings into the
  byte-level preprocessor and checks for panics / unbounded growth.
