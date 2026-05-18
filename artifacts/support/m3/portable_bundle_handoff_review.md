# Portable Bundle Handoff Review

This packet reviews the portable bundle and shelf contract used by offline
review, browser companion handoff, incident follow-up, support export, and
desktop shelf resume.

## Evidence

| Evidence | Path |
| --- | --- |
| Boundary schema | `schemas/change/portable_bundle.schema.json` |
| Rust contract | `crates/aureline-change-objects/src/portable_bundle/mod.rs` |
| Canonical fixtures | `fixtures/review/m3/portable_bundle/` |
| Shell inspector | `crates/aureline-shell/src/portable_bundle_inspector/mod.rs` |
| Support projection | `crates/aureline-support/src/portable_bundle_handoff/mod.rs` |
| UX contract | `docs/ux/m3/portable_bundle_and_shelf_beta.md` |

## Review Findings

| Area | Result |
| --- | --- |
| Export/import parity | One `portable_change_bundle_record` is shared by fixtures, Rust projection, shell rows, and support rows. |
| Offline inspection | Every fixture includes `inspect_offline`; stale fixtures include `compare_only_reopen`. |
| Browser handoff | Browser companion fixture is read-only and carries stale provider-overlay labels. |
| Incident follow-up | Incident fixture preserves runbook and incident refs with environment-capsule staleness. |
| Desktop resume | Shelf fixture requires revalidation before desktop resume. |
| Target identity | Workspace, repo, worktree, base, target, and environment refs are opaque and required. |
| Diff and evidence lineage | Diff refs and evidence refs are required, counted by shell/support projections, and never replaced by raw bodies. |
| Redaction | Raw paths, remote URLs, secrets, credentials, and raw diff bodies are excluded by schema and Rust validation. |
| Authority | Live bearer authority, ambient credentials, and secret material are explicitly false in every record. |
| Destruction semantics | Redaction profile includes retention or destruction posture plus optional receipt refs. |

## Support Export Posture

The support projection compiles a metadata-safe envelope with:

- schema, UX contract, and artifact refs;
- target and worktree refs;
- review-pack version and parity;
- diff/evidence ref counts;
- validation freshness and stale labels;
- reopen modes;
- authority and redaction classes;
- support lineage refs.

The envelope is export-safe only when raw paths, raw remote URLs, raw
credentials, and live provider authority are excluded and every row preserves
the no-live-authority invariant.

## Follow-Ups

- Attach live importer UI once the concrete desktop import route exists.
- Add hosted-provider replay drills after provider integrations expose current
  browser companion snapshots through the same schema.
