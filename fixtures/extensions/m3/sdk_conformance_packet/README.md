# SDK conformance packet fixtures

Input fixtures for the SDK conformance packet generator at
[`tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py`](../../../../tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py).

Each fixture pins exactly one SDK / runtime line and one expected
outcome. Drift between the fixture's declared expectations and the
underlying source rows (validator report, sample-pack record,
lifecycle packet, bridge matrix, docs-freshness sweep) fails closed at
generation time and emits the typed non-green reason on the packet.

| Fixture | Decision class | Reason class |
|---|---|---|
| [`ready_for_authors_beta_line.json`](./ready_for_authors_beta_line.json) | `ready_for_authors` | `all_claimed_surfaces_available_in_beta` |
| [`partially_ready_preview_beta_line.json`](./partially_ready_preview_beta_line.json) | `partially_ready_preview_surfaces_only` | `some_claimed_surfaces_preview_in_beta` |
| [`refused_docs_freshness_drift.json`](./refused_docs_freshness_drift.json) | `refused_inconsistent_input` | `docs_freshness_drift_detected` |
| [`refused_bridge_matrix_missing_required_state.json`](./refused_bridge_matrix_missing_required_state.json) | `refused_inconsistent_input` | `bridge_matrix_missing_required_state` |

The canonical artifacts checked in at
[`artifacts/extensions/m3/sdk_conformance_packet.json`](../../../../artifacts/extensions/m3/sdk_conformance_packet.json),
[`artifacts/extensions/m3/sdk_conformance_packet.md`](../../../../artifacts/extensions/m3/sdk_conformance_packet.md),
and
[`artifacts/extensions/m3/bridge_compatibility_scorecard.json`](../../../../artifacts/extensions/m3/bridge_compatibility_scorecard.json)
are emitted from `ready_for_authors_beta_line.json`. The other
fixtures exist so the generator's refusal paths are inspectable
without mutating the canonical artifacts.

The synthetic bridge matrix at
[`synthetic_bridge_matrix_missing_states.yaml`](./synthetic_bridge_matrix_missing_states.yaml)
is the input the bridge-state-missing fixture references. It only
declares a native lane so the generator records
`bridge_matrix_missing_required_state` and the packet refuses closed.
