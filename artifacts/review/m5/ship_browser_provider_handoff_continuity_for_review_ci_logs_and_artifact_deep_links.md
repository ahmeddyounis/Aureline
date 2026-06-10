# Browser/Provider Handoff Continuity for Review, CI, Logs, and Artifact Deep Links

- Packet: `handoff-continuity:stable:0001`
- Schema: `schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json`
- Support export: `artifacts/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/support_export.json`
- Contract doc: `docs/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md`
- Fixtures: `fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/`
- Producer: `aureline_review::current_handoff_continuity_export`

## Coverage

- **Handoff targets** name each surface a handoff leaves from (`review_thread`,
  `ci_pipeline`, `ci_run`, `log_viewer`, `artifact_deep_link`, `generic_target`,
  `unknown_target_provider_owned`) and disclose whether the target supports
  anchored deep links, safe preview, and a provider handoff, so a handoff can
  never claim continuity the target does not have.
  `unknown_target_provider_owned` is never flattened into a known target.
- **Handoff continuity rows** carry, per handoff, the durable review anchor, the
  target id, a redaction-aware subject label, an attention block, and an actor
  attribution with an audit row, so a handoff is always anchored, attributable,
  and honest about what it carries across the boundary. An unknown destination, an
  unverified/untrusted/unknown trust class, an unanchored/unknown link, a stale/
  unknown freshness, an unsafe/unsupported preview, or a blocked action each
  require an explicit attention reason.
- **Target identity** records, per handoff, the destination class
  (`in_product_surface`, `browser_tab`, `provider_web_surface`, `native_app`,
  `unknown_destination_provider_owned`), the trust class (`first_party_trusted`,
  `provider_verified`, `provider_unverified`, `untrusted_external`,
  `unknown_trust_provider_owned`), a host label, a provider label, and a
  disclosure flag, so a handoff can never hide which destination, host, or
  provider it lands on.
- **Deep-link continuity, safe preview, and action** record, per handoff, the
  link exactness (`anchored_deep_link`, `path_scoped_link`, `opaque_token_link`,
  `unanchored_link`, `unknown_link_provider_owned`), the truth freshness
  (`fresh_current_truth`, `stale_prior_truth`, `unknown_freshness_provider_owned`),
  the safe-preview class (`safe_preview_sandboxed`, `safe_preview_read_only`,
  `unsafe_preview_blocked`, `preview_unsupported`, `unknown_preview_provider_owned`),
  and a typed action kind (`open_in_product`, `copy_deep_link`,
  `reveal_target_local`, `open_in_browser_handoff`, `open_in_provider_handoff`,
  `unsupported_no_continuity`). A handoff is read-only navigation unless an
  attributable `open_in_browser_handoff` / `open_in_provider_handoff` cites a
  `handoff_ref`, and a stale truth, an unanchored link, or an unsafe preview
  narrows the action (`blocked_stale_truth_review_required`,
  `blocked_no_durable_anchor`, `blocked_untrusted_target`,
  `blocked_unsafe_preview`) rather than jumping blind.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: the target host/provider identity is disclosed; the target
trust class is explicit; the deep link, truth freshness, safe-preview class, and
handoff action are all disclosed; a handoff action is read-only unless an
attributable handoff is cited; every handoff is anchored and attributable; no
action creates hidden write scope; a stale truth narrows the action; downgrade
narrows the claim instead of hiding the lane; and stale or underqualified rows
block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`handoff_attribution_missing`, `truth_stale`, `deep_link_unanchored`,
`target_identity_undisclosed`, `target_trust_unknown`, `safe_preview_unsupported`,
`trust_narrowing`, and `upstream_dependency_narrowed`.

## Boundary

Raw deep-link URLs, raw host names, raw provider payloads, raw log bodies, raw
artifact bytes, raw absolute paths, raw author email addresses, credentials, and
live provider responses never cross this boundary. The packet carries only
metadata, target capabilities, target classes, destination classes, trust
classes, link classes, freshness classes, safe-preview classes, action kinds,
blocked classes, reviewable labels, and contract references. Every handoff, target
identity, deep link, safe preview, and action stays attributable and reviewable
before any handoff or upstream effect fires.
