# M5 Manifest / Build Component Accessibility Fallback Contract (M05-818)

This contract certifies — per frozen manifest / build component family — that a
manifest / build lane stays **inspectable and honestly narrowed rather than
falsely executable** whenever schema freshness, connector state, adapter
capability, or target-graph truth is partial, stale, or policy-blocked. It is the
accessibility-hardening capstone over the frozen
[M5 manifest / build component matrix](m5_manifest_build_component_matrix.md)
(M05-812) and its 813–817 implementation / consumer lanes.

- **Rust module:** `crates/aureline-infra/src/add_manifest_build_component_accessibility_keyboard_screen_reader_cli_export_parity_and_auto_narrowing/`
- **Boundary schema:** [`schemas/ui/m5-manifest-build-component-accessibility-fallback.schema.json`](../../schemas/ui/m5-manifest-build-component-accessibility-fallback.schema.json)
- **Support export (canonical):** [`artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json`](../../artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json)
- **Matrix CSV / report:** `artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/{matrix.csv,report.md}`
- **Fixtures:** [`fixtures/ui/m5-manifest-build-component-accessibility-fallback/`](../../fixtures/ui/m5-manifest-build-component-accessibility-fallback/)
- **Emit bin:** `cargo run -p aureline-infra --bin emit_manifest_build_component_accessibility_fallback_fixture -- {support|csv|summary}`

## Scope

The packet carries one `ComponentAccessibilityRow` per frozen
`M5ManifestBuildComponentFamily` (all 10). Each row reuses the frozen matrix
vocabulary — family, required labels, downgrade triggers, schema freshness,
adapter source kind, and discovery confidence — rather than minting synonyms, so
the certified labels and confidence states stay byte-identical to the matrix and
the sibling primitive packets. The packet is **metadata-only**: raw manifests,
adapter payloads, credentials, and provider bodies never cross the boundary; only
typed class tokens, opaque summary / evidence refs, booleans, target IDs, and
redacted labels are recorded.

## Axes

Each row is certified on five axes:

1. **Keyboard / screen-reader / CLI reach** (`keyboard_reach`, `screen_reader_reach`,
   `cli_reach`, tri-state `NonVisualReachState`). Target-context chips, resource
   rows, adapter badges, target-graph rows, and fallback-confidence drawers are
   each keyboard-complete, screen-reader-reachable, and reachable from a headless
   CLI / export surface — never a view-only graph / matrix that traps
   assistive-tech or headless users. Visual-heavy families (`target_graph_row`,
   `capability_matrix`) also bind their visual surface to a list / table / textual
   modality.
2. **Interactive-claim honesty** (`baseline_claim`, `granted_claim`,
   `claim_affordance`, backed by `truth_signals`). When schema freshness, connector
   state, adapter capability, or discovery confidence weakens — or a policy block
   applies — the interactive claim auto-narrows down the ladder
   `fully_executable → review_required → read_only → inspect_only`. A weak-truth
   lane can never present as `fully_executable`; a claim that exceeds what the
   current truth supports is `overclaimed` (red).
3. **Export parity** (`export_summary`, `copy_export`, `target_id`,
   `target_context_ref`). The CLI / support / release export carries the same
   target IDs, target-context refs, schema-freshness, adapter-source, and
   confidence states shown in-product, is copyable as text / JSON / Markdown, and
   never relies on a screenshot to carry meaning.
4. **Honest auto-narrowing** (`auto_narrow`, `narrowing_disclosures`). A narrowed
   component discloses why with a precise frozen `M5ManifestBuildDowngradeTrigger`
   and preserves the key target context rather than silently dropping it; every
   narrower rendering surface discloses its reduction and keeps its labels.
5. **Cross-surface alignment** (`consumer_surfaces`). Each row is ingested by ≥2
   consumer surfaces, and across the packet the same narrowed states surface in
   UI, docs / help, release packets, and support / incident triage.

## Derived status

`ComponentAccessibilityStatus` rolls the axes up:

- **`parity` (green):** full reach / claim / export parity, no narrowing.
- **`narrowed_disclosed` (yellow):** reduced but fully disclosed, reachable, and
  honestly auto-narrowed.
- **`stranded` (red):** strands assistive-tech / CLI, overclaims execution, drops
  target truth from the export, drops context silently, or omits a mandatory
  label. Red rows may not ship.

## Acceptance criteria

- **AC1 — a stale or partial lane can no longer present as fully executable or
  fully current.** `claim_is_honest()` caps the granted claim at what
  `truth_signals.max_supported_tier()` allows (weak truth ⇒ at most
  `review_required`), rejects any claim above its baseline, and requires the
  declared `claim_affordance` to match the actual narrowing. Overclaimed rows are
  stranded.
- **AC2 — accessibility and export surfaces preserve the same target IDs,
  contexts, and confidence states shown in-product.**
  `reaches_target_backed_truth_via_at()` forbids view-only traps and requires a
  non-visual fallback for visual-heavy families; `export_preserves_meaning()`
  requires a non-screenshot summary that carries `target_id`,
  `target_context_ref`, and the confidence-bearing `copy_export` fields.
- **AC3 — claim publication and field triage stay aligned on downgrade
  behavior.** `field_triage_and_publication_aligned()` requires the packet to reach
  `incident_support`, `release_proof`, and `docs_help`, and every narrowed state is
  disclosed identically across those surfaces via `narrowing_disclosed()`.

## Seeded certification

The checked-in packet certifies all 10 families: **6 green / 4 yellow / 0 red**.
The yellow rows demonstrate honest auto-narrowing under each weakness class:

| Row | Trigger | Baseline → granted |
| --- | --- | --- |
| `manifest_editor_header` | `schema_stale` | fully_executable → review_required |
| `resource_link_row` | `connector_loss` | fully_executable → read_only |
| `target_graph_row` | `low_confidence_discovery` | fully_executable → review_required |
| `capability_matrix` | `policy_block` | fully_executable → inspect_only |

## Verification

```
cargo test -p aureline-infra --lib add_manifest_build_component_accessibility
cargo run -p aureline-infra --bin emit_manifest_build_component_accessibility_fallback_fixture -- support
```

`on_disk_export_matches_builder` pins the checked-in support export to the seeded
builder, so the artifact, the fixtures, and the module stay byte-aligned.
