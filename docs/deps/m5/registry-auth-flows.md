# Registry auth flows: sign-in continuity, handle-only secrets, and degradation truth

This document describes the **registry auth flows** object — the one that makes
registry authentication a first-class M5 package workflow rather than an
undocumented prerequisite. Each row answers, for one registry or mirror: **who is
signed in, how, and is the registry reachable right now?** It is the user-facing
companion to the governed artifact at `artifacts/deps/m5/registry-auth-flows.json`,
the schema at `schemas/deps/registry-auth-flows.schema.json`, and the typed model
in the `aureline-deps` crate (`registry_auth_flows`).

Where the
[package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
*freezes the registry-source and auth-mode vocabulary* and the
[manifest-scope review](./manifest-scope-review.md) *names the source behind one
mutation*, this object describes the **authentication flow itself**. One row is
reused by the desktop package workspace, CLI/headless, the review workspace, AI
context, and support/export packets, so registry auth stays product-visible
across search, review, and mutation rather than hidden inside a per-ecosystem
adapter.

## Current profile, provider, and credential source

Each row names:

- the **current profile** — a durable profile id, a redacted display label, and
  whether it is the active profile for its registry source. At most one profile
  is current per registry source identity;
- the frozen **source class** — `public_registry`, `private_registry`,
  `enterprise_mirror`, `local_cache`, or `offline_snapshot` — and, for a private
  registry or enterprise mirror, the redacted **mirror owner**;
- the **credential source** — how the user authenticates. This is finer-grained
  than the frozen auth mode it maps to, so a browser sign-in is never confused
  with a device-code continuity flow and a keychain handle is never confused with
  a vault handle:

  | Credential source        | Frozen auth mode             | Handle-backed |
  | ------------------------ | ---------------------------- | ------------- |
  | `browser_interactive`    | `browser_or_device_sign_in`  | yes           |
  | `device_code_continuity` | `browser_or_device_sign_in`  | yes           |
  | `os_keychain_handle`     | `os_store_credential`        | yes           |
  | `secret_vault_handle`    | `token_credential`           | yes           |
  | `policy_broker_handle`   | `policy_inherited_credential`| yes           |
  | `anonymous_access`       | `anonymous`                  | no            |
  | `auth_unsatisfied`       | `auth_required_unsatisfied`  | no            |

## Browser and device-code continuity

A browser or device-code sign-in carries a **continuity state** —
`awaiting_browser_return`, `awaiting_device_code`, `established`, `expired`, or
`failed` — so a half-finished sign-in is a visible, resumable state rather than an
opaque hang. A non-continuity credential source is `not_applicable`. While a flow
is awaiting a browser return or a device code, or has expired or failed, trust is
blocked and the flow offers a matched sign-in action.

## Secrets stay handle-only

A credential is persisted only as a **handle**: an opaque `handle_ref` the secret
broker resolves on demand, a redacted account label, and a lifecycle `state`
(`active`, `revoked`, `expired`). The handle never carries a token body, a private
registry URL, or a full auth payload, its retention is always
`broker_resolved_never_persisted`, and the `stores_secret_body` guard must stay
`false`. A revoked or expired handle blocks trust until it is rebound. Anonymous
access and an unsatisfied auth carry no handle at all.

## Degradation truth: never a generic failure

Each row carries a **reachability** state that keeps the degraded paths distinct
and renders each to a *specific* message class — never a generic "no results" or
"connection failed":

| Reachability               | Renders                        | Degraded | Blocks trust | Mutation |
| -------------------------- | ------------------------------ | -------- | ------------ | -------- |
| `reachable_fresh`          | `reachable_fresh_source`       | no       | no           | allowed  |
| `no_results_authoritative` | `no_results_authoritative`     | no       | no           | n/a      |
| `auth_required`            | `auth_required_disclosure`     | yes      | yes          | blocked  |
| `mirror_stale`             | `mirror_stale_disclosure`      | yes      | no           | blocked  |
| `offline_snapshot_only`    | `offline_snapshot_disclosure`  | yes      | no           | blocked  |
| `cache_only`               | `cache_only_disclosure`        | yes      | no           | blocked  |
| `policy_blocked`           | `policy_blocked_disclosure`    | yes      | yes          | blocked  |

An **authoritative no-results** — a fresh, authenticated index that genuinely
returned nothing — is a specific outcome kept distinct from any auth-required,
offline, or connection-failed state. A mutation needs a reachable, fresh,
authenticated registry; a stale, offline, cache-only, auth-required, or
policy-blocked path may still serve a disclosed read but never an install or
update.

## Keyboard-complete retry, revoke, and switch-account

Every row offers at least the actions its credential source, continuity state,
and reachability require, and every offered action carries a command id and a key
hint so the flow is reachable without a pointer:

- a **revocable handle** always offers `revoke` and `switch_account`, so a leaked
  or wrong account is recoverable;
- an **unsatisfied auth** offers a matched sign-in (`sign_in_browser`,
  `sign_in_device_code`) or a `rebind_handle`, plus `retry`;
- an **offline or cache-only** path offers `use_offline_snapshot`; a
  **policy-blocked** source offers `request_policy_exception`.

## The same object feeds every surface

Each row projects into:

- `view()` — the desktop, review, and AI inspect surfaces;
- `export_row()` — the redaction-safe row reused by support/export packets and the
  CLI inspect surface;
- `surface_projection(surface)` — the row rendered for a marketed surface with the
  write authority that surface may carry, pinned from the frozen matrix, so only a
  mutating surface can mutate an already-mutation-ready flow.

Every row binds to the frozen matrix through `references_matrix_id`: its source
class resolves to a frozen registry cell and every label it surfaces resolves to a
frozen state row, so product, CLI, and support/export paths express registry
identity, auth posture, and degradation truth mechanically.
