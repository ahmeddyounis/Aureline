# Error/Recovery Copy Objects and Degraded-State Reason Chips

This document is the contract for the error/recovery copy catalog. The catalog is
the single source of truth for the wording Aureline renders when a workflow fails
or runs in a degraded state. Dynamic banners, inline blockers, Project Doctor,
CLI/help summaries, support exports, screenshot/demo captions, and narrated
surfaces resolve recovery copy through this catalog rather than inventing
surface-local failure text.

Where the [safety-critical string catalog](./m5_safety_critical_string_catalog.md)
locks *which* state terms are governed, this catalog locks the *shape* of a recovery
explanation: a structured failure object that can never collapse into a generic
"something went wrong".

- Record kind: `m5_error_recovery_copy_catalog`
- Schema: [`schemas/content/m5-error-recovery-copy.schema.json`](../../../schemas/content/m5-error-recovery-copy.schema.json)
- Canonical support export: [`artifacts/content/m5-recovery-copy-proof/support_export.json`](../../../artifacts/content/m5-recovery-copy-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-recovery-copy-proof/m5_error_recovery_copy.md`](../../../artifacts/content/m5-recovery-copy-proof/m5_error_recovery_copy.md)
- Fixtures: [`fixtures/content/m5-error-recovery-copy/`](../../../fixtures/content/m5-error-recovery-copy/)
- Producer: `aureline_shell::content::error_patterns::current_error_recovery_copy_catalog_export`
- Headless emitter: `aureline_shell_m5_error_recovery_copy`

## Recovery blocks

A `RecoveryBlock` is a four-part error/recovery object. Its structure forces a
complete explanation — a surface can never stop at the failure:

- **`what_failed`** — a bounded statement of what failed.
- **`why_likely`** — the likely cause, hedged honestly.
- **`what_still_works`** — what remains safe to do locally / what still works.
- **`next_action`** — a verb-first `NextAction` label with a `RecoveryLink`.

Each block also carries a `failure_domain` (`runtime`, `network`, `repair`,
`install`, `review`, `docs_help`), a `severity` (`notice`, `caution`, `warning`,
`critical`, `blocking`), the `reason_chips` it embeds, and the `consumer_surfaces`
that must resolve it. `render_block_reference` composes the whole explanation with
chips resolved and variables left as named slots, so a support export reconstructs
exactly the explanation the user saw in-product.

## Degraded-state reason chips

A `ReasonChip` is a reusable, first-class degraded-state object: one reserved
meaning, one locale-neutral machine token (aligned with the controlled glossary's
state vocabulary), a canonical label, a severity, `self_heals` / `offers_recovery`
flags, and the surfaces it may appear on. The catalog carries one chip per
`DegradedState`, exactly the states the recovery flows enumerate:

- `restricted` — permitted only within a narrower, disclosed scope.
- `partial_index` — index still building; results cover only the indexed part.
- `remote_host` — work depends on a remote host whose reachability is not guaranteed.
- `policy_blocked` — an active policy blocks the action on this deployment.
- `cached` — data shown from a local cache, not proven current.
- `stale` — prior data shown after its freshness floor was passed.
- `reconnecting` — a connection is being re-established; self-heals when it returns.
- `rollback_available` — a prior, known-good state can be restored.

Every chip is `grounded`: stated in technically grounded cause/boundary language and
never softened into a playful, anthropomorphic, or euphemistic phrase.

### Copy lives in the catalog, not the literal string

A copy line never inlines a chip label. Its `reference_template` is built from
placeholders:

- `{chip:<chip_id>}` resolves against the chip register to the chip's canonical label.
- `{var:<name>}` resolves against the line's declared variables.

This is what makes the catalog — not a scattered literal — the source of truth.
`render_block_reference` resolves a block's chips while leaving variables as named
slots, so the same chip resolves identically across every surface.

### Variable semantics

Each `CopyVariable` declares a `role` (`entity_name`, `location`, `code`, `count`,
`duration`, `scope_label`), whether its *value* is locale-neutral (codes, counts,
durations are), and whether it is `truncatable`.

### Next actions and recovery links

A `NextAction` carries a locale-neutral `action_id`, a verb-first `label`, its
declared variables, and a `RecoveryLink`. The label must open with a controlled
recovery verb (`Reconnect`, `Retry`, `Rebuild`, `Request`, `Roll`, `Refresh`, …) —
never a vague `Continue`, `Accept`, or `Submit`. The `RecoveryLink` (`docs_topic`,
`help_topic`, `repair_flow`, `settings_pane`, `reconnect_flow`, `rollback_flow`,
`support_export`) must be `offline_available`, so the recovery entry point is
reachable even while the user is degraded or disconnected.

## Locale neutrality

Machine-facing identity stays locale-neutral while human prose localizes around it.
Block ids, chip ids, machine tokens, link ids, target refs, variable names, and the
`{chip:...}` / `{var:...}` placeholders are lowercase ascii (`[a-z0-9_.]`). Only the
canonical labels, reserved meanings, and reference templates carry human prose. The
localized overlay fixture rewrites every copy-line template into a pseudo-localized
form while keeping every id and placeholder byte-for-byte identical — proving a
translation can never fork the meaning of a failure or a degraded state.

## Cross-surface reuse

The same chip objects are reused across banners, inline blockers, Project Doctor,
CLI/help summaries, support exports, and captions. The `shared_reuse_chip_ids` —
`policy_blocked`, `stale`, `cached` — must each span at least
`SHARED_CHIP_MIN_REUSE_SURFACES` (3) distinct consumer surfaces.
`cross_surface_reuse` maps each chip to the surfaces that embed it, and validation
fails if a shared chip collapses to fewer surfaces.

## Validation invariants

`ErrorRecoveryCopyCatalog::validate` enforces, among others:

- record kind, schema version, and identity are present;
- the four closed inventories match the canonical token lists;
- chip ids and machine tokens are unique and locale-neutral, every chip is
  `grounded`, and there is one chip per degraded state;
- block ids are unique and locale-neutral, and every block carries all four parts
  (`what_failed`, `why_likely`, `what_still_works`, `next_action`);
- `what_still_works` always says something remains — never "nothing";
- the next-action label is verb-first and its recovery link resolves offline;
- every `{chip:...}` / `{var:...}` placeholder resolves to a declared chip/variable,
  and every declared chip/variable is used by its template;
- a chip is only embedded on a surface its `allowed_surfaces` permits;
- no block copy uses playful, anthropomorphic, or generic failure language;
- every failure domain, severity, and consumer surface is represented;
- each shared reuse chip spans at least three surfaces;
- the trust-review and consumer-projection invariants all hold;
- the export carries no raw boundary material.

## Fixtures

The fixtures are valid, export-safe catalog packets minted from the same seed
builder as the canonical export by `aureline_shell_m5_error_recovery_copy`. See
[the fixtures README](../../../fixtures/content/m5-error-recovery-copy/README.md).
