# Advisory identity and affected-install assessment contract

This document freezes the pre-implementation contract for the two
foundational security records that every advisory surface relies on:

- **Advisory identity records** (stable advisory ID family + copy-safe reference).
- **Affected-install assessment records** (local installed-state assessment for one advisory).

The goal is one resolvable identity model: advisories, emergency disables,
revocations, updates, Help/About, support exports, admin exports, and
offline bundles must all be able to talk about the same incident without
alias drift, browser dependence, or privacy-widening side effects.

Companion artifacts:

- [`/schemas/security/advisory_identity.schema.json`](../../schemas/security/advisory_identity.schema.json)
  - machine boundary for `advisory_identity_record`.
- [`/schemas/security/affected_install_assessment.schema.json`](../../schemas/security/affected_install_assessment.schema.json)
  - machine boundary for `affected_install_assessment_record`.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - canonical advisory record; its identity and severity vocabulary are shared with `advisory_identity_record`.
- [`/schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  - advisory-surface projection; consumes `advisory_identity_record` and affected-install assessment fields.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md)
  - severity vocabulary and subject-kind vocabulary.
- [`/docs/security/emergency_disable_bundle_contract.md`](./emergency_disable_bundle_contract.md)
  and [`/schemas/security/emergency_disable_bundle.schema.json`](../../schemas/security/emergency_disable_bundle.schema.json)
  - emergency disable bundle contract referenced by `disable_bundle_refs[]`.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  and [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  - install-profile cards and exact-build identities every assessment binds to.
- [`/fixtures/security/advisory_identity_cases/`](../../fixtures/security/advisory_identity_cases/)
  - worked fixtures covering active, mirror-only, resolved/superseded history linkage, and emergency-disable linkage.

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` §10.9 (vulnerability disclosure and response).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.8 and Appendix AS.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13, §9.28, Appendix BS.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` “Security advisory cards, emergency notices, and disclosure links”.

If this contract disagrees with those source documents, the source
documents win and this file, schema, and fixtures update together.

## Why this exists

Security notices are only trustworthy if users and operators can:

- copy stable advisory IDs (Aureline advisory ID, CVE, GHSA) from any surface;
- determine whether their install is affected without opening a browser; and
- export or mirror the same truth without leaking unrelated account, tenant,
  or private-report information.

These records pin the minimum identity + assessment vocabulary so every
surface can render, export, and archive security response truth
mechanically.

## Advisory identity record

The `advisory_identity_record` is the copy-safe identity envelope every
surface can cite.

Required identity fields:

- `advisory_identity.aureline_advisory_id` — stable advisory ID (required).
- `advisory_identity.cve_id` — optional CVE alias; nullable until assigned.
- `advisory_identity.ghsa_id` — optional GHSA alias; nullable until minted.
- `advisory_identity.additional_alias_refs[]` — reserved opaque alias refs for
  future schemes (kept under one array so new alias families do not mint a new
  top-level identity system).

Required classification fields:

- `severity_class` — frozen machine-readable severity vocabulary.
- `advisory_subject_kind` — frozen subject-kind vocabulary for the advisory’s
  primary subject family.

Required publication/lifecycle fields:

- `publication_state` — draft/published/withdrawn state for the identity record.
- `visibility_class` — disclosure boundary for safe copy and export
  (`private`, `internal`, `staged_private`, `public`, `mirror_only`).
- `history.history_state` plus `history.*` linkage — keeps resolved/superseded
  advisories reachable for audit and support continuity.

Copy-safe reference fields:

- `title` and `summary` are export-safe by design (no raw exploit payloads, no
  raw reporter identities, no raw hostnames/paths, no private registry URLs).
- `copy_safe_ids[]` is the cross-surface “copy contract”:
  every visible Aureline/CVE/GHSA ID appears as a typed row with explicit
  copy availability and visibility boundary.

Related-response linkage:

- `emergency_action_refs[]`, `revocation_refs[]`, and `disable_bundle_refs[]`
  keep emergency disable and revocation state tied to the same advisory family.

## Affected-install assessment record

An `affected_install_assessment_record` answers: “Is this install affected,
what is the mitigation state, and what remains safe locally?” without relying
on browser-only disclosure pages.

Every assessment binds to one advisory identity:

- `advisory_identity.aureline_advisory_id` is the stable join key across
  advisory cards, release notes, support/admin exports, and offline bundles.

Minimum required fields (frozen):

- `install_mode_class` + `channel_class` + `install_profile_card_ref` —
  explains which install lane is being evaluated.
- `exact_build_identity_ref` — the concrete build identity being assessed.
- `affected_subjects[]` — typed subject refs binding the assessment to exact
  artifact identities (never free-text “version 1.x”).
- `local_mitigation_state` — what the user/admin has already done
  (not started, in progress, complete, blocked by revocation, etc.).
- `mirror_freshness_class` — whether mirrored/offline metadata is current.
- `local_continuity_note` — what still works safely if the affected lane is
  disabled or narrowed before a fix ships.

Optional but preferred fields:

- `impacted_components[]` — component refs the UI can group under, without
  weakening the authoritative `affected_subjects[]` bindings.
- `fixed_exact_build_identity_ref` or `compensating_control_note` — the “fixed
  version or compensating control” that makes mitigation explainable offline.

## Surface rules (non-negotiable)

- All surfaces resolve one `aureline_advisory_id` into one identity record; CVE
  and GHSA are aliases, not parallel primary keys.
- Support/admin exports and offline bundles must include enough data to copy IDs
  and understand local impact without external web resolution.
- History is durable: resolved/superseded/withdrawn advisories remain
  inspectable with downgraded prominence; silent disappearance is forbidden.

## Surface obligations (minimum)

Every surface that renders, exports, or mirrors security response truth MUST be
able to project from the same advisory identity and assessment records.

| Surface | Must carry (identity) | Must carry (assessment) |
|---|---|---|
| Advisory center card / row | `advisory_identity`, `severity_class`, `title`, `summary`, `copy_safe_ids[]`, `history.*` | `install_mode_class`, `channel_class`, `exact_build_identity_ref`, `local_mitigation_state`, `mirror_freshness_class`, `local_continuity_note` |
| Emergency banner / notice | identity fields above + `emergency_action_refs[]` / `revocation_refs[]` / `disable_bundle_refs[]` as applicable | same assessment minimum, with `local_mitigation_state` reflecting the block/disable state |
| Update flow / installer notice | identity fields above | same assessment minimum + fixed build or compensating control when available |
| Help/About | identity fields above (copy-safe) | one assessment for the running install lane/build identity |
| Support export packet | identity fields above (copy-safe) | assessment rows for the local install lanes cited by the packet |
| Admin export / fleet view | identity fields above | assessment rows aggregated per install lane, with explicit mirror freshness and continuity notes |
| Offline bundle / mirror snapshot | identity fields above | assessment rows included for the shipped lanes in the bundle, with explicit freshness state |
