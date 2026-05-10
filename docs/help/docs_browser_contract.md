# Docs/help browser skeleton contract

This page is the reviewer-facing entry point for the **in-product docs/help browser** skeleton: an opinionated docs/help surface that always shows where its content came from, which build it claims to match, how fresh that claim is, which client scope is in force, and how the user can hand off to the system browser.

The skeleton does **not** redefine embedded-surface boundary semantics. It consumes the shared embedded boundary card contract (owned by the embedded surfaces lane) and projects a focused row card whose vocabulary names the docs-truth axes a reader has to see before trusting an embedded docs page.

## Truth rows

The skeleton renders five rows. Every row is required; none are hover-only. When upstream truth is missing or unverified, the row stays present and labels itself explicitly (no blanking out):

1. **Source row** — closed `source_class` token plus a plain-language label. When `source_truth` is missing entirely, the row reads "Unknown source (no source-of-truth disclosed)" rather than vanishing.
2. **Version row** — closed `version_match_state` token (`exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build`) plus the running build identity ref. Drift and unknown states render their label verbatim; they never collapse to "OK".
3. **Freshness row** — closed `freshness_class` token plus a `degraded` boolean. `authoritative_live` and `warm_cached` are non-degraded; `degraded_cached` / `stale` / `unverified` are degraded. The boolean is what badge chrome and a11y descriptions key off; the label is what the user reads.
4. **Client-scope row** — quotes the upstream `data_boundary_label` and `boundary_state_label`, plus closed `identity_mode` (`account_free_local` / `self_hosted_org` / `managed_workspace`) and `trust_state` (`trusted` / `restricted`) tokens. This is how the docs pane discloses *which* client scope it is anchored to without inventing a parallel scope vocabulary.
5. **Browser-handoff row** — quotes the host-owned `Open in browser` action label and `browser_handoff_packet_ref`, plus the fallback posture and target tokens. The handoff stays available even when the embedded body is degraded (failure-drill rule).

## Failure drill

The skeleton MUST keep all five rows explicit when:

- the upstream record reports `version_match_state == unknown_target_build`,
- the upstream record reports `freshness_class == unverified` or `stale`, or
- the upstream record carries no `source_truth` block at all.

In each case, the chrome paints the unknown / degraded label rather than dropping the row, and the host-owned browser-handoff action remains the path of last resort to the authoritative source.

## Canonical sources

- Render-side projection: `crates/aureline-shell/src/docs_browser/state.rs`
- Module entry: `crates/aureline-shell/src/docs_browser/mod.rs`
- Live consumer: `crates/aureline-shell/src/bootstrap/native_shell.rs` (`draw_docs_help_boundary_card`)
- Embedded boundary card contract (shared substrate): `docs/ux/embedded_boundary_contract.md`
- Boundary card schema: `schemas/ux/embedded_boundary_card.schema.json`
- Fixture cases: `fixtures/help/docs_browser_cases/`
  - `project_docs_live_verified.json` — happy path, exact build match, authoritative live.
  - `mirrored_docs_stale_snapshot.json` — degraded path, mirrored docs with detected drift and stale snapshot.
  - `unknown_metadata_unverified.json` — failure drill, derived explanation with unknown target build and unverified freshness.

## Live shell walkthrough

1. Run the desktop shell: `cargo run -p aureline-shell --bin aureline_shell`.
2. Use `Tab` until focus reaches `right_inspector`.
3. Inspect the docs/help pane and confirm the five truth rows render — Source, Version, Freshness, Client scope, Action / Handoff packet.
4. Press `Enter` while focused on `right_inspector` to invoke `cmd:docs.open_in_browser`. Confirm the system browser opens.

## Acceptance

- The pane never paints fewer than the five truth rows.
- The browser handoff stays available even when the source/version/freshness state is degraded or missing.
- The pane does not host any native-approval-equivalent action (trust elevation, rollback/restore confirmation, AI apply review, high-risk approval sheet); those stay host-owned.

## Validation

`cargo test -p aureline-shell docs_browser::state::tests` exercises:

- the live verified, stale snapshot, and unknown-metadata fixture cases,
- the `source_truth = None` failure-drill fallback labels,
- the open-in-browser packet-ref round-trip.
