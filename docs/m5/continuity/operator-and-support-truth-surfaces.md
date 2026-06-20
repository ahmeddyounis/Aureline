# Operator and support continuity truth surfaces

This contract gives About, Help, service health, the support center, and
support/issue-report exports **one** canonical object to read when a person asks
"what continuity is in effect, and what happens if a managed lane breaks?" —
instead of each surface re-deriving the answer from deployment folklore or hidden
admin context.

The object is the **continuity row summary**
(`aureline_continuity::m5_operator_support_continuity_summary::ContinuityRowSummary`).
One summary exists per claimed continuity row. It names the exact row in effect
and summarizes — in plain product language — the three things every operator and
support interaction needs:

1. **Locality, tenant, and key posture** — where processing and storage happen,
   which tenant boundary applies, and which key mode protects durable state.
2. **Affected outage taxonomy** — the current severity, the control-plane-vs-
   data-plane degraded state, the affected plane, and the **narrower fallback**
   that remains while a managed lane is impaired.
3. **Backing-evidence freshness** — whether the continuity evidence behind the
   summary is current, stale, or missing.

The summary reuses the vocabulary that the rest of the continuity lane already
publishes — the profile/lane/locality/tenant/key tokens from the continuity
claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`) and the
severity/degraded-state/fallback tokens from the outage taxonomy
(`aureline_continuity::m5_control_plane_vs_data_plane_outage`) — so a person
reads the same words on every surface and in every exported packet. Once the
summary is present, no surface needs a bespoke continuity explanation.

## What every surface answers the same way

- Which exact continuity row is in effect right now (named, not "service
  degraded")?
- Where does processing and storage happen, which tenant applies, and how are
  keys held?
- If a managed lane is impaired, which plane is affected and what narrower
  fallback remains?
- Is the backing continuity evidence current, stale, or missing?

## Stable conditions

A page qualifies `stable` only when every summary holds at once:

1. It names the exact continuity row in effect (id and label).
2. It discloses processing locality, storage locality, and a residency label.
3. A managed, self-hosted, or sovereign summary discloses tenant scope and key
   mode in plain language.
4. Its outage-taxonomy state is labeled consistently with its current severity,
   and an impaired lane names the narrower fallback that remains.
5. Its backing continuity evidence is current (or stale-within-grace).
6. It is reused by every required operator/support surface (managed summaries
   reach all five surfaces; local-core summaries reach all but support export).
7. Its posture is consistent with its claimed profile.

## Fail-closed guardrails

Two guardrails **withhold** a summary entirely (`withdrawn`) rather than narrow
it, so a broken summary is never rendered on any surface:

- `generic_degraded_wording_used` — a summary uses generic "service degraded"
  wording when the exact continuity row and its narrower fallback class are
  known. The phrasing must name the row, the degraded state, or the fallback.
- `admin_only_material_leaked` — a summary is not export-safe because admin-only
  internal routing or raw secret material is present.

## Claim narrowing

When a summary's disclosures or backing evidence are incomplete, the claim
narrows automatically instead of inheriting green managed language:

- `active_continuity_row_unnamed` — the exact row in effect is not named
  (**preview**).
- `locality_posture_missing` — processing/storage locality or residency is not
  disclosed (**beta**).
- `tenant_key_posture_missing` — a managed-lane summary omits tenant scope or key
  mode (**beta**).
- `outage_taxonomy_unlabeled` — the outage state is not labeled consistently with
  severity (**beta**).
- `narrower_fallback_undeclared` — an impaired lane does not name the fallback
  that remains (**beta**).
- `canonical_summary_stale` — the backing continuity evidence is stale
  (**beta**).
- `canonical_summary_missing` — the backing continuity evidence is missing
  (**preview**).
- `surface_reuse_incomplete` — the summary is not reused across every required
  surface (**beta**).
- `profile_mismatch` — the posture is inconsistent with the claimed profile
  (**preview**).

## Local-core is never narrowed by a managed lane

A local-core continuity summary (a `local_only` row on the `local_core` lane)
keeps its claim and stays export-safe even when every managed lane goes stale,
missing, or impaired. It is never narrowed or withheld because a managed row
broke, and it still reaches About, Help, service health, and the support center.
This is the guardrail against conflating a stale managed row with the local
editing core.

## Export safety

Every summary is metadata-only. It carries closed-vocabulary tokens, export-safe
plain-language labels, UTC timestamps, and opaque refs. Raw hostnames, raw tenant
identifiers, raw KMS handles, raw routing, raw incident bodies, and secret
material never cross this boundary. The support-export wrapper
(`OperatorSupportContinuitySupportExport`) embeds the page, lists the narrow
reasons present, counts defects by reason, and asserts that raw private material
is excluded — so a support bundle or partner packet can copy the same continuity
truth a person sees in-product.

## Surfaces, artifact, and validation

- Typed model and audit:
  `aureline_continuity::m5_operator_support_continuity_summary`
- Canonical schema:
  `schemas/continuity/operator_support_continuity_summary.schema.json`
- Canonical artifact:
  `artifacts/m5/continuity/operator_support_continuity_summary.json`
- Fixtures: `fixtures/continuity/operator_support_summary_cases/`
- CLI/headless explain (reads a page, re-audits, emits a redaction-safe export):
  `cargo run -p aureline-continuity --bin aureline_operator_support_continuity_inspect`
- Schema/fixture validator:
  `python3 tools/validate_m5_operator_support_continuity_summary_fixtures.py`
- Crate audit:
  `cargo test -p aureline-continuity m5_operator_support_continuity_summary`

### Regeneration

```sh
DIR=fixtures/continuity/operator_support_summary_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_operator_support_continuity_summary_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX support-export > $DIR/support_export.json
$EX case-generic-wording-withdrawn > $DIR/case_generic_wording_withdrawn.json
$EX case-locality-undisclosed-beta > $DIR/case_locality_undisclosed_beta.json
$EX case-evidence-stale-beta > $DIR/case_evidence_stale_beta.json
$EX case-evidence-missing-preview > $DIR/case_evidence_missing_preview.json
$EX case-admin-leak-withdrawn > $DIR/case_admin_leak_withdrawn.json
$EX case-local-core-stays-green > $DIR/case_local_core_stays_green.json
cp $DIR/page.json artifacts/m5/continuity/operator_support_continuity_summary.json
```
