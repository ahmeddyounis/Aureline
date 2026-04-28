# Extension lockfile and recommendation contract

This document defines the continuity contract shared by workspace
recommendations, profile recommendations, imported extension mappings,
admin suggestions, registry mirrors, offline bundles, install review, and
offboarding exports.

The machine-readable schemas are:

- [`/schemas/ecosystem/extension_lockfile.schema.json`](../../schemas/ecosystem/extension_lockfile.schema.json)
- [`/schemas/ecosystem/extension_recommendation_set.schema.json`](../../schemas/ecosystem/extension_recommendation_set.schema.json)

The extension manifest, effective permission, publisher-continuity, and
registry / mirror rows remain owned by the existing extension contracts:

- [`/schemas/extensions/effective_permission.schema.json`](../../schemas/extensions/effective_permission.schema.json)
- [`/schemas/extensions/registry_manifest.schema.json`](../../schemas/extensions/registry_manifest.schema.json)
- [`/docs/extensions/registry_and_offline_bundle_seed.md`](../extensions/registry_and_offline_bundle_seed.md)
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)

This contract does not implement a resolver or registry service. It
freezes the artifact shapes, lane vocabulary, and review rules that later
resolver, importer, policy, mirror, and workspace-review code must share.

## Artifact roles

| Artifact | Typical path | Authority | Purpose |
|---|---|---|---|
| Extension recommendation set | `.aureline/extensions.recommend.jsonc`, profile export, admin policy projection, import report | Human, workspace, profile, importer, or admin intent | Declares desired, blocked, imported, or suggested extension choices before resolution. |
| Extension lockfile | `.aureline/extensions.lock.json` | Generated resolution output, reviewable by humans | Pins selected versions, artifact identity, publisher continuity, signature / attestation refs, permission-manifest refs, compatibility path, and export posture. |
| Registry / mirror row | registry manifest, mirror manifest, offline-bundle manifest | Registry or mirror metadata | Proves artifact content address, source class, trust inheritance, revocation snapshot, and mirror continuity. |
| Permission row | extension manifest and effective-permission summary | Extension package and policy engine | Proves declared and effective permissions after policy, host context, and dependency closure. |
| Import outcome / scorecard | migration report and compatibility scorecard | Importer and compatibility program | Explains whether an imported extension is native, bridge-backed, blocked, or waiting for manual review. |

Recommendation sets carry authoritative choice. Lockfiles carry generated
resolution. Consumers must not treat lockfile rows as the place where a
workspace owner expresses intent, except for an explicit emergency pin
recorded with a reason and then folded back into a recommendation set.

## Stable paths and scopes

`.aureline/extensions.lock.json` is the canonical workspace lockfile path.
A profile export may carry the same schema under its profile envelope. An
admin or importer may emit a lockfile-shaped projection for review, but
the projection must keep `lockfile_scope` distinct from the workspace or
profile lock.

The lockfile is JSON, generated, and deterministic. The recommendation set
is JSONC where it is human-owned and JSON where emitted inside a machine
packet. Unknown additive fields in human-authored recommendation sets
should be preserved by editors that can do so, but generated lockfiles use
strict schemas and stable ordering to keep diffs small.

## Recommendation sets

A recommendation set is composed of ordered groups. Each group has exactly
one owner lane:

| Lane | Meaning | Authority |
|---|---|---|
| `workspace_owned` | The repository or workspace asks for an extension for this project. | Workspace owner and code review. |
| `profile_owned` | A user profile asks for an extension across workspaces. | Profile owner. |
| `imported` | Migration tooling detected a source-tool extension, mapping, or gap. | Import report plus user review before apply. |
| `admin_suggested` | Admin policy suggests, requires review for, or blocks an extension. | Admin policy. |

Each recommendation item carries:

- extension identity or an imported source identity;
- intent (`required`, `recommended`, `optional`, `discouraged`, or
  `blocked`);
- desired continuity state (`native`, `compatibility_bridge`, `blocked`,
  or `manual_review_required`);
- version requirement and source provenance;
- permission-review expectation;
- registry or mirror preferences;
- refs to importer outcomes, equivalence maps, workflow scorecards, or
  admin policy rows where applicable.

The recommendation state is not a promise that resolution will succeed. It
is the requested continuity path. The lockfile row records the actual
resolved path.

## Lockfile rows

Every lockfile row represents one extension continuity decision. Rows may
pin a selected artifact, block a requested extension, or preserve a
manual-review placeholder so migration and policy surfaces can discuss the
same object without inventing temporary metadata.

Required row concepts:

| Concept | Required facts |
|---|---|
| Identity | Canonical extension identity, row id, and source recommendation refs. |
| Version selection | Requested requirement, selected version when present, resolution state, reason class, role in dependency closure, and deterministic order key. |
| Artifact trust | Content address, signature class, signature or attestation refs, registry source class, registry manifest ref, mirror continuity ref, revocation snapshot ref, and publisher-continuity ref. |
| Permission linkage | Permission manifest ref, declared-permissions digest, and effective-permission summary ref. |
| Compatibility path | `native`, `compatibility_bridge`, `blocked`, or `manual_review_required`, with bridge profile, denial reason, limitations, and scorecard refs as needed. |
| Policy linkage | Admin policy refs, registry policy refs, mirror policy refs, and trust inheritance rows that narrowed or admitted the install. |
| Reviewability | Review state, reason refs, stable diff key, generated input digest, generated row digest, and human summary text safe for support/export. |
| Export posture | Workspace, profile, support-bundle, and offboarding package handling. |

