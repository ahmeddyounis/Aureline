# Deep-link and file-association boundary contract

This page is the reviewer-facing entry point for the first runnable shell
projection of embedded boundary chrome and native desktop handoff review.

The goal is consistent ownership disclosure before an OS-originated or
embedded-web action can have product consequences. The shell must name the
owner, origin, target, channel/build, command class, trust/profile boundary,
freshness, and recovery path before it opens, mutates, restores authority, or
returns from a browser callback.

## Canonical Runtime Sources

- `crates/aureline-shell/src/embedded/boundary_alpha.rs` projects embedded
  docs/help, extension webview, and marketplace/account cards into support
  rows with owner/origin/scope/network/service-boundary fields.
- `crates/aureline-shell/src/deeplink/native_handoff.rs` projects
  `deep_link_intent_record` and `native_file_affordance_case_record` inputs
  into native review rows.
- `crates/aureline-shell/src/bootstrap/native_shell.rs` writes the current
  support evidence under `.logs/recovery/`.

The runtime evidence files are:

- `.logs/recovery/embedded_boundary_alpha_latest.json`
- `.logs/recovery/native_boundary_handoff_latest.json`

## Embedded Surface Rules

Embedded docs/help, extension webviews, marketplace/account panes, service
dashboards, and auth handoff surfaces must keep host-rendered chrome visible.
That chrome names:

- owner and publisher/service identity;
- origin and host/domain;
- profile, tenant, provider, or execution scope;
- network/provider health and freshness;
- service or data boundary;
- browser fallback posture and packet ref.

Embedded content may inherit theme, zoom, density, focus, reduced motion, high
contrast, and forced-color posture from the host. It may not impersonate native
product trust, approval, update, rollback, or AI-apply review chrome. Those
surfaces remain product-owned.

## Native Handoff Rules

Protocol handlers, default-browser callbacks, file associations, system-open,
open-with, recents, dock/taskbar jump actions, native open/save dialogs,
reveal-in-system-shell, terminal entry, and similar OS surfaces all route
through a product-owned review projection.

Each review row names:

- source surface and origin class;
- route and requested command class;
- canonical target identity, availability, and freshness;
- handler owner, owning channel, and owner build;
- trust state, policy epoch, and tenant/workspace scope;
- replay posture, authority delta, fallback, and recovery actions.

High-risk approvals, destructive or mutating actions, trust elevation,
rollback, AI apply, and expired/replayed authority cannot execute directly from
the OS surface. They must reopen a product-owned review surface or fail closed
with recovery.

## Recovery Behavior

Missing, moved, unmounted, ambiguous, policy-blocked, remote-unreachable, and
expired targets degrade to explicit recovery instead of silent retargeting.

The recovery action vocabulary includes:

- Locate
- Open cached context
- Reconnect or restart in browser
- Review intent or review write
- Choose another target
- Close placeholder or cancel

Native open/save/reveal and file association flows preserve canonical object
identity and safe-preview/write-safety posture. Native dialogs select targets;
they do not grant trust, write authority, or policy exceptions.

## Fixture Coverage

The compact shell fixture set is listed in
`fixtures/ux/embedded_boundary_alpha/manifest.yaml`.

It covers:

- docs/help embedded boundary chrome;
- extension webview owner/origin/scope/network disclosure;
- marketplace/account service-boundary and stale-provider disclosure;
- file association owner/channel/target review before local file open;
- system-open exact workspace entry;
- dock/taskbar recent missing-target recovery;
- dock/taskbar jump action command routing without a privileged OS shortcut;
- protocol-handler review handoff and privileged command review;
- default-browser auth callback replay denial;
- native open dialog identity binding;
- native save dialog write-review routing;
- reveal-in-system-shell identity-preserving handoff;
- removable-volume placeholder recovery.
