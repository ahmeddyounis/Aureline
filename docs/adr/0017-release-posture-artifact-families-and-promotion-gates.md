# ADR 0017 - Release posture, artifact families, and promotion gates

- **Decision id:** D-0010 (see `artifacts/governance/decision_index.yaml#D-0010`)
- **Status:** Accepted
- **Decision date:** 2026-04-22
- **Freeze deadline:** 2026-08-31
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** release_council
- **Related requirement ids:** none

## Context

The repository already froze the exact-build identity model, the release
artifact graph, the release-evidence packet shape, the evidence
freshness policy, the shiproom review order, the support-packet
families, the docs/help truth contracts, and the claim-manifest parity
rules. What it did not yet freeze was the decision that ties those
pieces together at promotion time: which release channels exist, which
artifact families belong to which release posture, which owner lane is
responsible for promoting each family, what the rollback unit is, how
debug sidecars and source maps are retained, when mirror or offline
publication must stay parity-complete, which changes are same-change-set
obligations rather than later cleanup, how fine-grained advisories and
revocations are allowed to be, which waiver classes may exist at which
stage, and what blocks an RC or stable promotion even when the binary
itself compiles.

Leaving that implicit was dangerous because the failure mode is not
abstract architecture drift; it is operational overclaim. One team could
treat a successful binary build as a promotion candidate while another
still reads the release as blocked on docs/help parity, mirror drills,
support/export truth, clean-room proof, or source-map retention. Support
could preserve a broader rollback or symbolication story than release is
actually carrying. Docs and claim rows could narrow on different clocks.
An emergency revocation could either cut too broadly and strand
unaffected installs or too narrowly and leave broken update metadata in
place. A stable candidate could therefore look healthy in one packet and
non-promotable in another simply because the repository lacked one
governing release-posture decision.

The source documents are explicit that Aureline must support monthly
stable releases and optional LTS lines, that promoted channels are
artifact graphs rather than lone binaries, that mirrors and air-gap
bundles preserve revocation and audit lineage, that emergency response
must support manual import or mirror-only environments, that evidence
freshness is a first-class blocker, and that docs, support notes, known
limits, release notes, and evidence packets must use the same truth row.
The release and governance docs already encode most of the pieces. This
ADR closes the missing release-posture decision so later release,
clean-room, docs/help, support/export, known-limit, and claim-parity
work cite one governing rule instead of restating a local interpretation
of the same release controls.

## Decision

Aureline freezes one release-posture policy for channel movement, one
coordinated rollback atom, one artifact-family posture map, one
promotion-gate map, one waiver and late-proof policy, one mirror/manual
import emergency-transport policy, and one advisory/revocation scope
policy. The narrative rules live here; the machine-readable companions
live in `artifacts/release/artifact_family_map.yaml` and
`artifacts/release/promotion_gate_map.yaml`.

### Channel vocabulary and candidate stages

The exact-build channel vocabulary remains the closed set already frozen
in `schemas/build/exact_build_identity.schema.json`:

| Channel | Default owner posture | Minimum promotion truth | What the channel may still tolerate |
|---|---|---|---|
| `dev_local` | local builder only | no promotion; no public or support claim | dirty trees, unsigned outputs, incomplete sidecars |
| `nightly` | engineering plus QE/perf | traceable build identity, provenance link, rollback to previous nightly | incomplete docs, incomplete public proof, experimental flags |
| `preview` | product/release with honest narrowing | migration note, visible deltas, no compounding trust or data-loss regression | scoped instability and non-final public truth as long as it is explicit |
| `beta` | release with partner-facing support posture | blocker review cadence, upgrade guidance, support/export and rollback drills current | published workarounds, but no hidden trust or rollback gap |
| `stable` | release plus support | current release packet, compatibility report, known-limit set, support-window statement | only low-risk known issues outside protected claims |
| `lts` | release plus long-lived support posture | stable discipline plus proven backport, security-response, and support continuity | slower change rate in exchange for stronger support promises |
| `hotfix` | correction of a stable or LTS line | named rollback target, updated known-limit state, explicit backport scope, current response packet | only the narrow correction scope; no silent support-window or schema widening |

`rc_candidate` is frozen as a **review stage**, not as a new build
channel. A candidate may be an `rc_candidate` only while it is moving
from `beta` to `stable`, from `stable` to `lts`, or through a `hotfix`
review. No `exact_build_identity_record` may mint `rc` as a new channel
value. RC is a shiproom state over an existing channel-bound build set.

Promotion rule: no surface jumps directly from `nightly` to `stable`,
`lts`, or `hotfix` claim-bearing publication. It must first live long
enough in `preview` or `beta` for its real operational cost to become
visible.

