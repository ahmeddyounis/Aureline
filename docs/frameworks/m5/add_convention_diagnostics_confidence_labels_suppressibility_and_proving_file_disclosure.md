# Convention diagnostics with confidence labels, suppressibility, and proving-file disclosure

This contract describes the export-safe packet that carries the **framework-pack
convention-diagnostic** truth for the editor diagnostics, problems panel,
diff-review, run, diagnostics, and support surfaces: the confidence label each
diagnostic is presented with, how analysis-fresh it is, whether and how it may be
suppressed, which proving file or manifest grounds it, its support class, and its
downgrade banner. The packet is the canonical truth those surfaces ingest instead
of re-describing diagnostics by hand or presenting a heuristic, bridged, or
ungrounded convention as exact first-party truth.

- Boundary schema:
  `schemas/templates/add-convention-diagnostics-confidence-labels-suppressibility-and-proving-file-disclosure.schema.json`
- Implementation:
  `crates/aureline-templates/src/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/`
- Checked support export:
  `artifacts/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/support_export.json`
- Fixtures:
  `fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/`

This packet **references** the upstream template-manifest and framework-pack
records — the `template_manifest_alpha` contract and the framework-pack header
packet frozen in `docs/templates/template_registry_and_scaffold_contract.md` and
the framework-pack lane — by opaque ref (`app_id`, `framework_pack_ref`,
`diagnostic_id`) rather than embedding them, and reuses the prior support-class
and downgrade vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw source bodies, raw manifests, repository URLs,
hostnames, secrets, and user-authored content never cross this boundary. Rows
carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
structural locators (`convention_locator`, `proving_file_refs`), and export-safe
chip labels. `validate` rejects any export that leaks obviously forbidden
material.

## Row truth

Each `diagnostic_row` binds one convention diagnostic to:

- **Kind and message** — `diagnostic_kind` (`naming_convention`,
  `file_location`, `required_registration`, `config_convention`, or
  `api_usage_convention`), `message_summary`, `severity`, and
  `convention_locator`.
- **Confidence label** — `confidence_label` (`exact`, `high`, `heuristic`, `low`,
  or `confidence_unknown`) and `confidence_summary`. Every diagnostic shows a
  confidence label. A non-confident label (`heuristic`, `low`, or
  `confidence_unknown`) must show a confidence or downgrade banner; a confident
  label (`exact` or `high`) must disclose a proving file or manifest.
- **Freshness chip** — `freshness_class` (`fresh`, `rescan_available`, `aging`,
  `stale`, or `freshness_unknown`), `freshness_chip_label`, and `last_analyzed`.
  A `stale` or `freshness_unknown` chip must show a downgrade banner.
- **Suppressibility** — `suppression_class` (`suppressible`, `not_suppressible`,
  `suppressed_by_user`, `suppressed_by_scope`, or `suppression_unknown`) and
  `suppression_summary`. Suppressibility is disclosed for every diagnostic. A
  suppressed diagnostic is withdrawn from active display, carries the
  `suppression_applied` downgrade trigger, and stays present in the packet —
  labeled as suppressed, never silently hidden.
- **Proving-file disclosure** — `proving_disclosure_class`
  (`proving_file_disclosed`, `proving_manifest_disclosed`, `no_proving_file_needed`,
  or `proving_file_unavailable`), `proving_file_refs`, and `proving_summary`. A
  grounded disclosure carries at least one proving-file ref. A
  `proving_file_unavailable` diagnostic must narrow its confidence to
  `confidence_unknown`, raise the proving-file banner, and is blocked from any
  confident claim.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled.
  A `bridge_behavior` or `heuristic_mapping` diagnostic must disclose a known
  issue, carry the matching `bridge_behavior_disclosed` /
  `heuristic_confidence_disclosed` downgrade trigger, and show a banner, so an
  inferred or bridged convention is never presented as exact first-party truth.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`,
  `freshness_banner`, `confidence_banner`, `support_class_banner`,
  `proving_file_unavailable_banner`, or `policy_block_banner`) makes the narrowing
  cue explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_display`. A blocked diagnostic (stale or unknown freshness,
  unknown confidence, unavailable proving file, or a hard-block banner) can never
  be admitted for confident display.

## Downgrade automation

`apply_downgrade_automation` narrows diagnostics from observed runtime signals so
a stale or underqualified diagnostic narrows before it is presented, instead of
being hidden or presented as exact truth:

- **An unavailable proving file** marks the proving disclosure unavailable,
  narrows the confidence to unknown, clears the proving-file refs, raises the
  proving-file banner, and withdraws display.
- **An unverified confidence** narrows the confidence to unknown, raises a
  confidence banner, and withdraws display.
- **A stale analysis** narrows freshness to `stale`, raises a freshness banner,
  and withdraws display.
- **A newly active suppression** labels the diagnostic suppressed, carries the
  suppression-applied trigger, and withdraws active display.
- **Stale proof or a narrowed upstream** withdraws display.

A raised banner is never lowered to a softer cue, and a narrowed diagnostic stays
a valid, export-safe packet, so the diagnostics and support surfaces show a
current, labeled state rather than an optimistic placeholder.

## Consumers

`current_convention_diagnostic_export()` reads and validates the checked support
export. It is the first real consumer: an editor diagnostics, problems-panel,
run, diagnostics, or support-export surface ingests the canonical packet through
it. The two checked fixtures (`proving_file_unavailable_blocked.json`,
`confidence_unverified_withheld.json`) are valid, narrowed packets that exercise
the downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_convention_diagnostics -- canonical
cargo run -p aureline-templates --example dump_convention_diagnostics -- markdown
cargo run -p aureline-templates --example dump_convention_diagnostics -- proving_file_unavailable
cargo run -p aureline-templates --example dump_convention_diagnostics -- confidence_unverified
```
