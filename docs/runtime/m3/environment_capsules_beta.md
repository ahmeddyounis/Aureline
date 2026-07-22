# Beta environment-capsule resolver

This document is the reviewer-facing landing page for the beta finalize layer
of the environment-capsule resolver. It pins the closed source vocabulary the
beta resolver inspects, the precedence ladder it follows, the typed
confidence and conflict labels every parsed source carries, and the typed
drift evaluator that invalidates stored bindings the moment a declarative
input changes.

The machine-readable boundary lives at
[`/schemas/runtime/environment_capsule_beta.schema.json`](../../../schemas/runtime/environment_capsule_beta.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/capsule_resolver/beta.rs`](../../../crates/aureline-runtime/src/capsule_resolver/beta.rs);
the alpha resolver it extends still lives at
[`/crates/aureline-runtime/src/capsule_resolver/mod.rs`](../../../crates/aureline-runtime/src/capsule_resolver/mod.rs).

The beta promise:

- **One capsule model.** Devcontainer, Nix, and Compose inputs project onto
  one [`EnvironmentCapsuleBetaResolution`](../../../crates/aureline-runtime/src/capsule_resolver/beta.rs)
  body that wraps the alpha resolution verbatim and adds a sources ledger,
  a precedence ladder, a source-set digest, a drift label, and a
  source-bound capsule reference. Every field is read-only: no container is
  spawned, no Nix evaluator is invoked, and no lifecycle hook is executed.
- **Typed confidence and conflict labels.** Every parsed source is stamped
  with one of three confidence tokens — `imported`, `heuristic`,
  `unsupported`. Sources that did not shape the primary binding carry the
  `overridden_by_higher_precedence` note so the conflict is visible rather
  than silently merged.
- **Bounded source evidence.** Every source body is read through a
  workspace-contained, no-redirect, identity-checked reader. Symlinks and
  Windows junction/reparse-point descendants are rejected. A file is limited
  to 8 MiB and one resolution is limited to 32 MiB. Present sources that fail
  those checks remain in the ledger with a typed read state; they are never
  silently treated as absent or hashed as an empty body.
- **Drift you can act on.** A typed
  [`evaluate_capsule_drift`](../../../crates/aureline-runtime/src/capsule_resolver/beta.rs)
  evaluator compares a stored
  [`CapsuleBetaSourceBaseline`](../../../crates/aureline-runtime/src/capsule_resolver/beta.rs)
  against a freshly resolved beta resolution. The outcome is one of
  `in_sync`, `stale_inputs`, `manually_diverged`, or `unknown_lineage`.
  Drift is surfaced rather than blocking — local inspection and editing
  remain safe even when the source set has drifted.
- **Exportable evidence.** The
  [`EnvironmentCapsuleBetaSupportExport`](../../../crates/aureline-runtime/src/capsule_resolver/beta.rs)
  packet projects the canonical coverage manifest, the resolved beta
  resolution, and any drift evaluations the support flow attached. Raw
  bodies, raw command lines, and raw secrets are out of scope.

## Source vocabulary and precedence

The beta resolver inspects the following declarative inputs in this rank
order. Lower rank wins.

| Rank | Source class | What the resolver reads | Default confidence |
| --- | --- | --- | --- |
| 0 | `devcontainer` | `devcontainer.json` or `.devcontainer/devcontainer.json` | `imported` |
| 1 | `docker_compose` | `docker-compose.yml` / `.yaml` / `compose.yml` / `.yaml` | `imported` |
| 2 | `nix_flake` | `flake.nix` (digest only — body is not evaluated) | `unsupported` |
| 3 | `nix_shell` | `shell.nix` (digest only) | `unsupported` |
| 4 | `nix_default` | `default.nix` (digest only) | `unsupported` |
| 5 | `node_manifest` | `package.json` plus lockfile family | `imported` |
| 6 | `python_manifest` | `pyproject.toml`, `.python-version`, lockfile family | `imported` |

Devcontainer wins over Compose because devcontainer.json typically references
the compose file it should reuse; surfacing the devcontainer body keeps the
parsed view aligned with how the user authored the workspace.

## Confidence labels

| Label | Meaning |
| --- | --- |
| `imported` | The body parsed cleanly into structured tokens. |
| `heuristic` | The body parsed but at least one field had to fall back to a heuristic (malformed body, missing required field, dependent source missing). |
| `unsupported` | The file class is recognised but the contract does not parse the body (Nix sources). The content digest is still tracked so drift detection still applies. |

## Source read states

| State | Meaning |
| --- | --- |
| `complete` | The exact source bytes were read under containment, identity, per-file, and aggregate byte checks. |
| `unavailable` | The source is present but is a symlink or Windows junction/reparse-point descendant, escapes the workspace, is not a regular file, changed during the read, or could not be read safely. `content_digest` is `null`. |
| `resource_limit_exceeded` | The source exceeded the 8 MiB per-file or 32 MiB aggregate budget. `content_digest` is `null`. |

Any non-`complete` source makes capsule lineage `unknown_lineage` and rejects
prebuild reuse as drifted. When the primary source is incomplete, the resolver
also mints an `.unavailable` capsule id. This keeps local inspection available
without presenting incomplete evidence as current.

## Source notes

| Note | Meaning |
| --- | --- |
| `body_unparseable` | The body could not be parsed against the expected JSON / YAML grammar. |
| `required_field_missing` | A required field for the body shape was missing. |
| `dependent_source_missing` | The body referenced a sibling source the resolver could not locate. |
| `unsupported_body_parse` | The contract does not parse this body; drift tracking still applies. |
| `overridden_by_higher_precedence` | The source was parsed but did not shape the primary capsule binding. |
| `unknown_field_kept` | The body declared a feature outside the beta vocabulary. |
| `source_read_unavailable` | The bounded workspace reader could not obtain a stable, contained regular-file body. |
| `source_resource_limit_exceeded` | The source exceeded the per-file or aggregate read budget. |
| `body_invalid_utf8` | Exact bytes were read and digested, but the text body was not valid UTF-8 and could not be parsed. |

## Drift outcomes

| Outcome | Meaning |
| --- | --- |
| `in_sync` | Stored source-set digest matches the freshly resolved digest. |
| `stale_inputs` | At least one source body changed content. |
| `manually_diverged` | Sources were added or removed since the stored snapshot. |
| `unknown_lineage` | Stored snapshot referenced no sources, or a present source lacks complete bounded-read evidence. |

## Capsule binding

The capsule reference the beta resolver mints encodes both the primary source
and the source-set digest:

- `capsule_id` is the primary-source-bound identifier (for example
  `capsule.beta.devcontainer.parsed`, `capsule.beta.compose.parsed`,
  `capsule.beta.nix_flake.metadata`, `capsule.beta.unknown.uncertain`).
- Direct `content_digest` values are SHA-256 over the exact raw bytes. Node
  and Python multi-file rows, source-set digests, and capsule hashes use
  domain-separated SHA-256 with an explicit part count and unsigned 64-bit
  big-endian byte length before every ordered part. This prevents ambiguous
  concatenations such as `("ab", "c")` and `("a", "bc")` from sharing an
  encoding.
- `capsule_hash` is the length-framed SHA-256 of `"capsule.beta"`,
  `capsule_id`, `source_set_digest`, and `archetype_hint`. Editing any source
  body advances the source-set digest, which advances the capsule hash, which
  causes a downstream
  [ticket-drift evaluator](execution_context_beta.md) to invalidate any
  stored binding.

## Failure-drill fixtures

Reviewer fixtures live under
[`/fixtures/runtime/m3/environment_capsules/`](../../../fixtures/runtime/m3/environment_capsules/)
and exercise these scenarios:

- `devcontainer_only_case.json` — clean devcontainer parse with imported
  confidence and no conflict notes.
- `devcontainer_with_compose_case.json` — devcontainer wins precedence over
  the sibling compose body and the compose source carries
  `overridden_by_higher_precedence`.
- `compose_only_case.json` — standalone compose parse mints a
  compose-class capsule.
- `nix_flake_case.json` — flake.nix is recognised, digested for drift, and
  marked unsupported because the contract does not embed a Nix evaluator.
- `conflict_devcontainer_nix_compose_case.json` — three input families
  coexist; devcontainer wins precedence and the conflict notes record the
  override.
- `empty_workspace_case.json` — workspaces with no declarative inputs mark
  capsule lineage unknown.
- `drift_after_edit_case.json` — editing the devcontainer body advances the
  source-set digest and the drift evaluator returns `stale_inputs`.
- `source_added_drift_case.json` — adding a new declarative input returns
  `manually_diverged` with the new source listed under added_sources.
- `beta_source_coverage.json` — canonical coverage manifest the runtime
  emits.

The integration test that replays these fixtures lives at
[`/crates/aureline-runtime/tests/capsule_resolver_beta.rs`](../../../crates/aureline-runtime/tests/capsule_resolver_beta.rs).

## Out of scope for this beta

- Full Nix evaluation: the contract does not embed a Nix evaluator. Nix
  sources are tracked by digest only, so drift detection works but the
  parsed-fields body remains opaque.
- Devcontainer feature execution and lifecycle hook execution.
- Compose `up` / image pull side effects.
- Cross-workspace capsule import.

## Version 2 migration

Schema version 2 and resolver token `environment_capsule_resolver.beta.v2`
replace the earlier SHA-256-shaped pseudo-hashes with actual SHA-256, require
`read_state` plus `read_state_token` on every source row, allow
`content_digest: null` only when the read is incomplete, and add the three
failure notes above. Digest values minted by resolver v1 are not compatible
with v2 baselines: callers must resolve a fresh baseline rather than comparing
or reusing a v1 digest. The alpha resolver likewise advances its implementation
token to `environment_capsule_resolver.alpha.v2`; its record schema remains v1
because no alpha fields or vocabularies changed.

## How to verify

```sh
cargo test -p aureline-runtime --lib capsule_resolver::beta
cargo test -p aureline-runtime --test capsule_resolver_beta
```

## Cross-references

- Alpha capsule resolver — [`environment_capsule_alpha.md`](../environment_capsule_alpha.md)
- Capsule body schema — [`/schemas/runtime/environment_capsule.schema.json`](../../../schemas/runtime/environment_capsule.schema.json)
- Beta execution-context resolver — [`execution_context_beta.md`](execution_context_beta.md)
- Devcontainer profile contract — [`../container_devcontainer_contract.md`](../container_devcontainer_contract.md)