### Rollback atom

Promotion and rollback operate on one **coordinated release family**,
not on a single binary. The rollback atom is the smallest set that can
truthfully restore the claimed release:

- the exact-build identity set for every artifact family in scope;
- the runtime payloads and any paired debug/source-map sidecars;
- docs/help, release-note, known-limit, and compatibility truth that
  describe those payloads;
- schema or reference exports shipped with the promoted set;
- SBOM, attestation, source, and reproducibility evidence tied to the
  same build identity set;
- the release-evidence packet and the install-topology rows that define
  the deployment envelope; and
- any active advisory, revocation, or correction metadata that changes
  installability or support truth on that line.

Rules:

1. Rollback never targets a binary alone. A rollback target is invalid
   if it cannot restore the coordinated release family for the claimed
   scope.
2. Partial rollback is allowed only when the public claim is already
   scoped to that smaller coordinated family. There is no hidden
   package-level rollback escape hatch for stable-facing claims.
3. Revocation is not rollback. Revocation may target a smaller affected
   node set; rollback always restores one last-known-good coordinated
   family.

### Artifact-family posture classes

Every exact-build `artifact_family_class` maps to one release posture
and one functional promotion owner lane:

| Release posture class | Families | Functional promotion owner lane | Governing expectation |
|---|---|---|---|
| `primary_payload` | `ide_binary`, `cli_binary`, `sdk_library` | `release_evidence` | runnable or directly consumed product payloads; these define install and rollback truth |
| `debug_retention_sidecar` | `ide_debug_symbols`, `cli_debug_symbols`, `source_map_bundle`, `crash_symbols_archive` | `support_export` | sidecars and supportability bytes that must stay tied to the paired release family |
| `public_truth_payload` | `docs_pack`, `reference_pack` | `docs_public_truth` | human-facing truth surfaces that must match claims, known limits, and release notes |
| `public_truth_payload` | `schema_export` | `governance_packets` | machine-readable contract exports that must move with the same promoted truth set |
| `supply_chain_proof` | `sbom_document`, `signed_attestation`, `source_bundle`, `reproducibility_pack` | `release_evidence` | provenance and rebuild proof that make promotion supportable and auditable |
| `supportability_payload` | `support_runbook_bundle` | `support_export` | support-facing operational truth that must stay aligned with exact-build and known-limit posture |
| `release_control_packet` | `release_evidence_packet` | `release_evidence` | aggregate packet that binds the rest of the promoted family into one shiproom decision surface |

The machine-readable family-level rules, including minimum candidate
stage, required gate refs, same-change-set groups, retention floor, and
advisory/revocation granularity, live in
`artifacts/release/artifact_family_map.yaml`.

### Exact-build identity propagation

Every release-bearing family that can resolve through an exact-build
identity does so. Version-only naming is forbidden once an
`exact_build_identity_ref` exists.

Rules:

1. A promoted row must be explainable by exact-build identity first and
   by human-readable version string second.
2. Docs/help version-match, support bundles, release evidence, crash
   symbolication, mirror/offline publication, and advisory scope all use
   the same exact-build identities; no packet may invent a second build
   handle.
3. If the exact-build identity meaning changes, the same change set must
   update the exact-build schema or narrative, the install-topology
   contract, the artifact-family map, the promotion-gate map, and any
   affected release-evidence or support/export truth.

### Symbol, source-map, and support-runbook retention

Debug and supportability sidecars are release-bearing truth, not
optional leftovers.

Rules:

1. If a runnable payload is stripped, ships split debug bytes, uses
   external source maps, or relies on a crash-symbol archive, the paired
   sidecar family is part of the same coordinated release family.
2. `ide_debug_symbols`, `cli_debug_symbols`, `source_map_bundle`, and
   `crash_symbols_archive` stay available for at least the support
   window of the paired release family and for any longer period needed
   to resolve an open advisory or correction line on that family.
3. `support_runbook_bundle` stays aligned with the same exact-build,
   known-limit, and release-note posture as the payload it supports. A
   support runbook may narrow further for redaction or audience, but it
   may not widen the claim.

### Mirror and offline publication parity

Hosted publication, customer-managed mirrors, and manual import or
air-gapped paths are different transports for the same release truth.
They are not separate release graphs.

Rules:

1. If a profile claims mirror, offline-bundle, or air-gapped support,
   the promoted set must preserve signatures, compatibility metadata,
   revocation state, and audit lineage across those paths.
2. Mirror or manual-import transport never waives exact-build identity,
   rollback-atom completeness, same-change-set parity, or advisory
   review. It changes transport only.
