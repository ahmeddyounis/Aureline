# Default-redacted support and incident export profile

The default-redacted support export profile is the one trustworthy
posture a stable-facing user (or a claimed support row) sees when they
start a support bundle or incident packet export. It pins what travels,
what stays referenced by id, what stays on the local machine, and what
never crosses the boundary.

The implementation lives in
[`crates/aureline-support/src/export_review/mod.rs`](../../../crates/aureline-support/src/export_review/mod.rs).
The JSON-schema boundary lives at
[`schemas/support/export_redaction_profile.schema.json`](../../../schemas/support/export_redaction_profile.schema.json).
The protected fixture corpus lives at
[`fixtures/support/m3/redaction_and_escalation/`](../../../fixtures/support/m3/redaction_and_escalation/).
The drill test lives at
[`crates/aureline-support/tests/export_review_default_redacted.rs`](../../../crates/aureline-support/tests/export_review_default_redacted.rs).

The profile composes with the existing escalation-packet schema at
[`schemas/support/escalation_packet.schema.json`](../../../schemas/support/escalation_packet.schema.json)
and the support-bundle schema at
[`schemas/support/support_bundle.schema.json`](../../../schemas/support/support_bundle.schema.json)
rather than minting a parallel vocabulary.

## What the profile preserves

Every default-redacted profile MUST preserve and surface:

- the **exact-build identity ref**, **release channel class**, and
  **platform class** so a support reviewer can reconstruct the running
  build without raw paths or binary bodies;
- the **scenario family** (closed token from the scenario picker
  vocabulary) so the case can be routed without free-form prose;
- one or more **doctor finding codes** so the underlying finding stays
  stable across re-export;
- the **repair history refs** (recovery action ids, repair transaction
  ids) so the support reviewer sees what the user already tried;
- **crash manifest refs** and **symbolication report refs**, carried
  by stable id only;
- **support-bundle refs** and **incident-workspace packet refs** for
  joinability with the rest of the support corpus;
- the active **policy and trust state** as closed tokens.

## What the profile excludes by default

Every default-redacted profile MUST exclude or retain-local-only:

- raw crash dump payloads, raw trace captures, raw log excerpts, raw
  transcript excerpts (referenced by id only);
- code-adjacent attachments — code snippets, notebook cells, mutation-
  journal excerpts (require an explicit broaden-evidence review marker
  before any inclusion);
- full shell history, secret-bearing material, and ambient credential
  material (always prohibited, never widenable).

The profile schema pins `raw_dumps_attached`, `raw_transcripts_attached`,
`code_adjacent_attached`, and `secret_bearing_attached` as `const false`;
a profile that flips any of them is non-conforming.

## Local-first destination posture

Every default-redacted profile MUST keep the **local-only save/copy
path** available with equal prominence to any upload or handoff path.
The validator refuses a profile whose
`destination_posture.local_only_path_available` is `false` or whose
`destination_posture.local_only_equal_prominence` is `false`. Vendor
case handoff, user-initiated upload, managed-admin handoff, and private
security channels are still expressible — but they never silently
replace the local-only path.

## Reopen-truth posture

Every export emits one
`support_export_reopen_manifest_record` that preserves:

- the profile ref the export was minted from;
- the build identity block (exact-build, release channel, platform);
- the **included** evidence-class list, in profile order;
- the **excluded** evidence-class list, in profile order;
- the selected destination class;
- whether the export ever left the machine
  (`local_only = true` and `exported_at_or_null = null` for a
  local-only review);
- the broaden-evidence review marker, if any, that allowed a code-
  adjacent widening.

A user (or a support reviewer) can later reopen this manifest and see
exactly what was shared, why, to which destination class, and whether a
broaden-evidence review was required. The reopen manifest never quotes
raw bodies — it carries the same stable ids as the original profile so
the reopen surface remains metadata-safe.

## Failure-drill posture

The validator fails closed:

- A profile that attaches a raw dump, raw transcript, code-adjacent
  body, or secret-bearing body to the export is refused.
- A profile whose crash linkage flips `raw_dump_attached` is refused.
- A profile whose local-only path is hidden or rendered below equal
  prominence is refused (`LocalOnlyPathHidden`).
- A profile that drops any of the four default-required evidence
  classes (`build_identity`, `channel_and_platform_identity`,
  `scenario_family`, `doctor_finding_codes`) is refused.
- A profile that widens a code-adjacent class
  (`code_snippet_attachment`, `notebook_cell_attachment`,
  `mutation_journal_excerpt`) past `excluded_by_default` without a
  recorded `broaden_evidence_review_ref` is refused.
- A profile whose prohibited classes (`full_shell_history_capture`,
  `secret_bearing_material`, `ambient_credential_material`) are not
  `excluded_always` with `broaden_review_class = prohibited` is refused.
- A reopen manifest that selects `local_only_review` but also records
  an export timestamp is refused
  (`LocalOnlyManifestExported`).
- A reopen manifest that claims `local_only = true` but targets a
  non-local destination is refused
  (`LocalOnlyDestinationMismatch`).
- A reopen manifest that includes a code-adjacent or always-prohibited
  evidence class without an attached broaden-evidence review marker is
  refused.

## Fixture index

| Fixture | Posture |
|---|---|
| [`default_redacted_profile.yaml`](../../../fixtures/support/m3/redaction_and_escalation/default_redacted_profile.yaml) | local-only default-redacted export for an extension regression |
| [`vendor_handoff_profile.yaml`](../../../fixtures/support/m3/redaction_and_escalation/vendor_handoff_profile.yaml) | vendor case handoff with raw payloads referenced by id |
| [`broaden_evidence_review_required.yaml`](../../../fixtures/support/m3/redaction_and_escalation/broaden_evidence_review_required.yaml) | managed-admin handoff that widens a mutation-journal evidence class under an explicit review marker |
| [`reopen_manifest_local_only.yaml`](../../../fixtures/support/m3/redaction_and_escalation/reopen_manifest_local_only.yaml) | reopen manifest for a local-only review that never crossed the boundary |
| [`reopen_manifest_vendor_handoff.yaml`](../../../fixtures/support/m3/redaction_and_escalation/reopen_manifest_vendor_handoff.yaml) | reopen manifest for a vendor case handoff |

## Acceptance and how this profile meets it

- **Support and incident exports are redaction-safe by default on
  claimed rows without losing exact-build identity, scenario family,
  or stable finding codes.** The profile pins the exact-build identity
  block, the scenario family, and the doctor finding codes as
  `default_required_evidence_classes`; the validator refuses an export
  that drops any of them.
- **Crash manifests and local symbolication reports are linked by
  reference in exported packets instead of silently attaching raw
  dumps.** `crash_linkage.raw_dump_attached` is pinned to `false` and
  every `crash_manifest_refs` / `symbolication_report_refs` entry
  travels as a stable id.
- **Users can reopen a manifest after export and inspect included
  classes, excluded classes, build identity, and destination class
  truthfully.** Every reopen manifest carries the included/excluded
  class lists, the build identity block, the destination class, the
  local-only flag, and an optional broaden-evidence review marker.
- **Any export path that would widen evidence classes beyond the
  default profile requires an explicit reviewed choice.** Widening a
  code-adjacent class without a `broaden_evidence_review_ref` is
  refused by the profile validator and by the reopen-manifest
  validator; always-prohibited classes refuse widening unconditionally.
