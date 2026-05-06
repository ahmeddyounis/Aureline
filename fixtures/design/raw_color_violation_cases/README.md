# Raw-color violation fixtures

These fixtures are used by `tools/ci/check_semantic_token_conformance.py` to
prove that the semantic-token conformance gate can:

- detect raw-color literals in consuming surface code; and
- detect cross-domain token borrowing when a surface declares a domain intent.

The fixtures are **synthetic**: they embed small code snippets under
`source_text` instead of pointing at real source files.
