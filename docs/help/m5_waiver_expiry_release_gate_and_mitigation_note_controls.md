# M5 waiver-expiry queue item, release-gate banner, and mitigation note card controls

This contract implements three governed governance-dashboard component families —
the **waiver-expiry queue item**, the **release-gate banner**, and the
**mitigation note card** — frozen in the
[M5 governance-dashboard component matrix](m5_governance_dashboard_components_contract.md)
as one reusable controls packet, so the assurance center, the operator board, the
shiproom, the CLI, and the support/export packet all read the same
temporary-exception and ship/no-ship truth.

- Boundary schema:
  [`schemas/ui/m5-waiver-gate-mitigation-controls.schema.json`](../../schemas/ui/m5-waiver-gate-mitigation-controls.schema.json)
- Per-component contracts:
  [`schemas/ui/m5-waiver-expiry-queue-item.schema.json`](../../schemas/ui/m5-waiver-expiry-queue-item.schema.json),
  [`schemas/ui/m5-release-gate-banner.schema.json`](../../schemas/ui/m5-release-gate-banner.schema.json),
  [`schemas/ui/m5-mitigation-note-card.schema.json`](../../schemas/ui/m5-mitigation-note-card.schema.json)
- Proof artifacts:
  [`artifacts/release/m5-waiver-gate-mitigation-controls-proof/`](../../artifacts/release/m5-waiver-gate-mitigation-controls-proof/)
- Protected fixtures:
  [`fixtures/ui/m5-waiver-gate-mitigation-controls/`](../../fixtures/ui/m5-waiver-gate-mitigation-controls/)

The Rust validator in `crates/aureline-release` is the authoritative gate; this doc
describes the intent.

## Waiver-expiry queue item

`resolve_waiver_expiry_item` takes one waiver's identity, the failure it holds, its
lifecycle state, the affected milestone or release, its mitigation posture, owner
alias, expiry, and evidence freshness, and derives one readiness state drawn from the
frozen `M5GovernanceReadinessState` vocabulary. Every item keeps its expiry visible
and always offers an open-detail action. The derivation is degrade-first:

| Condition | Readiness |
| --- | --- |
| Evidence unknown | `not_evaluated` |
| No resolved owner | `owner_unresolved` |
| Waiver expired or revoked | `expired_waiver` |
| Evidence missing | `blocked` |
| Evidence stale | `evidence_stale` |
| Waiver expiring soon | `waived` (expiry stays visible) |
| Active waiver holding a failure | `waived` |
| No waiver, failure not fully mitigated | `blocked` |
| No waiver, failure fully mitigated, fresh evidence, owner resolved | `passing` (exception retired) |

**Acceptance criterion 1**: a waived or expiring failure never renders as a clean
pass, and an expiring waiver remains visible wherever the affected lane is summarized.
An active or expiring waiver holding a failure resolves to `waived` — never `passing` —
with `expiry_visible` always true.

## Release-gate banner and mitigation note card

`resolve_release_gate` takes one gate's blocker, waived, and stale-evidence counts, its
declared ship/no-ship decision, its mitigation posture, the user-facing mitigation text,
the fallback path, and evidence freshness, and derives one readiness state, an honestly
re-derived gate decision, and a `M5MitigationClarity` reading for the mitigation note
card. The derivation is degrade-first:

| Condition | Readiness | Decision |
| --- | --- | --- |
| Evidence unknown | `not_evaluated` | `held_pending_evidence` |
| No resolved owner or forum | `forum_unresolved` | `blocked_by_owner_or_forum` |
| Evidence missing | `blocked` | `held_pending_evidence` |
| Evidence stale, or stale-evidence count > 0 | `evidence_stale` | `held_pending_evidence` |
| Blocker count > 0 | `blocked` | `no_go` |
| Waived count > 0 | `waived` | `conditional_go` |
| Mitigation absent or jargon | `warning` | `conditional_go` |
| Mitigation only partial or risk accepted | `warning` | `conditional_go` |
| No blockers, waived, or stale evidence; fully mitigated, plain-language note; fresh evidence; resolved owner/forum | `passing` | `go` |

A `go` declared over open blockers therefore never stays `go`. The blocker, waived, and
stale-evidence counts, the fallback path, and packet/export continuity are always
carried.

**Acceptance criterion 2**: the mitigation note stays understandable to users, support,
and release reviewers without collapsing into internal-only jargon. The mitigation-clarity
reading is `plain_language` only when the user-facing note reads as a plain-language
sentence carrying no internal-only jargon marker; a jargon-only or absent note reads as
`jargon_detected` or `mitigation_absent` and degrades the gate.

## Hard invariants

Every controls row asserts, and the validator enforces, that no consumer:

- renders a waived or expired exception as a clean pass;
- hides the waiver expiry or the owner;
- hides mitigation behind internal-only jargon; or
- invents a gate-local status word outside the frozen readiness vocabulary.

Raw URLs, raw tokens, credentials, private endpoints, and user text bodies never cross
this boundary; every waiver id, held-failure ref, affected-target id, owner alias,
expiry, and fallback path is carried only as an opaque, export-safe representation, and
an owner alias is a role alias, never a personal contact detail.
