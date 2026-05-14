# Package Mutation Alpha

This document freezes the bounded package-mutation runtime lane for the TS/JS
launch wedge. The lane reviews one `package.json` plus its coupled npm/pnpm
lockfile before any dependency mutation can write files, open registry network
routes, or execute lifecycle hooks.

## Artifacts

- [`/crates/aureline-runtime/src/packages`](../../crates/aureline-runtime/src/packages)
  owns the Rust records and the read-only `NodePackageMutationReviewer`.
- [`/schemas/runtime/manifest_scope_alpha.schema.json`](../../schemas/runtime/manifest_scope_alpha.schema.json)
  defines the manifest-scope descriptor.
- [`/schemas/runtime/registry_source_alpha.schema.json`](../../schemas/runtime/registry_source_alpha.schema.json)
  defines registry-source and auth disclosure.
- [`/schemas/runtime/lockfile_impact_alpha.schema.json`](../../schemas/runtime/lockfile_impact_alpha.schema.json)
  defines resolver, lockfile-impact, transitive-impact, and mutation-mode truth.
- [`/schemas/runtime/package_operation_alpha.schema.json`](../../schemas/runtime/package_operation_alpha.schema.json)
  defines review, audit, and support-export packets.
- [`/fixtures/runtime/packages/package_mutation_alpha`](../../fixtures/runtime/packages/package_mutation_alpha)
  protects the checked-in TS/JS package review packet and export packet.

## Scope

The claimed lane is intentionally narrow:

- package manager: npm or pnpm as detected by the runtime Node detector;
- manifest input: one active `package.json`, including workspace-member
  manifests such as `apps/web/package.json`;
- lockfile input: `pnpm-lock.yaml`, `package-lock.json`, or
  `npm-shrinkwrap.json`;
- operation: review-time install/update/remove/audit packets;
- apply behavior: review packet only. The runtime alpha does not execute a
  package manager or edit a lockfile directly.

Python and other package ecosystems can reuse the packet vocabulary later, but
they are not claimed by this runtime consumer.

## Required Truth

Every mutating packet carries:

- manifest scope: package-manager family, root/member/module refs, active
  manifest path, inherited registry source, and affected lockfiles;
- registry/auth banner: source class, auth mode, policy owner or credential
  refs when present, freshness, mirror/offline state, and revocation state;
- script/native-build risk: lifecycle script refs, postinstall refs, sandbox or
  native-toolchain refs, consent ticket refs, and a stable risk class;
- lockfile impact: resolver identity/version, affected lockfiles, impact
  class, transitive-impact class, count buckets, and mutation mode;
- rollback/checkpoint: lockfile or workspace checkpoint refs before apply; and
- audit lineage: actor, command, execution context, target, policy epoch,
  review result, and rollback refs.

When lockfile round-trip safety is not proven, the packet uses
`regenerate_and_review`. The lane must not silently inline-edit lockfiles or
relabel hook/native-build work as metadata-only.

## Export Posture

Support exports carry refs, enum tokens, count buckets, relative manifest and
lockfile paths, and short review-safe disclosure sentences. They do not carry
raw manifest bodies, raw lockfile bodies, registry URLs, package-manager logs,
script bodies, tokens, certificate material, or raw credential handles.

## Verification

```sh
cargo test -p aureline-runtime --test package_mutation_alpha
```
