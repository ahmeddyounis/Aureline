# Start Center warm-start choices beta

The warm-start choice card is the single, reviewable object the Start Center,
the workspace switcher, the CLI/headless entry review, docs/help, and the
support export all read **before any networked or trust-widening side effect
occurs**. It exists to close one start-of-work gap: when a user enters Aureline,
they must be able to tell whether picking a card will resume something live,
reuse a stored snapshot, clone something fresh, or simply open local files —
*before they commit*.

This is the beta hardening of the alpha lanes that already feed Start Center
(workspace templates, prebuild fingerprints, and warm-start descriptors). Where
those alpha records each describe one slice of the picture, the warm-start
choice card unifies them into one entry-surface object with explicit lanes,
per-lane side-effect truth, snapshot freshness, and an environment-starter
summary, so every surface renders the same choices instead of forking copy.

## Where the contract lives

- Schema:
  [`/schemas/workspace/warm_start_choice.schema.json`](../../../schemas/workspace/warm_start_choice.schema.json)
- Fixtures:
  [`/fixtures/workspace/m3/template_and_resume_cards/`](../../../fixtures/workspace/m3/template_and_resume_cards/)
- Rust projection, seed, validation, and plaintext:
  [`/crates/aureline-shell/src/start_center/warm_start_choice/mod.rs`](../../../crates/aureline-shell/src/start_center/warm_start_choice/mod.rs)
- Headless inspector:
  `cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- <subcommand>`

The shared contract ref `shell:start_center_warm_start_choice_beta:v1` is stamped
on every page, card, and export so a drifting surface can be caught.

## The object model

A `warm_start_choice_page_record` carries a list of
`warm_start_choice_card_record`s plus summary counters and the page-level
invariants. A `warm_start_choice_support_export_record` wraps a page for
support bundles and excludes raw secret material by construction.

Each card carries:

- **Identity and class** — `card_id`, `surface_origin`, `display_label`,
  `source_class` (`workspace_template`, `prebuild_snapshot`, `live_workspace`,
  `remote_repository`, `local_folder`), `support_class`, and
  `runtime_or_host_model` (`local_host`, `devcontainer`,
  `managed_cloud_workspace`, `ssh_workspace`). `local_first` records whether the
  resulting workspace claims a local-first runtime.
- **Template-gallery facts** — `expected_setup_actions`: the plain-language
  setup a starter would perform.
- **Snapshot facts** (when present) — `fingerprint_ref`, `freshness`
  (`fresh`/`cached`/`stale`/`invalidated`/`unverified`), `age_class`,
  `captured_at`, and the `invalidation_reason` when a snapshot is stale or
  invalidated. `stale_must_not_render_as_live_resume` is always `true`.
- **Side-effect summary** — `network_egress`, `extension_installs`,
  `setup_tasks`, `trust_prompt`, `managed_or_remote_attach`, plus reviewable
  `notes`.
- **Environment starter** — `setup_location_class` (where setup runs), whether
  `downloads`/`extensions`/`tasks`/`trust_prompt` are involved, and the
  `bypass_route_ids` / `defer_route_ids` that let the user open without setup or
  defer it.
- **Choice lanes** — one `choice_lane` per entry path the card offers
  (`resume_live_workspace`, `start_from_snapshot`, `clone_fresh`, `open_minimal`,
  `set_up_later`, `use_template`). Every lane carries its `availability`,
  `side_effect_class`, and the booleans `requires_trust_grant`,
  `requires_network`, `runs_setup_tasks`, `materializes_remote_work`, and
  `same_weight_local_path`.
- **Safe default** — `safest_next_action`, which always resolves to a
  local-safe lane; `default_widens_trust` and `default_runs_networked_work` are
  always `false`.
- **Honesty marker** — `honesty_marker_present`, true whenever a snapshot is
  stale/invalidated or a lane requires re-authorization, is disabled by a stale
  snapshot, or is blocked by policy.

## Invariants the validator enforces

`validate_warm_start_choice_page` returns the full list of findings (so a
reviewer sees every problem at once) and holds:

1. **Path clarity before commitment.** Every card exposes its lanes explicitly;
   the lane vocabulary is closed.
2. **The default is always local-safe.** `safest_next_action` must resolve to a
   lane with no network egress, no setup tasks, no trust grant, and no remote
   materialization. At least one local-safe lane must exist on every card.
3. **Open-minimal and set-up-later stay same-weight on local-first cards.** Both
   lanes must be present, `same_weight_local_path = true`, and local-safe.
4. **A stale snapshot is never a live resume.** When a snapshot is
   stale/invalidated the card names the `invalidation_reason` and the
   `resume_live_workspace` lane must not be takeable
   (`unavailable_stale_snapshot`).
5. **Remote/managed lanes cannot masquerade as a local open.** Any lane that
   requires the network, attaches managed/remote runtime, or widens trust must
   not advertise a local-safe side-effect class and must not be one of the
   local escape-hatch lanes.
6. **Disclosure consistency.** A trust prompt or managed attach on any lane must
   be named in the card's side-effect summary; a starter that runs setup
   anywhere must offer both bypass and defer routes; the snapshot lane is
   present iff the card carries snapshot facts; every closed-vocabulary token
   mirrors its enum.
7. **Surface consistency.** The page lists all five consuming surfaces
   (`start_center`, `workspace_switcher`, `cli_headless_entry_review`,
   `docs_help`, `support_export`) and summary counters match the cards.

## How this maps to the acceptance criteria

- *Users can tell whether Aureline will resume live, reuse a snapshot, clone
  fresh, or open local files before they commit* — the closed lane vocabulary
  with per-lane side-effect truth.
- *Template and prebuild choices never silently widen trust, run setup tasks, or
  materialize hidden networked work* — the local-safe default invariant plus the
  no-masquerade rule; trust/setup/network are disclosed on the lane and the
  card.
- *Open minimal and set up later remain same-weight on claimed local-first rows*
  — invariant 3.
- *Snapshot age, source class, and invalidation reason remain visible after
  restart and in support-safe exports* — the snapshot facts are part of the
  card and are carried verbatim into the support export.

## Out of scope

The projection is read-only. It does not create cloud control-plane workspaces,
productize collaboration/session-join, run setup tasks, mint credentials, or
perform any clone/resume itself. It only describes the choices so the user can
decide before anything happens.
