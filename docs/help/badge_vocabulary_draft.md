# Help / About / service-health / docs-browser badge vocabulary draft

This page is the **consumer-facing** draft of the closed badge vocabulary
the M1 docs/help/About/service-health/docs-browser surfaces project.
It is a named runtime consumer of
[`/artifacts/help/m1_truth_source_examples.yaml`](../../artifacts/help/m1_truth_source_examples.yaml)
under the boundary schema
[`/schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json);
it never mints private tokens or surface-local synonyms.

The four named surfaces and the support / export copy that quotes
them MUST render the rows below verbatim. The reviewer-facing model
that explains why each row is closed lives at
[`/docs/help/truth_source_model.md`](truth_source_model.md).

## Vocabulary role tokens

Every token on every row is one of:

| Role | Meaning |
|---|---|
| `live_state_token` | Healthy state. Renders without a honesty marker. |
| `degraded_state_token` | Degraded but typed. Renders with a honesty marker; the row is **not** allowed to collapse to a live state. |
| `seed_placeholder_token` | Reserved seed placeholder awaiting wiring. Renders with a typed "seed placeholder" cue; the chrome MUST NOT promote it to a live state. |
| `honesty_fallback_token` | Renders when upstream truth is missing or unverified. Every row carries exactly one. The surface MUST keep the row visible rather than blanking it out. |

## Docs/help source class

Token | Role | Label
--- | --- | ---
`project_docs` | `live_state_token` | Project docs (this build's authoritative pack)
`generated_reference` | `live_state_token` | Generated reference
`mirrored_official_docs` | `live_state_token` | Mirrored official docs
`curated_knowledge_pack` | `live_state_token` | Curated knowledge pack
`derived_explanation` | `live_state_token` | Derived explanation (not authoritative)
`vendor_provider_docs` | `live_state_token` | Vendor / provider docs
`support_runbook` | `live_state_token` | Support runbook
`external_status_feed` | `live_state_token` | External status feed
`unknown_source` | `honesty_fallback_token` | Unknown source (no source-of-truth disclosed)

Honesty fallback: `unknown_source`. A surface that has no upstream
source-truth block renders this row verbatim rather than blanking out.

## Docs/help version-match state

Token | Role | Label
--- | --- | ---
`exact_build_match` | `live_state_token` | Exact build match
`compatible_minor_drift` | `live_state_token` | Compatible (minor drift)
`incompatible_drift_detected` | `degraded_state_token` | Incompatible drift detected
`pre_release_unverified` | `degraded_state_token` | Pre-release (unverified)
`unknown_target_build` | `honesty_fallback_token` | Unknown target build

Honesty fallback: `unknown_target_build`. Drift and unknown states MUST
NOT collapse to `exact_build_match`.

## Docs/help freshness class

Token | Role | Label
--- | --- | ---
`authoritative_live` | `live_state_token` | Authoritative (live)
`warm_cached` | `live_state_token` | Warm cached
`degraded_cached` | `degraded_state_token` | Degraded cached
`stale` | `degraded_state_token` | Stale
`unverified` | `honesty_fallback_token` | Unverified

Honesty fallback: `unverified`. Degraded states stay paired with the
degraded role; the chrome MUST surface a honesty marker on any row
whose token is `degraded_cached`, `stale`, or `unverified`.

## Client-scope badge family

Token | Role | Label
--- | --- | ---
`local_desktop` | `live_state_token` | Local desktop
`remote_host` | `live_state_token` | Remote host
`managed_workspace` | `live_state_token` | Managed workspace
`local_to_remote` | `live_state_token` | Local desktop to remote host
`local_to_managed` | `live_state_token` | Local desktop to managed plane
`account_free_local` | `live_state_token` | Account-free local
`self_hosted_org` | `live_state_token` | Self-hosted organisation
`trusted` | `live_state_token` | Trusted
`restricted` | `degraded_state_token` | Restricted
`degraded_trust` | `honesty_fallback_token` | Trust pending — review before action

Honesty fallback: `degraded_trust`. The client-scope row degrades to
this token when an execution context is missing, pending evaluation,
or carries a degraded field.

## Install-mode class

Token | Role | Label
--- | --- | ---
`dev_local_built_from_source` | `live_state_token` | Dev (built from source)
`nightly_local_install` | `live_state_token` | Nightly install
`preview_local_install` | `live_state_token` | Preview install
`beta_local_install` | `live_state_token` | Beta install
`stable_local_install` | `live_state_token` | Stable install
`lts_local_install` | `live_state_token` | LTS install
`hotfix_local_install` | `live_state_token` | Hotfix install
`unknown_install_mode` | `honesty_fallback_token` | Unknown install mode

Honesty fallback: `unknown_install_mode`. An unrecognised
release-channel-class token MUST render as `unknown_install_mode` and
light a visible honesty chip rather than silently rendering as
`stable_local_install`.

## Provenance row state

Token | Role | Label
--- | --- | ---
`seed_placeholder_awaiting_wiring` | `seed_placeholder_token` | Seed placeholder (wiring pending)
`signed_verified` | `live_state_token` | Signed and verified
`attestation_verified` | `live_state_token` | Attestation verified
`checksum_verified` | `live_state_token` | Checksum verified
`sbom_attached_verified` | `live_state_token` | SBOM attached and verified
`no_open_advisories` | `live_state_token` | No open advisories
`not_verified_this_seed` | `honesty_fallback_token` | Not verified by this seed

Honesty fallback: `not_verified_this_seed`. The M1 seed renders every
row with `seed_placeholder_awaiting_wiring`; live state tokens are
reserved for the milestone that lands the verifier.

## Service-health state

Token | Role | Label
--- | --- | ---
`seed_placeholder_awaiting_wiring` | `seed_placeholder_token` | Seed placeholder (wiring pending)
`healthy` | `live_state_token` | Healthy
`degraded` | `degraded_state_token` | Degraded
`unavailable` | `degraded_state_token` | Unavailable
`stale_snapshot` | `honesty_fallback_token` | Stale snapshot

Honesty fallback: `stale_snapshot`. The M1 seed renders every row with
`seed_placeholder_awaiting_wiring`; live state tokens are reserved for
the milestone that lands the aggregator.

## Consuming-surface parity

Every row in the seed binds these four surfaces on
`consuming_surface_classes`; surface-local copy on any of them is
forbidden:

- `help_pane`
- `about_pane`
- `service_health_pane`
- `docs_browser_pane`

`support_export_card` also appears on every row so support / export
copy is parity-consistent with the live surfaces. The seed's
`support_export_compatible` field is the structural constant `true` on
every row.

## Authoring rule

When a new badge family is added, the change MUST:

1. Add the family to `badge_family_class_vocabulary` and
   `required_badge_family_class_coverage` in
   [`/schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json).
2. Add a row to
   [`/artifacts/help/m1_truth_source_examples.yaml`](../../artifacts/help/m1_truth_source_examples.yaml)
   with a closed `vocabulary_tokens` list, an `honesty_fallback_token`
   present in the list, all four required surfaces on
   `consuming_surface_classes`, one named runtime consumer, one
   example payload under `fixtures/help/m1_truth_source_examples/`,
   and one named `failure_drill`.
3. Add the new row's vocabulary to this draft so docs / support /
   export copy reads the same tokens.
4. Add the new failure drill to `failure_drill_id_vocabulary`.

Repurposing an existing token, surface, role, or honesty fallback is
breaking and requires a new decision row plus a superseding seed.
