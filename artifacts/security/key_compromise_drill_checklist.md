# Key-compromise and trust-root rotation drill checklist

This checklist is the operator-facing companion to:

- `docs/security/trust_root_rotation_rehearsal.md`
- `artifacts/security/root_rotation_sequences.yaml`
- `fixtures/security/trust_root_cases/`

It is designed to make compromise/rotation rehearsal **repeatable** and to
ensure each drill produces exportable, reviewable evidence instead of ad-hoc
notes.

## Drill invariants

- Local edit/search/save/local tooling/local Git MUST remain usable throughout.
- No silent trust widening: mirror-only must not fall back to public routes.
- Offline paths must remain honest: `unknown_offline` continuity is acceptable;
  “live verified” language is not.
- A broken or revoked trust root blocks new trust-bearing actions, not the whole
  product.

## Preflight (before the drill)

- [ ] Identify the deployment postures being rehearsed: connected, self-hosted,
      mirrored, offline/air-gapped.
- [ ] Identify the trust-root families in scope for the rehearsal run (release
      signing, update metadata, policy signing, mirror continuity, offline import
      roots).
- [ ] Confirm the rehearsal uses seeded fixtures / simulated states only; no raw
      private key material is present in the packet outputs.
- [ ] Confirm the “last-known-good” baseline is pinned and can be referenced by
      stable ids in the evidence output.

## Run (compromise → containment → rotation → recovery)

### 1) Trigger and containment

- [ ] Mint or import a signed `emergency_action_record` (channel freeze or
      equivalent containment) that states what still works locally and what is
      blocked.
- [ ] Mint or import a signed `revocation_record` when a signer/root must be
      revoked (durable history, explicit successor guidance).
- [ ] Verify the action/revocation includes:
      - signer continuity state,
      - distribution statuses for connected + mirror + manual import + offline
        paths (as applicable),
      - required actions with owners and deadlines, and
      - local continuity retained vs blocked capabilities.

### 2) Rotation publication

- [ ] Publish a `trust_root_rotation_state_record` (or the equivalent trusted
      pointer record) for every trust-root family in scope.
- [ ] Ensure the rotation state explicitly encodes:
      - planned overlap vs emergency no-overlap posture,
      - continuity statement ref when admissible, or explicit no-continuity
        emergency posture when predecessor is compromised,
      - revokes/supersedes relationships, and
      - review deadline / freshness floor.

### 3) Verification behavior under failure

- [ ] Exercise the failure-mode expectations:
      - stale trust root state
      - missing rotation metadata
      - revoked root/signer
      - lower-order bundle attempt under broken trust
      - mirror/offline continuity failure (stale/mismatch/continuity-broken)
- [ ] Confirm each failure produces a typed refusal/degraded state and does not
      collapse into “intermittent failure”.

### 4) Mirror and offline continuity (if applicable)

- [ ] Produce `mirror_integrity_packet_record` for at least one artifact family.
- [ ] Produce `revocation_propagation_record` for emergency metadata propagation.
- [ ] When manual import is used, produce:
      - `manual_import_receipt_record` and
      - `metadata_chain_entry_record` that roots at the authoritative origin.
- [ ] For offline/air-gapped paths, produce `offline_verification_packet_record`
      with a non-empty trust-root pointer block and explicit freshness semantics.

### 5) Customer-managed key rehearsal (if applicable)

- [ ] Simulate `tenant_org_managed_data_keys` outage (`customer_managed_unreachable`).
- [ ] Confirm managed-data families block while local work continues.
- [ ] Confirm ciphertext/metadata-only exports remain admissible when plaintext
      export is blocked.
- [ ] Restore reachability and require explicit recheck before resuming.

## Post-drill (evidence + review)

- [ ] Capture the drill outputs as export-safe artifacts (no raw secrets, no raw
      key bytes).
- [ ] Record what was blocked, what degraded, and what continued locally.
- [ ] Confirm every deployment posture rehearsed has a corresponding evidence
      packet set (connected / mirrored / offline).
- [ ] Confirm a post-incident review reference exists (even for a rehearsal) so
      “what we learned” can be tracked without relying on memory.

