# Beta support-matrix input fixtures

These fixtures are the typed input rows migration, partner, and release
packets feed into the
[`aureline_runtime::SupportMatrixBetaManifest`](../../../../crates/aureline-runtime/src/support_matrix_beta/mod.rs)
canonical matrix. Every fixture is replayed end-to-end by
[`/crates/aureline-runtime/tests/support_matrix_beta.rs`](../../../../crates/aureline-runtime/tests/support_matrix_beta.rs)
and asserted byte-for-byte against the canonical row for the same wedge.

The canonical matrix is published at
[`/artifacts/compat/m3/debug_execution_matrix.md`](../../../../artifacts/compat/m3/debug_execution_matrix.md);
the reviewer-facing companion doc is at
[`/docs/runtime/m3/language_runtime_support_beta.md`](../../../../docs/runtime/m3/language_runtime_support_beta.md).

| Fixture | Wedge | Launch | Attach | Test | Execution-context rollup |
|---|---|---|---|---|---|
| `python.json` | `python` | `supported` | `preview` | `supported` (claimed framework: `pytest`) | `preview` |
| `typescript_javascript.json` | `typescript_javascript` | `preview` | `preview` | `limited` (no claimed framework) | `limited` |

Adding a wedge, a lane, a support class, or a downgrade rule is a vocabulary
change that MUST update the canonical manifest, the schema, the reviewer
doc, the markdown matrix, and these fixtures together.
