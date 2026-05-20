# Beta durability and quorum contract

This contract makes human durability and critical-upstream survivability
executable release truth for every protected beta path. It exists so the
project does not ship a protected beta row that is operationally
single-human, single-key, or dependent on an unowned upstream.

It does not mint a second authority inventory. It projects the canonical
governance primitives onto the protected beta surfaces and emergency
release paths and adds a gate that fails protected rows when coverage,
signer separation, or critical-upstream ownership is missing or stale.

## Packet and generated outputs

The durability packet is the source of truth:

- Source packet: [`artifacts/release/m3/maintainer_coverage_matrix.json`](../../../artifacts/release/m3/maintainer_coverage_matrix.json)
  — record `release.beta_durability_packet.m3`.
- Schema: [`schemas/release/beta_durability_packet.schema.json`](../../../schemas/release/beta_durability_packet.schema.json).

Generated, machine-consumable projections (do not hand-edit; the gate
regenerates them):

- Critical-upstream register, exportable for procurement and support:
  [`artifacts/release/m3/critical_upstream_register.csv`](../../../artifacts/release/m3/critical_upstream_register.csv).
- Release signing, quorum, and break-glass projection:
  [`artifacts/release/m3/signing_quorum_and_breakglass.md`](../../../artifacts/release/m3/signing_quorum_and_breakglass.md).
- Validation capture:
  [`artifacts/release/m3/captures/durability_packet_validation_capture.json`](../../../artifacts/release/m3/captures/durability_packet_validation_capture.json).

The gate is [`tools/ci/m3/durability_gate`](../../../tools/ci/m3/durability_gate),
wrapped by [`ci/check_m3_durability_packet.py`](../../../ci/check_m3_durability_packet.py).

## Canonical sources this packet projects

| Concern | Canonical source |
|---|---|
| Primary/backup ownership and waivers | [`artifacts/governance/ownership_matrix.yaml`](../../../artifacts/governance/ownership_matrix.yaml) |
| Coverage policy narrative | [`docs/governance/maintainer_coverage_policy.md`](../../governance/maintainer_coverage_policy.md) |
| Signing, quorum, and break-glass action matrix | [`artifacts/governance/signing_quorum.yaml`](../../../artifacts/governance/signing_quorum.yaml) |
| Critical-upstream health | [`artifacts/governance/critical_upstream_health_register.yaml`](../../../artifacts/governance/critical_upstream_health_register.yaml) |
| Protected beta rows | [`artifacts/release/m3/claim_manifest.json`](../../../artifacts/release/m3/claim_manifest.json) |

## 1. Maintainer coverage

Every protected beta surface and emergency release path in the packet
names a primary maintainer, a backup maintainer or an active backup
waiver, a succession note, and a milestone-review cadence. The coverage
state of each row is one of:

- `covered` — named primary and backup, standing reviewer depth of two
  or more.
- `waiver_backed` — named primary, no named backup, but a current
  `single-maintainer-backup` waiver is cited.
- `uncovered` — no backup and no active waiver. This is a gate failure.

The gate fails a protected row when it is `uncovered`, when the declared
coverage state does not match the actual backup/waiver posture, when the
cited waiver is closed or expired, or when the coverage review is
overdue. Succession notes record what a backup must be able to do to
absorb the lane during a maintainer outage.

## 2. Current single-maintainer posture

The repository still carries the `single-maintainer-backup` waiver
recorded in the ownership matrix (expires `2026-10-19`). Every protected
beta row is therefore `waiver_backed`: it names a primary maintainer and
cites the waiver in place of a named backup. The waiver makes the gap
visible; it does not let a protected beta claim behave as though reviewer
depth were already real, and it does not permit sole-human custody of
release-signing, rollback, revocation, or registry-emergency authority as
an acceptable steady state. When a second maintainer lands, the matching
rows move to `covered` and the waiver closes.

## 3. Signing, quorum, and break-glass

Release-bearing authority is visibly split across the
`split_authority_actions` matrix, each row citing an action id from
`signing_quorum.yaml`:

- `release_signing` — stable/LTS promotion quorum (three distinct
  humans); never uses break-glass.
- `rollback` — channel freeze/rollback quorum (two distinct humans,
  cross-forum); audited break-glass allowed for containment only.
- `revocation` — revocation/disable/kill-switch quorum (two distinct
  humans, cross-forum); audited break-glass allowed for containment only.
- `registry_emergency` — emergency policy/disable bundle quorum (two
  distinct humans, cross-forum); audited break-glass allowed for
  containment only.
- `signer_roster_change` — planned signer/trust-root change quorum (three
  distinct humans); never uses break-glass.

The gate fails the packet if any split authority is encoded as a
single-human path, if it allows author-only approval, if it drifts from
the signing-quorum action matrix, if release signing or a signer-roster
change permits break-glass, or if the break-glass contract drifts from
the policy. Break-glass is for bounded containment only and always
requires a retrospective co-sign.

## 4. Critical-upstream survivability

Every red-risk upstream on a protected path carries a health rating, a
license/risk posture, a named sustainment owner and sponsor state, and a
fork/replace strategy. The gate fails a red-risk upstream that has no
named sustainment owner, no fork/replace strategy, no license class, or a
stale review, and it fails the packet if a red-risk upstream named in the
critical-upstream register is missing from the durability packet.

## 5. How shiproom and release packets consume this

Release, procurement, and support review consume the generated CSV
register and the signing/quorum/break-glass projection directly; they do
not reassemble durability truth from chat or folklore. The validation
capture records the per-run coverage counts, fixture-drill results, and
findings so a shiproom row can cite a single durable artifact.
