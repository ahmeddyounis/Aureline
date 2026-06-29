# M5 no-silent-omission guard parity

- Registry: `m5-omission-guard-registry:stable:0001`
- Label: `M5 no-silent-omission guard parity across public-truth consumers`
- Cases: 9
- Minted: `2026-07-06T00:00:00Z`
- One vocabulary across: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion

## Vocabulary

| State | Label | Weakening |
|-------|-------|-----------|
| `official` | Official | anchor |
| `vendor` | Vendor | yes |
| `community` | Community | yes |
| `mirrored` | Mirrored | yes |
| `offline` | Offline | yes |
| `side_loaded` | Side-loaded | yes |
| `unverified` | Unverified | yes |
| `partial` | Partial | yes |
| `retest_pending` | Retest pending | yes |
| `stale` | Stale | yes |
| `expired` | Expired | yes |
| `missing` | Missing | yes |
| `scoped_client` | Scoped client | yes |
| `handoff_required` | Handoff required | yes |
| `not_provided` | Not provided | yes |

## Cases

| Case | Descriptor | Present states | Claim state | Weakening |
|------|------------|----------------|-------------|-----------|
| `omission-guard:official` | `m5-descriptor-object:release-artifact-graph:stable:0001` | official | `fully_supported` | no |
| `omission-guard:mirrored` | `m5-descriptor-object:marketplace-mirror:mirrored:0001` | mirrored | `limited` | yes |
| `omission-guard:offline` | `m5-descriptor-object:docs-offline:offline:0001` | offline | `limited` | yes |
| `omission-guard:side-loaded` | `m5-descriptor-object:sideloaded-extension:sideloaded:0001` | side_loaded, unverified | `limited` | yes |
| `omission-guard:partial-evidence` | `m5-descriptor-object:evaluation-pack:partial:0001` | official, partial | `limited` | yes |
| `omission-guard:community-limited` | `m5-descriptor-object:marketplace-extension:limited:0001` | community, unverified, partial | `limited` | yes |
| `omission-guard:stale` | `m5-descriptor-object:evaluation-pack:stale:0001` | official, stale | `evidence_stale` | yes |
| `omission-guard:scoped-client` | `m5-descriptor-object:companion-action:scoped:0001` | official, scoped_client, handoff_required | `unsupported_client` | yes |
| `omission-guard:not-provided-blocked` | `m5-descriptor-object:sideloaded-doc:blocked:0001` | retest_pending, missing, scoped_client, handoff_required, not_provided | `unsupported` | yes |

## Consumer parity

### `omission-guard:official`

Present states (identical on every consumer):

- `official` (Official) — from source_class:first_party_signed

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 1 | yes |
| `help_about` | 1 | yes |
| `marketplace` | 1 | yes |
| `docs_help` | 1 | yes |
| `certification` | 1 | yes |
| `evaluation_packs` | 1 | yes |
| `support_export` | 1 | yes |
| `companion_handoff` | 1 | yes |

### `omission-guard:mirrored`

Present states (identical on every consumer):

- `mirrored` (Mirrored) — from source_class:mirror

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 1 | yes |
| `help_about` | 1 | yes |
| `marketplace` | 1 | yes |
| `docs_help` | 1 | yes |
| `certification` | 1 | yes |
| `evaluation_packs` | 1 | yes |
| `support_export` | 1 | yes |
| `companion_handoff` | 1 | yes |

### `omission-guard:offline`

Present states (identical on every consumer):

- `offline` (Offline) — from source_class:offline_bundle

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 1 | yes |
| `help_about` | 1 | yes |
| `marketplace` | 1 | yes |
| `docs_help` | 1 | yes |
| `certification` | 1 | yes |
| `evaluation_packs` | 1 | yes |
| `support_export` | 1 | yes |
| `companion_handoff` | 1 | yes |

### `omission-guard:side-loaded`

Present states (identical on every consumer):

- `side_loaded` (Side-loaded) — from source_class:side_loaded
- `unverified` (Unverified) — from signature_state:unsigned

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 2 | yes |
| `help_about` | 2 | yes |
| `marketplace` | 2 | yes |
| `docs_help` | 2 | yes |
| `certification` | 2 | yes |
| `evaluation_packs` | 2 | yes |
| `support_export` | 2 | yes |
| `companion_handoff` | 2 | yes |

### `omission-guard:partial-evidence`

Present states (identical on every consumer):

- `official` (Official) — from source_class:first_party_signed
- `partial` (Partial) — from qualification_evidence:partial

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 2 | yes |
| `help_about` | 2 | yes |
| `marketplace` | 2 | yes |
| `docs_help` | 2 | yes |
| `certification` | 2 | yes |
| `evaluation_packs` | 2 | yes |
| `support_export` | 2 | yes |
| `companion_handoff` | 2 | yes |

### `omission-guard:community-limited`

Present states (identical on every consumer):

- `community` (Community) — from source_class:community
- `unverified` (Unverified) — from signature_state:unsigned
- `partial` (Partial) — from qualification_evidence:limited

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 3 | yes |
| `help_about` | 3 | yes |
| `marketplace` | 3 | yes |
| `docs_help` | 3 | yes |
| `certification` | 3 | yes |
| `evaluation_packs` | 3 | yes |
| `support_export` | 3 | yes |
| `companion_handoff` | 3 | yes |

### `omission-guard:stale`

Present states (identical on every consumer):

- `official` (Official) — from source_class:first_party_signed
- `stale` (Stale) — from freshness_state:stale

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 2 | yes |
| `help_about` | 2 | yes |
| `marketplace` | 2 | yes |
| `docs_help` | 2 | yes |
| `certification` | 2 | yes |
| `evaluation_packs` | 2 | yes |
| `support_export` | 2 | yes |
| `companion_handoff` | 2 | yes |

### `omission-guard:scoped-client`

Present states (identical on every consumer):

- `official` (Official) — from source_class:first_party_signed
- `scoped_client` (Scoped client) — from client_kind:companion_scoped, authority_class:scoped_authority
- `handoff_required` (Handoff required) — from handoff_requirement:desktop_handoff_required

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 3 | yes |
| `help_about` | 3 | yes |
| `marketplace` | 3 | yes |
| `docs_help` | 3 | yes |
| `certification` | 3 | yes |
| `evaluation_packs` | 3 | yes |
| `support_export` | 3 | yes |
| `companion_handoff` | 3 | yes |

### `omission-guard:not-provided-blocked`

Present states (identical on every consumer):

- `retest_pending` (Retest pending) — from qualification_evidence:retest_pending
- `missing` (Missing) — from freshness_state:missing
- `scoped_client` (Scoped client) — from client_kind:browser_reference, authority_class:reference_only
- `handoff_required` (Handoff required) — from handoff_requirement:console_handoff_required
- `not_provided` (Not provided) — from source_class:not_provided, signature_state:not_provided, freshness_evidence:not_provided

| Consumer | States rendered | Omits none |
|----------|-----------------|------------|
| `release_center` | 5 | yes |
| `help_about` | 5 | yes |
| `marketplace` | 5 | yes |
| `docs_help` | 5 | yes |
| `certification` | 5 | yes |
| `evaluation_packs` | 5 | yes |
| `support_export` | 5 | yes |
| `companion_handoff` | 5 | yes |

