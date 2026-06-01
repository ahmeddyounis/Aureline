# Stabilize transport governance and egress classification across update, marketplace, AI, docs, provider, and remote lanes

This stable lane makes egress routing for every named subsystem — update,
marketplace, AI, docs, provider, remote, and mirror/offline — visible and
verifiable enough that product, security review, support export, diagnostics,
and release packets can all explain: which route class was used for a given
lane, what egress decision was made (allowed, blocked by policy, blocked by
transport failure, mirror-routed, cached-offline, last-known-good), which
mirror was used when traffic was redirected, what cached/offline posture
is in force, and what last-known-good policy epoch governs the lane. The
runtime owner is
`aureline_remote::stabilize_transport_governance_and_egress_classification_across_update`.

The packet does **not** re-derive raw endpoint URLs, raw hostnames, raw
credentials, raw PAC script content, or raw policy bundle bodies. All
subsystem-specific route and egress status is projected through the single
closed-vocabulary transport governance model. This packet replaces
subsystem-specific status strings with one inspectable vocabulary.

## Contract

For the stable claim to hold, **all seven** of the following conditions must be
verified simultaneously:

1. **All seven required egress lanes covered** — the transport policy snapshot
   carries at least one `TransportPolicyRecord` for each of: `update`,
   `marketplace`, `ai`, `docs`, `provider`, `remote`, and `mirror_offline`.
2. **No raw private material exposed** — every lane record carries
   `raw_private_material_excluded: true`; no raw endpoint URLs, raw
   credentials, raw private keys, or raw PAC content cross this boundary.
3. **Local-core continuity declared** — every lane record carries
   `local_core_continuity_allowed: true` so local editing is never blocked
   by managed or network-dependent lane failures.
4. **Dependency class explicit** — every lane record carries a non-empty
   `dependency_class_token` (`local_only`, `network`, `managed`, or
   `air_gapped`).
5. **Typed egress decision** — every lane record carries a non-empty
   `egress_decision_token` so route selection is reconstructable from the
   typed record without parsing raw logs.
6. **Policy epoch ref present for network-dependent lanes** — every lane
   whose `dependency_class` is `network`, `managed`, or `air_gapped` carries
   a non-empty `last_known_good_policy_epoch_ref`.
7. **Control-plane / data-plane distinction recorded** — every lane carries
   non-empty `control_plane_status_token` and `data_plane_status_token` so
   control-plane impairment (policy evaluation unavailable) is distinguishable
   from data-plane impairment (traffic cannot flow) without inferring meaning
   from raw logs.

## Required behavior

`validate_transport_governance_page` rejects a page when its `defects` list
is non-empty.

`audit_transport_governance_page` runs the combined check and returns a typed
`Vec<TransportGovernanceDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is
the stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A lane record with `raw_private_material_excluded: false` (narrow reason:
  `raw_private_material_exposed`). The function returns immediately with this
  single defect and skips all other checks.

A missing required lane narrows to `Preview` rather than `Beta` because the
coverage gap prevents any verifiable claim for that lane.

Missing local-core continuity, dependency class, policy epoch ref, typed
egress decision, or plane-impairment distinction each narrow to `Beta`.

## Required egress lanes

| Lane token | Subsystem covered |
| --- | --- |
| `update` | Software update channel |
| `marketplace` | Extension marketplace / registry |
| `ai` | AI inference provider |
| `docs` | Documentation pack distribution |
| `provider` | Connected VCS / CI / partner providers |
| `remote` | Remote targets (SSH, remote agent, managed workspace) |
| `mirror_offline` | Declared signed mirrors and air-gapped offline postures |

All seven lanes must be covered for a stable claim.

## Egress decisions

| Token | Meaning |
| --- | --- |
| `allowed` | Request routed normally; egress allowed by policy and transport |
| `blocked_policy` | Request blocked by a transport policy rule |
| `blocked_transport` | Request blocked due to transport failure |
| `mirror_routed` | Request redirected to a declared signed mirror |
| `cached_offline` | Response served from local cache; no live egress |
| `last_known_good` | Falling back to last-known-good snapshot |
| `control_plane_impaired` | Control plane unavailable; data-plane may still flow |
| `data_plane_impaired` | Data plane unavailable; control plane reachable |

## Offline posture

| Token | Meaning |
| --- | --- |
| `online` | Lane is online and using live egress |
| `cached_content` | Response from locally cached content store |
| `offline_grace` | Operating within a declared offline-grace window |
| `mirror_served` | Traffic redirected to and served from a declared signed mirror |
| `disconnected` | Fully disconnected; no cache, no mirror, no grace window |

## Dependency class

| Token | Meaning |
| --- | --- |
| `local_only` | No external network dependency; no-account operation |
| `network` | Requires live network to a public or hosted endpoint |
| `managed` | Requires managed service endpoint controlled by enterprise admin |
| `air_gapped` | Operates against a declared mirror or air-gapped media only |

## Boundary

The following material stays outside this packet's support boundary:

- Raw endpoint URLs, raw hostnames, raw IP addresses, raw port numbers.
- Raw credentials, bearer tokens, or proxy authentication secrets.
- Raw private keys, raw CA certificates, raw PAC script bodies.
- Raw policy bundle bodies or raw rule text.
- Raw log lines or raw trace output.

Every exported field carries either a closed-vocabulary token, a plain-
language label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_transport_governance_page()` in
[`/crates/aureline-remote/src/stabilize_transport_governance_and_egress_classification_across_update/mod.rs`](../../../crates/aureline-remote/src/stabilize_transport_governance_and_egress_classification_across_update/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
subsystem-specific status strings.

## Canonical paths

- Runtime owner: `aureline_remote::stabilize_transport_governance_and_egress_classification_across_update`
- Artifact: `artifacts/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update.md`
- Fixtures: `fixtures/enterprise/m4/stabilize-transport-governance-and-egress-classification-across-update/`
- Schema: `schemas/enterprise/stabilize-transport-governance-and-egress-classification-across-update.schema.json`

## Verify

```bash
# Build
cargo build -p aureline-remote

# Tests
cargo test -p aureline-remote -- stabilize_transport_governance
```

All tests under
`stabilize_transport_governance_and_egress_classification_across_update::tests`
must pass. `seeded_transport_governance_page()` must produce zero defects and
a `stable` overall qualification token.
