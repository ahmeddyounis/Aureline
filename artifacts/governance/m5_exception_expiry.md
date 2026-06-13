# M5 Exception / Expiry Packet — Artifact Summary

Canonical fixture: `fixtures/governance/m5_exception_expiry/canonical_packet.yaml`

Schema: `schemas/governance/m5_exception_expiry.schema.json`

Human-readable companion: `docs/governance/m5_exception_expiry.md`

Producer: `aureline-policy::m5_exception_expiry`
(`seeded_m5_exception_expiry_packet`).

Records surface this packet gates: `aureline-records::m5_records_policy`
(`seeded_m5_records_policy_packet`).

First support consumer: `aureline-support::m5_records_policy_governance`
(`M5RecordsPolicyGovernanceSupportExport::current`).

## Purpose

This artifact freezes the canonical, policy-side exception, waiver, and
remembered-decision contract for the durable M5 artifact families. Every row is
a narrow, time-bounded, auditable object: it pins an exact
actor/object/target/policy-epoch/environment scope, carries an explicit expiry
and approval-history lineage, and lists the reapproval triggers that revalidate
it on drift. A remembered decision never widens authority across any pinned
dimension; on drift or a lapsed expiry it must be re-reviewed rather than
silently reused. It is metadata-only and carries no credential bodies or raw
provider payloads.

## Projections

Each row projects into the surfaces the M5 connector, provider, remote, and
admin actions consume:

- **Exception request sheets** (`request_sheets`) — the exact variance, scope
  summary, reason, approver, mitigation, and expiry instead of generic bypass
  language.
- **Approval-history rows** (`approval_history`) — the ordered lineage from
  request to current state, kept visible across product, CLI/headless, and
  support exports.
- **Expiry banners** (`expiry_banners`) — `active`, `expiring_soon`, or
  `expired`, computed by comparing the exact expiry and review target against
  the packet's `as_of` instant.
- **Remembered-decision revalidation** (`revalidate` / `self_revalidation`) —
  given the real-world context observed at reuse time, returns `still_valid`
  only when every pinned dimension still matches and the decision is unexpired;
  any drift or lapse returns `must_re_review` with the drifted dimensions and
  never widens the original grant.

## Invariants enforced by `validate()`

- Schema version and record kind match the frozen constants.
- Every exception is bounded by an expiry, names an exact expiry timestamp, and
  never widens authority.
- Every authority dimension (actor, object, target, policy epoch, environment)
  is pinned; no dimension is left unbound.
- Every exception lists at least one reapproval trigger and a fallback behavior
  on lapse.
- Every exception carries an approval-history lineage whose latest event matches
  the row's current state.

## Roll-up (canonical packet)

- Exceptions / waivers / remembered decisions: 4
- Pinned authority dimensions per row: 5
- Subject classes covered: policy waiver, policy exception, remembered decision

## Regeneration

Regenerate the fixture whenever the seeded packet changes:

```
cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures \
  > fixtures/governance/m5_exception_expiry/canonical_packet.yaml
```

The crate test `checked_in_canonical_fixture_matches_seeded_packet` asserts the
checked-in fixture equals the seeded packet.
