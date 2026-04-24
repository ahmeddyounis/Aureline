# Public-proof coverage audit

One canonical report that describes the mechanical joins CI runs to make
it hard for public truth to exist without a requirement, evidence, or a
caveat. The check is implemented by
`tools/ci/validate_public_proof_coverage.py`, wrapped for CI by
`ci/check_public_proof_coverage.sh`, and its JSON output at
`target/public-proof-coverage/public_proof_coverage_report.json` is the
canonical input signoff and shiproom tasks consume.

Reviewers should be able to read the human stdout summary and see
exactly why a claim row is incomplete without opening each upstream
artifact manually.

## What the audit joins

The validator loads every artifact on the left and joins it through the
keys on the right. Every join is additive-minor — adding a row must land
here in the same change, so reviewers always see the current picture.

| Artifact | Role | Join keys |
| --- | --- | --- |
| `artifacts/governance/requirement_register_seed.yaml` | Canonical requirement ids | `requirement_rows[].requirement_id` |
| `artifacts/governance/claim_manifest_seed.yaml` | Public-truth claim rows + channel bindings | `claim_rows[].claim_row_id`, `requirement_ids`, `launch_bundle_refs`, `exact_build_identity_refs`, `known_limit_refs`, `channel_bindings[].minimum_version_match_state` |
| `artifacts/release/assurance_claim_rows.yaml` | Assurance claim matrix (per contract family) | `assurance_claims[].claim_id`, `claim_subject_family`, `effective_claim_class`, `known_limit_refs`, `exclusion_refs`, `docs_version_match`, `publication_destinations`, `workflow_bundle_refs` |
| `artifacts/qe/workflow_bundle_ids.yaml` | Workflow-bundle and scoreboard register | `workflow_bundles[].bundle_id`, `cutline_refs`, `cutline_scoreboard_pairings[].cutline_ref`, `scoreboard_family_vocabulary`, `public_proof_packet_shape_vocabulary` |
| `fixtures/qe/public_proof_packets/*.json` | Public-proof packet fixtures | `packet_id`, `workflow_bundle_ref.bundle_id`, `archetype_row_ref.archetype_row_id`, `cutline_ref`, `scoreboard_family_id`, `packet_shape`, `environment_ref.exact_build_identity_ref`, `result_class`, `active_downgrade_reasons`, `docs_help_version_match.state` |
| `artifacts/product/known_limit_classes.yaml` | Known-limit / exclusion vocabulary | `schema_ref` |
| `artifacts/docs/destination_descriptor_seed.yaml` | Docs destinations and disclosure-safety axes | `boundary_schema`, `required_product_fields` |
| `fixtures/build/exact_build_examples/*.json` | Exact-build identity fixtures | `exact_build_identity_ref` |

The nine contract families the spec names are each bound to one or more
`assurance_claim_row` subjects. The map is the single source of truth
the `contract_family_coverage` check reads:

| Contract family | Admissible `claim_subject_family` |
| --- | --- |
| language-provider truth | `provider_aware_language_intelligence` |
| execution surfaces | `replay_safe_execution_history`, `trustworthy_diagnostics_and_quick_fixes` |
| Git / review / history-edit | `provider_integrated_review`, `replay_safe_execution_history` |
| portability | `export_and_offboarding_support`, `theme_package_portability` |
| localization / theme assets | `localization_readiness`, `theme_package_portability` |
| onboarding / voice | `voice_privacy` |
| repair | `repair_rollback_safety` |
| SDK publication | `regulated_environment_assurance` |
| hosted-review state | `provider_integrated_review`, `regulated_environment_assurance` |

## Failure classifications

The audit emits three severities. Signoff and shiproom reviewers read
the JSON report, group findings by `severity`, and apply the rules
below.

### Blocking (`severity = "error"`)

These findings block promotion. The public claim cannot stand until the
citation is fixed at source.

