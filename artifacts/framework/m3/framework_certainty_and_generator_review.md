# Framework certainty and generator-review truth report

Status: seeded
Schema version: 1
As of: 2026-05-18

## Scope

This report covers the cross-surface framework support / status strip and
the framework-object certainty record (route / component / service /
entity rows, convention-diagnostic rows, and generator / codemod /
scaffold previews) that govern claimed framework-aware beta wedges.
Together they give one explicit support / certainty contract route
explorers, component / service trees, convention-diagnostics lanes,
generator-preview review sheets, notebook framework-context inspectors,
AI-assist context lanes, and support exports must read before a
framework claim becomes actionable. The records preserve source linkage,
authored-vs-generated truth, pack or bridge source identity, and
review-first generator behavior in product, docs / help, and exported
evidence.

Two record kinds are governed:

- `framework_support_strip_record` — header strip naming detected
  framework / version, support class (core native / framework pack /
  bridge / heuristic / unsupported), pack or bridge source, pack /
  runtime health, freshness, local-or-remote scope, version-compat
  posture, and the closed list of fallback-open actions. The boundary
  schema is at
  [`/schemas/framework/framework_support_strip.schema.json`](../../../schemas/framework/framework_support_strip.schema.json).
- `framework_object_certainty_record` — row-level exact-vs-heuristic
  record carried by framework-object rows, convention-diagnostic rows,
  and generator-preview rows. The boundary schema is at
  [`/schemas/framework/framework_object_certainty.schema.json`](../../../schemas/framework/framework_object_certainty.schema.json).

Both records compose with — and do NOT duplicate — the
`framework_certainty_row` and `source_sync_chip` contracts frozen at
[`/docs/framework/framework_certainty_and_source_sync_contract.md`](../../../docs/framework/framework_certainty_and_source_sync_contract.md).
Certainty rows handle subject-level provenance and source-sync chips
handle runtime alignment; the strip and the certainty record carry the
user-visible support, exact-vs-heuristic, authored-vs-generated, and
file-effect truth those records cite.

The Rust implementation lives at
[`/crates/aureline-framework/src/status_and_certainty/`](../../../crates/aureline-framework/src/status_and_certainty/mod.rs).
The fixture corpus lives at
[`/fixtures/framework/m3/status_strip_and_convention_truth/`](../../../fixtures/framework/m3/status_strip_and_convention_truth/).

## Support class ladder

The strip is the single header every framework-aware surface reads
before a claim becomes actionable. The support class is a closed token
identical in UI, audit events, docs, and support exports.

| Support class                  | What it means                                                                 | Admits exact certainty? | Required pack source                                                        |
|--------------------------------|-------------------------------------------------------------------------------|:-----------------------:|------------------------------------------------------------------------------|
| `core_native`                  | First-party or certified pack provides exact project-model support            | yes                     | `first_party_native` or `governed_framework_pack`                            |
| `framework_pack`               | Governed pack provides structured support on shared contracts                 | yes                     | `first_party_native`, `governed_framework_pack`, or `community_framework_pack` |
| `bridge_compatibility_layer`   | Capability is translated from another ecosystem or tool model                 | no                      | `bridge_compatibility_layer`                                                 |
| `heuristic_convention_mode`    | Routes / components / services / entities inferred from naming or layout only | no                      | `heuristic_convention_only`, `imported_snapshot_only`, or `no_pack_or_bridge` |
| `unsupported_or_unclaimed`     | Nothing claims this framework                                                 | no                      | `no_pack_or_bridge`                                                          |

Exact certainty (`exact_pack_backed` / `exact_runtime_confirmed`) is
structurally inadmissible outside `core_native` / `framework_pack`.
`unsupported_or_unclaimed` forces `no_admissible_evidence`.

## Row-level certainty labels

The framework-object certainty record carries one of eight closed
certainty labels at the row level (route / component / service / entity),
the convention-diagnostic row, or the generator-preview row.

| `certainty_label_class`     | Admissible support class                                            | Visible note required? |
|------------------------------|---------------------------------------------------------------------|:----------------------:|
| `exact_pack_backed`          | `core_native`, `framework_pack`                                     | no                     |
| `exact_runtime_confirmed`    | `core_native`, `framework_pack`                                     | no                     |
| `derived_by_convention`      | any                                                                 | yes                    |
| `imported`                   | any                                                                 | no                     |
| `partial_evidence`           | any                                                                 | yes                    |
| `heuristic_suspicion`        | `bridge_compatibility_layer`, `heuristic_convention_mode`           | yes                    |
| `stale_against_source`       | any                                                                 | yes                    |
| `no_admissible_evidence`     | any                                                                 | no                     |

