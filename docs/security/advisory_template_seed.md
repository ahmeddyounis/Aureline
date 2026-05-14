# Advisory Template Seed

This seed is the reusable advisory template for alpha incident response,
release packets, support exports, Help/About notices, and mirror-safe
distribution notes. It uses the existing advisory identity, affected-install,
support-bundle, exact-build, and rollback vocabulary instead of creating a
new publication dialect.

Canonical contracts:

- [`advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
- [`advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
- [`affected_install_assessment.schema.json`](../../schemas/security/affected_install_assessment.schema.json)
- [`severity_matrix.md`](./severity_matrix.md)
- [`advisory_surface_contract.md`](./advisory_surface_contract.md)
- [`release_support_crosswalk.yaml`](../../artifacts/release/release_support_crosswalk.yaml)
- [`support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`update_rollback_sequence.yaml`](../../artifacts/release/update_rollback_sequence.yaml)

## Required Header

| Field | Required content |
|---|---|
| Advisory ID | `AURELINE-ADV-YYYY-NNNN+`; this is the stable copy-safe join key. |
| CVE / GHSA aliases | `not_assigned`, `pending_publication`, or the assigned public ID. |
| Severity | One value from `security_severity.*`; do not invent local labels. |
| Disclosure state | Public, staged private, mirror-only, or private as defined by the advisory schemas. |
| Affected surface | Build binary, docs pack, extension, package/dependency, release channel, trust root, capability/route, or managed surface. |
| Current action state | Informational, review recommended, action required, blocking, immediate remediation, or mitigation complete. |
| Publication state | Draft, pending review, pending publication, published, withdrawn, or exercise-only. |

## Affected Build Scope

Every advisory draft MUST name the affected builds with exact-build identity
refs. Version labels are not enough.

Use this shape for each row:

```text
- Exact-build identity:
- Source artifact or packet:
- Channel class:
- Install-profile card refs:
- Installed match state:
- Impacted components:
- Current mitigation state:
- Local continuity note:
```

Rules:

- At least one exact-build identity ref is required for any moderate,
  high, critical, or emergency advisory.
- If a row uses mirrored or offline metadata, its freshness state must be
  visible in the same row.
- The local continuity note must say what remains safe. It must not imply
  local work, user-authored files, or recovery state are unsafe unless the
  evidence proves that.

## Current Mitigation

Use this shape:

```text
Current mitigation:
- Mitigation state:
- Mitigation summary:
- Fixed exact-build identity ref:
- Compensating control:
- Support packet refs:
- Evidence refs:
```

Rules:

- A mitigation may be available, in progress, complete, blocked by
  revocation, or not started. Do not collapse those states into "fixed".
- A compensating control is not a fixed build. State that distinction in
  the summary.
- Support bundles and incident packets stay redaction-safe by default.
  Raw dumps, raw logs, raw paths, raw command lines, raw reporter identity,
  raw exploit material, and secrets do not belong in advisory copy.

## Rollback Route

Use this shape:

```text
Rollback route:
- Route class:
- Rollback target ref:
- Rollback sequence ref:
- Required checkpoint ref:
- Reversal class:
- Preserved state classes:
- User-visible summary:
```

Rules:

- Rollback copy must name the reversal class: exact, compensating,
  regenerate, manual, or audit-only.
- Rollback guidance must preserve user-authored files, local recovery
  evidence, support-bundle refs, and advisory history unless a specific
  row proves otherwise.
- Do not suggest broad reset or deletion as advisory guidance. Escalate
  with a support packet when safe local rollback is not available.

## Known Limits

Use this shape:

```text
Known limits:
- Truth state:
- Summary:
- Known-limit refs:
- Overclaim blockers:
```

Rules:

- Known limits are release truth, not caveats hidden in prose.
- If protected fitness, support diagnosis, symbolication, mirror freshness,
  or publication automation is stale or partial, the advisory must say so.
- A known limit narrows the claim. It must not be worded as a passing
  health statement.

## Support And Export Refs

Every advisory draft keeps support/export linkage in one section:

```text
Support/export refs:
- Support packet refs:
- Incident workspace refs:
- Release evidence packet refs:
- Redaction class:
- Data classes included:
- Data classes excluded:
- Local-only export path:
```

Default support wording:

> Support packets are generated locally first, previewable before export,
> and metadata-safe by default. Raw private material is excluded unless a
> separate reviewed opt-in path exists.

## Reusable Advisory Body

```text
Title:
Advisory ID:
Severity:
Status:

Summary:

Affected builds:

Current mitigation:

Rollback or recovery route:

Known limits:

Local continuity:

Support/export evidence:

Disclosure and history:
```

The body above is intentionally stable. Future incidents should fill the
fields, update the refs, and narrow claims where evidence is missing; they
should not rewrite the vocabulary.
