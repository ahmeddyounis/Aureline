# M1 representation-labeled safe preview and copy/export wedge

This page is the reviewer-facing landing page for the bounded prototype
that lands the first explicit raw-versus-rendered and safe-preview model
on one live shell row. The wedge lives at
[`crates/aureline-preview/src/safe_preview/`](../../crates/aureline-preview/src/safe_preview/mod.rs)
and the named shell consumer lives at
[`crates/aureline-shell/src/safe_preview_card/`](../../crates/aureline-shell/src/safe_preview_card/mod.rs).
The fixture-driven integration test lives at
[`crates/aureline-shell/tests/safe_preview_card_cases.rs`](../../crates/aureline-shell/tests/safe_preview_card_cases.rs).

The wedge is bounded:

- It covers three risky preview lanes (risky text, oversized artifacts,
  and generated content) on one live shell card. It does not rewrite the
  broader content viewer or own notebook / runtime / install-review
  rendering depth.
- It does not invent new representation, action, scope, transform, or
  omission tokens. Every closed vocabulary is mirrored verbatim from
  [`docs/ux/copy_export_representation_parity.md`](copy_export_representation_parity.md)
  and the boundary schema in
  [`schemas/ux/representation_copy_export.schema.json`](../../schemas/ux/representation_copy_export.schema.json).
- It does not silently widen authority. The
  [`SafePreviewRecord::validate`](../../crates/aureline-preview/src/safe_preview/mod.rs)
  invariants surface every representation-honesty rule the spec freezes,
  and the card snapshot surfaces each violation as an addressable
  blocked row.

## What the wedge owns

- One canonical [`SafePreviewRecord`](../../crates/aureline-preview/src/safe_preview/mod.rs)
  data shape, serialized verbatim for support exports and proof captures.
- A typed `ContentClass` enum that names which of the three M1 lanes
  the preview belongs to: `risky_text`, `oversized_artifact`,
  `generated_content`.
- A `CurrentlyVisibleRepresentation` enum that names which representation
  the user is presently looking at: `raw`, `rendered`, `escaped`,
  `sanitized`, `sandboxed`, `generated`, or `blocked_metadata_only`.
- A list of paired [`CopyExportOption`](../../crates/aureline-preview/src/safe_preview/mod.rs)
  rows each carrying the frozen
  [`aureline_content_safety::RepresentationActionId`](../../crates/aureline-content-safety/src/transfer.rs)
  and
  [`aureline_content_safety::RepresentationClass`](../../crates/aureline-content-safety/src/transfer.rs)
  vocabulary, plus typed `ScopeClass`, `TransformKind`, and
  `OmissionReason` tokens and an honest `ShareSafety` posture.
- A `PrototypeLabel::M1PrototypeSafePreviewAndCopyExport` chip carried on
  every preview. The chrome quotes the token verbatim; surfaces MUST NOT
  drop the chip even when the preview is nominally clean, because the
  wedge as a whole is a bounded prototype.
- Stable claim-limit strings (`bounded_prototype_only`,
  `risky_oversized_generated_lanes_only`,
  `transfer_actions_must_carry_representation`,
  `no_remote_or_publish_boundary_moves`) the chrome quotes verbatim.
- A typed `SafePreviewInvariantViolation` enum surfacing every
  representation-honesty rule:
  - `no_copy_export_options` — preview offers zero transfer actions.
  - `action_kind_mismatch` — an option's `action_kind` disagrees with its
    `action_id`.
  - `missing_representation_label` — an option drops the
    `representation_label` disclosure.
  - `missing_paired_action` — a risky-text preview is missing one of
    `copy_raw`, `copy_escaped`, or `export_metadata_only`.
  - `unpaired_risky_text_action` — risky text offers `copy_raw` or
    `copy_escaped` without listing the peer action in `must_offer_also`.
  - `unlabeled_rendered_copy` — risky text offers `copy_rendered` with a
    representation class other than `rendered`.
  - `generated_origin_mismatch` / `generated_visible_mismatch` — a
    generated preview reports an origin or currently visible
    representation other than `generated`.
  - `generated_copy_raw_without_citation` — a generated preview offers
    `copy_raw` without supplying citation-anchor refs.
  - `oversized_missing_window_transform` /
    `oversized_scope_overclaim` / `oversized_missing_omitted_bytes` —
    an oversized preview reports a windowed body but does not advertise
    the matching transform, scope, or omission estimate.

## Named consumer

The shell consumer at
[`crates/aureline-shell/src/safe_preview_card/`](../../crates/aureline-shell/src/safe_preview_card/mod.rs)
projects the preview record into a `SafePreviewCardSnapshot` carrying:

- seven always-present sections in canonical reading order:
  `prototype_label`, `header`, `currently_visible`, `body_extent`,
  `copy_export_options`, `claim_limits`, `invariants`;
- typed `SafePreviewRowAddress` values so the chrome's `Inspect` /
  `Copy` / `Export` / `Resolve` actions route to the matching option or
  invariant violation;
- a `has_invariant_violations` chip that lights when at least one rule
  fires so the chrome must surface the failure verbatim;
- a deterministic `render_plaintext()` block downstream exports quote
  verbatim.

## Protected walks

