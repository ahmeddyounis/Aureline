# Harden the connected-provider registry, capability matrix, and snapshot degradation across stable provider families

This stable packet makes the connected-provider registry explicit and
verifiable for every required (provider family, actor identity) pair across
the three stable provider families: `code_host`, `issue_tracker`, and
`ci_checks`. For each pair the descriptor declares which callback/ingress path
class is in use, which object kinds are supported and with what mutation
posture, whether local-core continuity is preserved, which dependency class
applies, and whether the provider snapshot is fresh or degraded. The runtime
owner is
`aureline_remote::harden_the_connected_provider_registry_capability_matrix_and`.

The packet does **not** expose raw credentials, raw OAuth tokens, raw API
keys, raw private keys, raw endpoint URLs, or raw callback secrets. All
provider-specific status is projected through the single closed-vocabulary
provider registry model. This packet replaces provider-specific status strings
with one inspectable, export-safe vocabulary.

## Contract

For the stable claim to hold, **all six** of the following conditions must be
verified simultaneously:

1. **All nine required (family, actor) pairs covered** — the descriptor
   snapshot must carry at least one `ProviderDescriptorRecord` for each of the
   nine pairs: (`code_host`, `human_account`), (`code_host`,
   `installation_grant`), (`code_host`, `delegated_credential`),
   (`issue_tracker`, `human_account`), (`issue_tracker`, `installation_grant`),
   (`issue_tracker`, `delegated_credential`), (`ci_checks`, `human_account`),
   (`ci_checks`, `installation_grant`), (`ci_checks`, `delegated_credential`).
   A gap narrows to `Preview`.
2. **No raw private material exposed** — every descriptor record carries
   `raw_private_material_excluded: true`. No raw credentials, tokens, keys, or
   callback secrets cross this boundary. A violation immediately withdraws the
   page (hard guardrail; skips remaining conditions).
3. **Local-core continuity declared** — every descriptor carries
   `local_core_continuity_allowed: true` so local editing is never blocked by
   provider-connectivity failures. Absence narrows to `Beta`.
4. **Dependency class explicit** — every descriptor carries a non-empty
   `dependency_class_token` (`local_only`, `network`, `managed`, or
   `air_gapped`). Absence narrows to `Beta`.
5. **Callback/ingress path class named** — every descriptor carries a non-empty
   `callback_path_token` (`public_saas`, `mirrored_ingress`, or
   `customer_controlled`). Absence narrows to `Beta`.
6. **Object support declared** — every descriptor carries at least one
   `ObjectSupportEntry` with an explicit mutation posture. An empty
   `object_support` list narrows to `Beta`.

## Required behavior

`validate_provider_registry_page` rejects a page when its `defects` list
is non-empty.