- `public_proof_coverage.orphan_claim_row_no_requirement` — a claim row
  in `claim_manifest_seed.yaml` declares zero `requirement_ids`. The
  row has no upstream obligation and must either cite one or be
  retired.
- `public_proof_coverage.orphan_claim_row_unresolved_requirement` — a
  claim row cites a requirement id that does not resolve to
  `requirement_register_seed.yaml`. Either land the requirement row or
  fix the citation.
- `public_proof_coverage.claim_missing_downgrade_policy_known_limit` —
  a claim row's `downgrade_policy.required_known_limit_refs` lists a
  caveat the row itself does not carry. The downgrade cannot project
  truthfully.
- `public_proof_coverage.claim_narrowed_without_known_limit` — a claim
  row projects as `limited`, `experimental`, or `replacement_grade` but
  lists no `known_limit_refs`. Narrowed claims must disclose why they
  narrow.
- `public_proof_coverage.assurance_row_known_limit_floor` — an
  assurance row's effective class (for example `exception_recorded`)
  requires at least one known-limit ref and the row carries none.
- `public_proof_coverage.assurance_row_exclusion_floor` — an assurance
  row's effective class (for example `not_claimed`) requires at least
  one exclusion ref and the row carries none.
- `public_proof_coverage.assurance_row_missing_docs_version_match` —
  an assurance row does not declare a docs/help version-match floor;
  docs drift would be silent.
- `public_proof_coverage.assurance_row_required_evidence_missing` — an
  assurance row cites a required evidence path that does not exist.
- `public_proof_coverage.assurance_row_known_limit_ref_missing` — an
  assurance row cites a known-limit path that does not exist.
- `public_proof_coverage.contract_family_uncovered` — one of the nine
  named contract families has no backing `assurance_claim_row`. Public
  truth for that family would have no home.
- `public_proof_coverage.packet_missing_bundle_id`,
  `public_proof_coverage.packet_unresolved_bundle_id` — a public-proof
  packet without a bundle id or with an unregistered bundle id. Every
  packet must trace to an entry in
  `artifacts/qe/workflow_bundle_ids.yaml`.
- `public_proof_coverage.packet_missing_cutline_ref`,
  `public_proof_coverage.packet_cutline_not_paired` — a packet without
  a cutline ref or with a cutline that has no
  `cutline_scoreboard_pairings` row. The launch-wedge lineage cannot be
  verified.
- `public_proof_coverage.packet_missing_archetype_row`,
  `public_proof_coverage.packet_missing_scoreboard_family`,
  `public_proof_coverage.packet_unresolved_scoreboard_family`,
  `public_proof_coverage.packet_unresolved_packet_shape` — a packet
  that cannot be pinned to the archetype / scoreboard / shape
  vocabulary that shiproom reviews read.
- `public_proof_coverage.packet_missing_exact_build_identity` — a
  packet that does not cite an `exact_build_identity_ref`. Every claim
  must trace to a reproducible build.
- `public_proof_coverage.packet_narrow_result_without_reason` — a
  `narrow_claim_before_publish` / `retest_pending` / `fail_claim_blocked`
  / `quarantined` packet with no `active_downgrade_reasons` fails the
  schema invariant.
- `public_proof_coverage.packet_full_proof_with_reasons` — a
  `pass_full_proof` packet declaring downgrade reasons fails the
  schema invariant.
- `public_proof_coverage.packet_not_valid_json`,
  `public_proof_coverage.exact_build_fixture_not_valid_json` — a
  fixture that the validator cannot read.
- `public_proof_coverage.exact_build_fixture_missing_identity` — an
  exact-build fixture without `exact_build_identity_ref`.
- `public_proof_coverage.destination_boundary_schema_missing`,
  `public_proof_coverage.destination_required_field_malformed`,
  `public_proof_coverage.known_limit_schema_missing` — governance
  registers that point at missing schema files or declare malformed
  required-field lists.

### Narrowing (`severity = "warning"`)

