# TS/JS Quality Alpha Fixtures

Protected fixtures for the TypeScript/JavaScript formatter, linter, and
test-adapter quality wedge.

The JSON fixture covers:

- a missing formatter where lint and test hooks remain usable;
- a policy-blocked linter where formatter and test hooks remain usable;
- normalized diagnostic bus rows for formatter, linter, and test-adapter
  signals; and
- execution-plane task hooks with command ids, diagnostic refs, and normalized
  task-event refs.

Run:

```sh
cargo test -p aureline-language --test tsjs_quality_alpha
python3 -m json.tool fixtures/language/tsjs_quality_alpha/quality_cases.json >/dev/null
```
