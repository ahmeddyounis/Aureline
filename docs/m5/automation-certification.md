# Automation surface certification

The [automation contract baseline](automation-safety-labels.md) froze the
automation object families, the controlled safety-label vocabulary, and the reuse
rules. The [recipe builder](recipe-builder.md),
[parameter review](parameter-review-and-secret-references.md),
[dry-run / explain](dry-run-and-explain.md),
[run history](automation-run-history.md), and
[macro recorder](macro-recorder-and-replay.md) lanes then each proved one slice of
the safe-automation story. This lane closes the loop at the *claim* level: it turns
those frozen proofs into a claim-bearing **surface certification matrix** so each
claimed M5 automation surface can only present itself as *safe or shareable* when
its own current evidence proves builder parity, parameter review, dry-run/explain
coverage, run-history/evidence integrity, macro-scope safety, and label reuse.

The stable truth source is `AutomationCertificationPacket` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_automation_certification/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_automation_certification`.

## Claimed automation surfaces

Each claimed M5 automation surface carries one certification:

| Surface | Certifies |
| --- | --- |
| `notebook_automation` | notebook run / cell automation |
| `request_api_automation` | saved request / API automation |
| `package_automation` | dependency / package automation |
| `test_debug_automation` | task, test, and debug automation |
| `incident_automation` | incident response automation |
| `ai_linked_automation` | AI-linked / assistant-proposed automation |

A missing surface blocks stable (`missing_surface`), so the matrix cannot silently
shrink to the surfaces that still happen to pass.

## Six graded certification dimensions

Every surface is graded on the dimensions the docs require, and a surface is
`certified` only when **all** pass:

- `builder_parity` — the surface authors automation through the canonical
  declarative recipe builder (its `authoring_path` is `declarative_recipe_builder`)
  and cites at least one upstream builder proof, rather than an
  `ad_hoc_feature_dialog`, `hidden_command_metadata`, or an `unreviewed_free_text`
  path.
- `parameter_review` — inputs route through a typed parameter-review sheet
  (`parameters_reviewed`) with safe secret-reference handling
  (`secret_references_safe`).
- `dry_run_explain_coverage` — a dry-run/explain preview is shown before apply
  (`side_effect_preview_shown`) and discloses the predicted writes, process,
  network, and remote effects (`predicted_effects_disclosed`).
- `run_history_integrity` — durable run history is recorded (`run_history_durable`)
  with retention/redaction (`run_history_redaction_safe`) and a
  rerun-under-current-policy resolution (`rerun_under_current_policy`).
- `macro_scope_safety` — recorded macros declare their target scope
  (`macro_scope_declared`) and fail closed on a context, scope, or
  supported-command mismatch (`macro_fails_closed_on_mismatch`).
- `label_reuse` — the surface reuses the controlled safety-label vocabulary
  (`reuses_controlled_labels`) rather than minting surface-local synonyms.

A surface that fails any dimension emits a precise blocker finding
(`ad_hoc_authoring`, `missing_builder_evidence`, `parameter_review_missing`,
`side_effect_preview_missing`, `run_history_integrity_missing`,
`macro_scope_unsafe`, `label_reuse_broken`) and blocks stable. This is how the lane
blocks or narrows any surface that still lacks reviewed input, side-effect preview,
scope-safe macro behavior, or durable automation evidence.

## Each surface cites machine-readable evidence

Every surface draws its proof from the checked-in upstream automation packets — the
declarative recipe builder, the typed parameter-review sheets, the dry-run/explain
previews, the run-history / evidence panel, the macro recorder, the cross-surface
label-parity proof, and the automation contract baseline that froze the vocabulary
they all reuse. A surface that cites no evidence blocks stable
(`missing_evidence_ref`), so a claim can never rest on recipe-pack presence or macro
promotion alone.

## A shareable claim needs full current proof

A surface may only present itself as safe or shareable when its current evidence
proves every dimension. A surface that `presents_as_shareable` but is not certified
emits a blocker (`shareable_claim_unproven`); a shareable surface whose proof has
aged out emits a warning (`shareable_claim_narrowed`). This is the track invariant
the release lane relies on so no automation claim ships on partial proof.

## Freshness narrows aged proof

Every surface carries a recorded proof age and a freshness window. A surface whose
proof has aged past its window emits a **warning** (`surface_evidence_stale`), its
claim state becomes `narrowed_below_stable`, and the packet **narrows below stable**
rather than blocking — but it cannot stay shareable. This is the stale-evidence
narrowing the release lane relies on so an automation claim cannot coast on aged
proof.

## Certification index

The derived `certification_index` is the one canonical automation-evidence index
release, support, AI, and docs/help surfaces ingest. It names which surfaces are
`shareable` (current and certified), which have `narrowed` below stable on aged
proof, and which are `blocked`, and records whether every surface is current and
certified. Release and public-truth tooling ingests this index to narrow or block
underqualified automation claims automatically instead of re-deriving surface
maturity by hand.

## Stability rules

- All six automation surfaces must be present exactly once.
- Every surface must author through the declarative recipe builder and cite at
  least one upstream automation proof.
- Every surface must certify across all six dimensions.
- A surface presenting as shareable without full current proof is itself a finding.
- A stale surface narrows below stable (warning); a non-certified surface blocks
  stable (blocker).
- The stored per-surface dimension outcomes, freshness/claim states, the surface
  digest, and the certification index must all match the derivation; any drift
  blocks stable.
- A packet with any blocker finding is `blocks_stable`; a packet with only warnings
  is `narrowed_below_stable`; otherwise it is `stable`.

## Companion artifacts

- `schemas/automation/m5-automation-certification.schema.json` — boundary schema
  for the packet, its support export, its evidence joins, and the CLI/headless view.
- `schemas/automation/automation-contract-baseline.schema.json` — the contract
  baseline whose safety-label, promotion-state, and finding-severity vocabulary this
  lane reuses.
- `artifacts/m5/automation/automation-certification/` — the checked-in packet,
  support export, AI evidence join, incident packet join, CLI/headless view, and
  compact rendering.
- `fixtures/automation/m5/automation-certification/` — the baseline and the blocking
  / narrowing mutation cases the typed consumer and the gate replay.
- `tools/ci/m5/automation_certification_check.py` — the fail-closed gate.

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_automation_certification` enforces the
same structural invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.