3. A mirror-only or manual-import emergency flow is allowed only when:
   the matching drill is current inside freshness SLO; the same
   exact-build identities and install-profile rows are named as the
   hosted path would name; the same claim rows, known limits, and
   release-note disclosures move in the same packet refresh; and the
   signed manual-import or mirror manifest is published with the same
   advisory or correction packet.
4. A hosted line that has fallen back to mirror-only or manual-import
   emergency publication is narrowed until hosted parity is restored and
   recorded in the next packet refresh.

### Same-change-set obligations

The repository-wide drift-blocking rules already freeze several
mandatory same-change-set cases. This ADR adds the release-facing
bundles that later tasks must treat as one change:

| Same-change-set group | Trigger | Minimum same-change-set updates |
|---|---|---|
| `exact_build_and_publication_bundle` | exact-build identity, release channel, install channel, provenance semantics, artifact-family posture, or rollback-atom meaning changes | exact-build schema or narrative, install-topology contract, artifact-family map, promotion-gate map, release-evidence packet template, and affected artifact-graph rules |
| `claim_docs_known_limit_bundle` | claim row, support window, migration posture, release note, known limit, exclusion note, docs/help version-match, or destination-route posture changes | claim-manifest projections, docs/help or release-note surfaces, support/export copy, and public-proof or evaluation surfaces that quote the row |
| `supportability_and_symbolication_bundle` | debug sidecar, source-map, crash-symbol, support-bundle, or support-runbook retention posture changes | exact-build sidecar links, support packet family refs, support-bundle or runbook contract updates, and any affected release or shiproom packet refs |
| `advisory_and_revocation_bundle` | advisory severity, revocation scope, emergency-disable scope, or mitigation guidance changes | advisory record, release note, known-limit or claim-row narrowing, shiproom/dashboard references, and mirror/manual-import guidance when claimed |
| `mirror_and_offline_bundle` | mirror continuity, offline import behavior, bundle expiration, or emergency transport posture changes | install-topology rows, release evidence, affected docs/help or support/export disclosures, and any manual-import or mirror manifests used by the response path |

No promotion-grade change is allowed to land as "binary now, truth
later" on any of those groups.

### Promotion vetoes

A successful build is necessary but not sufficient. An RC, stable, LTS,
or hotfix candidate is blocked when any of the following is red or
missing on the claimed scope:

- the coordinated exact-build identity set;
- the rollback target or rollback evidence for the coordinated release
  family;
- required docs/help, release-note, support-note, known-limit, or
  claim-row parity;
- required schema, reference, or support-runbook export parity;
- required symbol/source-map retention or exact-build symbolication
  proof;
- current clean-room or release-center parity proof on the exact row
  being widened;
- current mirror/offline, advisory, revocation, or emergency-disable
  drill on a profile that claims those paths;
- current support/export or recovery-ladder evidence on the release line
  being widened;
- an unresolved publication, security, or compliance waiver; or
- an unresolved same-change-set drift defect.

The binary may still be useful internally, but the promotion stays
`hold_for_refresh` or `no_go`; it does not quietly widen.

### Waiver classes and late-proof policy

Release waiver packets reuse the `scope_kind` vocabulary already frozen
in `schemas/release/waiver_packet.schema.json`.

Rules:

1. Active release waiver packets are admitted only on `preview` and
   `beta` candidate stages, and only as `go_with_narrowing`.
2. `rc_candidate`, `stable`, `lts`, and `hotfix` promotion admit **no
   active release waiver packet**. If a surface still needs a release
   waiver, the candidate stays in `preview` or `beta` and the claim
   remains narrowed there.
3. `publication_gap` and `security_or_compliance_gap` waivers never
   cross into stable-facing promotion. Those gaps are blocking, not
   deferrable.
4. The only late-proof exception is a preview-only, non-claim-widening
   proof attachment. It may not cover exact-build, rollback, security,
   mirror/offline, docs/help parity, or release-note parity. The
   candidate stays `hold_for_refresh` until the packet is current.
5. Non-release waivers, such as maintainer-coverage waivers owned by
   governance, do not authorize release-gate bypass. They stay visible,
   but they are not substitutes for release proof.

### Advisory and revocation granularity

Advisory and revocation scope stays as small as the facts allow.

Rules:

1. The default advisory scope is the tuple
   `(exact_build_identity_refs, install_profile_card_refs, claim_row_refs
   when a marketed row is affected)`.
2. The default revocation scope is the minimal affected node set inside
   the coordinated release family.
3. Scope widens from node set to channel manifest, mirror profile, or
   signing root only when the affected metadata or trust boundary makes
   the narrower scope non-credible.
