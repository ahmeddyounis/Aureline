# IME + dead-key text input fixtures

These fixtures exercise the portable text-input normalizer in
`crates/aureline-input/src/text_input/`.

## Format

Each JSON file contains a `steps` array. Every step provides exactly one of:

- `ime`: an `ImeEvent`
- `key`: a `TextKeyEvent`

The step also includes `expected`, which is either:

- a `TextInputAction` object, or
- `null` (meaning no action should be produced for that step)

The fixture runner lives in `crates/aureline-input/tests/ime_and_deadkey_cases.rs`.

