<!-- SPDX-License-Identifier: Apache-2.0 -->

# Alpha Mirror and Offline Publication Dry Run

This packet records the mirror-only, deny-all, and offline-verification
postures consumed by
[`alpha_publication_manifest.yaml`](alpha_publication_manifest.yaml). It is a
dry run. It does not mutate public channels, mirror feeds, registry state, or
support-bearing release pointers.

## Postures

| Posture | Network expectation | Import instruction | Verification receipt | Freshness limit |
| --- | --- | --- | --- | --- |
| Mirror-only | Approved mirror reachable; public origin unreachable | Import from the approved mirror and verify digest/material refs from the manifest | `receipt:alpha.publication.mirror_only.*` | `PT24H` for mirror metadata, `P7D` for docs/help, `P30D` for notices and policy packs |
| Deny-all | No outbound network | Verify the preloaded local bundle and require all receipts to be present before any claim | `receipt:alpha.publication.deny_all.*` | `PT12H` for advisory/revocation-sensitive checks |
| Offline verification | Offline bundle only | Import the sealed offline bundle, validate family coverage, and render degraded live-truth states | `receipt:alpha.publication.offline_verification.*` | `PT12H` for advisory/revocation-sensitive checks |

Each posture has `publication_mutations_allowed: false` in the manifest. A
validator failure means the dry run is incomplete; it is not a reason to fall
back to a live public endpoint.

## Artifact-Family Coverage

| Family | Mirror-only | Deny-all | Offline verification | Vendor reachability |
| --- | --- | --- | --- | --- |
| Binaries | Covered by mirror receipt and graph metadata | Covered by local bundle receipt | Covered by offline bundle receipt | Not required for metadata verification |
| Docs/help packs | Covered with seven-day freshness | Covered with cached local docs | Covered with offline docs pack | Not required until freshness expires |
| Policy bundles | Covered by signed policy pack fixture | Covered by local fixture | Covered by offline policy pack fixture | Not required; graph linkage gap remains visible |
| Symbols | Covered by exact symbolication fixture | Covered by local fixture | Covered by offline symbol receipt | Not required for fixture-backed linkage |
| Support schemas | Covered by checked-in schema refs | Covered locally | Covered locally | Not required |
| Notices | Covered by notice seed and delta packet | Covered locally | Covered locally | Not required until reserved imports admit bytes |
| SBOM/provenance | Covered as placeholder-only evidence | Covered locally | Covered locally | Not required, but standards conformance is not claimed |

## Import Instructions

1. Mirror-only import reads the manifest row set and verifies that each
   required family has a mirror receipt. It must not try public origin fallback
   when the mirror is incomplete.
2. Deny-all import starts from the preloaded local bundle. Missing receipts
   block publication proof instead of triggering network repair.
3. Offline verification imports the bundle and evaluates artifact coverage,
   freshness, and live-truth degradation from the manifest.

These instructions are represented in `publication_postures[*].import_instructions`
inside the manifest so support, CLI, and release review can quote the same
steps.

## Live-Truth Degradation

| Truth surface | Mirror/offline behavior |
| --- | --- |
| Service health | Render as cached or unavailable. Do not imply live vendor health. |
| Advisory metadata | Render the bundled snapshot age and block stronger claims after `PT24H`. |
| Revocation metadata | Render the bundled snapshot age and block stronger claims after `PT12H`. |
| Docs/help freshness | Render the mirrored/offline pack age and block fresh-docs claims after `P7D`. |
| Notice freshness | Render the notice-pack source and reserved-import exceptions for up to `P30D`. |

The required behavior is degradation with explicit labels, not disappearing
rows or stale green states.

## Review Outcome

The dry run proves that the required alpha artifact families can be inspected
from one manifest in mirror-only, deny-all, and offline postures. It does not
prove broader alpha publication readiness because the manifest blockers remain
open.

## Verification

```sh
python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --posture mirror_only --validate-only
python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --posture deny_all --validate-only
python3 ci/release/check_alpha_publication_dry_run.py --repo-root . --posture offline_verification --validate-only
```
