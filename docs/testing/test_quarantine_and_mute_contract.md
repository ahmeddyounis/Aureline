# Test quarantine, mute-state, and release treatment contract

This contract freezes the release-facing treatment of quarantined,
muted, and recovered tests. It extends the existing test identity,
session, attempt, flaky-history, and discovery contracts with the
owner, expiry, scope, surface, review, unblock, and release-packet
rules needed to keep test debt visible.

Quarantine and mute are not the same thing:

- **Quarantine** removes or narrows execution because the test or its
  environment is not trustworthy enough to count as ordinary pass/fail
  evidence.
- **Mute** changes delivery, noise, or local visibility. It does not
  erase the underlying test row, the failure history, the release debt,
  or any policy attribution.

Machine-readable companions:

- [`/schemas/testing/quarantine_record.schema.json`](../../schemas/testing/quarantine_record.schema.json)
  - the `test_quarantine_or_mute_record`, including owner, reason,
    evidence, scope, expiry, allowed surfaces, mandatory visibility
    surfaces, review cadence, unblock conditions, automation controls,
    and release-packet treatment.
- [`/fixtures/testing/quarantine_cases/`](../../fixtures/testing/quarantine_cases/)
  - worked YAML fixtures covering time-bounded quarantine, ownerless
    expired mute, stable-again recovery, and policy-muted enterprise
    restriction.
- [`/artifacts/testing/quarantine_policy_rows.yaml`](../../artifacts/testing/quarantine_policy_rows.yaml)
  - policy rows for owner/expiry requirements, local-mute limits,
    policy attribution, release-packet inclusion, batch review,
    automation, and team-transfer handoff.

This contract composes with and does not replace:

- [`/docs/execution/test_truth_contract.md`](../execution/test_truth_contract.md),
  [`/schemas/execution/flaky_history.schema.json`](../../schemas/execution/flaky_history.schema.json),
  and
  [`/schemas/execution/test_discovery_state.schema.json`](../../schemas/execution/test_discovery_state.schema.json).
  Those records remain the execution-level source for flaky history,
  discovery state, and base quarantine state. This contract owns the
  release-facing owner, expiry, mute, and packet-treatment layer.
- [`/docs/testing/test_item_identity_contract.md`](./test_item_identity_contract.md)
  and
  [`/schemas/testing/test_item_identity.schema.json`](../../schemas/testing/test_item_identity.schema.json).
  Quarantine and mute scope always cites canonical test-item identity
  or selector refs; it never matches display labels.
- [`/docs/testing/test_session_and_attempt_contract.md`](./test_session_and_attempt_contract.md),
  [`/schemas/testing/test_session.schema.json`](../../schemas/testing/test_session.schema.json),
  and
  [`/schemas/testing/test_attempt.schema.json`](../../schemas/testing/test_attempt.schema.json).
  Sessions and attempts cite mute and quarantine refs. This contract
  defines what those refs must preserve.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md),
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json),
  [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md),
  and
  [`/schemas/release/waiver_packet.schema.json`](../../schemas/release/waiver_packet.schema.json).
  Release packets, claim manifests, scorecards, and waiver packets
  consume the debt treatment declared here rather than creating their
  own hidden exception language.