These findings trigger narrowing of the claim wording but do not block
the promotion. They ride the release evidence as a caveat until they
close.

- `public_proof_coverage.docs_version_state_without_exact_build` — a
  claim row channel binding sets `minimum_version_match_state` to
  `exact_build_match` or `compatible_minor_drift` but the row lists no
  `exact_build_identity_refs`. Narrow the channel's posture or cite a
  build identity.
- `public_proof_coverage.packet_docs_version_mismatch` — a
  `pass_full_proof` packet declares a `docs_help_version_match.state`
  outside `exact_build_match` / `compatible_minor_drift`. The
  publication must narrow the result class or raise the match state.
- `public_proof_coverage.exact_build_ref_without_fixture` — a claim
  row cites a non-seed/non-placeholder exact-build identity that no
  fixture anchors. Either land the fixture or retire the reference.
- `public_proof_coverage.family_expects_public_proof_no_fixture` — an
  assurance row publishes to `public_proof_packet` but the repository
  carries no public-proof packet fixture of the matching family yet.
- `public_proof_coverage.evidence_without_claim_binding` — a
  public-proof packet cites a workflow bundle and cutline that no
  claim or assurance row binds. Either widen a claim row's
  `launch_bundle_refs` / `workflow_bundle_refs`, or retire the orphan
  packet.

### M0 disclosure (`severity = "warning"` that we accept pre-launch)

At M0, some warnings are expected because live product parity has not
landed. Reviewers treat these as disclosures, not blockers:

- `evidence_without_claim_binding` findings on fixtures whose bundle is
  already seeded but whose claim-row `launch_bundle_refs` entry remains
  pending for a later milestone. The warning stays in the signoff packet
  so the gap is visible.
- `family_expects_public_proof_no_fixture` findings on families that
  land their first fixture only after the foundations milestone.

Any warning that persists past M0 must be explicitly waived through the
decision register (`artifacts/governance/decision_index.yaml`) before
the claim widens.

## Running the audit

```
./ci/check_public_proof_coverage.sh --out-dir target/public-proof-coverage
```

Outputs:

- stdout: human summary identical to the JSON report
- `target/public-proof-coverage/public_proof_coverage_report.json` —
  canonical machine-readable report the signoff and shiproom packets
  consume
- `target/public-proof-coverage/public_proof_coverage_summary.txt` —
  captured stdout for CI log reuse

Signoff and shiproom tasks MUST consume the JSON report. They read
`summary.error_count`, the per-check `status`, the
`coverage.contract_families_uncovered` list, and the
`coverage.public_proof_packets.by_result_class` counters to quote the
release posture without re-deriving it locally.

## Reviewer workflow

1. Run `./ci/check_public_proof_coverage.sh`. Stdout names every
   finding with its owner artifact ref and remediation hint.
2. Sort errors first; each one names the artifact file and the row id
   to edit. Fix at source rather than muting the check.
3. Group warnings. If a warning must ride the release, quote the
   remediation text in the release-evidence packet and cite the
   finding's `check_id` so the caveat is machine-linkable.
4. Repeat until the error count is zero. CI enforces this; local runs
   match CI exactly (same Python / Ruby toolchain, same inputs).

## Adding a new contract family

1. Add an `assurance_claim_row` in `artifacts/release/assurance_claim_rows.yaml`
   with the new `claim_subject_family`.
2. Extend `CONTRACT_FAMILIES` in
   `tools/ci/validate_public_proof_coverage.py` so the audit recognizes
   the family.
3. Update the "What the audit joins" table here in the same change.
4. Re-run `./ci/check_public_proof_coverage.sh` and commit.

## Out of scope

- Validating internal-only evidence families (the audit only joins the
  public-truth surfaces listed above).
- Live docs content validation; docs-version mismatches are caught
  through the declared `docs_help_version_match` envelope, not by
  rendering docs.
- Rewriting claim wording. The audit only confirms that the citations
  are sound; it does not judge narrative fidelity.
