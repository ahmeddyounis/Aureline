# Stabilize deployment summary, residual-dependency ledger, region/key-mode truth, and control-plane/data-plane continuity

This stable lane makes deployment mode, residency, region, key ownership,
residual vendor dependency, and local-core continuity explicit enough that no
managed or sovereign claim survives on implication alone. It covers five
deployment profiles — `individual_local`, `managed_cloud`, `enterprise_online`,
`self_hosted`, and `air_gapped` — and proves that every profile can explain
control-plane impairment, data-plane impairment, residual vendor services, and
mirror/local fallback without contradictory wording between product, docs, and
support export. The runtime owner is
`aureline_policy::stabilize_deployment_and_residency_truth`.

The packet does **not** re-derive deployment bundle bodies, raw hostnames,
raw tenant identifiers, or raw key bytes. It re-exports closed-vocabulary
tokens from the deployment summary card schema verbatim and adds the stability
invariants needed for a single evidence packet.

## Contract

For the stable claim to hold, **all five** of the following conditions must be
verified simultaneously:

1. **Vocabulary consistent across surfaces** — every claimed deployment profile
   token appears in the closed `deployment_profile` vocabulary; About, Help,
   diagnostics, service-health, and support-export surfaces all resolve the same
   closed-vocabulary token for the same running deployment profile.
2. **Residual-dependency ledger complete** — every non-`individual_local` profile
   row carries at least one residual-dependency row per vendor-bound or externally
   owned control-plane service; no profile with hosted control-plane services
   claims zero residual dependencies.
3. **Plane separation enforced** — every plane-status strip keeps control-plane
   service impairment (identity, policy, catalog, relay) separate from
   data-plane capability impairment (local editing, save, search, Git); a
   continue-local path is preserved whenever data-plane capabilities remain
   `available_local_safe`.
4. **Mirror/offline artifact rows present** — every profile claiming
   `online_mirror_only` or `offline_air_gapped` mirror/offline state carries at
   least one mirror/offline artifact row with signer, digest, freshness, and
   pin-state fields.
5. **Sign-out/deprovision scope declared** — every profile whose tenant/org
   scope is `customer_tenant` or `shared_multi_tenant` carries sign-out/
   deprovision scope metadata naming what remains local on device, what stays
   tenant-scoped, and what is retained for policy or audit reasons.

## Required behavior

`validate_deployment_residency_stabilize_page` rejects a page when its
`defects` list is non-empty.

`audit_deployment_residency_stabilize_page` runs the combined check and returns
a typed `Vec<DeploymentResidencyStabilizeDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A `ImpliedSovereigntyUnproven` defect when a profile asserts a sovereignty or
  independence claim (`self_hosted`, `air_gapped`) with no supporting evidence
  rows. The function returns immediately with this defect and skips all other
  checks for that profile.

A vocabulary inconsistency narrows to `Beta` (not `Withdrawn`) because it
prevents claim verification but does not represent a hard security guardrail.

## Deployment profile vocabulary

| Profile token | Product label | Sovereignty | Residual dep required | Mirror artifact required |
|---|---|---|---|---|
| `individual_local` | Desktop — Local first | No | No | No |
| `managed_cloud` | Managed SaaS | No | Yes | No |
| `enterprise_online` | Hybrid remote-attach | No | Yes | No |
| `self_hosted` | Self-hosted sovereign | Yes (evidenced) | Yes | When mirror/offline claimed |
| `air_gapped` | Air-gapped mirror only | Yes (evidenced) | Yes | Yes |

## Plane separation guardrail

Every service-health and outage surface that registers a
`DeploymentResidencyPlaneStrip` must set both:

- `control_data_plane_separated: true` — control-plane service impairment
  (identity, policy, catalog, relay) does not appear in the data-plane
  capability blockage list and vice versa.
- `continue_local_path_preserved: true` — when data-plane capabilities are
  `available_local_safe`, the surface preserves a continue-local path rather
  than forcing the user to wait on a control-plane recovery.

## Boundary

The following material stays outside this packet's support boundary:

- Raw hostnames, raw tenant identifiers, raw region labels.
- Raw key bytes or trust-root fingerprints.
- Raw policy rule bodies, raw exception justification text.
- Raw extension or model artifact binaries.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Deployment profile vocabulary | `/schemas/deployment/deployment_summary_card.schema.json` |
| Residual-dependency ledger | `/artifacts/governance/residual_dependencies.yaml` |
| Plane separation contract | `/schemas/deployment/plane_status.schema.json` |
| Mirror/offline artifact rows | `/schemas/deployment/deployment_summary_card.schema.json` |
| Stable qualification | `aureline_policy::stabilize_deployment_and_residency_truth` |
| Artifact evidence | `artifacts/policy/m4/stabilize-deployment-and-residency-truth.md` |
| Proof packet schema | `schemas/policy/deployment-profile.schema.json` |

## Verify

```bash
# Build
cargo build -p aureline-policy

# Tests
cargo test -p aureline-policy -- stabilize_deployment_and_residency_truth
```

All tests under `stabilize_deployment_and_residency_truth::tests` must pass.
`seeded_deployment_residency_stabilize_page()` must produce zero defects and a
`stable` overall qualification token covering all five deployment profiles.
