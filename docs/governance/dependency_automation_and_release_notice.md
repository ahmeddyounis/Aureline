# Dependency automation and release-notice generation contract

This document defines the **seed automation contract** that keeps third-party
dependency governance, upstream-health review, and release-notice / SBOM /
provenance publication wired together as the repository grows.

It is a contract and workflow description, not an implementation of a full SBOM
generator, a legal notice portal, or a package-mirror service.

## Canonical inputs (stable rows)

All automation composes from these stable-row artifacts:

- `artifacts/governance/dependency_register.yaml` — upstream choices + posture.
- `artifacts/governance/third_party_import_register.yaml` — copied/mirrored bytes.
- `artifacts/governance/upstream_health_scorecard.yaml` — health review for critical upstreams.
- `artifacts/governance/release_notice_seed.yaml` — publication binding keyed by stable source ids.

The **stable ids** in the two registers are the only ids automation is allowed
to key on. No separate “notice ids” are permitted.

## Ingest model (machine facts vs curated fields)

The registers intentionally separate:

- **Curated governance fields** (human-owned): owner, criticality, fork/replace
  triggers, build-vs-buy linkage, release-notice class, and review decisions.
- **Ingested evidence fields** (machine-owned, best-effort): upstream activity
  timestamps, observed versions/tags, license expressions, mirror revision
  digests, and similar “what did we observe?” facts.

Automation MUST:

1. preserve stable ids (`dep.*`, `import.*`) verbatim;
2. preserve an **evidence trail** for every refresh (which sources were queried,
   when, and with what tool version);
3. update only fields explicitly declared in each row’s
   `automation_refresh.machine_refresh_fields`; and
4. surface a review checkpoint whenever an automation run would change a notice
   binding or any curated governance field.

### Seed ingest fields

The seed registers already declare the minimum “machine facts” fields that
automation may capture as evidence. For dependencies, the intended upstream
health ingest fields are:

- `latest_release_or_tag`
- `latest_release_published_at`
- `default_branch_last_commit_at`
- `open_security_advisory_count` (when a trusted advisory source is wired in)
- `maintainer_count_estimate`
- `license_expression`

For imported/mirrored bytes, the intended ingest fields are:

- `upstream_version_or_revision`
- `upstream_source_archive_digest`
- `mirror_revision`
- `signature_or_digest_status`
- `local_patch_digest`
- `notice_text_ref`

These are **evidence fields**, not “approval fields”: they feed review and
auditing, but they do not replace the curated governance posture declared in the
register rows.

## Release-notice generation binding

`artifacts/governance/release_notice_seed.yaml` is the contract between stable
dependency/import rows and downstream publication:

- third-party notice rows,
- SBOM entries (SPDX and/or CycloneDX),
- provenance statements, and
- docs-pack manifests for mirrored packs.

The seed file MUST remain:

- complete (every dependency/import row has exactly one seed row), and
- stable (keys off `source_register` + `source_id` only).

Automation may generate downstream notice/SBOM/provenance artifacts, but it must
never mint a second identifier space or require a hand-maintained copy table.

## Sample automation paths

### 1) Normal library update (registry dependency)

1. Refresh upstream observations (non-mutating report) to see latest versions,
   tags, and activity signals.
2. Propose a manifest/lock update (when the dependency is admitted).
3. Update the dependency row’s curated fields only if posture changes (owner,
   criticality, trigger, notice class).
4. Ensure `release_notice_seed.yaml` remains complete and unchanged unless the
   dependency’s publication posture actually changed.

### 2) Vendored patch stack (local fork or long-lived patch carry)

1. Add or update the dependency row’s fork/replace trigger and evidence refs.
2. Add or update the import row to record the copied bytes, local path, and
   local modification posture.
3. Keep publication binding keyed to the same stable source ids.
4. Require human review before any automation updates notice text or distribution posture.

### 3) Imported asset (bundled fonts, docs, icons)

1. Record the upstream family choice in the dependency register (stable id).
2. Record the imported bytes in the import register (stable id), including:
   local-path home and local modifications.
3. Bind publication through `release_notice_seed.yaml` using the import row id.

### 4) Mirrored component (docs pack or offline bundle)

1. Record the mirrored pack in the import register (mirror kind + posture).
2. Keep mirrorability/provenance evidence in the import row’s evidence refs.
3. Publish attribution via `release_notice_seed.yaml` using `docs_pack_manifest`
   (not the core binary notice), unless a later distribution embeds the bytes.

### 5) Upstream health crosses a fork-or-replace threshold

1. Upstream observations or a scorecard update indicates a sustained risk band
   downgrade or a block condition.
2. Open a visible review: update `upstream_health_scorecard.yaml` with the new
   assessment and required follow-up.
3. Do not let automation silently “fix” the situation by rewriting curated
   governance fields. Any change to fork/replace triggers is a human decision.

## CI gate (integrity, not crawling)

The dependency health gate is offline by default. It enforces:

- stable ids are unique;
- import rows reference valid dependency ids when applicable;
- every dependency/import row has a matching release-notice seed row; and
- critical dependencies have upstream-health scorecard coverage.

Entry point:

```sh
./ci/check_dependency_health.sh
```

## Optional upstream probe (review input)

For a best-effort upstream metadata report (networked, non-mutating):

```sh
python3 tools/governance/dependency_ingest/refresh_upstream_observations.py \
  --out target/dependency-ingest/upstream_observations.json
```

This report is evidence for human review; it does not imply license, security,
or provenance clearance by itself.
