# Signed And Shared Recipe Packs, Safe Automation Graduation, And Preview-First Replay

- Packet: `recipe-pack:stable:0001`
- Schema: `schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json`
- Support export: `artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/support_export.json`
- Fixture: `fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/`
- Contract: `docs/automation/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay.md`

## Coverage

The packet ships user-authored automation into one row per signed, shareable recipe
pack. Every pack carries a content-addressed manifest, the signature and share
scope it is distributed under, the automation authority it has graduated to, and a
step disclosure for every effect it can produce.

- The organization review-comment template resolves to the managed mode at Stable:
  signed by author and organization, distributed on the managed channel under a
  managed-only template authority, with an inspect-only read and a reversible
  external comment that previews in full before replay and is per-invocation
  approved and audited to the run-record timeline, a checkpoint-reversible verified
  rollback, and downgrade rules that narrow to Beta on stale proof and to
  Unavailable on provider outage.
- The BYOK issue-publish pack resolves to BYOK at Beta: signed by author and
  organization and exported as a portable profile under an admin-gated irreversible
  authority, an irreversible external publish that shows a diff before replay,
  requires admin approval, and is audited to the support export, an
  evidence-preserved rollback, and downgrade rules that narrow to Preview on stale
  proof and to Held on provider outage.
- The local symbol-inspector pack resolves to the local mode at Preview: unsigned
  and user-local under an inspect-only authority, a single inspect-only disclosure
  with no side effect, and downgrade rules that narrow to Experimental.
- The quarantined deploy template resolves to the enterprise-gateway managed
  channel but claims Held: its signature failed, so it is quarantined, its
  irreversible publish is denied by policy, and every downgrade rule narrows to
  Unavailable.

## Invariants

The support export validates against the same closed rule set the shell, docs, and
release tooling enforce: every mutating step previews before replay, gates, and
audits; every irreversible publish is externally auditable and rides an admin-gated
or managed-template authority; a shared pack is signed and a managed-channel pack
rides a managed mode; a blocked pack drops its public claim; and every claimed pack
carries evidence, a verified reversible rollback, and the proof-stale and
provider-unavailable downgrade triggers.

## Boundary

The packet carries content addresses, classes, and review-safe labels only. Raw
shell fragments, raw filesystem paths, raw endpoint URLs, credential bodies, raw
API keys, and OAuth tokens never cross this boundary.
