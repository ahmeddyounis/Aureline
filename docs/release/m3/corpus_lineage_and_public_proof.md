# Corpus Lineage and Public Proof

Beta benchmark, compatibility, migration, conformance, and supportability
claims now resolve through one governed corpus registry:
`fixtures/registry/corpus_registry.yaml`.

The registry records corpus owners, source class, privacy and licensing
posture, refresh cadence, intended claim impact, and change history. The
freshness gate derives whether each claim binding is still current or has
aged into `retest_pending`.

## Governed Assets

| Proof lane | Corpus assets | Evidence packet | Current freshness source |
|---|---|---|---|
| Benchmark publication | `corpus.asset.benchmark.beta_publication_pack` | `artifacts/benchmarks/m3/publication_packet/packet.md` | `corpus_claim_binding.benchmark_publication` |
| Reference workspaces | `corpus.asset.reference.*` | `artifacts/compat/m3/reference_workspace_report.json` | `corpus_claim_binding.reference_workspaces` |
| Migration suite | `corpus.asset.migration.top_incumbent_switching` | `artifacts/migration/m3/migration_scoreboard.md` | `corpus_claim_binding.migration_suite` |
| Extension conformance | `corpus.asset.conformance.extension_kit_beta` | `artifacts/compat/m3/extension_conformance_kit_report.json` | `corpus_claim_binding.extension_conformance` |
| Support scenarios | `corpus.asset.support.m3_scenario_corpus` | `artifacts/support/m3/drill_harness_report.md` | `corpus_claim_binding.support_scenarios` |

Partner, customer, or field-derived samples are represented only by the
blocked holding asset `corpus.asset.external.partner_intake_holding_area`
until the intake checklist is complete. That asset is not referenced by
any claim binding and is not admitted to CI or public proof.

## Freshness Automation

Run:

```sh
python3 ci/check_corpus_freshness.py --repo-root .
```

The generated report at `artifacts/registry/corpus_freshness_report.json`
contains one derived row per claim binding with:

- corpus asset refs
- evidence packet ref
- evidence capture time
- stale window
- current freshness state
- effective claim state

When evidence ages past its window, the effective claim state becomes
`retest_pending` and release packets, compatibility reports, support
exports, partner packets, and public-proof rows must not keep green or
certified wording.

## Release Packet Requirements

Any release-evidence packet that cites a benchmark, compatibility,
migration, conformance, or supportability claim must include:

- `fixtures/registry/corpus_registry.yaml`
- `fixtures/registry/evidence_freshness_policy.yaml`
- `artifacts/registry/corpus_freshness_report.json`
- the relevant `corpus_claim_binding.*` id
- the evidence packet ref and `evidence_captured_at` timestamp

If a new corpus or fixture changes a claim boundary, update the registry,
refresh this report, and rerun the downstream packet validator in the
same change set.