`derived_by_convention`, `partial_evidence`, `heuristic_suspicion`, and
`stale_against_source` rows MUST keep a `partial_or_derived_note`
visible on the row itself — secondary panels are not admissible.

## Source round-trip

User-authored route / component / service / entity rows MUST preserve a
`source_file_anchor` or `source_symbol_anchor` evidence entry. Heuristic
or imported rows MAY substitute a `convention_pattern_anchor` or
`imported_snapshot_anchor`, but the partial / derived note remains on
the row so the user can audit why Aureline thinks the object exists.

## Convention diagnostics

Diagnostic rows use one closed `convention_diagnostic_class` vocabulary
so framework surfaces no longer collapse fundamentally different states
into one generic warning.

| `convention_diagnostic_class`   | Admissible support class                                   | Notes                                                                                |
|----------------------------------|------------------------------------------------------------|---------------------------------------------------------------------------------------|
| `hard_contract_violation`        | `core_native`, `framework_pack`, `bridge_compatibility_layer` | Proven violation by pack / bridge evidence; cannot be suppressed silently.            |
| `framework_version_mismatch`     | any                                                        | Detected version is outside the support strip's supported range.                      |
| `pack_capability_limitation`     | any                                                        | Pack does not support inspecting / fixing this subject; never collapsed into a hard violation. |
| `heuristic_suspicion`            | `heuristic_convention_mode`, `bridge_compatibility_layer`  | Inferred suspicion only; `no_fix_available` is admissible.                            |
| `ambiguous_convention`           | any                                                        | Pattern matches more than one convention.                                             |
| `missing_registration`           | any                                                        | Source artifact exists but is not registered.                                         |
| `unreachable_route`              | any                                                        | Route declared but unreachable at runtime.                                            |
| `generated_artifact_drift`       | any                                                        | Generated artifact has drifted from user-owned source.                                |
| `not_available_in_this_mode`     | any                                                        | Capability is mode-narrowed (e.g. notebook-only or remote-only).                      |

Every diagnostic offers at least one closed fix action. If the action is
`open_generator_preview`, the row MUST carry a paired
`generator_preview_ref`; the schema and the Rust validator enforce this.

## Generator / codemod / scaffold review

Generator preview rows are review-first. Each preview names:

- generator kind class (`framework_pack_scaffold`,
  `framework_pack_codemod`, `framework_native_generator`,
  `bridge_compatibility_scaffold`, `heuristic_pattern_scaffold`);
- generator id ref, label, and version label;
- input summary;
- per-file effect rows with `file_effect_class` (create / modify /
  delete / rename), `file_ownership_class` (user-owned / pack-managed /
  framework-generated / shared / unknown), opaque path handle, and a
  `requires_user_confirmation` flag;
- `dependency_impact_class`;
- `rollback_class` and (when applicable) the paired `checkpoint_ref`;
- `regenerate_path_available` flag.

Truth-rule highlights enforced by both the schema and the Rust
validator:

- `framework_pack_*` and `framework_native_generator` are admissible
  only under `core_native` / `framework_pack` support;
- `bridge_compatibility_scaffold` is admissible only under
  `bridge_compatibility_layer`;
- `heuristic_pattern_scaffold` is admissible only under
  `heuristic_convention_mode`;
- a `delete_file` effect on a `user_owned_authored` file MUST set
  `requires_user_confirmation = true`;
- `rollback_via_checkpoint` requires a non-null `checkpoint_ref`.

Generated / scaffolded changes therefore cannot bypass preview,
checkpoint, or support-class caveats — the schema rejects records that
attempt to.

## Action vocabulary

The strip exposes a closed `framework_support_action_class` set
(`open_compatibility_details`, `open_pack_status`, `open_pack_docs`,
`open_migration_path`, `open_raw_source_fallback`, `request_pack_install`,
`request_pack_update`, `request_policy_review`,
`open_runtime_inspector`). Every non-unsupported strip MUST offer
`open_compatibility_details`. Pack-specific actions (`open_pack_status`,
`request_pack_update`) are inadmissible when the pack source is
`heuristic_convention_only`, `imported_snapshot_only`, or
`no_pack_or_bridge`.

Row, diagnostic, and generator records carry their own closed
`row_action_class` set so the row never advertises a dead-end action.

## Composition with the certainty row and source-sync chip

Both new records reference the existing cross-surface records:

