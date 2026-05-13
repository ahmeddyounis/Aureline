# Python Navigation Alpha Fixtures

These protected fixtures exercise the first Python hover, definition,
references, and rename-preview path in `aureline-language`.

The fixture set is synthetic and export-safe. It uses opaque refs, workspace
scope labels, interpreter/environment refs, provider health states, and range
refs instead of raw private source bodies. The cases cover a ready Python
language-service lane, a sparse active workset with omitted generated/outside
root candidates, an unavailable provider, a host bound to the wrong
interpreter, and an unresolved interpreter selection.

Verify with:

```sh
cargo test -p aureline-language --test python_nav_alpha
python3 -m json.tool fixtures/language/python_nav_alpha/wedge_cases.json >/dev/null
```
