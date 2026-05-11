# Proof packet: representation-labeled safe preview and copy/export wedge

Purpose: anchor proof captures for the M1 bounded prototype wedge that lands
the first explicit raw-versus-rendered and safe-preview model on one live
shell row. The wedge covers three risky preview lanes (risky text, oversized
artifacts, generated content), pins a typed copy/export option set on every
preview, and surfaces a closed `SafePreviewInvariantViolation` vocabulary
through the named shell consumer so a buggy preview can never silently
substitute rendered output for raw or escaped.

Reviewer landing page:
[`docs/ux/m1_safe_preview_and_copy_export.md`](../../../../docs/ux/m1_safe_preview_and_copy_export.md).

Canonical sources:

- Crate: `crates/aureline-preview/`
  - `src/lib.rs` — public re-exports
  - `src/safe_preview/mod.rs` — `SafePreviewRecord`, content / origin /
    representation / scope / transform / omission vocabularies, the three
    named builders (`build_risky_text_preview`,
    `build_oversized_artifact_preview`, `build_generated_content_preview`),
    and the `SafePreviewRecord::validate` invariants.
- Crate (consumer): `crates/aureline-shell/`
  - `src/safe_preview_card/mod.rs` — `SafePreviewCardSnapshot` projection
    with seven canonical sections, typed row addresses, and a deterministic
    plaintext render.
- Reviewer landing page:
  `docs/ux/m1_safe_preview_and_copy_export.md`
- Fixture suite:
  `fixtures/preview/representation_cases/`
- Tests (named-consumer wiring):
  - `crates/aureline-preview/src/safe_preview/tests.rs`
  - `crates/aureline-shell/src/safe_preview_card/tests.rs`
  - `crates/aureline-shell/tests/safe_preview_card_cases.rs`

Upstream contracts the seed projects against (without forking):

- `docs/ux/copy_export_representation_parity.md` /
  `schemas/ux/representation_copy_export.schema.json` — the frozen
  representation-action / representation-class / scope / transform /
  omission vocabulary the wedge mirrors verbatim.
- `docs/security/safe_preview_trust_classes.md` /
  `crates/aureline-content-safety/` — the closed trust-class,
  suspicious-content, and representation-action vocabularies.
- `docs/ux/shell_interaction_safety_contract.md` — the shell-wide
  representation-bearing copy/export rules; the wedge is one bounded
  surface that quotes these rules.

## Protected walks

Three fixtures drive the protected walks end to end:

1. **Risky text** —
   `fixtures/preview/representation_cases/01_risky_text_bidi_identifier.json`.
   A bidi-override / zero-width-joiner identifier renders as `escaped`;
   `copy_raw` and `copy_escaped` are paired through `must_offer_also`;
   `export_metadata_only` is the support-safe fallback.
2. **Oversized artifact** —
   `fixtures/preview/representation_cases/02_oversized_log_capture_windowed.json`.
   A 12.5 MB log capture renders a 64 KB / 1 024-line window; the
   `copy_rendered` option names `scope_class=visible_rows_or_events` and
   carries the `truncated_or_windowed` + `buffered_unseen_excluded`
   transforms; the omission summary publishes
   `omitted_bytes_estimate=12_436_000`.
3. **Generated content** —
   `fixtures/preview/representation_cases/03_generated_change_summary.json`.
   A generated change summary pins `origin_class=generated` and
   `currently_visible_representation=generated`; `copy_rendered` carries
   the `citation_anchors` disclosure; `copy_raw` is offered only for the
   canonical-source bytes and only when at least one citation anchor
   backs the option.

Evidence:
`crates/aureline-shell/tests/safe_preview_card_cases.rs::protected_walk_risky_text_fixture_drives_full_card`,
`crates/aureline-shell/tests/safe_preview_card_cases.rs::protected_walk_oversized_fixture_carries_window_scope_and_omission`,
`crates/aureline-shell/tests/safe_preview_card_cases.rs::protected_walk_generated_fixture_requires_citation_anchors_for_copy_raw`,
`crates/aureline-preview/src/safe_preview/tests.rs::protected_walk_risky_text_offers_raw_and_escaped_paired`,
`crates/aureline-preview/src/safe_preview/tests.rs::oversized_windowed_preview_names_scope_and_omission`,
`crates/aureline-preview/src/safe_preview/tests.rs::generated_preview_pins_origin_and_currently_visible_representation`.

## Failure drill

`fixtures/preview/representation_cases/04_failure_drill_unlabeled_rendered_copy.json`
exercises the spec's named failure drill. A buggy preview replaces the
paired `copy_escaped` action with a `copy_rendered` button that pretends
to carry raw bytes. The drill confirms:

- `SafePreviewRecord::validate` surfaces both
  `missing_paired_action` (with `missing_action_id="copy_escaped"`) and
  `unlabeled_rendered_copy`;
- the `SafePreviewCardSnapshot` invariants section renders both rows as
  `Blocked` with the typed token quoted verbatim;
- `has_invariant_violations` lights so the chrome must surface the
  failure before letting copy/export proceed.

Evidence:
`crates/aureline-shell/tests/safe_preview_card_cases.rs::failure_drill_unlabeled_rendered_copy_fixture_surfaces_typed_violations`,
`crates/aureline-preview/src/safe_preview/tests.rs::failure_drill_risky_text_unlabeled_rendered_copy_is_rejected`,
`crates/aureline-preview/src/safe_preview/tests.rs::failure_drill_oversized_scope_overclaim_is_rejected`,
`crates/aureline-preview/src/safe_preview/tests.rs::failure_drill_generated_copy_raw_without_citation_is_rejected`,
`crates/aureline-preview/src/safe_preview/tests.rs::failure_drill_generated_origin_mislabel_is_rejected`.

## Validation command

```
cargo test -p aureline-preview && cargo test -p aureline-shell --lib safe_preview_card && cargo test -p aureline-shell --test safe_preview_card_cases
```

## Evidence storage

- Crate sources: `crates/aureline-preview/`,
  `crates/aureline-shell/src/safe_preview_card/`
- Reviewer doc: `docs/ux/m1_safe_preview_and_copy_export.md`
- Fixture suite: `fixtures/preview/representation_cases/`
- Tests:
  - `crates/aureline-preview/src/safe_preview/tests.rs`
  - `crates/aureline-shell/src/safe_preview_card/tests.rs`
  - `crates/aureline-shell/tests/safe_preview_card_cases.rs`