Rows sort by deterministic order key, then extension identity, then row id.
Generated output must use two-space indentation, LF line endings, and a
stable key order chosen by the eventual writer. Re-resolving unchanged
inputs must produce byte-stable rows except for fields whose source data
actually changed.

## Compatibility states

The closed state set is intentionally small:

| State | Meaning | Required refs |
|---|---|---|
| `native` | Aureline has a native package or mapping for the requested extension. | Registry row, permission row, publisher-continuity row, and compatibility scorecard when a public claim is made. |
| `compatibility_bridge` | The selected extension runs through an explicit compatibility bridge. Native parity is not claimed. | Bridge profile ref, source extension identity when imported, compatibility scorecard or importer outcome ref. |
| `blocked` | The requested extension is represented but not installed or activated. | Denial reason and policy, revocation, permission, missing-bridge, or missing-native refs. |
| `manual_review_required` | A human must choose a mapping, accept a bridge, substitute an extension, or resolve publisher / permission uncertainty. | Review reason refs and importer outcome, policy, scorecard, or permission refs that explain the block. |

A surface that renders `partial`, `best effort`, or `probably works`
instead of one of these states is non-conforming at this layer.

## Manual edit and regeneration model

Recommendation sets are the authoritative edit surface. Lockfiles are the
generated result.

Manual edits follow these rules:

1. Add, remove, block, or prefer extensions in the recommendation set.
2. Regenerate the lockfile from recommendation sets, policy, registry /
   mirror rows, imported mappings, and platform context.
3. Review the lockfile diff before applying installs, updates, removals,
   or bridge activation.
4. Direct lockfile edits are rejected by default. An emergency pin is
   allowed only when the writer records an emergency override reason and
   the next regeneration projects that reason back into the authoritative
   recommendation set.

The lockfile carries both `resolution_inputs_digest` at file level and
per-row input digests. A resolver that changes ordering, whitespace, row
ids, or derived refs without an input change creates review noise and is
non-conforming.

## Import behavior

Importers may propose recommendation groups and lockfile preview rows, but
they do not silently install imported extensions. Imported rows must link
to:

- source tool family and source extension identity;
- importer outcome row;
- equivalence map row when mapping to a native package;
- bridge profile when compatibility is bridge-backed;
- workflow or extension compatibility scorecard;
- rollback or compare/export packet when manual review is needed.

An imported extension can end in `native`, `compatibility_bridge`,
`blocked`, or `manual_review_required`. `native` is allowed only when the
importer has a declared native mapping and the selected artifact satisfies
the permission, publisher, compatibility, and policy checks. A bridge row
must never display as native merely because the user can continue a
workflow.

## Mirror and offline behavior

Lockfile rows reference registry and mirror rows rather than embedding
registry metadata. The selected artifact identity is the content-address
pair. Mirrors and offline bundles must preserve that pair, the signature
or attestation refs, permission-manifest refs, compatibility notes, and
publisher-continuity refs.

Mirror promotion may narrow trust or policy. It may not widen trust,
rewrite signatures, mutate content address, or hide revocation state. A
row restored from an offline bundle must still report whether the source
was public registry, approved mirror, private registry, offline bundle,
local archive, or quarantined local copy.

## Export and offboarding behavior

Extension lockfiles and recommendation sets are included by default in
workspace review, profile export when profile-owned, support bundles, and
managed-customer offboarding packets unless policy says otherwise.

Exports carry refs and metadata only. Raw artifact bytes, raw signing-key
material, raw attestation bundles, raw policy bundles, raw mirror-cache
bodies, raw local archive paths, and raw secrets do not appear in these
records.

Offboarding packages should include the latest extension lockfile or an
extension inventory derived from it, plus policy and registry refs needed
to reconstruct whether each row was native, bridge-backed, blocked, or
manual-review-only at export time.

## Consumer expectations

- Workspace review reads recommendation sets for intent and lockfiles for
  generated resolution.
- Install review reads lockfile rows, permission summaries, registry
  rows, and policy refs together before offering a mutating install.
- Registry mirrors read content address and continuity refs from the
  lockfile and must not recompute a parallel identity.
- Migration review reads imported recommendation groups and lockfile
  preview rows to display native, bridge, blocked, and manual-review
  outcomes.
- Support bundles and offboarding exports preserve row ids, source refs,
  continuity refs, and export posture so a later reviewer can reconstruct
  what was recommended, what resolved, and why.

## Fixtures

Example artifacts live under
[`/fixtures/ecosystem/extension_lock_cases`](../../fixtures/ecosystem/extension_lock_cases):

- `native_workspace_lock.json`
- `imported_bridge_and_blocked_lock.json`
- `recommendation_set_mixed_lanes.json`

