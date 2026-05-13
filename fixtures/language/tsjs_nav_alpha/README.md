# TS/JS Navigation Alpha Fixtures

These protected fixtures exercise the first TypeScript/JavaScript hover,
definition, references, and rename-preview path in `aureline-language`.

The fixture set is synthetic and export-safe. It uses opaque refs, workspace
scope labels, provider health states, and range refs instead of raw private
source bodies. The cases cover a ready language-service lane, a sparse active
workset with omitted generated/outside-root candidates, and an unavailable
provider that must fall back to file-local syntax/text truth.

Verify with:

```sh
cargo test -p aureline-language --test tsjs_nav_alpha
python3 -m json.tool fixtures/language/tsjs_nav_alpha/wedge_cases.json >/dev/null
```
