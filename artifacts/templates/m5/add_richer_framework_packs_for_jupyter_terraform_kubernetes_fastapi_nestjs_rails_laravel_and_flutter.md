# Richer Framework Packs: Jupyter Adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter

- Packet: `richer-framework-pack:stable:0001`
- Label: `Richer Framework Packs: Jupyter Adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter`
- Rows: 8 (6 admitted for offer)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)

## Rows

- **Jupyter Adjacency Pack** `1.3.0` (gen `1.3.0`, Notebook adjacency — notebook_adjacency): partner_certified / bridge_behavior
  - Header: Partner-certified Jupyter adjacency pack; the header shows partner provenance and the pinned pack and generator versions, the freshness chip reads fresh, and a support-class banner discloses that notebook execution is bridged to an adjacent kernel rather than generated as first-party source
  - Freshness chip: v1.3.0 · fresh (fresh)
  - Capability: partial_capability | Origin: bridged_adjacent | Health: healthy_uncertified
  - Banner: support_class_banner | Offered: true
- **Terraform Provisioning Pack** `4.1.0` (gen `4.1.0`, Infrastructure provisioning — infrastructure_provisioning): first_party / officially_supported
  - Header: First-party Terraform provisioning pack; the header shows certified first-party provenance and the pinned pack and generator versions, the freshness chip reads fresh, and the pack generates managed infrastructure modules natively with no downgrade banner
  - Freshness chip: v4.1.0 · fresh (fresh)
  - Capability: full_capability | Origin: generated_managed | Health: certified_healthy
  - Banner: no_banner | Offered: true
- **Kubernetes Manifests Pack** `3.0.2` (gen `3.0.2`, Infrastructure provisioning — infrastructure_provisioning): first_party / officially_supported
  - Header: First-party Kubernetes manifests pack; the header shows first-party provenance and the pinned versions, the freshness chip reads update-available because a newer pack version exists, and the pinned version still offers full first-party generation
  - Freshness chip: v3.0.2 · update available (update_available)
  - Capability: full_capability | Origin: generated_managed | Health: certified_healthy
  - Banner: no_banner | Offered: true
- **FastAPI Service Pack** `2.7.1` (gen `2.7.1`, Web API service — web_api_service): first_party / officially_supported
  - Header: First-party FastAPI service pack; the header shows certified first-party provenance and the pinned versions, the freshness chip reads fresh, and routes, models, and dependency wiring are generated natively into a managed zone with no downgrade banner
  - Freshness chip: v2.7.1 · fresh (fresh)
  - Capability: full_capability | Origin: generated_managed | Health: certified_healthy
  - Banner: no_banner | Offered: true
- **Nest Service Pack** `2.4.0` (gen `2.3.0`, Web API service — web_api_service): community / community_supported
  - Header: Community Nest service pack; the header shows community provenance and the pinned pack and generator versions, the freshness chip reads update-available, and a capability banner discloses that the background-worker generator is partial so the pack is offered with the partial-capability banner rather than as full first-party truth
  - Freshness chip: v2.4.0 · update available (update_available)
  - Capability: partial_capability | Origin: generated_managed | Health: healthy_uncertified
  - Banner: capability_banner | Offered: true
- **Rails Service Pack** `5.2.0` (gen `5.2.0`, Web API service — web_api_service): community / community_supported
  - Header: Community Rails service pack; the header shows community provenance and the pinned versions, the freshness chip reads fresh, and controllers, models, and migrations are generated natively into a managed zone with no downgrade banner
  - Freshness chip: v5.2.0 · fresh (fresh)
  - Capability: full_capability | Origin: generated_managed | Health: certified_healthy
  - Banner: no_banner | Offered: true
- **Laravel Bridge Pack** `0.9.0` (gen `0.9.0`, Web API service — web_api_service): community / heuristic_mapping
  - Header: Laravel bridge pack that maps some structure through heuristic conventions rather than exact first-party generation; the header marks community provenance, the capability banner reads heuristic, the support-class banner discloses the heuristic mapping and its known issues, the archetype health reads degraded, and the pack is held from being offered as exact truth
  - Freshness chip: v0.9.0 · aging (aging)
  - Capability: heuristic_capability | Origin: runtime_observed | Health: degraded
  - Banner: support_class_banner | Offered: false
- **Flutter Pack (Mirror, Unverified)** `0.0.0` (gen `0.0.0`, Mobile application — mobile_app): provenance_unknown / support_unknown
  - Header: Flutter mobile pack served from a mirror whose provenance could not be verified; the header is marked provenance-unknown, the freshness chip reads unverified, the capability and origin truth read unknown, the archetype health reads unknown, and the provenance-unknown downgrade banner blocks the pack from being offered rather than hiding it
  - Freshness chip: v0.0.0 · unverified (freshness_unknown)
  - Capability: capability_unknown | Origin: origin_unknown | Health: health_unknown
  - Banner: provenance_unknown_banner | Offered: false
