# Artifact: Stabilize SDK schemas, samples, templates, and conformance kits

**Task:** Promote the SDK author surfaces into the stable line — bind the kit artifacts (the published SDK schemas, the canonical sample extensions, the project templates, and the conformance kit), derive the aggregate conformance posture, instrument the worst-case activation budget, bind the publisher-continuity packet, and derive the stability qualification with automatic narrowing below Stable so authors, admins, and reviewers can all inspect and trust the lane.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the author-lane identity (SDK starter-pack ref, pinned SDK version, publisher trust tier, lifecycle state), the kit artifacts (each with an artifact kind — `sdk_schema` / `sample_extension` / `project_template` / `conformance_kit` — a host class, a published-version ref, a pinned schema version, a conformance state backed by a report ref, and a bounded-permission flag), the aggregate conformance summary (derived from the artifacts: conformant / nonconformant / not-run / waived counts, present kinds, and any missing required kinds), the worst-case activation-budget instrumentation, and the publisher-continuity binding into one validated packet, and derives the stability qualification it may claim. A `stable` author-lane claim is only allowed when the lane pins the published SDK version, is conformance-backed, keeps its publisher trust tier out of quarantine, stays on an installable lifecycle, ships every required artifact kind, keeps every artifact conformant and pinned to the published schema version, never scaffolds ambient template privilege, keeps its activation cost bounded and within budget, keeps its publisher continuity current, and is fully attributed. A sample or template that scaffolds an unbounded permission set, an unbounded activation cost, a nonconformant artifact, a below-version artifact, a missing required kind, or a revoked continuity withdraws the lane; an above-published artifact version, an over-budget activation cost, or a stale continuity narrows to `beta`; a catalog-asserted basis, a quarantined trust tier, a not-measured budget, or a missing continuity narrows to `preview`. When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. The checked-in packet is canonical: the SDK docs surface, author onboarding, publication review, the conformance dashboard, diagnostics, and support exports ingest it instead of inventing a generic "SDK is ready" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/stabilize_sdk_schemas_samples_templates_and_conformance_kits/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_sdk_author_lane.schema.json`
- New fixtures: `fixtures/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits/`
  - `verified_publisher_stable_current.json` — a verified SDK kit shipping all four required artifact kinds, every artifact conformant and pinned to the published version, activation within budget, continuity current; it holds Stable.
  - `artifact_above_published_version_narrows_to_beta.json` — a kit shipping a v2 preview schema ahead of the published v1; it narrows to `beta` (unverified) without a hard block.
  - `catalog_asserted_narrows_to_preview.json` — a community kit claiming Stable on catalog assertion alone (`catalog_asserted_only`); it narrows to `preview`.
  - `ambient_template_privilege_withdrawn.json` — a kit whose project template scaffolds an unbounded permission set; the lane is `withdrawn`, a lane-review banner is raised, and `ambient_template_privilege_present` is surfaced.
  - `nonconformant_sample_withdrawn.json` — a kit whose external-host debugger sample fails conformance; the lane is `withdrawn`, a banner is raised, and the nonconformant count is surfaced.
- New dump example: `crates/aureline-extensions/examples/dump_stable_sdk_author_lane_records.rs`
- New docs: `docs/extensions/m4/stabilize-sdk-schemas-samples-templates-and-conformance-kits.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_sdk_author_lane.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`artifact_above_published_version_narrows_to_beta.json`, `catalog_asserted_narrows_to_preview.json`, `ambient_template_privilege_withdrawn.json`, `nonconformant_sample_withdrawn.json`, `sdk_version_mismatch_narrows_below_stable`)
- [x] Users and admins can inspect permissions (bounded-scaffold flag per sample/template), compatibility (pinned artifact schema versions vs the published version), activation cost (worst-case sample budget class), lifecycle label, publisher provenance (trust tier + continuity packet), and rollback/revocation state (continuity `revoked` / lifecycle) for the touched ecosystem row. (`stable_sdk_author_lane_inspection`, `stable_sdk_conformance_summary`, `stable_sdk_activation_budget`, `stable_sdk_publisher_continuity`)
- [x] Conformance fixtures, activation-budget instrumentation, and a publisher-continuity packet make the ecosystem claims supportable and mirrorable. (`stable_sdk_conformance_summary` derived from artifacts, `stable_sdk_activation_budget`, `stable_sdk_publisher_continuity`, all five fixtures)
- [x] Stable inspect/export shows the kit artifacts, their conformance, and the lane qualification together with the narrowing reasons. (`artifacts[*]`, `conformance_summary`, `claim.downgrade_reasons`, `support_export_quotes_conformance_counts`)

## Guardrails honored

- No ambient extension privilege: a sample or template with `declares_bounded_permissions == false` is flagged `ambient_template_privilege_present`, withdraws the lane, and raises a banner (`ambient_template_privilege_withdraws_and_raises_banner`); a schema or conformance kit may not even declare an unbounded scaffold set (`schema_or_kit_cannot_declare_unbounded_permissions`).
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- No unbounded activation cost: an `unbounded` budget withdraws the lane (`unbounded_activation_cost_withdraws_the_lane`); `over_budget` narrows to `beta`.
- No catalog-only / nonconformant stable claim: the conformance summary is re-derived from the artifacts at validation time; a nonconformant or missing required kind withdraws the lane (`nonconformant_artifact_withdraws_and_excludes_stable`, `missing_required_kind_withdraws_the_lane`).
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_sdk
cargo run -q -p aureline-extensions --example dump_stable_sdk_author_lane_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_sdk_author_lane.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The conformance state per artifact is supplied by the producing conformance kit. When the headless validator CLI is wired to emit these states directly, the `conformance_state_class` should be sourced from the kit's report output rather than a producer-supplied class.
- The activation budget is summarized as a single worst-case sample posture; a later revision should carry a per-sample activation row so a reviewer can see which sample drove the worst case.
- Trust-tier, lifecycle, stability-tier, and claim-basis vocabularies are closed string sets shared with the manifest-hardening, runtime-ABI, external-host, and Wasm-host-governance stable lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The publisher-continuity binding carries a continuity-packet ref; a later revision should resolve the bound packet's own freshness rather than accepting a producer-supplied `current` / `stale` state.
