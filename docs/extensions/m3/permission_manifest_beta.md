# Permission manifest, capability classes, and re-consent deltas (beta)

This page is the reviewer-facing entrypoint for the typed permission-
manifest beta lane. It is the contract by which an extension declares
its network, filesystem, process, data, UI, and credential-related
capabilities through one machine-readable schema; by which permission
changes across versions produce a typed diff that the install / update
review surface and mirror workflows read verbatim; and by which mirror
and manual-install flows preserve the same closed permission vocabulary
as the primary registry path.

The contract is governed. The canonical Rust source of truth lives in
[`crates/aureline-extensions/src/permission_manifest/`](../../../crates/aureline-extensions/src/permission_manifest/);
the cross-tool boundary schema is
[`schemas/extensions/permission_manifest.schema.json`](../../../schemas/extensions/permission_manifest.schema.json);
the checked-in fixtures live under
[`fixtures/extensions/m3/permission_deltas/`](../../../fixtures/extensions/m3/permission_deltas/).

## Why a permission-manifest record on top of the manifest baseline

The manifest baseline
([`docs/extensions/m1_permission_and_publisher_baseline.md`](../m1_permission_and_publisher_baseline.md))
already pins publisher identity, lifecycle, declared permission scopes,
and the effective-permission diff. It does not by itself answer:

- which of the six capability classes (network, filesystem, process,
  data, UI, credential) the extension actually exercises;
- what changed between two versions of the same extension, in a form
  the install / update review surface and a mirror review can read
  verbatim;
- whether the change requires re-consent, a soft user inform, or no
  action.

Without one typed record the install / update review surface, the
permission inspector, the support export, the partner packet template,
and the CLI / headless lanes are each free to invent a local
"permissions changed" copy. The permission-manifest beta closes that
gap.

## Record shape

The lane emits three records that share the
`permission_manifest_schema_version` integer:

1. `PermissionManifestRecord` — one per `(extension_identity,
   extension_version)`. Pairs the manifest baseline's declared
   permissions with a closed `CapabilityClassClass` per entry
   (`network`, `filesystem`, `process`, `data`, `ui`, `credential`) and
   a deterministic per-class summary. The record always cites a
   `manifest_baseline_ref` so consumers can join back to the baseline
   for publisher / origin / signing-key truth.
2. `PermissionManifestDeltaRecord` — one per `(prior, next)` version
   pair. Carries one typed `PermissionDeltaEntry` per
   `(scope_class, scope_target)` pair touched, with `delta_class` from
   the closed `unchanged / scope_added / scope_removed /
   scope_constraint_widened / scope_constraint_narrowed /
   rationale_only_changed` vocabulary, plus a typed
   `CapabilityClassDeltaEntry` rollup per capability class. The
   evaluator pairs the diff with a closed
   `ReConsentDecisionClass` and `ReConsentReasonClass` so the install /
   update review surface never invents its own "permissions changed"
   string.
3. `PermissionManifestSupportExportRecord` — the first consumer
   projection. Pins `RedactionClass::MetadataSafeDefault`, exposes the
   declared capability classes, the widening / narrowing counts, the
   re-consent decision and reason, a `requires_re_consent` flag, and a
   `blocks_activation` flag the support export, partner packet
   template, and CLI / headless review surfaces read verbatim.

Every record pins `RedactionClass::MetadataSafeDefault`. Raw manifest
bytes, raw signing-key material, raw policy bodies, raw paths, raw
tokens, and raw publisher-private data MUST NOT appear anywhere; every
field is an opaque ref or a closed vocabulary value.

## Capability classes

Every `PermissionScopeClass` resolves to exactly one
`CapabilityClassClass` through `capability_class_for_scope`:

| Permission scope                | Capability class |
| ------------------------------- | ---------------- |
| `network_egress`                | `network`        |
| `filesystem_read`               | `filesystem`     |
| `filesystem_write`              | `filesystem`     |
| `shell_execute`                 | `process`        |
| `execution_context_bind`        | `process`        |
| `ai_provider_access`            | `data`           |
| `connected_provider_access`     | `data`           |
| `workspace_settings_read`       | `data`           |
| `workspace_settings_write`      | `data`           |
| `subscription_subscribe`        | `data`           |
| `capability_inherit`            | `data`           |
| `ui_command_contribute`         | `ui`             |
| `secret_handle_use`             | `credential`     |

The mapping is total and stable; adding a new permission scope is
additive-minor with a `permission_manifest_schema_version` bump.

## Re-consent decision precedence

[`evaluate_permission_manifest_delta`](../../../crates/aureline-extensions/src/permission_manifest/mod.rs)
is deterministic and fails closed. In strict precedence order:

1. **Identity guardrails.** If the prior and next records disagree on
   `extension_identity`, if either `permission_manifest_id` is missing
   the `permission_manifest:` prefix, or if both records carry the same
   `extension_version`, refuse with `refused_inconsistent_input` and
   the matching reason class.
2. **Publisher / lifecycle / origin guardrails.** If the next
   manifest's publisher is quarantined, retired, or the extension
   lifecycle is retired / quarantined, or the origin source is
   `unknown_source_class`, refuse with the matching reason class.
3. **Widening rationale required.** Any widening delta
   (`scope_added` or `scope_constraint_widened`) MUST cite a non-empty
   `next_rationale_label`. A widening row that omits the rationale
   refuses with `refused_rationale_missing_on_widening_entry`.
