# Search/navigation truth packet cases

Worked YAML fixtures for the search/navigation truth packet,
bookmark-history audit, and query-export redaction contract:

- `milestone_close_informational.yaml` — informational
  milestone-close packet with engineering-internal audience, zero
  hidden results, no scope/workset caveats, no continuity caveats,
  and the export-redaction floor in place.
- `release_train_parity_assertion.yaml` — release-bearing
  release-train packet with release-readiness audience asserting
  parity across every consuming-surface class. Includes a
  non-zero hidden-result aggregate from a sparse-slice scope, a
  bookmark-remapped continuity entry from a renamed module, and a
  cross-surface back/forward continuity entry. Supersedes the
  milestone-close baseline.
- `claim_narrowing_semantic_disabled.yaml` — claim-narrowing packet
  under public-proof-safe audience. Admin policy disabled
  provider-hosted semantic scoring, so the AI-explanation overlay
  degrades to lexical-only and the previously-published "semantic-
  aware code search" claim is pulled back. Hidden-result aggregate
  captures the policy-blocked rows.
- `claim_widening_blocked_remote_partial.yaml` — claim-widening-
  blocked packet under enterprise-audit audience. Remote workspace
  shard returned a partial load and a cross-repo deep link from a
  teammate's collection is unresolvable under the current viewer's
  policy; the packet enumerates both blocking caveats and refuses
  to widen the search-relevance claim.
- `audit_denial_export_redaction_floor_unmet.yaml` — audit denial
  event recording an attempt to mint a packet that asserted only
  three of the five export-redaction floor classes.

Each packet fixture validates against
`schemas/search/search_truth_packet.schema.json` and carries only
opaque ids (workspace, workset, scope-binding, search-result-packet,
saved-query, deep-link binding, navigation-history entry, bookmark,
cross-repo-result-group, scope-diff-review, fixture, claim-manifest,
policy-epoch, build-identity) plus monotonic ISO 8601 placeholder
timestamps and redaction-aware reviewable labels — never raw query
bodies, raw document bodies, raw symbol definitions, raw notebook
cell bodies, raw URLs, raw absolute paths, raw provider payloads, or
raw secrets.
