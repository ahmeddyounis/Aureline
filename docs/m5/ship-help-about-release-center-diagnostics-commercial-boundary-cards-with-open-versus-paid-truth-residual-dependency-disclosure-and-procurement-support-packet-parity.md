# Help/About, release-center, diagnostics, and procurement commercial-boundary cards with open-versus-paid truth, residual-dependency disclosure, and procurement/support packet parity

Reviewer contract for the canonical commercial-boundary-card set: the
commercial-boundary surface a user, admin, or procurement reviewer sees on
Help/About, the release center, diagnostics, and in a procurement/support packet.
One card for the local open core plus one per managed lane states which
capabilities are local and open-source, which are optional managed/paid lanes,
what residual dependencies remain vendor-hosted, which deployment profiles each
boundary actually holds in, and what procurement/support evidence is available.
This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-commercial-boundary-cards.json`
- Boundary schema: `schemas/service/m5-commercial-boundary-cards.schema.json`
- Human-readable rendering: `artifacts/m5/ship-help-about-release-center-diagnostics-commercial-boundary-cards-with-open-versus-paid-truth-residual-dependency-disclosure-and-procurement-support-packet-parity.md`
- Overview companion: `docs/service/m5_commercial_boundary_cards.md`
- Fixture corpus: `fixtures/service/m5-commercial-boundary-cards/`
- Owning crate module: `crates/aureline-service/src/m5_commercial_boundary_cards/`

## Projects the frozen control-plane matrix

Each managed card reuses the closed vocabularies already frozen by the
commercial-control-plane matrix (`docs/service/m5_commercial_control_plane.md`) —
the service-family, marketed-claim, export-guarantee, and posture-origin classes —
rather than minting a parallel synonym set. A managed card is cross-checked by
`BoundaryCardSet::cross_check_against_control_plane`, which confirms the card's
declared marketed claim, export guarantee, and non-empty local-safe baseline match
the control-plane lane for its service family. The residual-dependency class and
deployment-profile vocabularies are re-exported from
`artifacts/governance/residual_dependencies.yaml` and
`artifacts/governance/deployment_profiles.yaml`. The new tokens are only the
commercial-boundary vocabulary those sources did not carry: the open-versus-paid
boundary class, the deployment-profile qualifier, the procurement/support packet
kind, the boundary-evidence status, the cost-figure disclosure, and the
boundary-action kind.

## The cards

One local-open-core card plus one card per managed service family — a 7-card set:

- **Local open-source core** — `local_open_source`, claim `local_safe_only`. The
  editor core, search, navigation, local Git, and already-authorized local
  automation run with no managed dependency and no payment, in every deployment
  profile including air-gapped. It declares no residual vendor dependency and
  carries no upsell.
- **Managed AI gateway** — `managed_paid_optional`, `ai_gateway_family`. Optional
  paid managed-broker inference; bring-your-own-key and local AI providers are the
  open alternative. Residual: a vendor-hosted AI provider and control-plane
  reachability, both eliminated under self-host/BYOK. Not offered air-gapped.
- **Managed settings sync** — `sync_family`. Optional paid cross-device sync; local
  settings stay authoritative. Residual: control-plane reachability (localized by
  self-host) and a managed sign-in. Not offered individual-local or air-gapped.
- **Companion relay** — `collaboration_relay_family`. Optional paid live
  collaboration; local notes and offline packets continue. Residual: control-plane
  reachability and a vendor notification channel. Not offered individual-local or
  air-gapped.
- **Managed registry and mirror** — `registry_or_mirror_metadata_family`. Optional
  paid discovery/install metadata; installed and sideloaded packages keep running.
  Residual: the package registry and remote mirror, both resolvable against a
  signed mirror or offline bundle — so it is offered in every profile.
- **Managed support ingest** — `telemetry_or_support_ingest_family`. Optional paid
  bundle upload; local support-bundle export is always available. Residual:
  control-plane reachability and a managed redaction/policy bundle. Not offered
  air-gapped.
- **Managed workspace** — `remote_workspace_control_plane_family`. Optional paid
  remote sessions; local checkout, tasks, and Git continue. Residual: a
  vendor-hosted remote agent and control-plane reachability, run locally under
  self-host. Not offered individual-local or air-gapped.

## What the set proves

- **Local-core productivity is never blocked.** Every card keeps a non-empty
  `local_safe_baseline`, so a stale or unavailable metering/rating path narrows
  only an optional managed action — never local editing, search, Git, or
  already-authorized local automation. Boundary evidence that goes stale or
  missing narrows the managed claim, not the local core.
- **Open-versus-paid is stated, not implied.** The local-open-core card makes only
  the local-safe claim, binds no managed service family, and declares no residual
  dependency; every managed card declares the full managed claim and discloses at
  least one residual vendor-hosted dependency, so a buyer can see exactly where the
  paid boundary is.
- **No open or self-hosted boundary is overstated.** Every residual dependency
  names whether it `remains_vendor_hosted` and whether self-hosting
  `eliminated_under_self_host`, and every card names the deployment profiles its
  boundary `holds_in_profiles`. A managed lane that is genuinely unavailable
  air-gapped says so rather than implying a stronger self-hosted boundary than the
  running lane supports.
- **Procurement and support packets reuse one object model.** Both the
  procurement-packet and support/admin surfaces bind the same
  `ProcurementSupportEvidence` object — open-source license manifest,
  residual-dependency disclosure, usage/forecast and chargeback exports,
  entitlement summary, and support bundles — at the same export guarantee, so a
  buyer and a support engineer read one vocabulary.
- **Commercial prompts never outrank support/export/local-continuation truth.**
  Each card's ranked actions put `export_evidence`, `continue_local`, and
  `view_procurement_packet` above any `learn_about_paid` prompt; only managed cards
  carry an upsell, and it always ranks last.
- **No spend or quota number without unit, as-of time, and scope owner.** Boundary
  cards defer every figure to the usage, forecast, and chargeback surfaces
  (`cost_figure_disclosure: deferred_to_metering_surfaces`); each card carries an
  `as_of` time so a future bound figure is never shown without one.
- **The marketed claim narrows automatically.** A managed card's effective claim is
  the declared claim capped by its `evidence_status` — `managed_full` when current,
  `managed_narrowed` when stale, `local_safe_only` when missing or downgraded — so
  an affected marketed/support claim narrows when required evidence is missing,
  stale, or downgraded. The local open core never narrows.
- **One packet, many surfaces.** Help/About, the release center, diagnostics, the
  procurement packet, the support/admin packet, and claim/public-truth automation
  each bind the set and project the effective claim — never a stronger one — render
  the local-safe baseline, disclose the residual dependencies, name the
  deployment-profile qualifier, and keep evidence above any upsell.

## Note on account-state distinctness

These cards disclose the open-versus-paid commercial boundary; they are not account
error surfaces. The distinctness of a seat loss, an org switch, a grace window, and
a sign-in failure is owned by `m5_commercial_control_plane` (managed-state
vocabulary) and `m5_offboarding_cards` (lifecycle events); the boundary cards never
collapse those conditions into a generic account error because they do not render
them at all.

## Regeneration

`canonical_commercial_boundary_card_set` builds the set;
`current_stable_commercial_boundary_card_set` reads and validates the checked-in
packet. Drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_commercial_boundary_cards/tests.rs`. Regenerate the
artifact with
`cargo run -p aureline-service --example dump_m5_commercial_boundary_cards -- canonical`.
