# M5 Repository-Bootstrap Accessibility & Auto-Narrowing Parity (M05-1194)

This contract is the accessibility-localization-support-export parity and honest auto-narrowing capstone over
the frozen [M5 repository-bootstrap matrix](m5_repository_bootstrap_contract.md). Where the freeze matrix
defines the five governed project-entry acquisition families and the 1189–1192 implementation lanes resolve
their per-surface source-locator, checkout-plan, credential-posture, staged-trust, post-open-queue, and
bootstrap-evidence truth, this lane certifies — per family — that acquisition truth stays reachable and
exportable even when the active profile is degraded or only partially qualified.

- **Schema:** `schemas/workspaces/m5-repository-bootstrap-accessibility-parity.schema.json`
- **Support export / CSV / report:** `artifacts/release/m5-repository-bootstrap-accessibility-parity/`
- **Fixtures:** `fixtures/workspaces/m5-repository-bootstrap-accessibility-parity/`
- **Canonical packet id:** `m5-repository-bootstrap-accessibility-parity:stable:0001`

## What it certifies

Each row keys on one `M5RepositoryBootstrapFamily` and reuses the frozen matrix vocabulary — family tokens,
required labels, downgrade triggers, and consumer surfaces — rather than minting parallel synonyms.

1. **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family exposes a
   non-visual path into the same repository-bootstrap identity, semantic role, registry reference, source
   locator, checkout plan, and credential posture the rendered surface shows. Structure-heavy families
   (open-archive extraction-plan, import-bundle staged-trust, resume-snapshot evidence-lineage) additionally
   bind their structured layout to a flat list / textual / CLI path.
2. **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed tokens
   and opaque refs without a raw payload.
3. **Honest auto-narrowing.** When an open-archive family's checkout / extraction-plan proof can only be
   partially disclosed, an import-bundle staged-trust fence cannot be confirmed, or a resume-snapshot bootstrap
   / mirror-signer evidence has aged out or is policy-blocked, the family's claim auto-narrows from
   `trusted_acquisition_surface` / `reviewable_acquisition_surface` to a `checkout_plan_disclosed_projection` /
   `trust_stage_unverified_projection` / `bootstrap_evidence_unverified_projection`, discloses the narrowing
   with a precise frozen trigger and binding dimension, and preserves the canonical identity. A
   trust-overclaimed, evidence-aged, or policy-blocked state can never keep a trusted, stable acquisition claim.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the acquisition engine, shell UI, workspace
   service, git service, trust service, diagnostics, docs / help, CLI export, and support export so product,
   help, and release publication stay aligned on downgrade behavior.

## Acceptance criteria mapping

- **Accessibility and CLI/export paths inspect the same bootstrap truth shown in GUI entry and recovery
  surfaces** → the per-row reach axes plus the export-summary and copy-export parity.
- **Claim publication and support exports downgrade automatically when B142 evidence is stale, partial, or
  missing** → the auto-narrow blocks bound to the frozen `proof_stale`, `staged_trust_rule_unstated`, and
  `lost_signer_or_mirror_provenance_across_offline_or_mirrored_fetches` triggers.
- **No claimed entry profile can stay green after checkout-plan, trust-stage, or bootstrap evidence proof ages
  out** → `cannot_be_shown_trusted` flags the trust-stage-unconfirmed and bootstrap-evidence-unconfirmed states
  so the effective claim can never assert a trusted acquisition surface.

The packet is metadata-only: raw secret blobs, machine-specific sensitive paths, plaintext payloads, and
endpoint refs never cross this boundary.
