# Managed-Copy vs Local Truth Registry

This document is the human entry point for the alpha schema and
record-class registries that govern durable state, support exports,
portable packages, managed copies, retention labels, and delete/export
semantics.

Canonical artifacts:

- [`/artifacts/governance/schema_registry_alpha.yaml`](../../artifacts/governance/schema_registry_alpha.yaml)
  names the schema or placeholder that constrains each alpha state,
  support/export, portable-package, managed-copy, export-packet, and
  receipt family.
- [`/artifacts/governance/record_class_registry_alpha.yaml`](../../artifacts/governance/record_class_registry_alpha.yaml)
  names the record classes support, docs, CLI, export, and
  product-boundary surfaces should cite when they need local-vs-managed
  truth, retention, hold, delete, and export behavior.
- [`/fixtures/governance/schema_record_registry_alpha_cases/manifest.yaml`](../../fixtures/governance/schema_record_registry_alpha_cases/manifest.yaml)
  captures the protected acceptance cases for this lane.
- [`/ci/check_schema_record_registry_alpha.py`](../../ci/check_schema_record_registry_alpha.py)
  validates the registries and renders the first support/export
  projection.

## Contract

The registries keep four concepts separate:

- **Local truth** is the local state or local export file the user can
  inspect, export, compare, or delete from the device.
- **Managed copy** is a support, admin, control-plane, or policy-owned
  retained reference. Deleting a local file does not imply the managed
  copy was destroyed.
- **Export packet** is generated output whose manifest is the user
  visible proof of what crossed a boundary.
- **Receipt** is evidence about a delete, redaction, held, skipped, or
  retained subset. It is not a hidden payload store.

Every alpha record row therefore carries:

- owner and schema row refs;
- local authority and managed-copy posture;
- retention label and retention source refs;
- hold eligibility;
- delete support, completion evidence, and blockers;
- export availability, formats, manifest requirement, and local-copy
  disclosure.

## First Consumer

The validator is also the first CLI/support/export consumer. It renders
metadata-only JSON directly from the checked-in registries:

```bash
python3 ci/check_schema_record_registry_alpha.py --repo-root . --render-support-export-projection
```

The projection includes record-class ids, schema rows, owner, local
authority, managed-copy posture, retention labels, delete semantics,
export semantics, and hold semantics. It intentionally excludes raw
workspace content, credentials, prompts, logs, and provider payloads.

## Validation

Run:

```bash
python3 ci/check_schema_record_registry_alpha.py --repo-root .
```

Refresh the checked-in validation capture:

```bash
python3 ci/check_schema_record_registry_alpha.py \
  --repo-root . \
  --report artifacts/milestones/m2/captures/schema_record_registry_alpha_validation_capture.json
```

The lane fails when:

- durable state, support bundle, portable package, managed-copy, export
  packet, or receipt coverage is missing;
- an implemented schema path is absent;
- a placeholder lacks owner, expiry, or exit criteria;
- a record row has no schema row refs;
- delete, export, hold, or retention semantics are missing;
- a base record-class ref is not present in the existing record-class
  registry; or
- the support/export projection cannot be rendered from the registries
  without local status text.

## Use By Other Lanes

Support bundle preview, data portability, offboarding, managed-boundary
reviews, CLI/headless explainers, and docs tables should link to these
two registries instead of restating local-only, managed-copy, export,
delete, or retention wording. Existing deeper contracts still own their
own details; this page only gives alpha surfaces one join point.
