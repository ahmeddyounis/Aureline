# Stabilize SDK schemas, samples, templates, and conformance kits

**Status:** Stable extension-author lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the SDK author surfaces into the **stable line**. Every claimed stable extension-author lane carries one canonical, checked-in author-lane truth: the published SDK version, the kit artifacts (the **SDK schemas**, the canonical **sample extensions**, the scaffolding **project templates**, and the **conformance kit**), the aggregate conformance posture across those artifacts, the worst-case **activation-budget** instrumentation, and the **publisher-continuity** binding. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the conformance kit can no longer back a `stable` author-lane claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. The SDK docs surface, author onboarding, publication review, the conformance dashboard, diagnostics, and support exports ingest this packet instead of inventing a generic "SDK is ready" badge.

## Design principles

1. **Required artifact kinds** — A stable lane must ship every required kind: `sdk_schema`, `sample_extension`, `project_template`, and `conformance_kit`. A missing kind withdraws the lane.
2. **Conformance is derived, not asserted** — Each artifact carries a `conformance_state_class` (`conformant` / `nonconformant` / `not_run` / `waived`) backed by a `conformance_report_ref`. The aggregate `stable_sdk_conformance_summary` is re-derived from the artifacts at validation time, so a stored packet can never hide a nonconformant or missing artifact.
3. **Schema-version pinning** — Each artifact pins an `artifact_schema_version`. A stable lane requires every artifact at the published version; an artifact below the published version withdraws the lane, one above narrows it to `beta` (ahead-of-published, unverified).
4. **No ambient template privilege** — A sample or template with `declares_bounded_permissions == false` scaffolds an unbounded permission set; the lane is withdrawn and a lane-review banner is raised. (Only samples and templates may even carry the flag false; a schema or conformance kit declaring an unbounded scaffold set is rejected at input.)
5. **Bounded activation cost** — The worst-case sample's activation cost carries a `budget_class` (`within_budget` / `over_budget` / `unbounded` / `not_measured`). An `unbounded` cost withdraws the lane; `over_budget` narrows to `beta`; `not_measured` narrows to `preview`.
6. **No catalog-only stable claim** — A `stable` author-lane tier must be `conformance_backed`; a `catalog_asserted_only` basis narrows below Stable.
7. **Publisher continuity stays inspectable** — A `current` continuity must bind a continuity-packet ref; `stale` narrows to `beta`, `missing` to `preview`, `revoked` withdraws the lane.
8. **Downgraded-lane banner** — Ambient template privilege, a nonconformant artifact, a missing required kind, a below-version artifact, an unbounded activation cost, a revoked/missing continuity, a non-installable lifecycle, or a quarantined trust tier raises a banner that names the reason a reviewer must see before relying on the lane.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_sdk_author_lane_packet` | Top-level packet consumed by the SDK docs surface, author onboarding, publication review, the conformance dashboard, diagnostics, support export, docs/help, and release packets. |
| `stable_sdk_author_lane_identity` | SDK starter-pack ref, pinned SDK version, source package, publisher trust tier, lifecycle state. |
| `stable_sdk_kit_artifact` | Per-artifact kind, host class, published-version ref, pinned schema version, conformance state, and bounded-permission flag. |
| `stable_sdk_conformance_summary` | Conformant / nonconformant / not-run / waived counts, present kinds, and any missing required kinds, derived from the artifacts. |
| `stable_sdk_activation_budget` | Worst-case sample activation-cost posture, measured-cost and ceiling refs, measured sample count. |
| `stable_sdk_publisher_continuity` | Publisher-continuity state and continuity-packet ref. |
| `stable_sdk_author_lane_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_sdk_downgraded_lane_banner` | Whether a lane-review banner must display and why. |
| `stable_sdk_author_lane_inspection` | Compact boolean/count projection for CLI and dashboard surfaces. |
| `stable_sdk_author_lane_support_export` | Metadata-safe support/partner export row. |

## Closed vocabularies

### Artifact kinds
`sdk_schema`, `sample_extension`, `project_template`, `conformance_kit` (a stable lane must ship one of each)

### Artifact host classes
`wasm`, `external_host`, `webview`, `headless`, `not_applicable`

### Conformance states
`conformant`, `nonconformant`, `not_run`, `waived` (only `conformant` may keep a stable claim)

### Activation-budget classes
`within_budget`, `over_budget`, `unbounded`, `not_measured` (only `within_budget` may keep a stable claim)

### Publisher-continuity states
`current`, `stale`, `missing`, `revoked` (only `current` may keep a stable claim)

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable author-lane claim)

### Claim basis
`conformance_backed`, `catalog_asserted_only` (only `conformance_backed` may keep a stable claim)

### Narrowing reasons
`sdk_version_not_published`, `catalog_only_trust_not_conformance_backed`, `trust_tier_quarantined`, `lifecycle_not_installable`, `missing_required_artifact_kind`, `artifact_below_published_version`, `artifact_above_published_version`, `artifact_nonconformant`, `artifact_conformance_not_run`, `ambient_template_privilege`, `activation_cost_unbounded`, `activation_cost_over_budget`, `activation_cost_not_measured`, `publisher_continuity_revoked`, `publisher_continuity_missing`, `publisher_continuity_stale`, `attribution_incomplete`

## Key invariants

- A `stable` effective tier requires `sdk_version == published`, `claim_basis_class == conformance_backed`, a non-quarantined trust tier, an installable lifecycle, every required artifact kind present, every artifact conformant and pinned to the published schema version, no ambient template privilege, a `within_budget` activation cost, a `current` publisher continuity, and complete attribution.
- The conformance summary is re-derived from the artifacts at validation time, so the packet cannot drift from its evidence.
- The effective tier, downgrade flag, narrowing reasons, and the downgraded-lane banner are re-derived from the posture at validation time.
- `allows_ambient_template_privilege`, `allows_catalog_only_trust`, `allows_unbounded_activation_cost`, and `allows_nonconformant_stable_claim` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-extensions/src/stabilize_sdk_schemas_samples_templates_and_conformance_kits/mod.rs` |
| Schema | `schemas/extensions/stable_sdk_author_lane.schema.json` |
| Fixtures | `fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/` |
| Tests | `crates/aureline-extensions/src/stabilize_sdk_schemas_samples_templates_and_conformance_kits/tests.rs` |
| Dump example | `crates/aureline-extensions/examples/dump_stable_sdk_author_lane_records.rs` |
| Proof packet | `artifacts/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits.md` |

## Integration with existing lanes

- Sits above the SDK v1 starter pack (`crates/aureline-extensions/src/sdk_v1/`): that module owns the first inspectable *beta* author lane (published API surfaces, manifest-authoring guides, sample pack); this module owns the **stable, conformance-backed** author-lane truth and its stability qualification. The `identity.starter_pack_ref` points back at the starter pack (`sdk_v1_starter_pack:` prefix).
- Reuses the same trust-tier, lifecycle, stability-tier, and claim-basis shapes carried by the manifest-hardening, runtime-ABI, external-host, and Wasm-host-governance stable lanes, so publication review and support surfaces share one author-lane vocabulary.

## Verification

```bash
cargo test -p aureline-extensions stabilize_sdk
cargo run -q -p aureline-extensions --example dump_stable_sdk_author_lane_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_sdk_author_lane.schema.json` (checked with a Draft 2020-12 validator).
