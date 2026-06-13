# Structured config parameter-source and round-trip review

This document is the narrative companion for Aureline's canonical M5 packet for
per-parameter provenance, secret/reference chips, compare-before-save review,
and export/support disclosure.

Machine-readable companions:

- [`/artifacts/config/structured_config_parameter_source_and_round_trip_review.json`](../../artifacts/config/structured_config_parameter_source_and_round_trip_review.json)
  — canonical packet consumed by editors, CLI inspect, help/docs, support
  export, and release evidence.
- [`/artifacts/config/structured_config_parameter_source_and_round_trip_review.md`](../../artifacts/config/structured_config_parameter_source_and_round_trip_review.md)
  — generated summary suitable for human review beside the JSON packet.
- [`/schemas/config/structured_config_parameter_source_and_round_trip_review.schema.json`](../../schemas/config/structured_config_parameter_source_and_round_trip_review.schema.json)
  — boundary schema for the packet.
- [`/fixtures/config/structured_config_parameter_source_and_round_trip_review/canonical.json`](../../fixtures/config/structured_config_parameter_source_and_round_trip_review/canonical.json)
  — replayable fixture anchored to the same packet shape.

Related contracts:

- [`/docs/config/structured_config_policy_bundle_and_entitlement_matrix.md`](./structured_config_policy_bundle_and_entitlement_matrix.md)
  — freezes the config-bearing families this packet covers.
- [`/docs/config/structured_config_artifact_modes_and_layers.md`](./structured_config_artifact_modes_and_layers.md)
  — freezes the shared source/effective/live and environment-layer vocabulary
  this packet deepens.
- [`/docs/config/m4/structured-config-manifest-environment-editor-qualification.md`](./m4/structured-config-manifest-environment-editor-qualification.md)
  — earlier generic source/effective/live and round-trip guardrail contract this
  M5 packet narrows into family-specific rows and review sheets.

Normative product sources remain the `.t2/docs/` design documents. If this file
disagrees with those sources, those sources win and this file, the schema, and
the checked-in packet update together.

## What this packet freezes

The packet is the shared metadata source for the new M5 structured-config
families. It freezes:

1. `parameter_row_vocabulary[]`
   Every per-parameter row must expose:

   - stable key/path identity,
   - masked display value,
   - source class,
   - resolution time,
   - winning layer or deferred state,
   - layer-bounded override action, and
   - copy/export posture.

2. `value_chip_vocabulary[]`
   Every surface distinguishes:

   - literal values,
   - env refs,
   - secret handles,
   - policy-injected values, and
   - runtime-discovered values.

   Secret-handle and policy-injected rows never require raw-secret exposure in
   order to explain provenance.

3. `artifact_reviews[]`
   One row exists for each current M5 structured-config family:

   - request-workspace environments,
   - database profiles,
   - API profiles,
   - notebook runtime manifests,
   - preview runtime config,
   - workflow bundle manifests,
   - CI environment descriptors,
   - infrastructure environment descriptors, and
   - managed policy overlays.

   Each row carries parameter-source rows, visible chips, effective-value
   review, export summary, and where required:

   - round-trip-risk banner,
   - compare-before-save sheet, and
   - safe raw-source fallback wording.

4. `output_disclosure_vocabulary[]`
   Effective-value review and export/support summary must always disclose
   whether the reviewed output contains:

   - literal values,
   - references/handles,
   - redacted placeholders, or
   - key-path metadata only.

5. `surface_vocabulary[]`
   Desktop shell, CLI inspect, docs/help, and support export all reuse the same
   parameter rows, chips, compare-before-save sheets, effective-value review,
   and export summaries.

## Core invariants

The packet is conforming only if all of these remain true:

- every covered family exposes at least one winning parameter-source row;
- secret-handle and policy-injected chips block raw-secret export by default;
- any family with a round-trip-risk banner also exposes an explicit
  compare-before-save sheet;
- effective-value review and export/support summary disclose the same output
  classes;
- support export reuses the same export-review metadata rather than paraphrasing
  it; and
- all four round-trip-loss classes remain covered somewhere in the packet:
  comments, unknown keys, ordering, and extension namespaces.

## How downstream surfaces use it

This packet is the ceiling for M5 structured-config provenance and save-review
truth. Downstream surfaces may narrow below it, but they may not:

- collapse literal/env-ref/secret-handle/policy/runtime distinctions into one
  generic “source” badge,
- omit compare-before-save when the packet says structural rewrite risk exists,
- imply that support export contains raw values when the review sheet discloses
  only handles/placeholders/key paths, or
- invent a different export/support disclosure vocabulary.

## Regeneration

Regenerate the canonical packet and markdown summary with:

```sh
cargo run -q -p aureline-config --bin aureline_config_structured_parameter_source_and_round_trip_review -- json
cargo run -q -p aureline-config --bin aureline_config_structured_parameter_source_and_round_trip_review -- markdown
```
