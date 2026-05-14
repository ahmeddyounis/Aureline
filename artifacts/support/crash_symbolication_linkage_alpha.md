# Crash symbolication linkage alpha packet

This packet is the checked-in proof path for alpha crash, symbolication,
and support-bundle incident linkage.

## Canonical surfaces

| Surface | Path |
|---|---|
| Incident-trail implementation | `/crates/aureline-crash` |
| Support preview consumer | `/crates/aureline-support/src/bundle/crash_linkage.rs` |
| Shell support/export consumer | `/crates/aureline-shell/src/support_seed/mod.rs` |
| Protected fixtures | `/fixtures/support/incident_trail_alpha/` |
| Operator doc | `/docs/support/incident_trail_alpha.md` |

## Proof claims

| Claim | Evidence |
|---|---|
| Alpha crash capture links to exact-build symbols | `incident_trail_alpha::exact_build_symbolication_links_crash_to_support_bundle_manifest` |
| Partial symbolication is honest | `incident_trail_alpha::partial_symbolication_is_labeled_without_breaking_bundle_linkage` |
| Missing symbolication is honest | `incident_trail_alpha::missing_symbolication_keeps_evidence_and_safe_actions_visible` |
| Build mismatches cannot claim exact stacks | `incident_trail_alpha::exact_build_mismatch_refuses_to_claim_exact_symbolication` |
| Support bundle manifest linkage is surfaced | `crash_incident_trail_support_preview::support_preview_embeds_incident_trail_linkage_as_metadata` |
| Shell support/export can consume the trail | `support_bundle_alpha_manifest::shell_support_surface_consumes_crash_incident_trail` |

## Redaction posture

`support.item.crash_incident_trail` is environment-adjacent metadata.
It embeds refs, trace IDs, mapping states, exact-build identity, support
manifest refs, and safe next-action refs. It does not embed raw dump
bytes, raw stack bodies, raw memory pages, absolute paths, raw command
lines, or secrets.

Raw dump/core content remains governed by `support.item.crash_dump_or_core`
and stays local-only unless a separate reviewed upload path is approved.

## Verification

```sh
cargo test -p aureline-crash --test incident_trail_alpha
cargo test -p aureline-support --test crash_incident_trail_support_preview
cargo test -p aureline-shell --test support_bundle_alpha_manifest shell_support_surface_consumes_crash_incident_trail
```