- `.t2/docs/Aureline_Technical_Design_Document.md`,
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`, and
  `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`. If those
  documents disagree with this contract, those upstream documents win
  and this contract plus the companion schema update in the same
  change.

Raw command lines, raw stdout or stderr byte streams, raw environment
bodies, raw absolute paths, raw URLs, raw secret values, raw test
names, raw assertion bodies, raw source excerpts, raw artifact bytes,
raw provider payloads, raw policy payloads, and raw stack traces MUST
NOT cross this boundary. Records carry opaque refs, counts, class
labels, bounded summaries, and timestamps only.

## Record Model

Every quarantine or mute row MUST publish these fields:

| Field group | Required content | Rule |
|---|---|---|
| Identity | record id, canonical test-item refs, selector refs, prior treatment refs | Display names never define scope. |
| Owner | owner state, actor/team/policy owner refs, backup owner, handoff ticket | Active release-bearing debt without an owner fails closed. |
| Reason | closed reason class, human-bounded summary, source policy or threshold refs | Unknown reason is review-only and cannot clear release debt. |
| Evidence linkage | session refs, attempt refs, flaky-history refs, triage refs, release evidence refs | A quarantine or mute without evidence cannot count as an accepted waiver. |
| Scope | exact item, parameterized instance/family, suite, selector, target, enterprise-policy, or release-channel scope | Scope widening requires review; local-only scope cannot hide shared debt. |
| Expiry | opened time, expiry state, expiry time or policy/event trigger, lift time when cleared | Active quarantine and active manual mute are time-bounded; silent indefinite quarantine is forbidden. |
| Surfaces | surfaces where muting may lower noise and surfaces where the row must remain visible | Release scorecards, claim manifests, stable-promotion packets, and release evidence packets remain mandatory visibility surfaces. |
| Review cadence | cadence class, last review, next review, batch-review group, review owner | Expiry, release-candidate review, and team transfer force review. |
| Unblock conditions | stable window, required attempt count, owner triage, policy removal, dependency recovery, waiver closure, claim narrowing | A row is not stable again until the required unblock evidence is present. |
| Release treatment | scorecard inclusion, claim-manifest inclusion, stable-promotion inclusion, debt class, waiver refs, debt refs | Quarantined or muted tests remain separately countable and linked. |
| Automation controls | automation-open/extend posture, extension count, batch-review requirement, owner-handoff requirement | Automation may suggest or open rows, but cannot create ownerless indefinite debt. |

## UI And Export Vocabulary

The following labels are frozen for UI, CLI, support export, claim
manifest, scorecard, and release-packet projections:

| UI/export label | Schema value | Meaning | Release effect |
|---|---|---|---|
| `Suspected flaky` | `suspected_flaky` | Intermittent outcomes were observed but comparable reproduction is incomplete. | Count separately from stable pass/fail; may not justify automatic quarantine alone. |
| `Reproduced flaky` | `reproduced_flaky` | Comparable reruns reproduced divergent outcomes with preserved evidence. | May justify quarantine when owner, expiry, evidence, and review cadence are present. |
| `Stable again` | `stable_again` | A prior mute/quarantine or flaky state cleared through the required evidence window. | Remains visible as recovered debt until the next release packet records the closure. |
| `Manually muted` | `manually_muted` | A user or reviewer muted delivery or local noise for a scoped test row. | May lower local noise only; shared and release-bearing debt remains visible. |
| `Policy-muted` | `policy_muted` | Admin or enterprise policy muted the row or delivery class. | Must cite policy epoch, policy owner, expiry or event trigger, and release debt. |
| `Quarantined pending investigation` | `quarantined_pending_investigation` | Execution or result counting is narrowed while an owner investigates. | Must carry owner, expiry, evidence, review cadence, and unblock conditions. |
| `Expired quarantine` | `expired_quarantine` | A quarantine deadline passed without accepted renewal, lift, or conversion. | Blocks stable promotion until renewed, lifted with evidence, or converted to explicit debt/waiver. |

Unknown or untyped state MUST render as a review-required failure state,
not as `Stable again` or an ordinary pass.

## Owner, Expiry, And Handoff Rules

1. Active quarantine MUST carry a non-missing owner, evidence refs,
   opened time, expiry, review cadence, and unblock conditions.
2. Active manual mute MUST carry a non-missing owner and expiry. A
   local mute may suppress only local delivery surfaces named in
   `allowed_muting_surface_classes`.
3. Policy mute MUST carry the policy epoch or policy rule ref, policy
   owner, affected scope, review trigger, and release treatment.
4. Ownerless active debt is non-conforming. Ownerless expired debt is
   admitted only as a degraded row that requires handoff and blocks
   stable promotion on affected release-bearing scope.
5. Team transfer, owner departure, policy-owner transfer, or expiry
   forces an owner handoff before the row can renew or clear.
6. Expiry never lifts a row by itself. The state moves to expired
   pending review until a reviewer renews, unblocks, converts to a
   waiver-linked debt row, or closes it with stable-again evidence.

## Release-Packet Treatment

Release evidence consumers MUST apply these rules:

- Scorecards count muted, policy-muted, quarantined, expired, and
  stable-again recovery rows separately from pass/fail/skip counts.
- Claim manifests keep affected claim rows narrowed until the
  quarantine/mute debt is cleared, waived, or explicitly declared as a
  known limit.
- Stable-promotion packets include every active or expired treatment
  row that touches a claimed test scope. They include the owner, expiry,
  review cadence, unblock conditions, debt class, waiver refs, and
  claim-manifest refs.
- A release packet with active quarantined or muted debt may proceed
  only when the packet records either claim narrowing or a valid
  waiver. Optimistic promotion that omits the row is non-conforming.
- `Stable again` rows remain in release evidence for the packet that
  closes them so a reviewer can see which prior debt was cleared and
  which evidence window justified the recovery.

## Batch Review And Automation Rules

Batch review is required when:

- a row is expired or expires before the next release-candidate review;
- a row is ownerless, owner-transfer pending, or policy-owner-transfer
  pending;
- more than one canonical test item is muted or quarantined by the
  same policy rule, selector, suite, or target scope;
- automation opened or extended the row;
- a release packet, claim manifest, scorecard, or support export cites
  the row.

Automation MAY open a suspected flaky or policy-muted row only when it
cites evidence and assigns an owner or owner queue. Automation MAY NOT
extend a row indefinitely, clear a row on expiry alone, convert
suspected flaky to reproduced flaky without comparable evidence, or
remove a release-bearing row from scorecards or claim manifests.

## Invariants

| Invariant | Enforcement path | Failure this prevents |
|---|---|---|
| Muted and quarantined tests stay counted. | `release_packet_treatment` requires scorecard, claim-manifest, and stable-promotion inclusion. | A green release packet hides known test debt. |
| Local mute is local delivery, not shared truth. | `allowed_muting_surface_classes` is separate from `mandatory_visibility_surface_classes`. | A developer hides a failing shared row through a local preference. |
| Expiry requires review. | Expired rows move to `expired_requires_review` or `expired_ownerless_requires_handoff`. | A quarantine silently lifts when the timer elapses. |
| Owners survive handoff. | Owner state, backup owner, team transfer, and handoff refs are explicit. | Debt becomes unowned after team transfer or departure. |
| Policy mute is attributable. | Policy mute requires policy ref and policy owner. | Enterprise restrictions look like ordinary skipped tests. |
| Stable again needs evidence. | Stable-again rows cite prior treatment refs and unblock evidence. | A recovered row is promoted because the quarantine disappeared, not because the test stabilized. |