`audit_provider_registry_page` runs the combined check and returns a typed
`Vec<ProviderRegistryDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is
the stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A descriptor with `raw_private_material_excluded: false` (narrow reason:
  `raw_private_material_present`). The function returns immediately with this
  single defect and skips all other checks.

A missing required (family, actor) pair narrows to `Preview` rather than
`Beta` because the coverage gap prevents any verifiable claim for that pair.

Missing local-core continuity, dependency class, callback path class, or
object support each narrow to `Beta`.

## Required (family, actor) pairs

| Provider family | Actor identity class | Description |
| --- | --- | --- |
| `code_host` | `human_account` | Human sign-in (OAuth / SSO) to code host |
| `code_host` | `installation_grant` | App installation grant on code host |
| `code_host` | `delegated_credential` | Delegated machine credential on code host |
| `issue_tracker` | `human_account` | Human sign-in to issue tracker |
| `issue_tracker` | `installation_grant` | App installation grant on issue tracker |
| `issue_tracker` | `delegated_credential` | Delegated machine credential on issue tracker |
| `ci_checks` | `human_account` | Human sign-in to CI/checks provider |
| `ci_checks` | `installation_grant` | App installation grant on CI/checks provider |
| `ci_checks` | `delegated_credential` | Delegated machine credential on CI/checks provider |

All nine pairs must be covered for a stable claim.

## Callback / ingress path class

| Token | Meaning |
| --- | --- |
| `public_saas` | Provider callback/ingress goes through a public SaaS endpoint |
| `mirrored_ingress` | Callback/ingress goes through a declared signed mirror ingress |
| `customer_controlled` | Callback/ingress goes through a customer-controlled gateway |

## Object kinds

| Token | Meaning |
| --- | --- |
| `pull_request` | Pull request or merge request |
| `branch` | Git branch |
| `issue_or_work_item` | Issue, ticket, or work item |
| `check_run` | CI check run or status check |
| `pipeline_run` | Pipeline run or workflow run |
| `release_or_tag` | Release, tag, or deployment |
| `review_comment` | Review comment or annotation |
| `unknown_object_kind` | Unknown object kind (fallback; not export-stable) |

## Mutation posture

| Token | Meaning |
| --- | --- |
| `inspect_only` | Read-only; no mutation via this actor/path |
| `mutate_allowed` | Full mutation allowed via this actor/path |
| `mutate_gated` | Mutation allowed but gated by approval or policy |
| `mutate_disallowed_by_policy` | Mutation explicitly disallowed by policy |

## Publish modes

| Token | Meaning |
| --- | --- |
| `publish_now` | Publish immediately to provider |
| `local_draft` | Keep as local draft; no push to provider |
| `publish_later_queue` | Enqueue for later publish (publish-later queue) |
| `open_in_provider` | Open provider UI in browser (handoff) |
| `inspect_only` | Read-only; no publish path |

## Snapshot freshness

| Token | Usable | Meaning |
| --- | --- | --- |
| `fresh` | ✓ | Snapshot is current; within freshness window |
| `stale_within_window` | ✓ | Stale but within the declared grace window; still usable |
| `stale_expired` | ✗ | Stale and past grace window; must be refreshed |
| `revoked` | ✗ | Descriptor has been revoked |
| `degraded_unknown` | ✗ | Degraded for unknown reason |

## Dependency class

| Token | Meaning |
| --- | --- |
| `local_only` | No external network dependency; no-account operation |
| `network` | Requires live network to a public or hosted endpoint |
| `managed` | Requires managed service endpoint controlled by enterprise admin |
| `air_gapped` | Operates against a declared mirror or air-gapped media only |

## Boundary

The following material stays outside this packet's support boundary:

- Raw credentials, bearer tokens, OAuth tokens, API keys, or access tokens.
- Raw OAuth callback secrets, client secrets, or private keys.
- Raw endpoint URLs, raw hostnames, raw IP addresses, raw port numbers.
- Raw policy bundle bodies or raw rule text.
- Raw log lines or raw trace output.

Every exported field carries either a closed-vocabulary token, a plain-
language label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_provider_registry_page()` in
[`/crates/aureline-remote/src/harden_the_connected_provider_registry_capability_matrix_and/mod.rs`](../../../crates/aureline-remote/src/harden_the_connected_provider_registry_capability_matrix_and/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
provider-specific status strings.

## Canonical paths

- Runtime owner: `aureline_remote::harden_the_connected_provider_registry_capability_matrix_and`
- Artifact: `artifacts/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and.md`
- Fixtures: `fixtures/enterprise/m4/harden-the-connected-provider-registry-capability-matrix-and/`
- Schema: `schemas/enterprise/harden-the-connected-provider-registry-capability-matrix-and.schema.json`

## Verify

```bash
# Build
cargo build -p aureline-remote

# Tests
cargo test -p aureline-remote -- harden_the_connected
```

All tests under
`harden_the_connected_provider_registry_capability_matrix_and::tests`
must pass. `seeded_provider_registry_page()` must produce zero defects and
a `stable` overall qualification token.
