# M5 token overlays and scope (companion doc)

This page is the companion to the M5 token-overlay round-trip audit. It freezes
how appearance **overrides** become portable, inspectable, downgrade-safe
objects instead of a flat settings fragment. Appearance customization is
support-hostile and migration-hostile the moment an override loses its scope, an
unsupported token quietly disappears on export or sync, or nobody can say which
value actually won. This lane closes that gap: every override is a scope-explicit
object with provenance and portability flags, every token's winning-versus-
shadowed resolution is inspectable, and the export / import / sync round trip
keeps unsupported tokens alive as disclosed downgrades rather than dropping
them.

The lane mints no parallel token vocabulary. It re-exports the per-token overlay
state frozen in the canonical
[`schemas/design/token_overlay.schema.json`](../../schemas/design/token_overlay.schema.json)
(`token_overlay_record`) — the same object the appearance object-model index in
[theme-package-and-appearance-objects.md](theme-package-and-appearance-objects.md)
already designates — and promotes it into a portability/round-trip truth object
that the live shell appearance inspector, the docs/help and support-export
surfaces, the sync/import flows, the extension appearance inspectors, and the CI
gate all consume.

Authoritative artifacts:

- [`/artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md`](../../artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md)
  — the rendered audit (`artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md`).
- [`/fixtures/ux/m5/token-overlay-sync-import/report.json`](../../fixtures/ux/m5/token-overlay-sync-import/report.json)
  — the JSON snapshot (`fixtures/ux/m5/token-overlay-sync-import/report.json`) every surface consumes.
- [`/fixtures/ux/m5/token-overlay-sync-import/support_export.json`](../../fixtures/ux/m5/token-overlay-sync-import/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/token-overlay.schema.json`](../../schemas/ux/token-overlay.schema.json)
  — the boundary schema (`schemas/ux/token-overlay.schema.json`) the fixtures conform to.
- [`/schemas/design/token_overlay.schema.json`](../../schemas/design/token_overlay.schema.json)
  — the canonical per-token overlay-state record this lane re-exports.
- [`/tools/ci/m5/token_overlay_check.py`](../../tools/ci/m5/token_overlay_check.py)
  — the CI gate (`tools/ci/m5/token_overlay_check.py`) that keeps the audit fresh and honest.

## The override scopes and their precedence

An effective appearance value is resolved from a precedence stack. A
higher-precedence scope wins when more than one scope contributes an entry for
the same token; a managed policy is the hard cap, and the theme-package default
is the inherited base.

| Scope | Precedence | Meaning |
| ----- | ---------: | ------- |
| `policy_managed` | 100 | A managed-policy cap. Wins over every personal scope and stays local by design. |
| `workspace` | 50 | A workspace-scoped override. |
| `profile` | 40 | A profile-scoped override. |
| `user_global` | 30 | A user-global override. |
| `extension_contributed` | 20 | An extension-contributed appearance override. |
| `imported_theme` | 10 | An imported third-party theme. |
| `theme_package_default` | 0 | The active theme package's value — the inherited base. |

Resolution is explicit, not pixel-inferred: each `resolved_token` names exactly
one `winning_scope` (the highest-precedence contributing scope), lists every
`shadowed` entry with the reason it lost, and carries a one-line
`precedence_explained` sentence. That is what lets a user — or a support
reviewer — answer "why is my accent this colour?" without reverse-engineering
the rendered pixels.

## The override entry

Each `token_override_entry_record` carries:

- the **token** and its **family** (re-exported from the canonical token-family
  vocabulary);
- the **declared scope** it was authored in;
- the **value state** — `inherited`, `overridden`, `deprecated`, or `unmapped`;
- the **validation result** — `valid`, `valid_with_warnings`, `inert_unresolved`,
  `blocked_policy`, or `rolled_back`;
- the **provenance** — how the value got here (`authored_in_product`,
  `imported_from_theme_package`, `contributed_by_extension`, `applied_by_policy`,
  `migrated_from_legacy_settings`, or `synced_from_device`);
- the **portability flags** — its `portability_class`, plus whether it is
  `exportable`, `syncable`, and `survives_unsupported_target`;
- the **disclosed-downgrade class** (`none` when fully supported); and
- an explicit **fallback chain** that names how the effective value was resolved.

Scope is always explicit: an `inherited` entry resolves to the
`theme_package_default`; an `overridden`, `deprecated`, or `unmapped` entry
declares a real override scope.

## The single disclosed-downgrade vocabulary

Unsupported tokens never disappear. They survive as an inert or downgraded entry
with one of the closed `downgrade_class` values:

| Downgrade | Meaning |
| --------- | ------- |
| `none` | Fully supported and preserved. |
| `inert_unsupported_token` | The target does not support the token; kept as an inert placeholder. |
| `deprecated_alias_pending_replacement` | Points at a deprecated token awaiting replacement. |
| `policy_capped` | A managed policy capped the value on import. |
| `scope_demoted` | The scope could not be preserved on the target; demoted with disclosure. |
| `value_unsupported_kept_placeholder` | The value form is unsupported on the target; kept as a placeholder. |

An `unmapped` entry must cite its source slot, resolve to `inert_unresolved`,
and carry a disclosed downgrade — it is never treated as fully supported.

## The round-trip proof

The `round_trip_proof_record` traces the **portable** override set (entries that
are both exportable and syncable) across four channels: `export_bundle` →
`import_bundle` → `sync_push` → `sync_pull`. Each `round_trip_stage_record`
records how many entries were preserved versus downgraded; the invariant is that
no stage drops or rewrites an entry. Each `round_trip_entry_trace_record` records
one entry's disposition (`preserved`, `downgraded`, `dropped`, or `rewritten`),
its disclosed downgrade, and that its scope is preserved (`origin_scope` equals
`final_scope`).

Scope-local entries are not silently dropped — they are disclosed as
non-portable at the entry level. A `policy_managed` cap stays local
(`scope_local_non_portable`); a `theme_package_default` value rides the package
(`rides_theme_package`) and is re-resolved on import rather than serialized as an
override.

## What stays honest

The audit is the single source of truth for appearance overrides across the M5
surfaces. The same checked-in report drives the live shell appearance inspector,
the docs/help and support-export surfaces, the sync/import flows, and the CI
gate, so they never disagree on which value won, which were downgraded, and why.
The records become blockers the moment an override loses its scope, an
unsupported token is dropped or treated as fully supported, a resolution names
the wrong winner or hides a shadowed entry, or an overlay is flattened into an
opaque profile blob.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- validate
cargo test -p aureline-shell --test m5_token_overlays_fixtures
python3 tools/ci/m5/token_overlay_check.py
```
