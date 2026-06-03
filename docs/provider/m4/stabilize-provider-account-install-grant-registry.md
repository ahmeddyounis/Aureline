# Stabilize provider account/install-grant registry

## Overview

This document describes the stable provider account/install-grant registry that
turns beta account-scope and target-mapping contracts into one inspectable packet.

## Who is Aureline acting as?

Every claimed provider lane MUST surface:

- **Connected-account identity** — the human user account signed in through the
  system browser.
- **Installation-grant identity** — the app, bot, or project-scoped grant issued
  by the provider.
- **Delegated-credential identity** — the on-behalf-of credential chained from
  another actor.

The stable registry entry names the `acting_identity_class`, the
`canonical_host_ref`, the `org_tenant_scope_ref`, and the `connected_account_row_ref`
or `installation_grant_row_ref` that links back to the beta account-scope page.

## Which project/board/space is targeted?

Every mapping-review row names:

- `lane` — issue/work-item, review-decision, incident-handoff, or publish-later.
- `target_kind` — board, project, space, repository, or incident-queue.
- `target_ref` — the opaque canonical target reference.
- `fallback_target_ref` — the fallback target when the primary is stale.
- `action_mode` — read-only, comment/link, full-edit, offline-capture-only,
  publish-later, or handoff-only.

## Health state and action-mode coherence

A registry entry's `health_state` and `action_mode` are validated together. If the
health state blocks mutation (`blocked_policy_locked_mapping`,
`blocked_provider_unreachable`, `blocked_auth_loss`), the action mode MUST NOT
admit mutation (`full_edit`).

## Offline capture and publish-later

When a lane is `offline_capture_only` or `publish_later`, the registry preserves:

- The canonical work-item IDs.
- The queued actions.
- The redaction choices.
- The publish target and expiry.
- The retry/export/discard semantics.

These values survive restart, reconnect, and support export without silently
widening authority or dropping queued intent.

## CLI / headless inspect

The `StableRegistryInspectionRecord` exposes compact boolean projections so CLI
and inspector surfaces can answer:

- Is any account healthy?
- Is any mapping policy-blocked?
- Is any lane full-edit?
- Is any lane publish-later or offline-capture-only?

## Schema

`schemas/providers/stable_provider_account_install_grant_registry.schema.json`

## Fixture

`fixtures/providers/m4/stabilize-provider-account-install-grant-registry/registry_packet.json`
