# TypeScript / JavaScript web app or service

## Row binding

- Archetype row id: `archetype_row:ts_web_app_or_service`
- Archetype id: `ts_web_app`
- Initial support class: `experimental`
- Target support class: `certified`
- Inclusion target: `first_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

TypeScript on Node.js with pnpm or npm, React or Vite or Next.js, and
Vitest or Jest as the in-repo test runner. The archetype detector
classifies the row through a `package.json` plus at least one
TypeScript route module and one TypeScript test module; the synthetic
seed at `refws.ts_web_app_archetype_seed` carries those exact files
without vendoring a real third-party scaffold.

## Required-mode rationale

- `local_only` — the in-repo path covers first-open, dependency
  declaration, and basic edit/test flows without leaving the developer
  machine.
- `local_plus_one_remote_mode` — TypeScript web work is the canonical
  case for remote attach (devcontainer, codespace, or remote-agent).
  The certified-archetype report has to demonstrate at least one
  remote mode to back top-level launch claims.

## Evidence already on file

- Reference workspace: `refws.ts_web_app_archetype_seed`
  ([fixture](../../workspaces/reference/ts_web_app_archetype_seed.json)).
- Corpus scenarios:
  `archetype.ts_web_app_first_open_certified`,
  `archetype.ts_web_app_start_from_prebuild_bypass`,
  `workflow.first_useful_edit_ts_web_app`.
- Claim manifest row: `claim_row:certification.launch_wedge_typescript_web`
  in [`/artifacts/governance/claim_manifest_seed.yaml`](../../../artifacts/governance/claim_manifest_seed.yaml).
- Reserved task-success corpus id: `fixture.archetype_ts_web_app_seed`.

## Open evidence questions

- Materialise a remote-mode workflow scenario before the row may move
  out of `experimental`. The seed fixture covers local mode only.
- Stand up a certified-archetype report that names hardware,
  toolchain (`node_js_declared_only` is the seed placeholder), and
  the local-plus-one-remote matrix dimension.
- Decide whether `pnpm` or `npm` is the canonical package manager for
  the certified path or whether the row admits both as supported
  variants.