4. **New capability class.** If at least one capability class appears
   in `next` that was empty in `prior`, return
   `re_consent_required_new_capability_class` with reason
   `widening_added_new_capability_class`. The new class itself
   requires re-consent even if every entry change is otherwise
   narrowing.
5. **Scope added.** If any `(scope_class, scope_target)` pair appears
   in `next` that was not in `prior`, return
   `re_consent_required_widening` with reason
   `widening_added_new_scope`.
6. **Constraint loosened.** If any same-pair entry's constraint was
   relaxed or dropped, return `re_consent_required_widening` with
   reason `widening_constraint_loosened`.
7. **Narrowing-only.** If no widening was seen but at least one
   `scope_removed` or `scope_constraint_narrowed` entry exists, return
   `not_required_narrowing_only` with reason
   `narrowing_does_not_require_re_consent`.
8. **Rationale-only.** If no permission scope changed but at least one
   `rationale_only_changed` entry exists, return
   `inform_only_rationale_changed` with reason
   `rationale_changed_inform_only`.
9. **Otherwise.** Return `not_required_no_change` with reason
   `no_change_between_versions`.

## Mirror and manual-install vocabulary

The same record set is emitted for primary-registry,
private-registry, mirror, offline-bundle, and vendored-local manifests.
`ManifestOriginSourceClass` is the only knob; the permission vocabulary
itself never forks on origin. The
[`mirror_origin_preserved.json`](../../../fixtures/extensions/m3/permission_deltas/mirror_origin_preserved.json)
fixture demonstrates that a mirror-sourced prior and an
offline-bundle-sourced next manifest produce the same widening
decision the primary-registry lane would.

## First consumer: support / partner export

[`project_permission_manifest_support_export`](../../../crates/aureline-extensions/src/permission_manifest/mod.rs)
is the first reading consumer. It emits a
`PermissionManifestSupportExportRecord` that quotes:

- `manifest_ref`, optional `delta_ref`, and the version pair;
- the declared `CapabilityClassClass` list and entry count, so support
  reviewers see the capability footprint without re-deriving it from
  the entries;
- the widening, narrowing, and rationale-only counts;
- the `re_consent_decision_class` and `re_consent_reason_class`;
- a `requires_re_consent` flag (set on
  `re_consent_required_widening` and
  `re_consent_required_new_capability_class`);
- a `blocks_activation` flag (set on those two plus
  `refused_inconsistent_input`);
- an `export_safe_summary` string the partner packet template and
  CLI / headless review surfaces can render verbatim.

The support export is what the partner packet template, the install
review chrome's permissions sub-panel, and any later CLI / headless
review consumer read. They join through `manifest_ref` and quote the
re-consent decision and reason verbatim instead of inventing a local
"permissions changed" string.

## Fixtures

The checked-in fixtures replay every reserved decision class through
the support export bundle:

- [`unchanged_no_reconsent.json`](../../../fixtures/extensions/m3/permission_deltas/unchanged_no_reconsent.json)
  — identical prior and next; `not_required_no_change` /
  `no_change_between_versions`.
- [`narrowing_only.json`](../../../fixtures/extensions/m3/permission_deltas/narrowing_only.json)
  — `ai_provider_access` dropped between versions;
  `not_required_narrowing_only` /
  `narrowing_does_not_require_re_consent`.
- [`widening_added_scope.json`](../../../fixtures/extensions/m3/permission_deltas/widening_added_scope.json)
  — `filesystem_write` added under an already-declared capability
  class; `re_consent_required_widening` / `widening_added_new_scope`.
- [`widening_added_capability_class.json`](../../../fixtures/extensions/m3/permission_deltas/widening_added_capability_class.json)
  — `network_egress` introduces the `network` capability class;
  `re_consent_required_new_capability_class` /
  `widening_added_new_capability_class`.
- [`rationale_only_change.json`](../../../fixtures/extensions/m3/permission_deltas/rationale_only_change.json)
  — same scope and constraint, rewritten rationale;
  `inform_only_rationale_changed` /
  `rationale_changed_inform_only`.
- [`mirror_origin_preserved.json`](../../../fixtures/extensions/m3/permission_deltas/mirror_origin_preserved.json)
  — mirror-sourced prior, offline-bundle-sourced next, same vocabulary
  applies; `re_consent_required_widening` /
  `widening_added_new_scope`.
- [`quarantined_publisher_refused.json`](../../../fixtures/extensions/m3/permission_deltas/quarantined_publisher_refused.json)
  — the next manifest's publisher is quarantined and tries to widen
  into the credential capability class; the delta refuses with
  `refused_inconsistent_input` / `refused_publisher_quarantined`.

## Guardrails

The permission-manifest contract refuses to widen on:

- a prior and next manifest whose extension_identity disagree, or whose
  version pair is not strictly monotonic;
- a next manifest whose publisher is quarantined, retired, or whose
  extension lifecycle is retired / quarantined;
- a next manifest whose origin source is `unknown_source_class`;
- a widening delta whose `next_rationale_label` is empty.

These map directly to the spec's "no beta widening on opaque publisher
identity, missing diff reports, or unbounded host authority" rule.

## How to verify

```
cargo test -p aureline-extensions permission_manifest::
```

The permission-manifest tests replay every fixture above end-to-end
through `evaluate_permission_manifest_delta`,
`validate_permission_manifest_record`,
`validate_permission_manifest_delta_record`, and
`project_permission_manifest_support_export`, and exercise every
refuse / re-consent / inform-only / not-required path.
