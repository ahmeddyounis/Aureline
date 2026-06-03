# Dependency, security, compliance, and export truth

This document describes the canonical dependency-security-compliance export
packet used by Aureline's Help, About, support exports, and release surfaces.
It is the user-facing companion to the governed artifact at
`artifacts/deps/m4/dependency-security-compliance-export-truth.json`.

## What this packet covers

The export-truth packet answers four questions for every exact build:

1. **Advisories**: Are there active security advisories? Where did they come
   from? Are they suppressed, and if so, by whom, for what reason, until when,
   and will they reopen when the suppression expires?
2. **Licenses and notices**: What is the license-review posture for each
   dependency? What notice source backs it? Is it approved, requires notice,
   needs review, or is denied by policy?
3. **Lockfile risk**: How is each dependency resolved? Is it exact, pinned,
   out of date, unresolved, or known vulnerable?
4. **Build context**: What exact build, workspace, profile, and lockfile
   fingerprint produced this packet?

## Distinguishing "no findings" from "no feed"

A critical requirement of the packet is that it never claims a clean advisory
posture when the data needed to prove cleanliness is missing. The packet uses
two distinct states:

- **`no_active_findings`** — The feed was checked and no active advisories were
  found for the workspace packages.
- **`no_current_feed_data`** — The feed data is stale, missing, or the origin
  feed is offline. The absence of findings **cannot** be claimed.

Product surfaces, CLI output, support exports, and release packets must render
these states differently and never collapse `no_current_feed_data` into a green
"clean" badge.

## Suppression semantics

Suppressions are governed records, not hidden flags. Every suppression carries:

- **Actor**: Who authorized it.
- **Reason**: The policy or justification ref.
- **Scope**: What it applies to.
- **Expiry**: When it expires, if time-bound.
- **Reopen behavior**: Whether the underlying finding truth reopens when the
  suppression expires.

Expired suppressions that have `reopen_on_expiry: true` transition to the
`expired_reopened` state. Product surfaces must show the reopened finding rather
than leaving it visually green.

## Source and freshness classes

Advisory rows name both a **source class** and a **freshness class**:

| Source class | Meaning |
|---|---|
| `live_public_feed` | Direct from a public advisory database (OSV, GHSA, RustSec). |
| `enterprise_mirror` | From an enterprise-mirrored copy of a public feed. |
| `imported_report` | From an imported third-party scanner or audit report. |
| `stale_local_cache` | From a local cache that has breached its freshness SLO. |
| `offline_bundle` | From an offline bundle used in air-gapped environments. |

| Freshness class | Meaning |
|---|---|
| `current` | Data is within freshness SLO. |
| `stale` | Data is present but past freshness SLO. |
| `missing` | No data is available. |
| `mirror_only` | Only mirror data is available; origin feed status unknown. |
| `feed_outage` | Origin feed is explicitly in outage. |

## License-review posture

| Posture | Meaning |
|---|---|
| `approved` | License is approved for use. |
| `approved_with_notice` | License is approved but requires attribution notice. |
| `review_required` | Review is pending or incomplete. |
| `denied_by_policy` | License is denied by current policy. |
| `unknown_requires_review` | License state is unknown and must be reviewed. |

## Lockfile-risk class

| Risk class | Meaning |
|---|---|
| `resolved_exact` | Resolved to an exact version in the lockfile. |
| `policy_pinned` | Pinned by policy and matches the lockfile. |
| `out_of_date` | Out of date relative to the manifest or policy. |
| `unresolved` | Missing from the lockfile (e.g., optional feature not enabled). |
| `vulnerable` | Known vulnerable according to current advisory data. |

## How to inspect the packet

The checked-in JSON artifact is embedded in the `aureline-deps` crate and can
be loaded programmatically:

```rust
use aureline_deps::dependency_security_compliance_export_truth;

let packet = dependency_security_compliance_export_truth::current_dependency_security_compliance_export_truth()?;
```

The artifact is also readable directly at:
`artifacts/deps/m4/dependency-security-compliance-export-truth.json`

## Alignment with other surfaces

Help, About, support exports, review sheets, AI evidence, and release packets
all consume the same canonical packet. They do not clone stale text or badges.
When the packet downgrades due to stale feed data, expired suppressions, or
new findings, every consuming surface narrows its language automatically.
