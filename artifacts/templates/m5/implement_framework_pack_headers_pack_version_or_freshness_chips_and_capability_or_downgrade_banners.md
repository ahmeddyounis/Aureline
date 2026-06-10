# Framework-Pack Headers, Pack Version/Freshness Chips, and Capability/Downgrade Banners

- Packet: `framework-pack:stable:0001`
- Label: `Framework-Pack Headers, Pack Version/Freshness Chips, and Capability/Downgrade Banners`
- Rows: 4 (2 admitted for offer)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **Rust Axum Service Pack** `3.2.1` (Axum): first_party / officially_supported
  - Header: First-party Axum service pack; the header shows certified first-party provenance and the pinned pack version, the freshness chip reads fresh against the last verification, and the pack offers full first-party generation with no downgrade banner
  - Freshness chip: v3.2.1 · fresh (fresh)
  - Capability: full_capability (banner: no_banner)
  - Offered: true
- **Nest Service Pack** `2.4.0` (Nest): community / community_supported
  - Header: Community Nest pack; the header shows community provenance and the current pack version, the freshness chip reads update-available, and a capability banner discloses that one optional generator is partial so the pack is offered with the partial-capability banner rather than as full first-party truth
  - Freshness chip: v2.4.0 · update available (update_available)
  - Capability: partial_capability (banner: capability_banner)
  - Offered: true
- **Flask Bridge Pack** `0.8.0` (Flask): bridged_from_other_framework / heuristic_mapping
  - Header: Flask bridge pack that maps some structure through heuristic conventions rather than exact first-party generation; the header marks bridged provenance, the capability banner reads heuristic, the support-class banner discloses the bridge behavior and its known issues, and the pack is held from being offered as exact truth
  - Freshness chip: v0.8.0 · aging (aging)
  - Capability: heuristic_capability (banner: support_class_banner)
  - Offered: false
- **Mirror Pack (Unverified)** `0.0.0` (Unverified Mirror): provenance_unknown / support_unknown
  - Header: Mirror pack whose provenance could not be verified; the header is marked provenance-unknown, the freshness chip reads unverified, the capability banner reads unknown, and the provenance-unknown downgrade banner blocks the pack from being offered rather than hiding it
  - Freshness chip: v0.0.0 · unverified (freshness_unknown)
  - Capability: capability_unknown (banner: provenance_unknown_banner)
  - Offered: false
