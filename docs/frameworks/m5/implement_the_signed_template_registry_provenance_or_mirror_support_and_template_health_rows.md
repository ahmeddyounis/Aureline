# Signed template registry, provenance/mirror support, and template-health rows

This contract describes the export-safe packet that carries the **signed
template registry**: the set of template revisions the gallery may offer, each
row annotated with its provenance and mirror lineage, signing trust source and
signature class, certification and support class, declared freshness, and
template-health state. The packet is the canonical truth that the gallery,
scaffold preflight, run and recovery surfaces, diagnostics, and support exports
ingest instead of re-describing template trust or health by hand.

- Boundary schema:
  `schemas/templates/implement-the-signed-template-registry-provenance-or-mirror-support-and-template-health-rows.schema.json`
- Implementation:
  `crates/aureline-templates/src/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/`
- Checked support export:
  `artifacts/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/support_export.json`
- Fixtures:
  `fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/`

This packet **projects** the per-entry
`schemas/templates/template_registry_entry.schema.json` contract (frozen in
`docs/templates/template_registry_and_scaffold_contract.md`). It reuses that
contract's closed vocabularies — origin, trust source, signature, certification,
support, freshness, health cadence, and health state — rather than inventing
parallel terms, and references each full entry by an opaque `registry_entry_ref`
instead of embedding it.

## Boundary discipline

The packet is metadata only. Raw signing keys, certificate material, repository
URLs, absolute paths, manifest bodies, hook bodies, secrets, and user-authored
template content never cross this boundary. Rows carry opaque refs,
closed-vocabulary class tokens, content digests by reference, and short
reviewable summaries. `validate` rejects any export that leaks obviously
forbidden material.

## Row truth

Each `registry_row` binds one offerable template revision to:

- **Provenance and mirror lineage** — `origin_class`, plus
  `mirrored_from_origin_class` and `mirror_freshness_ref` for mirror and offline
  bundle rows so the upstream origin and mirror staleness stay inspectable and
  trust is never inferred from a mirror's location.
- **Trust and signature** — `trust_source_class`, `trust_root_ref`,
  `signature_class`. Official rows resolve through the core signing root; org
  mirrors through organization roots; repo-local and ad hoc rows through local
  trust only and cannot claim certification by location.
- **Certification and support** — `certification_class`, `support_class`.
- **Freshness** — `declared_freshness_class`.
- **Template health** — `health_state_class` (partitioned, not a single
  pass/fail bit), `health_cadence_class`, `health_check_refs`, and
  `known_issue_refs` disclosed before generation. A blocking health state
  (`*_blocks_starter`, `health_unknown_review_required`) forces
  `admitted_for_generation` to `false`.
- **Downgrade and projection** — `downgrade_triggers` and `consumer_surfaces`.

## Downgrade automation

`apply_downgrade_automation` narrows rows from observed runtime signals so a
stale or underqualified row narrows before it is offered, instead of being
hidden:

- A failed signature or unresolved trust root marks the row
  `signature_or_trust_failed_blocks_starter`, drops certification to
  `certification_unknown_review_required` and support to `support_unknown`, and
  withdraws admission.
- A stale mirror sets `mirror_stale`, marks the row stale-but-inspectable, and
  withdraws admission until refresh or review.
- Aged-out health checks narrow a healthy row to `stale_but_inspectable`.
- Stale proof or a narrowed upstream withdraws admission.

A narrowed row stays a valid, export-safe packet, so the gallery and support
surfaces show a current, labeled state rather than an optimistic placeholder.

## Consumers

`current_signed_template_registry_export()` reads and validates the checked
support export. It is the first real consumer: a gallery, preflight,
diagnostics, or support-export surface ingests the canonical packet through it.
The two checked fixtures (`health_stale_narrowed.json`,
`signature_failed_blocked.json`) are valid, narrowed packets that exercise the
downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_signed_template_registry -- canonical
cargo run -p aureline-templates --example dump_signed_template_registry -- markdown
cargo run -p aureline-templates --example dump_signed_template_registry -- health_stale
cargo run -p aureline-templates --example dump_signed_template_registry -- signature_failed
```
