# Trust-root rotation rehearsal, mirror-root continuity, and key-compromise drill packet

This document defines the rehearsal packet that makes trust-root continuity and
key-compromise response **testable, repeatable, and inspectable** across:

- release publication and update verification,
- policy bundle distribution,
- mirror and offline/air-gapped distribution, and
- customer-managed key rotation/revoke/outage boundaries.

The goal is to prevent trust-root handling from existing only as incident lore.
When a root is stale, missing, revoked, mismatched, or compromised, Aureline’s
behavior MUST be explicit about:

1. what blocks immediately,
2. what degrades (and how it is labeled), and
3. what can continue locally without widening trust.

## Companion artifacts

- [`/artifacts/security/root_rotation_sequences.yaml`](../../artifacts/security/root_rotation_sequences.yaml)
  — machine-readable rehearsal sequences and failure-mode expectations.
- [`/artifacts/security/key_compromise_drill_checklist.md`](../../artifacts/security/key_compromise_drill_checklist.md)
  — operator checklist for compromise/rotation rehearsal runs.
- [`/fixtures/security/trust_root_cases/`](../../fixtures/security/trust_root_cases/)
  — curated worked cases for trust-root and key-compromise scenarios.

Adjacent contracts this packet stays consistent with:

- [`/docs/release/artifact_verification_contract.md`](../release/artifact_verification_contract.md)
  and [`/schemas/release/trust_root_rotation_state.schema.json`](../../schemas/release/trust_root_rotation_state.schema.json)
  (rotation state records quoted by verification rows and offline review).
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  and [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  (trust-root rotation, channel freeze, and revocation objects, including local continuity).
- [`/docs/release/mirror_integrity_and_offline_verification_contract.md`](../release/mirror_integrity_and_offline_verification_contract.md)
  (mirror continuity, propagation records, offline-verification packets).
- [`/docs/security/manual_emergency_import_contract.md`](./manual_emergency_import_contract.md)
  and [`/schemas/security/manual_import_receipt.schema.json`](../../schemas/security/manual_import_receipt.schema.json)
  (manual-import receipts and metadata chains for mirrored/offline distribution).
- [`/docs/security/key_hierarchy_and_tenant_encryption_contract.md`](./key_hierarchy_and_tenant_encryption_contract.md)
  and [`/schemas/security/key_state.schema.json`](../../schemas/security/key_state.schema.json)
  (customer-managed key promises, scoped outage posture, offline import roots).
- [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  (release-evidence packets must state mirror continuity and trust-root rotation evidence).

Normative source alignment:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix BG (bundle precedence and failure matrix).
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.5 (mirror routes and transport governance) and §7.11.11 (key management and failure posture).

If this document disagrees with those sources, those sources win and this file,
the companion artifacts, and the fixtures update together.

## Scope

Frozen here:

1. Rotation and continuity **rehearsal sequences** for:
   - release-signing and update-metadata roots,
   - policy signing roots,
   - mirror continuity roots and sovereign/offline import roots,
   - customer-managed key rotation/revoke/outage behavior (managed data keys).
2. A closed set of failure-mode expectations:
   - stale trust root,
   - missing rotation metadata / missing continuity evidence,
   - revoked signer/root,
   - “lower-order bundle” attempts under a broken trust root,
   - offline mirror continuity failures (stale, mismatched, continuity-broken).
3. Readiness evidence requirements that back managed, self-hosted, mirrored, and
   air-gapped claims with explicit packets/records.

Out of scope:

- live incident tooling, paging/escalation systems, or production ceremony automation;
- defining concrete key material, real fingerprints, or real endpoints; and
- declaring that any profile is “sovereign” or “air-gapped safe” absent the
  evidence packets enumerated below.

## Rehearsal invariants (non-negotiable)

1. **Local-first continuity remains true.** Root failures may block updates,
   installs, promotions, and policy application — but MUST NOT prevent local
   open/edit/save/search/local tooling/local Git.
2. **No silent widening.** Mirror-only profiles must not silently fall through
   to public endpoints. Offline profiles must not be described as “live
   verified”.
3. **Honest offline continuity is admissible.** `unknown_offline` signer
   continuity is allowed for offline review surfaces; it is not a failure as
   long as the product does not overclaim freshness.
4. **A trust-root problem blocks trust-bearing actions, not everything.**
   Block the minimum action families that would widen trust.
5. **Typed failure states are mandatory.** “Random intermittent failure” is
   non-conforming; every refusal needs a stable class and a repair path.

## Rotation and continuity sequences (what must be rehearsable)

The canonical sequences live in:
[`/artifacts/security/root_rotation_sequences.yaml`](../../artifacts/security/root_rotation_sequences.yaml).

High-level categories:

- **Planned rotations with continuity evidence** (overlap, cross-signed, or
  otherwise reviewable continuity).
- **Emergency rotations after compromise** (no continuity statement from the
  compromised predecessor; explicit revocation and explicit successor root).
- **Mirror/offline trust-root updates** (manual import receipts, metadata chain,
  offline-verification packet trust-root pointer block).
- **Customer-managed key events** (rotation/revoke/outage bounded to managed-data
  families; local continuity preserved).

## Failure-mode behavior (block vs degrade vs local continuation)

This section defines the expected *runtime posture* when trust-root continuity
cannot be established.

### 1) Stale trust root (rotation state exists, but is stale)

Definition:

- A `trust_root_rotation_state_record` exists, but its review freshness floor is
  violated (`trust_root_stale`) or its `next_review_due_at` is in the past for a
  claim-bearing surface.

Required behavior:

- **Blocks:** promotion/widening decisions; accepting new trust-bearing payloads
  that would widen trust (new installers, new updates, new policy that widens).
- **Degrades (labeled):** installs/updates may be shown as “stale trust state”
  with explicit “refresh/review required” guidance.
- **Local continues:** local editing, local tooling, local Git, and inspection of
  already-installed artifacts.

### 2) Missing rotation metadata / missing continuity evidence

Definition:

- A surface needs a trust-root pointer, but cannot resolve the referenced
  rotation state record, or the continuity evidence is absent for a new root.

Required behavior:

- **Blocks:** accepting the new trust root as authoritative for verification;
  accepting lower-order bundles that would rely on that unresolved trust root.
- **Degrades (labeled):** surfaces render “trust root unknown” or “continuity
  review required” (not “verified”).
- **Local continues:** local work and inspection of last-known-good state.

### 3) Revoked signer/root

Definition:

- A `revocation_record` has revoked the signer/root, or a rotation state is in
  `trust_root_revoked`.

Required behavior:

- **Blocks:** verifying or promoting any new artifact signed solely under the
  revoked root; importing offline bundles whose trust-root pointer still binds
  to the revoked root.
- **Degrades (labeled):** show the bounded blast radius and the successor import
  action; keep history visible.
- **Local continues:** existing authenticated binaries remain usable for local
  work; support export remains available.

### 4) “Lower-order bundle” attempt under broken trust

Definition:

- A signed bundle with lower precedence (admin policy, offline entitlement,
  cached policy, mirror snapshot) attempts to apply while the trust-root/signer
  chain is stale/unknown/revoked/mismatched.

Required behavior:

- **Blocks:** accepting the lower-order bundle as new authority; widening or
  changing security-sensitive behavior based on it.
- **Degrades (labeled):** last-known-good bundle may remain in effect with a
  typed “stale/blocked due to trust-root repair required” posture.
- **Local continues:** local work; non-privileged read-only inspection of policy
  state and history.

### 5) Offline mirror continuity failure (stale, mismatched, or continuity-broken)

Definition:

- Mirror identity mismatch, signer continuity broken, mirror freshness past
  grace, or offline snapshot expired.

Required behavior:

- **Blocks:** using the mirror/offline bundle as a trust-bearing source for new
  installs/updates/policy distribution.
- **Degrades (labeled):** mirror/offline review surfaces remain available and
  explicitly show `stale`/`mismatched`/`continuity_broken` classes.
- **Local continues:** local work and support export; last-known-good remains
  inspectable.

## Readiness evidence requirements (what must exist to claim continuity)

This packet defines the minimum “readiness evidence” for each deployment
posture. These are *packet requirements*, not prose requirements.

### Managed / connected

Must be able to produce, for each trust-root family in use:

- `trust_root_rotation_state_record` (current or rotating) quoted by verification
  rows.
- `artifact_verification_row_record` for representative release payloads and
  update metadata.
- `emergency_action_record` / `revocation_record` when rotations or revocations
  occurred (including local continuity block).

### Self-hosted

All of the above, plus:

- explicit evidence of a file/mirror import path that replaces “vendor console
  only” dependency for trust-root updates;
- an export-safe audit trail linking the trust-root change to the same stable
  ids surfaced in support/admin exports.

### Mirrored (private mirror or customer-managed mirror)

All of the above, plus:

- `mirror_integrity_packet_record` proving mirror identity and signer continuity
  for at least one artifact family per release bundle.
- `revocation_propagation_record` showing emergency metadata propagation age and
  overlap/grace posture.
- `manual_import_receipt_record` when manual import is the claimed continuity
  path.

### Air-gapped / offline bundle

All of the above, plus:

- `offline_verification_packet_record` with a non-empty trust-root pointer block
  for every trust-root family required by the bundle’s artifact families.
- manual-import receipts and metadata chains for emergency metadata (revocations,
  channel freezes, trust-root rotation metadata) that arrived through offline
  transfer.
- explicit freshness/expiry semantics that prevent an expired offline snapshot
  from being mistaken for current authority.

## Curated worked cases

Worked cases live under:
[`/fixtures/security/trust_root_cases/`](../../fixtures/security/trust_root_cases/).

They are designed for rehearsal and for consuming-surface implementations to
exercise refusal/degrade/continuity behaviors without inventing incident steps.

