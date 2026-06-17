# M5 package-state, manifest-scope, registry-auth, and lockfile-authority matrix

This document describes the canonical packet that freezes the **M5 package-state
and mutation matrix** — the cross-ecosystem vocabulary every M5 package surface
must agree on before any mutation widens. It is the user-facing companion to the
governed artifact at
`artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json`
and the typed model in the `aureline-deps` crate
(`freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`).

Where [`package_mutation_and_registry_review`](../../help/deps/package-mutation-and-registry-review.md)
reviews one operation and
[`package_set_inventory_and_scope_truth`](../../help/deps/package-set-inventory-and-scope-truth.md)
inventories a package set, this packet is the shared **vocabulary and binding
contract** the whole lane references, so direct-versus-transitive state,
manifest scope, registry auth, lockfile authority, and rollback class stop being
ecosystem-specific folklore split across dashboards, the CLI, and package-manager
side effects.

## Canonical package-state labels

The packet freezes twelve canonical package-state labels. Each label is rendered
identically across the desktop workspace, CLI/headless, AI context, review
workspace, support exports, and release/public-truth surfaces:

1. **`direct`** — direct dependency of the target manifest.
2. **`transitive`** — transitive dependency resolved through another package.
3. **`workspace_local`** — workspace-local member dependency.
4. **`path_or_vcs_source`** — filesystem-path or version-control source.
5. **`resolved_exact`** — exact resolved version, commit, path, or snapshot id.
6. **`policy_pinned`** — policy-pinned version or source constraint.
7. **`advisory_open`** — an advisory is open against the package.
8. **`suppressed_until`** — an advisory is suppressed until a stated expiry.
9. **`license_review_required`** — license review is required before shipping.
10. **`offline_snapshot_only`** — only an offline snapshot or local cache is available.
11. **`auth_required`** — registry access requires authentication that is not satisfied.
12. **`unknown_or_stale`** — the package state could not be established or is stale.

## Requested-versus-resolved truth stays separate

Every label declares an **identity side**, which is how the matrix keeps
requested and resolved truth from collapsing into a single field:

- **`requested_constraint`** — a constraint the user or automation requested
  before resolution (`policy_pinned`).
- **`resolved_identity`** — a fact the resolver produced (`direct`, `transitive`,
  `workspace_local`, `path_or_vcs_source`, `resolved_exact`).
- **`finding_overlay`** — an advisory, suppression, or license overlay on a
  resolved package (`advisory_open`, `suppressed_until`, `license_review_required`).
- **`resolution_environment`** — a registry, mirror, cache, or auth posture of the
  resolution environment (`offline_snapshot_only`, `auth_required`).
- **`indeterminate_state`** — the package state could not be established
  (`unknown_or_stale`).

A label may describe the requested identity **or** the resolved identity, but
never both. Validation fails if a row claims both, so the requested-versus
resolved boundary can never be erased by hand.

## No state collapses into a generic message

Each state row and each registry-source cell maps to a **specific message class**.
The closed message vocabulary names two forbidden generic classes —
`generic_package_not_found` and `generic_install_failed` — only so the matrix can
**forbid** them: no row or cell may carry a generic class, and validation fails if
one does.

The offline-snapshot/cache-only, auth-required, and unknown/stale states are
explicitly **guarded**: they always render their specific disclosure
(`offline_snapshot_disclosure`, `auth_required_disclosure`,
`unknown_or_stale_disclosure`) so they can never read as "package not found" or
"install failed". The registry-source cells extend the same guard to the
**source authority**: an enterprise mirror discloses `mirror_backed_source`, a
local cache discloses `cache_only_source`, and an offline snapshot discloses the
offline posture, so a mirror miss, cache miss, or offline run discloses its real
cause instead of a generic failure.

## Control objects the lane shares

The packet pins six control-object vocabularies the operation review, registry
panel, and lockfile guards all draw from:

- **Manifest scope** — `whole_workspace`, `selected_manifest`, `workset_slice`,
  `workspace_member`, `path_or_vcs_target`. A whole-workspace scope requires an
  explicit, scoped confirmation; it can never be applied ambiently.
- **Registry source** — `public_registry`, `private_registry`, `enterprise_mirror`,
  `local_cache`, `offline_snapshot`.
- **Auth mode** — `anonymous`, `os_store_credential`, `token_credential`,
  `browser_or_device_sign_in`, `policy_inherited_credential`, and
  `auth_required_unsatisfied` (which blocks a mutation until auth is satisfied).
- **Lockfile authority** — `exact_lockfile_pinned`, `manifest_range_governed`,
  `frozen_by_policy`, `lockfile_missing`, `lockfile_divergent` (which blocks a
  mutation until reconciled), and `authority_unknown`.
- **Resolver identity** — `first_party_resolver`, `ecosystem_native_resolver`,
  `mirror_backed_resolver`, `offline_cache_resolver`, `resolver_unknown`.
- **Rollback class** — `reversible_checkpointed`, `reversible_manifest_only`,
  `compensating_only`, `irreversible`, `not_applicable`.

## Every claimed surface references one matrix

Six marketed M5 package surfaces bind to this packet's id and pin the write
authority they may carry, so product, CLI, and support/export paths express
identity, lockfile authority, and registry/auth posture mechanically rather than
by hand:

| Surface | Write authority |
| --- | --- |
| `desktop_package_workspace` | `mutates` |
| `cli_headless` | `mutates` |
| `ai_context` | `inspect_only` |
| `review_workspace` | `stages` |
| `support_export` | `redacted_export` |
| `release_public_truth` | `redacted_export` |

Each binding's `references_matrix_id` must equal the packet id and each write
authority must equal the surface's canonical authority, so a surface can never
quietly drift onto its own private vocabulary or claim write authority it does
not have.

## Privacy and retention

The packet binds a privacy/retention rule for each of three subjects, and **no
rule may ever record that the packet stores a credential body**:

- **`operation_history`** — `bounded_local_history`: retained locally for a
  bounded window.
- **`registry_credentials`** — `broker_resolved_never_persisted`: resolved by the
  secret broker on demand, never written into a packet, always redacted in
  projections.
- **`support_export_packet`** — `redaction_required_export`: carries only redacted
  source labels and state tokens, never raw provider payloads or secrets.

## Why this packet exists

Package-manager mutation can widen in M5 only when package-state vocabulary,
manifest scope, registry auth, and lockfile authority are **governed product
objects** instead of ecosystem-specific folklore. By freezing the labels, the
control objects, the surface bindings, and the retention rules in one validated
packet, the matrix lets docs/help, support export, and release/public-truth
surfaces prove the package lane shares one contract — and prove that
offline/mirror/cache-only and auth-required states always disclose their real
cause instead of a generic failure.
