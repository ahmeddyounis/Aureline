# Notebook Round-Trip Fixtures, Heavy-Output Corpora, and Canonical Notebook Support Packet

## Overview

This document describes the M5 notebook round-trip fixtures, heavy-output corpora, and canonical support packet subsystem. The subsystem ensures that:

- Round-trip test fixtures are explicit, typed, and evidence-based rather than ad hoc.
- Heavy-output notebooks are classified by size bucket, trust implication, and virtualization strategy so the chrome never freezes or silently escalates trust.
- The canonical support packet is the single source of truth for this lane, consumed by docs, help, CI, and support surfaces.

## Records

### `NotebookRoundTripFixture`

Represents a seed fixture for round-trip testing. Carries:

- `fixture_kind_class`: `clean_canonical`, `attachment_heavy`, `metadata_rich`, `unknown_namespace_dense`, `corrupted_then_repaired`, `export_only`, `no_kernel_editable`, or `cell_id_stress`.
- `assertion_kind_class_refs`: opaque refs to the round-trip assertion kinds exercised by this fixture.
- `expected_result_class`: `pass`, `fail`, `partial`, or `blocked_by_format_boundary`.
- `loss_summary`: required when expected result is not `pass`.

**Invariant**: Non-pass expected results require a `loss_summary`; pass results must not carry one.

### `HeavyOutputCorpusEntry`

Represents a heavy-output scenario in the corpus. Carries:

- `size_bucket_class`: `small`, `medium`, `large`, or `very_large`.
- `output_count`: total number of outputs in the notebook (must be > 0).
- `contains_rich_output`: whether the notebook contains images, widgets, or HTML.
- `trust_implication_class`: `trusted_inline`, `trusted_virtualized`, `sanitized_inline`, `sanitized_virtualized`, `sandboxed`, or `blocked`.
- `virtualization_class`: `none`, `truncated`, `paginated`, `externalized`, or `lazy_loaded`.

**Invariant**: Small size-bucket entries must use `none` virtualization.

## Checked-In Packet

The canonical packet lives at:

```
artifacts/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.json
```

This packet is embedded in the `aureline-notebook` crate and parsed at test time.

## Schema

The boundary schema lives at:

```
schemas/notebook/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.schema.json
```

## Fixtures

Worked YAML fixtures live under:

```
fixtures/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet/
```

## Downgrade Behavior

- When a fixture's expected result is `blocked_by_format_boundary`, the loss is explicit and a `loss_summary` is required.
- When a heavy-output entry is `blocked`, no rendering is attempted and the user sees a policy-blocked cue.
- Export-only fixtures are never round-trip safe by definition.

## CI / Release Integration

The `aureline-notebook` crate tests validate:

- Every example record in the embedded packet parses and validates cleanly.
- Closed vocabularies list every variant.
- Violations of invariants (e.g., pass with loss_summary, small with virtualization) are rejected.
