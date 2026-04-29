# Test item identity worked cases

These fixtures anchor the contract in
[`/docs/testing/test_item_identity_contract.md`](../../../docs/testing/test_item_identity_contract.md)
and validate against:

- [`/schemas/testing/test_item_identity.schema.json`](../../../schemas/testing/test_item_identity.schema.json)
- [`/schemas/testing/test_selector_grammar.schema.json`](../../../schemas/testing/test_selector_grammar.schema.json)

The fixture set covers:

| Fixture | Record kind | Key coverage |
|---|---|---|
| [`native_adapter_row.yaml`](./native_adapter_row.yaml) | `test_item_identity_record` | native adapter identity with logical suite/case path, source span, adapter/provider identity, target environment, display separation, and exact selector export |
| [`imported_ci_result_remap.yaml`](./imported_ci_result_remap.yaml) | `test_item_identity_remap_record` | provider-imported CI row crosswalked to a local identity while preserving read-only imported evidence |
| [`parameterized_family.yaml`](./parameterized_family.yaml) | `parameterized_expansion_record` | collapsed parameterized family with loaded, failed, skipped, and hidden instance counts plus exact failing-instance refs |
| [`tag_based_selector.yaml`](./tag_based_selector.yaml) | `test_selector_expression_record` | tag selector combined with a capability trait selector, import-safe escaping, matched canonical identities, and omitted policy scope |
| [`renamed_test_file.yaml`](./renamed_test_file.yaml) | `test_item_identity_remap_record` | exact source-anchor remap after a test file rename, preserving the canonical id and affected selector refs |

Fixtures MUST NOT encode raw command lines, raw stdout or stderr byte
streams, raw environment bodies, raw absolute paths, raw URLs, raw
secret values, raw assertion bodies, raw source excerpts, raw artifact
bytes, or raw stack traces. They use opaque refs, digests, counts,
class labels, bounded display summaries, and timestamps.
