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
  packet is an independently versioned, default-redacted support projection.
  It re-derives bounded source, capsule, detector, parsed-shape, and drift
  metadata instead of embedding the beta resolution. Raw bodies, workspace
  paths, source refs, detector payloads, parsed private strings, command
  lines, secrets, and the caller's manifest id are out of scope. Every
  included, transformed, omitted, or unavailable field family has an
  explicit disposition.

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

## Support-export privacy projection

The core beta resolution, drift, and coverage records remain schema version
2. The support-export record advances independently to schema version 3
because its boundary is intentionally narrower than the runtime record it
summarizes. A support packet is suitable for local preview under
`support.redaction.local_first_default`; it is never a hidden telemetry
upload and still requires a person or an explicitly governed workflow to
share it.

Every v3 packet pins these governance fields:

| Field | Required value |
| --- | --- |
| `purpose` | `environment_capsule_resolution_support` |
| `data_class` | `environment_adjacent` |
| `redaction_class` | `metadata_safe_default` |
| `redaction_profile_ref` | `support.redaction.local_first_default` |
| `export_posture` | `included_metadata_only` |
| `raw_private_material_exported` | `false` |

The constructor does not trust caller-provided tokens, digests, or counts.
It rebuilds source-class and precedence tokens from enums, de-duplicates and
bounds source rows to the seven-class vocabulary, re-derives all collection
counts, validates SHA-256 tokens before retaining them, and replaces malformed
digest claims with an explicitly classified redacted rehash where drift
comparison still needs an opaque token. A claimed `complete` source without a
valid SHA-256 content digest is downgraded to unavailable in the projection.
The projected source-set and capsule-binding digests are built from the
sanitized projection; the raw resolution's claimed source-set digest,
capsule id, and capsule hash are never copied.

The safe source projection includes only:

- source class, precedence rank, primary-source flag, read state, confidence,
  and bounded closed-vocabulary notes;
- a profile-scoped digest of the source ref, never the source ref itself;
- a validated content digest when complete bounded-read evidence exists;
- booleans and re-derived collection counts for recognized parsed shapes.

For devcontainers this means presence booleans plus feature and lifecycle-hook
counts, not image, Dockerfile, Compose, service, feature, or hook strings.
Compose exports a service count plus image/build booleans, never service keys.
Node and Python rows export lockfile counts, never lockfile refs. Nix exports
only its metadata-only shape; the raw variant token is unnecessary because
the source class already identifies the variant. Alpha Node/Python detector
reports become presence, fallback, failure, and ambiguity-count summaries;
their paths, provenance cards, requirements, candidates, values, summaries,
interpreter refs, and environment refs are omitted.

Support collections are bounded to seven projected sources, seven rows per
drift evaluation, and 32 drift evaluations per packet. Observed, exported,
and omitted counts make truncation visible. `absence_summary` uses
`unknown_until_field_disposition_present`, and `field_dispositions` follows
the security contract's closed vocabulary. In particular,
`omitted_by_redaction` is distinct from `not_recorded_by_design`, while an
unreadable source uses `unavailable_source`. The ledger covers the raw
manifest id, timestamps that fail the strict UTC shape, workspace and source
paths, the alpha/detector body, every parsed private-string group, raw
precedence/token fields, unvalidated digest/capsule claims, and truncated
collection tails.

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
- `redacted_support_export_v3.json` — schema-valid support projection with
  bounded drift evidence, the governed privacy posture, and explicit field
  dispositions; it contains no raw resolution, path, source-ref, detector, or
  parsed-private-string fields.

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

## Support-export version 3 migration

Support-export v3 replaces the v2 packet that embedded
`EnvironmentCapsuleBetaResolution` and raw drift records. Consumers must now
read `coverage_projection`, `resolution_projection`, and the bounded safe
`drift_evaluations` projection. `manifest_id` becomes `manifest_id_digest`;
the export declares its data class, redaction class/profile, export posture,
and raw-private-material posture; and absence is explained through
`absence_summary` plus `field_dispositions`. Resolution, drift, and coverage
records themselves do not change version. A consumer that requires raw local
resolution details must inspect the local v2 runtime record and must not
reinterpret the v3 support packet as an opt-in raw export.

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
