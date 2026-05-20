# Project-entry and workspace-admission conformance corpus

This corpus is the conformance, interoperability, failure / recovery, and
switching-parity drill harness for the M3 project-entry and workspace-admission
beta boundary owned by
[`aureline-workspace`](../../../../crates/aureline-workspace/src/entry/mod.rs)
(`build_project_entry_review` / `ProjectEntryReviewRecord`).

It converts the project-entry UX promise into a regression-gated proof system:
each positive drill carries one entry review request and pins the entry truth the
built review record must reproduce — the verb-specific review sheet, the
source-labelled access class, the first-useful entry source and first landing
surface, the resulting mode, the primary next action, the destination-collision
posture, the Blocking now / Recommended soon / Optional later readiness grouping,
the work the entry deliberately defers, and (for imports) the inspect/write
posture. Each negative drill applies a typed tamper to the built record and pins
the contract finding that must reject it.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/project_entry_admission/`](../../../../crates/aureline-qe/src/project_entry_admission/)
and replayed by
`cargo test -p aureline-qe --test project_entry_admission_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST build a contract-valid
record and match **every** `expect` field in the manifest. Negative drills MUST
raise a contract finding whose message contains `expected_failure_substring` after
the recorded tamper is applied. The fixtures carry only the entry review request
and a `$schema` prelude — they do **not** restate the expectations, so there is
exactly one place to read and audit the pinned truth.

Boundary schemas, contract, and published evidence:

- Record schema: [`/schemas/workspace/entry_review.schema.json`](../../../../schemas/workspace/entry_review.schema.json)
- Corpus schema: [`/schemas/workspace/entry_admission_conformance.schema.json`](../../../../schemas/workspace/entry_admission_conformance.schema.json)
- Beta contract: [`docs/workspace/m3/project_entry_and_admission_conformance.md`](../../../../docs/workspace/m3/project_entry_and_admission_conformance.md)
- First-landing truth matrix: [`artifacts/ux/m3/first_landing_truth_matrix.json`](../../../../artifacts/ux/m3/first_landing_truth_matrix.json)
- Project-entry / admission report: [`artifacts/migration/m3/project_entry_admission_report.md`](../../../../artifacts/migration/m3/project_entry_admission_report.md)

## Coverage axes

| Axis | Drill id |
| --- | --- |
| Open — single file, OS open-with | `open.single_file.os_open` |
| Open — local folder, Start Center | `open.local_folder.start_center` |
| Open — repo root with nested workspace candidates, CLI | `open.repo_with_nested_candidates.cli` |
| Open — multi-root workspace reopen, switcher | `open.multi_root_workspace.workspace_switcher` |
| Clone — clone-only, Start Center | `clone.clone_only.start_center` |
| Clone — clone-and-open, credential-bearing URL | `clone.clone_then_open.command_palette` |
| Clone — mirror-first route | `clone.mirror_first.command_palette` |
| Clone — offline snapshot bundle | `clone.offline_bundle.cli` |
| Clone — air-gapped media | `clone.air_gapped.cli` |
| Clone — duplicate-destination collision | `clone.duplicate_destination.cli` |
| Clone — policy-blocked destination (restricted) | `clone.policy_blocked_destination.cli` |
| Import — inspect-only portable state | `import.portable_state.inspect_only.command_palette` |
| Import — extract to staging for review | `import.portable_state.extract_then_review.start_center` |
| Import — apply competitor config to active workspace | `import.competitor_config.apply.command_palette` |
| Add root — into active workspace | `add_root.into_active_workspace.workspace_switcher` |
| Restore — recent-work reopen | `restore.recent_work_reopen.start_center` |
| Resume — live managed session | `resume.live_session.workspace_switcher` |
| Deep link — review / incident object | `deep_link.review_incident.protocol_handler` |
| Deep link — clone + open a managed repo | `deep_link.clone_managed_open.protocol_handler` |
| Negative — clone implies trust | `negative.clone_grants_trust` |
| Negative — clone label leaks credentials | `negative.clone_exposes_credentials` |
| Negative — import writes before review | `negative.import_writes_before_review` |
| Negative — inspect-only import advertises a write | `negative.import_inspect_only_advertises_write` |
| Negative — collision skips the explicit choice | `negative.collision_skips_explicit_choice` |
| Negative — cross-surface parity drift | `negative.surface_parity_drift` |
| Negative — failed entry drops typed inputs | `negative.failure_repair_drops_inputs` |
| Negative — route auto-trusts the workspace | `negative.route_auto_trust` |
| Negative — route auto-installs setup | `negative.route_auto_install` |
| Negative — review sheet mismatches the verb | `negative.review_sheet_mismatch` |

## Transverse invariants

The conformance suite also pins, across the whole positive set:

- every entry verb (open, clone, import, add-root, restore, resume) and every
  source surface (Start Center, command palette, OS open-with, protocol-handler
  deep link, CLI/headless, workspace switcher) keeps a drill;
- every source-access class (`local_filesystem`, `direct_online`, `mirror_first`,
  `offline_snapshot`, `air_gapped_media`) keeps a drill;
- every in-scope first landing surface keeps a drill, and a review/incident deep
  link lands on the linked object rather than collapsing into a generic open;
- on every drill: no silent trust grant, no setup execution, no task or hook
  execution, no route auto-trust or auto-install, a preserved entry intent, and a
  deep-link parity row that always requires deep-link intent review;
- the published first-landing truth matrix and project-entry / admission report
  cover every drill id and agree with the corpus on the landing surface, so they
  cannot drift from the corpus.

## Redaction guarantees

Every fixture is metadata-safe: only typed labels and `~/`-style placeholders
cross the boundary. The runner scans each fixture and the built record for
forbidden raw-content tokens (private keys, absolute home paths, cloud keys,
bearer tokens). The `clone.clone_then_open.command_palette` drill carries a
credential-bearing source URL on purpose and proves the built record never
reproduces the credential marker. Removing any positive or negative drill without
a replacement is a breaking contract change for the
`workspace.project_entry_and_admission.beta` corpus.
