# Notebook Save, Repair, and Round-Trip Safety

## Overview

This document describes the M5 notebook save, repair, and round-trip safety subsystem. The subsystem ensures that:

- `.ipynb` remains canonical through save/load cycles.
- Metadata (kernelspec, language_info, aureline namespaces, and vendor namespaces) is preserved or explicitly lossy.
- Cell attachments are never silently externalized or dropped.
- Unknown metadata namespaces survive open/save/import/export unless a documented format boundary forbids it.
- Repair actions are explicit, loss-bearing, and never silently fallback.
- Round-trip assertions are checked and reported with pass/fail/partial/blocked results.

## Records

### `NotebookSaveOperation`

Represents a save operation against a notebook document. Carries:

- `save_kind_class`: `full_save`, `auto_save`, `checkpoint_save`, or `export_derived_format`.
- `metadata_preservation_class`: whether metadata was preserved, partially lost, explicitly dropped, or blocked.
- `attachment_preservation_class`: whether attachments were preserved, externalized, dropped, or blocked.
- `unknown_namespace_preservation_class`: whether unknown namespaces were preserved, filtered, dropped, or blocked.
- `round_trip_safe`: boolean indicating whether the operation is believed to be round-trip safe.
- `loss_summary`: required when any preservation class indicates loss.

**Invariant**: `export_derived_format` saves must not claim `round_trip_safe=true`.

### `NotebookRepairAction`

Represents a repair applied to a damaged or invalid notebook. Carries:

- `repair_kind_class`: the specific repair applied (mint missing cell ID, restore attachment reference, etc.).
- `consequence_class`: `lossless`, `lossy_with_explicit_note`, or `lossy_with_silent_fallback`.
- `applied`: whether the repair was actually applied.

**Invariant**: `lossy_with_silent_fallback` is non-conforming and must surface as a finding.

### `NotebookRoundTripAssertion`

Represents an assertion that a specific property survives an open/edit/save cycle. Carries:

- `assertion_kind_class`: the property being asserted (metadata, attachment, unknown namespace, cell order, cell ID, source, output).
- `result_class`: `pass`, `fail`, `partial`, or `blocked_by_format_boundary`.
- `loss_summary`: required for non-pass results.

## Checked-In Packet

The canonical packet lives at:

```
artifacts/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.json
```

This packet is embedded in the `aureline-notebook` crate and parsed at test time.

## Schema

The boundary schema lives at:

```
schemas/notebook/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.schema.json
```

## Fixtures

Worked YAML fixtures live under:

```
fixtures/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces/
```

## Downgrade Behavior

- When a format boundary blocks preservation, the loss is explicit (`blocked_by_format_boundary`) and a `loss_summary` is required.
- When a repair is lossy, it must carry `lossy_with_explicit_note`; silent fallback is rejected by validation.
- Export-derived formats are never round-trip safe by definition.

## CI / Release Integration

The `aureline-notebook` crate tests validate:

- Every example record in the embedded packet parses and validates cleanly.
- Closed vocabularies list every variant.
- Violations of invariants (e.g., export claiming round-trip safety) are rejected.
