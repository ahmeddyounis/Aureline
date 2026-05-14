# Install-review alpha for marketplace and package lanes

This page is the reviewer-facing companion to the Rust contract in
[`crates/aureline-extensions/src/install_review/mod.rs`](../../crates/aureline-extensions/src/install_review/mod.rs)
and the protected fixtures in
[`fixtures/extensions/install_review_alpha/`](../../fixtures/extensions/install_review_alpha/).

The lane turns marketplace or package decisions into one canonical native
review sheet. Marketplace rows, package detail pages, hosted catalog content,
and extension webviews may display the same facts, but they are narrower
consumers of the native sheet and cannot approve install or enable mutations
directly.

## Truth sources consumed

The install-review packet reads existing contracts by reference:

| Concern | Source |
| --- | --- |
| Manifest origin, publisher trust, host family, and permission deltas | `crates/aureline-extensions/src/manifest_baseline/` |
| Publisher continuity, revocation, policy, rollback, and upstream review state | `crates/aureline-extensions/src/review_alpha/` |
| Provider-owned content source and actor scope | `crates/aureline-provider` |
| Reachability and host-boundary cues | `crates/aureline-runtime` |
| Local/managed/mirror service boundary | `crates/aureline-auth` |
| Install mode, channel, updater owner, state roots, repair/verify, and rollback owner | `crates/aureline-install/src/topology/` |

The install-review lane does not define a package installer, registry backend,
extension host, or hosted marketplace UI. It freezes the product-owned review
packet and the first projections those surfaces must consume.

## Required disclosure

Every install or enable review must render these disclosure classes before a
mutation can proceed:

- owner and origin;
- current profile, org, and workspace/workset scope;
- network reachability or policy-block state;
- service boundary and host boundary;
- canonical native review packet and authority relationship;
- declared-vs-effective permission delta;
- compatibility labels and evidence refs;
- activation-budget class, triggers, axes, and evidence refs;
- install-topology row truth;
- publisher trust tier and manifest origin.

If any disclosure is hidden, `evaluate_install_review_alpha` emits
`denied / missing_required_disclosure`. If provider-owned content omits provider
source or actor scope, it emits
`denied / owner_origin_scope_network_boundary_missing`.

## Compatibility and activation budget

Compatibility labels reuse the schema vocabulary already used by marketplace
discovery:

- `compatible_on_all_declared_targets`
- `compatible_on_subset_of_declared_targets`
- `compatibility_bridge_required`
- `compatibility_unknown_pending_reverification`
- `incompatible_blocked_on_policy`

Missing, stale, or unverified compatibility evidence blocks install and enable
with `compatibility_evidence_missing`. Bridge-backed or subset compatibility is
allowed only through native review acknowledgement.

Runtime-cost labels reuse the runtime-budget/discovery vocabulary. Low or
nominal claims must be backed by activation evidence or benchmark evidence, not
only self-reported counters. Quarantined runtime cost blocks mutation.

## Hosted and webview consumers

Hosted marketplace lanes and extension webviews must show the same
owner/origin/scope/network/service-boundary truth as the native sheet, but their
authority class must be read-only:

- `hosted_marketplace_read_only_consumer`
- `extension_webview_read_only_consumer`
- `provider_hosted_read_only_consumer`

A hosted or webview consumer that attempts native approval is denied with
`hosted_consumer_cannot_approve`. Its projection may offer
`open_native_review_sheet`, evidence panels, and support export only.

## Protected fixtures

The fixture corpus covers:

| Fixture | Proof |
| --- | --- |
| `native_marketplace_package_lane.json` | A native review sheet admits only after permission delta, compatibility, activation budget, and install topology are rendered. |
| `hosted_provider_lane_parity_denied.json` | Provider-owned hosted content renders boundary truth but is denied when it tries to approve directly. |
| `hosted_provider_lane_hidden_boundary_denied.json` | Hosted content is denied when it hides service-boundary disclosure. |

Run:

```bash
cargo test -p aureline-extensions install_review
```