4. Revocation must keep unaffected channels and unaffected artifact
   families installable whenever the narrower scope remains truthful.

The machine-readable scope defaults and escalation rules live in
`artifacts/release/promotion_gate_map.yaml`.

## Consequences

- Release promotion, rollback, support/export, docs/help truth, and
  claim-parity work now share one explicit release-posture contract
  instead of inferring it from adjacent packet templates.
- RC is no longer available as an implicit extra channel value. It is a
  review stage over an existing exact-build channel, which keeps the
  exact-build schema closed and prevents version-dialect drift.
- Debug sidecars, source maps, support runbooks, and mirror/manual
  import paths are now explicit release-bearing obligations rather than
  optional release engineering lore.
- Stable, LTS, and hotfix promotion become stricter: a binary build
  with stale proof, claim drift, or unresolved publication/security gap
  is still non-promotable.
- Later release-facing tasks can cite this ADR plus the two release maps
  instead of restating channel, waiver, rollback, and parity policy in
  local prose.

## Alternatives considered

- **Treat semver plus a binary build as the release truth.** Rejected.
  The repository already froze exact-build, claim-row, shiproom, and
  support/export contracts that require more than a version string.
- **Make RC a separate exact-build channel.** Rejected. RC is a review
  state over a candidate build, not a new long-lived publication
  channel, and minting a new channel would fork the exact-build
  vocabulary for little operational value.
- **Permit release waivers to flow into stable by default.** Rejected.
  That would turn stable proof gaps into steady-state release policy and
  undo the evidence-freshness and shiproom controls already frozen
  elsewhere in the repository.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:336` —
  "monthly stable releases" and "optional LTS lines".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4913` —
  release promotion should operate on an artifact graph.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4920` —
  debug symbols and source maps travel with the release family.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4928` —
  rollback metadata references the last-known-good artifact set.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4929` —
  mirrors and air-gap bundles preserve verification and audit lineage.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4931` —
  emergency revocation targets the minimal affected node set.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4313` —
  emergency-response artifacts travel through the same signed systems as
  normal releases.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4325` —
  manual-import path required for air-gapped or mirror-only
  environments.
- `.t2/docs/Aureline_Milestones_Document.md:4615` —
  channel-by-channel release governance posture.
- `.t2/docs/Aureline_Milestones_Document.md:4629` —
  active train evidence freshness and rerun cadence.
- `.t2/docs/Aureline_Milestones_Document.md:4657` —
  no emergency patch ships without rollback target and updated
  known-issue statement.
- `.t2/docs/Aureline_Milestones_Document.md:4698` —
  active RC windows require operational-readiness and incident drills.
- `.t2/docs/Aureline_Milestones_Document.md:5051` —
  shiproom checklist starts from exact-build identity and rollback.
- `.t2/docs/Aureline_Milestones_Document.md:5057` —
  docs, support notes, known limits, and release notes use the same
  claim vocabulary.
- `.t2/docs/Aureline_Milestones_Document.md:5098` —
  one build identity across binaries, docs packs, schemas, SBOMs,
  attestations, symbols, and rollback metadata.
- `.t2/docs/Aureline_Milestones_Document.md:5114` —
  advisory, revocation, and mirror/offline emergency paths drilled
  inside freshness SLO.
- `.t2/docs/Aureline_Milestones_Document.md:5189` —
  docs/help/migration truth parity.
- `.t2/docs/Aureline_Milestones_Document.md:5193` —
  exact-build release truth spans binaries, docs packs, symbols,
  support bundles, rollback metadata, and mirror/offline publications.
- `docs/governance/drift_blocking_rules.md:44` —
  mandatory same-change-set updates for claim, destination, and
  channel/provenance changes.
- `docs/governance/evidence_freshness_policy.md:89` —
  stale proof expires from metadata alone and propagates into promotion.

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0010`
- Artifact-family map: `artifacts/release/artifact_family_map.yaml`
- Promotion-gate map: `artifacts/release/promotion_gate_map.yaml`
- Release-artifact graph: `docs/release/release_artifact_graph.md`
- Release-evidence packet template: `docs/release/release_evidence_packet_template.md`
- Shiproom runbook: `docs/release/shiproom_runbook.md`
- Exact-build identity model: `docs/build/exact_build_identity_model.md`
- Clean-room rebuild lane: `docs/build/cleanroom_rebuild_lane.md`
- Help/About/service-health route contract: `docs/docs/help_about_service_health_routes.md`
- Public-surface truth map: `docs/governance/public_surface_truth_map.md`
- Support bundle contract: `docs/support/support_bundle_contract.md`

## Supersession history

None.
