# Claim Publication And Downgrade

The claim-publication manifest is the stable-line source of truth for public
and enterprise-facing support wording. Release notes, website/docs,
enterprise evaluation packets, in-product badges, Help/About,
service-health, CLI inspection, support export, and public proof rows must
consume:

`artifacts/release/stable/claim-publication-manifest/manifest.json`

No consuming surface should restate support scope from hand-maintained copy.
Each rendered row must carry the manifest id, the effective claim, the badge
label, and linked report refs from the manifest row it renders.

## Required Evidence

Every claim entry carries:

| Field | Purpose |
| --- | --- |
| `declared_support_class` | The widest class the owner wants to publish. |
| `effective_claim` | The claim actually rendered after evidence resolution. |
| `scope_caveat` | The caveat copied to every destination. |
| `linked_report_refs[]` | Current claim, compatibility, reference-workspace, evaluation, or proof refs. |
| `validity_window` | Capture date, expiry date, and freshness window. |
| `owner_ref` / `owner_signed` | Named accountability for the row and reports. |
| `surface_projections[]` | The exact destination rows that render the claim. |

Certified entries additionally require a current, signed
`reference_workspace_report` ref. If that report is absent or stale, the row
cannot render Certified wording.

## Downgrade Behavior

The manifest uses worst-supporting-truth-wins propagation:

| Trigger | Required result |
| --- | --- |
| Stale generic evidence | Render `retest_pending` or narrower. |
| Missing generic evidence | Render `unsupported`. |
| Compatibility class dropped | Render `limited` or narrower. |
| Stale reference workspace | Render `limited` or narrower. |
| Missing reference workspace | Render `unsupported`. |
| Missing owner signoff | Render `unsupported`. |
| Private evaluation filter | May filter or narrow, never widen beyond the public effective claim. |

A surface that renders wider wording than the entry's `effective_claim` holds
publication. A surface that renders the downgraded claim may continue, and the
manifest records `proceed_with_downgrades` when all downgraded copy is already
projected consistently.

## Current Stable-Line Packet

The current packet links:

- Manifest: `artifacts/release/stable/claim-publication-manifest/manifest.json`
- Schema: `schemas/release/claim-publication-manifest.schema.json`
- Compatibility report:
  `artifacts/release/stable/claim-publication-manifest/compatibility-publication-report.json`
- Reference-workspace reports:
  `artifacts/release/stable/reference-workspace-reports/index.json`
- Evaluation evidence pack:
  `artifacts/release/stable/evaluation-pilot-evidence-pack/pack.json`

The current manifest demonstrates both states needed by release tooling:

- `claim_entry:rust_workspace_self_host` and
  `claim_entry:typescript_web_app` render Certified because their
  reference-workspace and compatibility reports are current and signed.
- `claim_entry:legacy_remote_ssh` and
  `claim_entry:extension_author_workflow` render downgraded copy because
  their linked evidence is stale, dropped, missing, or unsigned.

## Verification

Run the Rust model test:

```sh
cargo test -p aureline-release claim_publication_manifest
```

Run the standalone validator:

```sh
python3 tools/release/publish-claim-manifest/publish_claim_manifest.py \
  artifacts/release/stable/claim-publication-manifest/manifest.json
```
