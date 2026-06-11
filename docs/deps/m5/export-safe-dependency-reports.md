# Export-safe dependency reports

This document describes the canonical export-safe dependency reports packet used
by Aureline's dependency-health, review, support-export, and release surfaces. It
is the user-facing companion to the governed artifact at
`artifacts/deps/m5/export-safe-dependency-reports.json` and the typed model in
the `aureline-deps` crate
(`export_safe_dependency_reports`).

## What this packet covers

The packet emits dependency-health reports without overstating their truth. It
answers, for every report row:

1. **What kind of finding is this?** Advisory, vulnerability, license, notice, or
   SBOM component.
2. **How strong is the claim?** A **claim class** of `verified`, `asserted`,
   `mirrored`, or `incomplete`.
3. **Where did the data come from?** A **source class** of `local_analysis`,
   `imported_feed`, `enterprise_mirror`, or `stale_snapshot`.
4. **Is the data current?** A **freshness class** of `current`, `stale`,
   `snapshot_only`, or `unknown`.
5. **How can it be exported?** A set of documented, open, redaction-safe export
   formats.

## Verified vs asserted vs mirrored vs incomplete

Badges must never imply more certainty than the data supports. The claim class is
constrained by the row's source and freshness:

- **`verified`** — Only permitted when the row was computed by current
  `local_analysis` of the exact build. This is the only class that asserts an
  independently confirmed claim.
- **`mirrored`** — Only permitted when the data is served from an
  `enterprise_mirror`. The origin feed could not be reached, so the row is
  labeled mirrored rather than verified.
- **`asserted`** — Asserted by an external feed or import; not independently
  confirmed. Permitted for any source.
- **`incomplete`** — Data is stale, snapshot-only, or policy-blocked; no full
  claim is made.

A row that claims `verified` from a non-local or non-current source, or `mirrored`
from a non-mirror source, is a validation failure (`OverstatedClaim`).

## Mirror, auth, and offline reality is always explicit

A report that returns no rows must never silently read as a clean "no findings"
result. The packet carries a connectivity disclosure with:

- **`connectivity_state`** — `online`, `mirror_only`, `auth_required`,
  `air_gapped`, or `offline_snapshot`.
- **`empty_result_reason`** — `genuinely_empty`, `mirror_stale`, `auth_required`,
  `snapshot_only`, or `feed_unreachable`.
- **`last_known_good_at`** — the last time live feeds were known good. Required
  for `mirror_only`, `air_gapped`, and `offline_snapshot` states so mirror and
  air-gapped profiles preserve last-known truth instead of appearing empty.

A clean "no findings" posture can be claimed only when the feed plane is `online`
**and** the empty-result reason is `genuinely_empty`. Any degraded state keeps the
absence of findings unprovable; claiming clean while degraded is a validation
failure (`MisleadingEmptyClaim`).

## Scope is always disclosed

Every report ties back to an exact build context and a scope kind:

- **`full_repo`** — the whole workspace; `manifests_in_scope` is empty.
- **`selected_manifests`** — a reviewer-selected manifest set; `manifests_in_scope`
  lists them.
- **`slice`** — a narrower slice (e.g., a single crate or path subtree);
  `manifests_in_scope` lists the covered manifests.

A bounded scope with no listed manifests, or a full-repo scope with listed
manifests, is a validation failure (`ScopeManifestMismatch`).

## Export formats are open, machine-readable, and redaction-safe

Every declared export format must be a documented, open standard
(`NonOpenExportFormat` otherwise). SBOM exports use `spdx_json` (SPDX) or
`cyclonedx_json` (CycloneDX); advisory and vulnerability findings export to
`sarif`; generic `json` and `csv` projections serve release and support tooling;
`markdown` is a human-readable summary that is never the machine-of-record.

When any SBOM row is present, the packet must declare an SBOM-capable export
format (`MissingSbomExportFormat` otherwise), so SBOM, license, and advisory
exports stay attributable and machine-readable for release/support workflows
without requiring reverse engineering.

Redaction is the default. Every format declares whether it redacts private
registry URLs and secrets:

- **Secrets are always redacted**, in every posture. A format that does not redact
  secrets is a validation failure (`SecretLeakRisk`).
- **Private registry URLs are redacted by default.** The `redacted_by_default`
  posture must redact them (`RegistryUrlLeakRisk` otherwise). The
  `opt_in_disclosure` posture exists only for internal-scope exports where an
  operator explicitly opts into including private registry URLs; secrets remain
  redacted.

This packet uses only open, on-disk formats. It introduces no hosted-only
compliance plane and makes no open-format export depend on a managed service.