The fixtures in
[`fixtures/preview/representation_cases/`](../../fixtures/preview/representation_cases/)
drive the protected walks through the integration test
[`crates/aureline-shell/tests/safe_preview_card_cases.rs`](../../crates/aureline-shell/tests/safe_preview_card_cases.rs):

1. **Risky text** —
   [`01_risky_text_bidi_identifier.json`](../../fixtures/preview/representation_cases/01_risky_text_bidi_identifier.json).
   A bidi override + zero-width joiner inside an identifier:
   - the rendered representation is `escaped`;
   - the preview offers `copy_raw`, `copy_escaped`, and
     `export_metadata_only`;
   - `copy_raw` lists `copy_escaped` in `must_offer_also` and vice
     versa, so the chrome cannot drop the paired path;
   - the card snapshot's invariants section reads "All
     representation-honesty invariants satisfied."
2. **Oversized artifact** —
   [`02_oversized_log_capture_windowed.json`](../../fixtures/preview/representation_cases/02_oversized_log_capture_windowed.json).
   A 12.5 MB terminal-log capture with only the last 64 KB (1 024 lines)
   rendered:
   - the rendered representation is `rendered`;
   - `copy_rendered` scope reads `visible_rows_or_events`, transforms
     include `truncated_or_windowed` and `buffered_unseen_excluded`, and
     the omission summary publishes `omitted_bytes_estimate=12_436_000`
     and `omitted_line_count_estimate=199_232`;
   - `export_sanitized_snapshot` is offered for a sanitized static
     snapshot and `export_metadata_only` is offered for support exports.
3. **Generated content** —
   [`03_generated_change_summary.json`](../../fixtures/preview/representation_cases/03_generated_change_summary.json).
   A model-produced change summary:
   - the rendered representation is `generated`, the origin class is
     `generated`;
   - `copy_rendered` is offered with the `citation_anchors` disclosure
     and preserves the generated label;
   - `copy_raw` is offered only for the canonical-source bytes
     (`src/router.rs#fn:dispatch`) and only when at least one citation
     anchor backs the option;
   - `export_sanitized_snapshot` and `export_metadata_only` cover the
     support paths.

## Failure drill

[`04_failure_drill_unlabeled_rendered_copy.json`](../../fixtures/preview/representation_cases/04_failure_drill_unlabeled_rendered_copy.json)
exercises the spec's failure drill ("preview risky content and confirm
raw/rendered labels plus copy/export posture stay explicit"). A buggy
preview replaces the paired `copy_escaped` action with a `copy_rendered`
button that pretends to carry raw bytes. The drill confirms:

- `SafePreviewRecord::validate` surfaces
  `SafePreviewInvariantViolation::MissingPairedAction { missing_action_id: "copy_escaped" }`
  and `SafePreviewInvariantViolation::UnlabeledRenderedCopy { .. }`;
- the card snapshot's invariants section renders both rows as `Blocked`
  with the typed violation token quoted verbatim;
- `snapshot.has_invariant_violations` lights so the chrome must surface
  the failure before letting copy/export proceed.

The drill is exercised by
[`failure_drill_unlabeled_rendered_copy_fixture_surfaces_typed_violations`](../../crates/aureline-shell/tests/safe_preview_card_cases.rs)
and by
[`failure_drill_risky_text_unlabeled_rendered_copy_is_rejected`](../../crates/aureline-preview/src/safe_preview/tests.rs).

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`docs/ux/copy_export_representation_parity.md`](copy_export_representation_parity.md)
  — the frozen representation-class / action-id / scope / transform /
  omission vocabulary the wedge mirrors verbatim.
- [`schemas/ux/representation_copy_export.schema.json`](../../schemas/ux/representation_copy_export.schema.json)
  — boundary schema for parity cases and copy/export metadata records.
- [`docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  and [`crates/aureline-content-safety/`](../../crates/aureline-content-safety/)
  — the closed trust-class, suspicious-content, and representation-action
  vocabularies. The wedge reads the detector outcome verbatim; it does
  not re-derive what is suspicious.
- [`docs/ux/shell_interaction_safety_contract.md`](shell_interaction_safety_contract.md)
  — the shell-wide representation-bearing copy/export rules. The wedge
  is one bounded surface that quotes these rules, not the broader
  shell-wide framework.

## Out of scope (deliberately)

- A universal content-viewer rewrite. The wedge covers the three named
  lanes (risky text, oversized artifacts, generated content) only.
- Notebook / runtime / install-review rendering depth. Those surfaces
  remain owned by their own future wedges; they may consume the
  representation-honesty rules surfaced here, but this wedge does not
  pull their truth into its surface.
- Broad AI / rendered-output policy stack. The generated lane carries
  one labeled preview path with citation anchors. The composer /
  context-inspector seed at
  [`docs/ai/m1_composer_and_context_inspector_seed.md`](../ai/m1_composer_and_context_inspector_seed.md)
  is the adjacent AI wedge.
- Remote, publish, or share-bundle boundary moves. The wedge is
  workspace-local; export rows surface `export_metadata_only` or
  `export_sanitized_snapshot` and never cross a trust boundary.

## Validation command

```
cargo test -p aureline-preview \
  && cargo test -p aureline-shell --lib safe_preview_card \
  && cargo test -p aureline-shell --test safe_preview_card_cases
```
