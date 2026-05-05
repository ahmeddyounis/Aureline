# Three-layer language model worked fixtures

These YAML fixtures exercise the three-layer language model contract frozen in:

- [`/docs/language/three_layer_model_contract.md`](../../../docs/language/three_layer_model_contract.md)
- [`/artifacts/language/layer_matrix.yaml`](../../../artifacts/language/layer_matrix.yaml)

Each fixture is a single case record that:

- names a `feature_family` (hover/completion/diagnostics/rename/…),
- names a `surface_family` (editor/search/navigation/refactor/docs/review/ai),
- declares the requested depth, the answering layer, and the downgrade reason,
- enumerates the minimum disclosure that the surface must render.

The corpus uses opaque provider/epoch/scope handles and typed layer ids. No
fixture includes raw source text, raw provider logs, raw hostnames, raw URLs,
or secret material.

## Cases

| Fixture | Scenario it freezes |
|---|---|
| `hover_syntax_only.yaml` | Hover answered by Layer 1 with explicit “semantic depth unavailable” disclosure. |
| `completion_compatibility_lsp.yaml` | Completion answered by Layer 2 with compatibility labeling and deeper-layer unavailability. |
| `diagnostics_semantic_depth_graph_overlay.yaml` | Diagnostics answered by Layer 3 while still disclosing Layer 2 contributors. |
| `rename_semantic_depth_preview.yaml` | Rename answered by Layer 3 with preview/rollback posture and impact scope disclosure. |
| `rename_text_fallback.yaml` | Rename downgraded to Layer 1 text-only behavior with strict warnings. |
| `docs_claim_must_name_layers.yaml` | Docs/onboarding claim example that forbids “supports language X” without layer naming. |

