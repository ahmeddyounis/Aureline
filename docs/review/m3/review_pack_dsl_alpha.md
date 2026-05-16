# Review-pack DSL alpha: repo-defined review packs with local/CI parity

The alpha review-pack DSL is the versioned, repo-defined declaration of
one bundle of review checks, ownership hints, and local/CI parity notes
the local and CI review-pack harnesses both consume. A review-pack
record is **pre-execution review truth**: it describes which checks
would run, who owns the affected scopes, and which fields are
unsupported on the current DSL version, without itself mutating any
branch, worktree, working tree, or remote.

A reviewer can answer five questions from a single record before either
the local or CI harness executes:

1. **Which pack am I about to run, and where does it live?** The
   `review_pack_id` and `repo_anchor_ref` pin the pack identity and repo
   anchor as opaque refs.
2. **Who authored or signed it?** `pack_authority_class` distinguishes
   first-party, team-shared, partner-signed, and uncertified-community
   packs so authority is never silently widened.
3. **What checks would run?** `checks` enumerates each check with its
   kind, severity class, parity class, and execution class.
4. **Who owns the affected scopes?** `ownership_hints` quote
   closed-vocabulary scope kinds and opaque owner refs.
5. **What is local-only, CI-only, or unsupported?**
   `parity_observations` and `unsupported_fields` make local/CI parity
   and forward-compat declarations explicit instead of leaving them to
   ad hoc plugin behavior.

The companion schema lives at:

- [`/schemas/review/review_pack.schema.json`](../../../schemas/review/review_pack.schema.json)

The canonical fixtures live under:

- [`/fixtures/review/m3/review_pack_dsl/`](../../../fixtures/review/m3/review_pack_dsl/)

The headless validator that gates every fixture lives at:

- [`/ci/check_review_pack_dsl_alpha.py`](../../../ci/check_review_pack_dsl_alpha.py)

The Rust types are exported from `aureline_review::review_pack_dsl`,
defined in
[`crates/aureline-review/src/review_pack_dsl/mod.rs`](../../../crates/aureline-review/src/review_pack_dsl/mod.rs).
The integration test
[`crates/aureline-review/tests/review_pack_dsl_alpha.rs`](../../../crates/aureline-review/tests/review_pack_dsl_alpha.rs)
replays every fixture and proves the closed acceptance states. The
first shell consumer is
[`crates/aureline-shell/src/review/review_pack_inspector/mod.rs`](../../../crates/aureline-shell/src/review/review_pack_inspector/mod.rs),
which renders deterministic review-pack rows directly from the
checked-in alpha fixtures and a matching CLI / headless plaintext export.

## 1 Why freeze this now

Review workflows already need a stable contract for "what would the
checks run, in what order, with what severity, and with what local vs.
CI parity guarantees" without forking that vocabulary into ad hoc plugin
metadata per surface. The change-object and change-lineage records
already name *where* a change will land; the review-pack record adds
*what would be checked* on that scope, with explicit ownership hints and
parity disclosures.

Freezing this now keeps three guarantees ahead of convenience:

- **Authority is named.** First-party, team-shared, partner-signed, and
  uncertified-community packs use distinct authority tokens so the local
  and CI harnesses can decide which packs they engage without silent
  authority widening.
- **Local and CI parity is explicit.** Every check declares a parity
  class, and every parity class used by any check must be backed by at
  least one `parity_observations` entry. A reviewer reading the row
  sees the parity claim and the reason it was made.
- **Unsupported fields are declared, not inferred.** Forward-compat,
  vendor extensions, experimental fields, and deprecation are all
  declared on the record so the harness never silently runs an
  unsupported feature.

## 2 Record shape

Every review-pack row is one `review_pack_alpha_record` carrying:

| Block | Required content |
| --- | --- |
| `review_pack_id` | Opaque, stable id quoted by support, CLI, and review surfaces. |
| `repo_anchor_ref` | Opaque repo-anchor id locating the pack inside the repo, never a raw path. |
| `display_label` | Short label for the inspector and review preview. |
| `summary` | Reviewable sentence summarising what the pack does. |
| `pack_authority_class` | One of `repo_first_party`, `repo_team_shared`, `repo_partner_signed`, `repo_uncertified_community`, `pack_authority_unknown_requires_review`. |
| `operator_caveat` | Reviewable sentence about authority and what running the pack will and will not do. |
| `checks` | Non-empty array of check definitions (`check_id`, `check_kind`, `severity_class`, `parity_class`, `execution_class`, optional `parity_note`, optional `ownership_scope_refs`). |
| `ownership_hints` | Array of ownership hints (`ownership_scope_id`, `ownership_scope_kind`, `display_label`, `owner_ref`, `summary`). |
| `parity_observations` | Non-empty array of `{parity_class, summary}` pairs explaining how local and CI execution relate. |
| `unsupported_fields` | Array of forward-compat / vendor / experimental / deprecated declarations with `field_path`, `unsupported_class`, and `summary`. |
| `consumer_surfaces` | Non-empty list drawn from `review_pack_inspector`, `review_preview`, `cli_headless_entry`, `support_export`, `docs_review`, `activity_center`; must include `review_pack_inspector`. |
| `support_export` | Packet refs and the closed `raw_path_export_allowed = raw_glob_body_export_allowed = raw_command_export_allowed = raw_check_output_export_allowed = false`. |
| `review_invariants` | All of `repo_anchor_pinned`, `checks_pinned`, `ownership_hints_pinned`, `local_ci_parity_declared`, `unsupported_fields_declared`, `no_hidden_writes` must be `true`. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Versioning is pinned.** `schema_version` and `dsl_version` both
   carry the alpha constant `1`. Bumping either is a deliberate event:
   adding a new closed-vocabulary value is additive-minor and bumps
   `schema_version`; repurposing a value or changing DSL semantics is
   breaking and bumps `dsl_version`.
