# M5 sideload review sheets

This document describes the canonical packet that freezes the **M5 sideload review
sheets** — one reviewed-install model per unpacked or archive-backed side-load of a
marketed M5 artifact family. It is the user-facing companion to the governed artifact
at `artifacts/ecosystem/m5/m5-sideload-review.json` and the typed model in the
`aureline-ecosystem` crate (`m5_sideload_review`).

Where the [`M5 install/update review sheets`](m5-install-review.md) review a package
fetched through the registry, and the
[`M5 author and publish preview`](m5-author-and-publish-preview.md) gate the whole
author-side lane before a package reaches the public registry, this packet freezes how
a **side-load** is reviewed before it is installed or reloaded. A side-load never
reaches the registry first, so the review sheet is the only place its source identity,
signing state, requested permissions, external executables, and registry-binding
decision are made explicit — and it must hold the same review discipline as a public
install rather than being waved through because the artifact is already on local disk.

## What each sheet makes explicit

Each sheet pins the side-load's identity and trust facts:

- **Source identity** — a [`source.kind`] of `unpacked_directory` or
  `archive_bundle`, a redacted `source.path_class` (workspace-relative, user-home,
  removable media, network mount, or process stream), a redacted `display_hint` that
  is never an absolute machine path, and — for an archive — an opaque
  `content_address_ref`.
- **Signing state** — one of `signed_verified`, `signed_unverified`,
  `unsigned_local_dev`, `unsigned_sideload`, or `revoked_signature`, with an opaque
  `signature_ref` present only when the artifact is signed or revoked.
- **Permissions** — the requested permission scopes, each carrying its kind, redacted
  target, rationale, and a `change` relative to the installed baseline.
- **External executables** — each disclosed external host process or helper binary,
  its purpose, optional content address and signer, and a `change` relative to the
  installed baseline.
- **Update binding** — the `stay_local`, `bind_to_registry_later`, or
  `bound_to_registry_identity` decision.
- **Runtime class and host/ABI** — the runtime class and host/ABI the side-load binds
  to, plus the declared `host_abi_range`.

A reload or update also carries an `installed_baseline` so the review can compute what
changed and preserve the installed row's limited-trust continuity.

## Three recomputed values

The sheet is honest by construction. Three published values are **recomputed** from
the sheet's facts, and the stored values must equal the recomputation or validation
fails:

- **`rendered_trust_tier`** is the weakest of the claimed tier, the signing-state
  ceiling, and the registry-binding ceiling. A still-local binding caps at
  `unsigned_local_only`; a `bound_to_registry_identity` binding caps at
  `registry_bound`. No binding ever permits a `verified_publisher` or
  `enterprise_approved` badge, so a locally-built or side-loaded artifact can never
  inherit a trusted-publisher badge just because it was built or signed on a trusted
  machine.
- **`review_triggers`** are computed from the installed baseline: a permission
  widening, runtime-class change, host/ABI rebind, newly introduced external
  executable, changed registry binding, or changed release channel each appears in the
  set. Any trigger forces a fresh review.
- **`disposition`** is the stronger of the trigger gate and a hard `blocked` for a
  revoked signature or an anti-abuse quarantine. A side-load with no triggers and no
  hard block is `reviewed_install_ready`; any widening or rebinding is
  `fresh_review_required`; a revoked or quarantined side-load is `blocked`.

## Guardrails

- **No silent hot reload of widened authority.** Any permission, runtime, executable,
  binding, or channel widening on an installed side-load forces
  `fresh_review_required` and disables the accept action until a fresh review clears
  it.
- **No inherited trust.** A side-load can never render a trusted-publisher badge; the
  validator flags any rendered tier above the recomputed cap.
- **Limited-trust continuity on installed rows.** A reload that does not rebind to the
  registry can never raise the installed row's rendered badge.
- **No install-style review bypass.** The review sheet is the install-style review; a
  side-load on local disk is never waved through.
- **Export-safe.** Every field is a typed state, a redacted hint, or an opaque ref —
  no absolute paths, archive bytes, signing secrets, or executable payloads.

## Consuming surfaces

`M5SideloadReview::export_projection()` produces a redaction-safe row set that support
exports, docs/help, and release/public-truth surfaces render instead of restating
sideload source, trust, and disposition text by hand.
