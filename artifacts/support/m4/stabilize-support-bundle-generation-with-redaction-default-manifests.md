# Stabilized support-bundle generation with redaction-default manifests and chain-of-custody fields — Artifact

## Status

**Stable** — hardened M4 support-bundle generation with redaction-default
manifests, chain-of-custody fields, and explicit consent escalation for
high-fidelity incident capture.

## Checked-in outputs

| Output | Path |
|--------|------|
| Implementation | `crates/aureline-support/src/stabilize_support_bundle_generation_with_redaction_default_manifests/mod.rs` |
| Boundary schema | `schemas/support/stabilize_support_bundle_generation_with_redaction_default_manifests.schema.json` |
| Reviewer doc | `docs/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests.md` |
| Fixture corpus | `fixtures/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests/` |

## What is stabilized

The M3 support-bundle contract is promoted to a stable generation contract by adding:

1. **Explicit generation-mode distinction** — `ordinary_redaction_default` vs
   `high_fidelity_incident_capture` so UI, CLI, and manifest surfaces never
   silently widen evidence.
2. **Consent escalation** — `not_required`, `explicit_user_consent`, or
   `admin_policy_override` pinned to every manifest.
3. **Destination class** — `local_only_review`, `vendor_case_handoff`,
   `user_initiated_upload`, `managed_admin_handoff`, `private_security_channel`.
4. **Retention class and note** — `short_term`, `medium_term`, `long_term`,
   `legal_hold` with a reviewer-visible retention sentence.
5. **Chain-of-custody entries** — mandatory, sequenced, with actor, action,
   location, and note fields.
6. **Recovery-ladder hooks** — eight required hooks covering safe mode,
   open without restore, export evidence, retry fault domain, disable recent
   extension, reset ephemeral cache, run Project Doctor, and bounded repair
   preview. Every hook preserves user state.

## Seeded support scenarios

The fixture corpus covers three profile classes:

- `ordinary_redaction_default` — local-only review with default redaction.
- `high_fidelity_incident_capture` — user-consented vendor case handoff.
- `policy_mandated_audit` — admin-policy-override managed handoff.

Each fixture includes:
- one stabilized support-bundle manifest,
- explicit included/excluded class lists,
- non-empty chain-of-custody,
- eight recovery-ladder hook bindings.

## Verification

Run the protected tests:

```bash
cargo test -p aureline-support --test stabilize_support_bundle_generation_with_redaction_default_manifests
```

## Risks and follow-ups

- Live support-bundle assembly, byte-level redaction, and upload transport are
  out of scope and land with the runtime-host and chrome consumers.
- Automated chain-of-custody event generation from the runtime is not covered;
  only the typed manifest contract and validation are certified.
- Cross-tenant policy reconciliation for admin-policy-override remains
  unsupported.
