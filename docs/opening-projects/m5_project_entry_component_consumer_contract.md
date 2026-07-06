# M5 project-entry component consumer contract (M05-842)

This contract is the **first-consumer adoption lane** over the frozen M5
project-entry component matrix
([`artifacts/design/m5-project-entry-component-matrix.md`](../../artifacts/design/m5-project-entry-component-matrix.md),
M05-836/839/840/841). It proves the ten reusable component families are genuine
**primitives** — not one Start Center page plus a handful of flow-specific
dialogs — by adopting each across every claimed M5 project-entry consumer
surface without letting system-open, deep-link, or headless paths fork the
entry vocabulary.

- **Rust module:**
  `crates/aureline-shell/src/add_shared_start_center_system_open_deep_link_and_cli_headless_project_entry_component_consumers/`
- **Boundary schema:**
  [`schemas/ui/m5-project-entry-component-consumer.schema.json`](../../schemas/ui/m5-project-entry-component-consumer.schema.json)
- **Frozen matrix / shared schema:**
  [`schemas/ui/m5-project-entry-component.schema.json`](../../schemas/ui/m5-project-entry-component.schema.json)
  and [`fixtures/ui/m5-project-entry-components/component_matrix.json`](../../fixtures/ui/m5-project-entry-components/component_matrix.json)
- **Checked support export:**
  [`artifacts/release/m5-project-entry-component-consumer-proof/support_export.json`](../../artifacts/release/m5-project-entry-component-consumer-proof/support_export.json)
  (plus `matrix.csv` and `report.md`)
- **Regenerate:**
  `cargo run -p aureline-shell --example dump_project_entry_component_consumers`

## Consumer classes

Each row is one consumer on one M5 entry surface. The five claimed consumer
classes are:

| Consumer group | Surfaces | Required behavior |
|---|---|---|
| `start_center_palette` | Start Center home, `Open recent` list, command palette | Uses quick-action, recent-work, restore, switcher, and entry-chooser records; cannot collapse the entry verbs into generic start copy. |
| `system_open_intake` | System-open / file-association, drag-and-drop intake | Resolves the OS-handed target through the same entry-chooser / entry-review / destination-collision records; preserves literal target and resulting mode. |
| `deep_link_handoff` | Protocol / deep-link, browser / mobile handoff | Resolves into entry-review and post-entry-handoff records; does not mint private trust, target, or resulting-mode labels. |
| `cli_headless` | CLI entry, headless automation | Exports the same entry-verb, target, resulting-mode, write-scope, trust, restore, and readiness tokens as desktop. |
| `support_diagnostics_docs` | Support / export replay, admin / diagnostics, docs / help center | Carries opaque entry-object ids, schema refs, component family, canonical tokens, and redaction-safe labels; docs/help quotes the matrix families, not feature-local prose (AC3). |

## What every consumer must preserve

A consumer may *narrow* authority (`read_only`, `inspect_only`,
`review_required`, `export_only`, `policy_blocked`) but must never rename or drop
the governed truth. Every row keeps:

- **Entry verb + command id.** Each distinct verb (`open`, `open_recent`,
  `clone`, `import`, `restore`, `resume`) owns exactly one canonical
  `command_id`. No surface may fork the command by client, trigger, or platform
  handoff origin — this is what stops entry surfaces from forking vocabulary.
- **The five track-invariant label families.** `entry_verb`, `literal_target`,
  `resulting_mode`, `write_scope_trust_host_auth`, and
  `restore_or_first_useful_work` stay explicit before Aureline writes, clones,
  imports, resumes, or widens scope.
- **The canonical degraded-state vocabulary.** `missing_target`,
  `remote_unreachable`, `policy_blocked`, `cached_only`, `partial_restore`, and
  `authority_expired` stay visible across desktop and exported evidence.
- **One canonical family reference.** Each row points at exactly one per-family
  schema plus the shared release-proof packet
  (`artifacts/release/m5-project-entry-component-proof/packet.json`) instead of
  cloning surface-local prose.
- **Copy / export parity.** Governed labels are copyable as text / JSON /
  Markdown; a screenshot-only export is prohibited.

## Acceptance criteria mapping

- **Entry surfaces no longer fork vocabulary by client, trigger, or platform
  handoff origin.** Enforced by `command_id_is_canonical` per row and
  `command_ids_stable_across_surfaces` across the packet: every entry verb
  resolves to one command id everywhere.
- **Deep links and system-open flows preserve literal target truth, resulting
  mode, and recovery path without special-case copy.** Enforced by
  `preserves_handoff_target_truth` on every `system_open_intake` and
  `deep_link_handoff` row (literal target + resulting mode required) and the
  generic-copy guard on disclosure banners.
- **Support and automation can reconstruct what entry path the user actually
  took.** Enforced by `supports_entry_path_reconstruction`: each row carries an
  opaque `entry_object_ref`, its canonical `command_id`, and complete
  copy/export parity so a support bundle or CLI replay rebuilds the taken path
  without leaking the literal target.

## Narrowing disclosure

A narrower consumer discloses the reduction with a `reduced_capability_banner`
whose `capability_state` matches the row's `authority_mode` and whose visible
label is precise (never generic "Get started" / "read only" / "unavailable"
copy). When a consumer punts to another surface it also carries a
`handoff_note_ref` naming the desktop / companion / browser / handoff-packet
target. A full-interactive consumer carries no banner.

## Non-conforming drift

The following changes must update this contract, the boundary schema, and the
checked support export in the same change:

- Adding or renaming an entry consumer group, surface, or component family.
- Adding an entry verb or changing its canonical command id.
- Shipping a deep-link, system-open, CLI/headless, or docs/help consumer that
  renames or drops a governed entry-verb, target, resulting-mode, write-scope,
  trust, restore, or readiness label.
