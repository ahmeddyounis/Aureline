# M5 supported-line truth-feed and audience-packet registries

This lane lets external evaluators and support paths consume one current supported-line truth feed instead of
hand-assembled fragments, over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It bundles one export-safe
*truth feed* per active stable or LTS-candidate line — a public-proof summary, a migration-scoreboard summary, a
transparency snapshot, a correction-history summary, a claim-history summary, and a release-evidence link — with
stable IDs and freshness dates, so partner reviews, procurement checks, support escalations, and OSS stewardship
inherit one current feed rather than re-synthesizing product truth by hand, and projects that one canonical feed
into export-safe *audience packet* variants — a support bundle, a procurement bundle, or a partner-review bundle —
that exclude internal-only incident / security detail by default while still naming the current claim, evidence
freshness, migration posture, and correction history. It records the *truth-feed* grammar (one typed feed section
per active supported line, tracked against exact build / release-line identity, each bound to one supported-line
identity with its stable ID and freshness date and its links out to compatibility reports, known limits, migration
guides, and release evidence, and public-safe correction-history and claim-history summaries separated from
internal-only incident / security payloads) and the *audience-packet* grammar (the export-safe packet variant one
canonical truth feed is projected into for a named audience — support, procurement, or partner review) into registry
resolvers that produce export-safe, honest projections, so release / help, docs, support, procurement, and partner
surfaces resolve one canonical, freshness-checked truth instead of re-synthesizing product truth by hand. The feed
sections and the audience packets are separated in runtime and serialized state: the feed section, its current
claim, its evidence freshness, its exact-build provenance, and the linked supported-line-matrix / active-claim /
migration-guide / release-evidence refs live on the truth feed, while the resolved line identity, bundled truth-feed
reference, public-safe-versus-internal reference, packet-scope state, and active packet note live on the audience
packet, and no packet variant leaks internal-only incident / security detail or lets a stale feed read as current.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_truth_feed_and_audience_packet_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-truth-feed-and-audience-packet-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-truth-feed.schema.json`](../../schemas/program/m5-truth-feed.schema.json)
  (minted by this lane — the export-safe supported-line truth feed each active line is bundled into) and
  [`schemas/program/m5-audience-packet.schema.json`](../../schemas/program/m5-audience-packet.schema.json)
  (minted by this lane — the audience-specific packet variant projected from one canonical feed) as its canonical
  domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-truth-feed-and-audience-packet-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first supported-line truth feed —
  it demonstrates one durable product-line proof loop end to end, with support, procurement, and partner-review packet
  variants generated from one canonical feed for at least one active supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-truth-feed-and-audience-packet-registries/`
  (`truth_feed_beta_narrowed.json`, `audience_packet_preview_narrowed.json`).

## Two registries

1. **Truth feed** (`resolve_truth_feed_entry`) — bundles one typed feed section per active supported line: the feed
   section and its canonical mode, the bundled evidence rows, the linked supported-line-matrix / active-claim /
   migration-guide / release-evidence refs, the current claim, the evidence freshness, and the owning roster, with
   public-safe correction-history and claim-history summaries separated from internal-only incident / security
   payloads. A clean feed names a canonical registry token, a classified feed section, and a transparency role,
   covers the canonical / accessible / audit resolution forms, publishes a complete object, preserves its exact-build
   provenance before a claim widens, and keeps a public-facing section's published summary matched to current proof.
   Otherwise it degrades honestly — a line widening its claim on stale proof, or a public-facing section running its
   published summary ahead of current proof, degrades to
   `truth_feed_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker reason a
   widen-on-stale-proof attempt must surface.
2. **Audience packet** (`resolve_audience_packet_entry`) — projects one canonical truth feed into an export-safe,
   audience-specific packet variant rather than a hand-assembled per-audience summary. A clean entry names a
   classified packet scope (support-bundle, procurement-bundle, or partner-review-bundle) and provides the complete
   line-identity / bundled-truth-feed / public-safe-versus-internal / packet-scope / active-note packet object; a
   packet variant that would keep a claim ahead of current proof, leak internal-only detail, or let a stale feed
   masquerade as current degrades to
   `audience_packet_runs_support_ahead_of_proof_or_drops_audience_packet`.

## Per-record truth-feed reference

Each bundled feed section carries its canonical mode, and the resolver publishes the full feed object, so the
registry — never a feed merely assumed to still be current — is the single source of truth.
`truth_feed_object_is_complete` rejects an object missing any feed field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale proof or a published
summary running ahead of current proof, and `audience_packet_stays_honest` rejects a packet variant that has kept a
claim ahead of current proof.

## Acceptance criteria (proven by resolved examples)

- **At least support and procurement or partner packet variants can be generated from one canonical supported-line
  truth feed with stable IDs, freshness dates, and consistent line identity.** Clean truth-feed entries cover the
  canonical public-proof-summary / migration-scoreboard-summary / transparency-snapshot / correction-history-summary /
  claim-history-summary / release-evidence-link feed sections and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  feed entry published an incomplete object.
- **Export-safe packets exclude internal-only incident / security detail by default while still naming the current
  claim, evidence freshness, migration posture, and correction history.** A widen-on-stale-proof example and an
  unbound example degrade, a clean feed entry is present, and no clean entry is unbound or missing its exact-build
  provenance.
- **At least one consumer opens the truth feed directly instead of rendering a hand-authored parallel summary.**
  Clean audience-packet entries cover the support-bundle / procurement-bundle / partner-review-bundle packet scopes
  with full resolution-form coverage while providing the complete packet object — the resolved line identity and the
  active packet note — and a packet variant that would keep a claim ahead of current proof or drop the packet
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- truth-feed-table
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- fixture-truth-feed-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_truth_feed_and_audience_packet_registries -- fixture-audience-packet-preview-narrowed
```