2. **Checks reference declared ownership scopes only.** A check that
   lists an `ownership_scope_refs` entry must back it with an
   `ownership_hints` row in the same record. Phantom scope refs are
   rejected.
3. **Every check parity class is observed.** A check declaring
   `parity_class = X` requires at least one `parity_observations` entry
   with the same class. The harness can never claim a parity guarantee
   the record does not document.
4. **Closed export.** Raw paths, raw glob bodies, raw command lines,
   and raw check outputs are never exported through the review-pack
   record. Support packets quote opaque refs, class tokens, and short
   reviewable sentences only.
5. **Consumer wiring.** `consumer_surfaces` always includes
   `review_pack_inspector` so the first product surface stays bound.
6. **Pre-execution review.** The record is inspectable before the
   harness runs and writes nothing on its own. Local and CI lanes still
   own their own execution; the record describes them.

## 4 Vocabulary, by block

### 4.1 `pack_authority_class`

- `repo_first_party`
- `repo_team_shared`
- `repo_partner_signed`
- `repo_uncertified_community`
- `pack_authority_unknown_requires_review`

### 4.2 `check_kind`

- `policy_lint`
- `schema_validation`
- `ownership_review`
- `test_replay`
- `doc_freshness`
- `format_check`
- `custom_local`
- `check_kind_unknown_requires_review`

### 4.3 `check_severity_class`

- `advisory`
- `blocking`
- `informational`
- `severity_unknown_requires_review`

### 4.4 `parity_class`

- `local_and_ci_parity`
- `ci_only_documented`
- `local_only_documented`
- `parity_unknown_requires_review`

### 4.5 `execution_class`

- `deterministic_replay`
- `stateful_local_only`
- `stateful_ci_managed`
- `execution_class_unknown_requires_review`

### 4.6 `ownership_scope_kind`

- `path_glob_first_party`
- `path_glob_team`
- `path_glob_external_partner`
- `path_glob_uncertified_community`
- `ownership_scope_unknown_requires_review`

### 4.7 `unsupported_field_class`

- `future_dsl_version`
- `vendor_specific_extension`
- `experimental_local_only`
- `deprecated_pending_removal`
- `unsupported_class_unknown_requires_review`

### 4.8 `consumer_surface`

- `review_pack_inspector`
- `review_preview`
- `cli_headless_entry`
- `support_export`
- `docs_review`
- `activity_center`

## 5 Fixtures

The checked-in fixtures under
[`/fixtures/review/m3/review_pack_dsl/`](../../../fixtures/review/m3/review_pack_dsl/)
cover every authority class and a spread of parity classes:

| Fixture | `pack_authority_class` | Parity classes covered | Why it exists |
| --- | --- | --- | --- |
| `first_party_local_and_ci_parity.json` | `repo_first_party` | `local_and_ci_parity` | Bundled schema, doc, and ownership checks share one validator across local and CI. |
| `team_shared_mixed_parity.json` | `repo_team_shared` | `local_and_ci_parity`, `local_only_documented`, `ci_only_documented` | Team-shared pack with a local-only format check and a CI-only deploy gate, both documented. |
| `partner_signed_ci_only_lane.json` | `repo_partner_signed` | `ci_only_documented`, `local_and_ci_parity` | Partner-signed pack whose audit gate runs only in CI; local lane previews it. |
| `uncertified_community_local_only_lane.json` | `repo_uncertified_community` | `local_only_documented`, `parity_unknown_requires_review` | Uncertified community pack where CI declines to engage and one probe is parity-unknown. |

Every fixture keeps raw paths, raw glob bodies, raw command lines, and
raw check outputs closed; only opaque ref labels, closed-vocabulary
tokens, and short reviewable sentences cross the boundary.

## 6 Consumer wiring

The first product surface bound to this record is the shell review-pack
inspector in
[`crates/aureline-shell/src/review/review_pack_inspector/mod.rs`](../../../crates/aureline-shell/src/review/review_pack_inspector/mod.rs).
It builds a deterministic review-pack row per fixture and exports a
matching plaintext block (`render_alpha_review_pack_plaintext`) for
CLI / headless / docs / support consumers, proving the bundle is
inspectable and not doc-only.

## 7 Out of scope

- Full M6 collaboration and full cloud-control-plane productization.
- Executing the checks themselves: the record describes what would run
  in either lane; the local and CI harnesses still own their executions.
- Embedding raw glob bodies, raw command lines, or raw check outputs in
  the record. Those stay in the harnesses' execution envelopes.
- Mutating branches, worktrees, or working trees. The Git mutation,
  publish, branch, and conflict-handoff services still own writes.
