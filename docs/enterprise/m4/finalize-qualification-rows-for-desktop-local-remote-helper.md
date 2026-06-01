# Qualification Matrix — Desktop Local, Remote/Helper, Provider-Linked, State/Schema, and Accessibility

This document is the stable canonical reference for the qualification-matrix proof packet that covers five IDE surfaces — desktop local, remote/helper, provider-linked, state/schema, and accessibility — across four deployment profiles: local OSS, self-hosted, managed, and air-gapped.

The runtime source of truth is `aureline_remote::finalize_qualification_rows_for_desktop_local_remote_helper::seeded_qualification_matrix_page()`. Dashboards, Help/About surfaces, support exports, and release evidence should ingest the packet produced by that function rather than cloning the text below.

## Overview

The proof packet answers the question "what is claimed as stable, and is that claim backed by explicit evidence?" for each combination of surface and deployment profile. Every row must carry:

- an explicit **dependency class** (`local_only`, `network`, `managed`, or `air_gapped`),
- an explicit **local-core continuity declaration** (local editing must never be blocked by a managed or network-dependent feature failure),
- an explicit **no-account compatibility flag** for `local_oss`, `air_gapped`, and accessibility rows,
- a typed **failure-mode downgrade class** so release docs and support can describe what degrades when the dependency is unavailable, and
- `raw_private_material_excluded: true` (hard guardrail; absence withdraws the packet).

## Surfaces

### Desktop local (`desktop_local`)

All buffer, LSP, tree-sitter, keybinding, and rendering features that work with no account and no outbound connection. The local editing floor is the unconditional baseline; managed and network-dependent features are additive.

| Profile | Dependency | Failure downgrade |
|---|---|---|
| `local_oss` | `local_only` | `not_applicable` |
| `self_hosted` | `managed` | `local_core_unaffected` |
| `managed` | `managed` | `local_core_unaffected` |
| `air_gapped` | `local_only` | `not_applicable` |

### Remote / helper (`remote_helper`)

SSH target, remote agent, managed workspace tunnel, and helper services that mediate access to a remote host. Remote features degrade when the dependency is unavailable; the local editing floor is preserved.

| Profile | Dependency | Failure downgrade |
|---|---|---|
| `local_oss` | `network` | `degraded_features` |
| `self_hosted` | `managed` | `degraded_features` |
| `managed` | `managed` | `degraded_features` |
| `air_gapped` | `air_gapped` | `mirror_fallback` |

### Provider-linked (`provider_linked`)

VCS hosts, CI, issue trackers, identity providers, and partner APIs connected via OAuth or enterprise SSO. Provider features degrade when the dependency is unavailable; the local repo and workspace continue.

| Profile | Dependency | Failure downgrade |
|---|---|---|
| `local_oss` | `network` | `degraded_features` |
| `self_hosted` | `managed` | `degraded_features` |
| `managed` | `managed` | `degraded_features` |
| `air_gapped` | `air_gapped` | `mirror_fallback` |

### State / schema (`state_schema`)

Workspace config, document state, settings sync, and schema-versioned records. Local state must survive upgrade, rollback, and offline transitions without data loss.

| Profile | Dependency | Failure downgrade |
|---|---|---|
| `local_oss` | `local_only` | `not_applicable` |
| `self_hosted` | `managed` | `offline_grace` |
| `managed` | `managed` | `offline_grace` |
| `air_gapped` | `local_only` | `not_applicable` |

### Accessibility (`accessibility`)

Six accessibility features that must be validated on every touched surface. Each feature is a local-only capability; none have a network or managed dependency.

| Feature | Scope | Dependency |
|---|---|---|
| `keyboard` | Full keyboard access, focus management, shortcut discoverability, no keyboard traps | `local_only` |
| `screen_reader` | Accessible name/role/state, live-region announcements (NVDA, JAWS, VoiceOver, Orca) | `local_only` |
| `ime_grapheme_bidi` | IME composition, grapheme-cluster cursor navigation, bidirectional text layout | `local_only` |
| `zoom` | No horizontal overflow, no clipped targets at 200 % zoom; OS text-scaling support | `local_only` |
| `high_contrast` | Forced-color and high-contrast mode; no information conveyed by color alone | `local_only` |
| `reduced_motion` | No persistent/looping animations when `prefers-reduced-motion: reduce` is active | `local_only` |

## Stability conditions

The stable claim holds when **all six** of the following conditions are verified simultaneously:

1. All 22 required rows are present.
2. No raw private material is exposed on any row record.
3. Every row explicitly declares its local-core continuity posture.
4. Every row carries an explicit dependency class.
5. Every `local_oss`, `air_gapped`, and accessibility row declares no-account local-use compatibility.
6. Every row names a typed failure-mode downgrade class.

If any surface still lacks stable qualification, it is automatically narrowed below `Stable` in product copy, docs/help, and release packets instead of inheriting adjacent green rows.

## Narrowing rules

| Condition violated | Narrow result | Narrow reason token |
|---|---|---|
| `raw_private_material_excluded: false` on any row | `Withdrawn` (immediate, non-overridable) | `raw_private_material_exposed` |
| Required row absent | `Preview` | `required_row_missing` |
| `local_core_continuity_allowed: false` | `Beta` | `local_core_continuity_undeclared` |
| `dependency_class_token` empty | `Beta` | `dependency_class_undeclared` |
| `no_account_local_compatible: false` on local_oss / air_gapped / accessibility rows | `Beta` | `no_account_compatibility_undeclared` |
| `failure_downgrade_token` empty | `Beta` | `failure_downgrade_undeclared` |

## Enterprise and managed claims

Enterprise and managed features must:
- never block local-core work by default,
- declare tenant, region, and policy source refs as opaque identifiers,
- document the failure-mode downgrade class so support and diagnostics can describe impact without reading raw configuration, and
- keep local, self-hosted, managed, and air-gapped truth separate and explicit in UI, docs, and release packets.

## Dependency truth

No deployment profile may blur dependency ownership. The dependency class on each row must accurately reflect whether the feature requires a live internet connection (`network`), a managed enterprise endpoint (`managed`), a declared signed mirror (`air_gapped`), or no external dependency at all (`local_only`).

## Canonical paths

- Runtime owner: `aureline_remote::finalize_qualification_rows_for_desktop_local_remote_helper`
- Artifact: `artifacts/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper.md`
- Fixtures: `fixtures/enterprise/m4/finalize-qualification-rows-for-desktop-local-remote-helper/`
- Schema: `schemas/enterprise/finalize-qualification-rows-for-desktop-local-remote-helper.schema.json`
- Contract ref: `remote:qualification_matrix:desktop:v1`