- `framework_certainty_row_record_ref` — opaque ref to the certainty
  row this record composes with (subject-level certainty, primary source,
  fallback inference reason).
- `source_sync_chip_record_ref` — opaque ref to the source-sync chip
  this record composes with (source revision / runtime revision /
  hot-reload status / target device).

This composition means a beta wedge that turns on a framework pack
header, a route row, and a generator preview must emit:

1. one support strip per framework-aware surface (header);
2. one framework-object certainty record per visible row (rows,
   diagnostics, generator previews);
3. the certainty row(s) and source-sync chip(s) cited by those records.

A reviewer can therefore audit, for any framework-aware beta row,
whether the row is backed by a core native model, governed framework
pack, bridge / compatibility layer, or heuristic convention mode; what
framework / version Aureline detected; and why a route, component,
service, or generator action is trustworthy enough to use.

## Worked fixture corpus

The corpus at
[`/fixtures/framework/m3/status_strip_and_convention_truth/`](../../../fixtures/framework/m3/status_strip_and_convention_truth/)
covers the cross-section every reviewer needs:

- `strip_framework_pack_healthy.yaml` — healthy governed pack, within-
  range, authoritative-live freshness, pack actions admissible.
- `strip_bridge_compat_degraded.yaml` — bridge compatibility layer,
  degraded partial health, between-versions compatibility, migration-path
  action surfaced.
- `strip_heuristic_convention_only.yaml` — no pack or bridge; heuristic
  mode; pack-specific actions absent; raw-source fallback admissible.
- `row_exact_route_pack_backed.yaml` — `exact_pack_backed`
  `route_row` with source-file, source-symbol, and pack-proving-artifact
  anchors.
- `row_component_partial_evidence.yaml` — `partial_evidence`
  `component_row` with a visible partial / derived note.
- `row_service_bridge_with_caveat.yaml` — `derived_by_convention`
  `service_row` under a bridge with a visible caveat.
- `diagnostic_hard_contract_violation.yaml` — proven hard contract
  violation routing to a paired generator preview.
- `diagnostic_pack_capability_limitation.yaml` — pack-capability
  limitation distinguished from a contract violation.
- `diagnostic_heuristic_suspicion.yaml` — heuristic-only suspicion that
  offers `no_fix_available` honestly.
- `generator_pack_scaffold_review.yaml` — review-first pack scaffold
  with paired checkpoint.
- `generator_codemod_with_user_delete.yaml` — codemod that deletes a
  user-owned file with `requires_user_confirmation = true`.

The Rust integration test at
[`/crates/aureline-framework/tests/status_strip_and_convention_truth.rs`](../../../crates/aureline-framework/tests/status_strip_and_convention_truth.rs)
loads the manifest, deserializes each case, runs the Rust validator,
and asserts each fixture's declared expectations match the record.

## Acceptance checklist

A reviewer can audit conformance without implementation code:

1. **Support class.** Does the strip name `core_native`,
   `framework_pack`, `bridge_compatibility_layer`,
   `heuristic_convention_mode`, or `unsupported_or_unclaimed`?
2. **Pack / bridge source.** Does the pack-source class pair with the
   support class per the table above?
3. **Exact-vs-heuristic.** Can you tell, at the row level, whether the
   evidence is `exact_pack_backed`, `exact_runtime_confirmed`,
   `derived_by_convention`, `imported`, `partial_evidence`,
   `heuristic_suspicion`, `stale_against_source`, or
   `no_admissible_evidence`?
4. **Authored vs generated.** Can you tell whether the object was
   authored by the user, generated by the framework / pack /
   codemod, managed by the pack, or imported?
5. **Source round-trip.** Does the row preserve at least one
   source-file or source-symbol anchor when the object is user-authored?
6. **Convention diagnostic class.** Can you tell whether the diagnostic
   is a hard contract violation, version mismatch, pack-capability
   limitation, heuristic suspicion, ambiguous convention, missing
   registration, unreachable route, generated-artifact drift, or
   mode-narrowed unavailability?
7. **Generator review.** Does the preview name the generator id +
   version, the input summary, every file effect with its ownership
   class, the dependency impact, the rollback class, and (when
   applicable) the paired checkpoint?
8. **Whole-framework guardrail.** Is it structurally impossible for a
   heuristic-mode strip to claim within-range compatibility, for a
   heuristic-only chain to produce `exact_pack_backed`, or for a
   codemod to delete a user-owned file without confirmation?

If any answer above requires reading implementation code or inferring
hidden pack state from UI chrome, the surface is non-conforming.
