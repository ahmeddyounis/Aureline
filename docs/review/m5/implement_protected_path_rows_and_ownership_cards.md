# M5 protected-path rows and ownership cards

Two reusable M5 protected-path governance components — the **protected-path row** and the
**ownership card** — so a user can tell *why* a file, package, or path is guarded more tightly,
*who* owns the resulting approval burden, whether the protection is advisory or authoritative,
whether it is the provider's final gate or a local estimate, whether owner backup coverage is
present or missing, and how ownership escalates, before they trust, merge, or escalate a governed
change.

- Implementation: `crates/aureline-review/src/implement_protected_path_rows_and_ownership_cards_with_protection_reason_owner_source_advisory_versus_authoritative_state_backup_coverage_and_escalation_continuity`
- Boundary schema: [`schemas/ui/m5-protected-path-ownership-controls.schema.json`](../../../schemas/ui/m5-protected-path-ownership-controls.schema.json)
- Checked support export: `artifacts/release/m5-protected-path-ownership-controls-proof/support_export.json`
- Narrowed fixtures: `fixtures/ui/m5-protected-path-ownership-controls/`

This lane *implements* the two families frozen by the
[protected-path governance component matrix](freeze_the_m5_protected_path_governance_component_matrix.md).
It reuses that matrix's frozen governance-state vocabulary — `advisory`, `authoritative`, `covered`,
`backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`, `local_estimate` — verbatim
rather than minting a drifted lexicon.

## Derived resolvers

The module derives every honesty axis from an honest input, so a component can never *assert* a
posture it did not earn:

- `resolve_enforcement_posture(source)` maps an owner-enforcement source to the exact enforcement
  posture (`provider_authoritative`, `locally_authoritative`, `advisory_only`, or `local_estimate`),
  and to the frozen governance-state token it renders under. Both the protected-path row and the
  ownership card call it, so their enforcement language is one truth. A `provider_branch_protection`
  or `provider_resolved_codeowners` source is provider-authoritative; a `local_manifest_enforced`
  source is locally authoritative; a `local_manifest_advisory` source is advisory-only; and a
  `local_heuristic_match` or `inferred_from_authorship` source is a local estimate.
- `resolve_owner_coverage_posture(source)` maps an owner-coverage source to the exact coverage
  posture (`covered_with_backup`, `backup_missing`, `unresolved`, or `policy_hidden`) and continuity
  state. Only `covered_with_backup` is clean coverage; every other posture degrades explicitly and
  renders under `backup_missing`, never `covered`.

## Components

- **Protected-path row** — names its path or pattern, protection reason, owner-source label,
  advisory-versus-authoritative enforcement, evaluation freshness, and the exact rule source it can
  open. `OpenRuleSource`, `InspectEnforcementAuthority`, and `ReviewProtectionReason` are always
  offered, so the governing rule, the enforcement authority, and the protection reason stay
  inspectable before a user trusts the guard.
- **Ownership card** — names its primary and backup owners as **export-safe role aliases** (never
  person-specific private contact detail), owner-source class, advisory-versus-authoritative
  enforcement, coverage posture, continuity state, and escalation path. `InspectOwnerSource`,
  `ReviewBackupCoverage`, and `OpenEscalationPath` are always offered.

## Guardrails

The validator refuses an export that would let any of these slip:

- An advisory hint claiming authoritative enforcement, or a local estimate claiming
  provider-authoritative enforcement.
- Missing backup, unresolved, or policy-hidden owner state presented as clean coverage.
- A guarded path with no named protection reason, owner source, evaluation freshness, or openable
  rule source.
- A missing advisory / local-estimate / backup-missing / unresolved / policy-hidden note where the
  derived posture requires one.
- A governance-state vocabulary that omits the derived enforcement or coverage token.
- An owner alias carrying person-specific contact detail (an email address) instead of a role alias.
- Any raw CODEOWNERS body, manifest, provider payload, credential, or secret in the export boundary.

## Regenerating artifacts

The checked support export, the Markdown summary, and the two narrowed fixtures are produced from
the one canonical seed builder in the module's tests, guarded behind an env gate:

```
GEN_PROTECTED_PATH_OWNERSHIP_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review --lib generate_artifacts -- --ignored
```
