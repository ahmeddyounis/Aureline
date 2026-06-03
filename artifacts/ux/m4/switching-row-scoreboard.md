# Onboarding and migration switching-row scoreboard

Reviewer-facing scoreboard for marketed switching rows. Each row names its
effective claim level, the narrowing reasons that apply, and the proof-dimension
status that drives the derivation. No row inherits an adjacent green claim — each
row carries its own evidence or is narrowed with a named reason.

Canonical machine source (do not clone status text from this scoreboard — ingest
the JSON):

- Index: [`/artifacts/ux/m4/onboarding-and-migration-proof-index.json`](./onboarding-and-migration-proof-index.json)
- Companion doc: [`/docs/m4/onboarding-and-migration-proof.md`](../../../docs/m4/onboarding-and-migration-proof.md)
- Upstream marketed rows: [`/docs/migration/m3/marketed_switching_rows.md`](../../../docs/migration/m3/marketed_switching_rows.md)

## Legend

**Effective claim:** `stable` · `beta` · `preview` · `withdrawn`

**Proof state per dimension:**

| Symbol | Meaning |
|--------|---------|
| ✅ proven | Dimension has current evidence; claim is backed at the stated level. |
| 🔶 narrowed | Dimension is applicable but claim is narrowed below stable with a named reason. |
| — | Dimension is not applicable to this row. |

**Consumer surface wiring:** all rows show `wiring_pending` for all surfaces in
this pre-implementation repository. The canonical source for each surface is
listed in the JSON index; surfaces should ingest that source rather than
maintaining bespoke status text.

---

## Entry rows

| Row | Effective claim | No-account entry | Setup-later | Import diff/rollback | Gap taxonomy | First-useful-work | Narrowing reasons |
|-----|----------------|-----------------|-------------|----------------------|--------------|-------------------|-------------------|
| `switching_row:entry.local_open` | **stable** | ✅ stable | ✅ stable | — | — | ✅ stable | — |
| `switching_row:entry.clone` | **beta** | ✅ stable | ✅ stable | — | — | 🔶 beta | `first_useful_work_retest_pending` |
| `switching_row:entry.import` | **beta** | ✅ stable | ✅ stable | 🔶 beta | ✅ stable | ✅ stable | `rollback_evidence_incomplete` |
| `switching_row:entry.restore` | **stable** | ✅ stable | ✅ stable | — | — | ✅ stable | — |
| `switching_row:entry.missing_root_recovery` | **stable** | ✅ stable | ✅ stable | — | — | ✅ stable | — |
| `switching_row:entry.workspace_switch` | **stable** | ✅ stable | ✅ stable | — | — | ✅ stable | — |
| `switching_row:entry.archetype_routing` | **beta** | ✅ stable | ✅ stable | — | — | 🔶 beta | `archetype_qualification_retest_pending` |

### Entry row notes

- **`entry.import`** reflects the most conservative ecosystem in scope (JetBrains,
  Vim/Neovim, Emacs each have `rollback_evidence_incomplete`). VS Code import is
  individually stable; see the migration source rows below for per-ecosystem detail.
- **`entry.clone`** and **`entry.archetype_routing`** are narrowed to beta because
  archetype-based routing after clone stays retest-pending until the archetype
  qualification report exits retest-pending for each target archetype.

---

## Archetype rows

All archetype rows carry the beta archetype claim ceiling. The archetype
qualification report must exit retest-pending before any archetype row may widen
to stable.

| Row | Effective claim | No-account entry | Setup-later | Import diff/rollback | Gap taxonomy | First-useful-work | Dedicated fixture | Narrowing reasons |
|-----|----------------|-----------------|-------------|----------------------|--------------|-------------------|-------------------|-------------------|
| `beta_archetype:ts_web_app_or_service` | **beta** | ✅ stable | ✅ stable | — | — | 🔶 beta | ✅ yes (`certified-ts-web-app.json`) | `archetype_qualification_retest_pending` |
| `beta_archetype:python_service_or_data_app` | **beta** | ✅ stable | ✅ stable | — | — | 🔶 beta | ✅ yes (`probable-python-service.json`) | `archetype_qualification_retest_pending` |
| `beta_archetype:java_or_kotlin_service` | **beta** | 🔶 beta | 🔶 beta | — | — | 🔶 beta | ❌ missing | `archetype_qualification_retest_pending` |
| `beta_archetype:rust_workspace` | **beta** | 🔶 beta | 🔶 beta | — | — | 🔶 beta | ❌ missing | `archetype_qualification_retest_pending` |
| `beta_archetype:go_service_or_monorepo_slice` | **beta** | 🔶 beta | 🔶 beta | — | — | 🔶 beta | ❌ missing | `archetype_qualification_retest_pending` |
| `beta_archetype:c_or_cpp_native_project` | **beta** | 🔶 beta | 🔶 beta | — | — | 🔶 beta | ❌ missing | `archetype_qualification_retest_pending` |

### Archetype row notes

- TypeScript web app and Python service have dedicated fixtures in the archetype
  preflight corpus; Java/Kotlin, Rust, Go, and C/C++ do not. Dedicated fixtures
  are required before the archetype qualification report can exit retest-pending for
  those rows.
- All six archetype rows are narrowed to beta by the archetype claim ceiling
  regardless of individual dimension proof states.

---

## Migration source rows

| Row | Effective claim | No-account entry | Setup-later | Import diff/rollback | Gap taxonomy | First-useful-work | Rollback live | Narrowing reasons |
|-----|----------------|-----------------|-------------|----------------------|--------------|-------------------|---------------|-------------------|
| `migration_source:vs_code_code_oss` | **stable** | ✅ stable | ✅ stable | ✅ stable | ✅ stable | ✅ stable | ✅ yes | — |
| `migration_source:jetbrains_family` | **beta** | ✅ stable | ✅ stable | 🔶 beta | ✅ stable | ✅ stable | ❌ projected | `rollback_evidence_incomplete` |
| `migration_source:vim_neovim` | **beta** | ✅ stable | ✅ stable | 🔶 beta | ✅ stable | ✅ stable | ❌ projected | `rollback_evidence_incomplete` |
| `migration_source:emacs` | **beta** | ✅ stable | ✅ stable | 🔶 beta | ✅ stable | ✅ stable | ❌ projected | `rollback_evidence_incomplete` |

### Migration source row notes

- **VS Code / Code-OSS** is the only ecosystem with a live per-ecosystem apply
  session and verified rollback evidence. It qualifies stable across all dimensions.
- **JetBrains, Vim/Neovim, and Emacs** have diff-reviewed, taxonomy-complete flows
  but their rollback evidence is projected from the VS Code checkpoint rather than
  verified with a live per-ecosystem apply session. They stay beta until live
  rollback evidence is captured for each.
- Vim/Neovim carries an unsupported gap for Lua plugin runtime execution; Emacs
  carries an unsupported gap for Elisp package runtime execution. Both are
  disclosed before apply in the gap taxonomy.

---

## Summary

| Family | Total | Stable | Beta | Preview | Withdrawn |
|--------|-------|--------|------|---------|-----------|
| Entry | 7 | 4 | 3 | 0 | 0 |
| Archetype | 6 | 0 | 6 | 0 | 0 |
| Migration source | 4 | 1 | 3 | 0 | 0 |
| **Total** | **17** | **5** | **12** | **0** | **0** |

**Narrowing reason distribution:**

| Narrowing reason | Rows affected |
|------------------|---------------|
| `archetype_qualification_retest_pending` | 7 |
| `rollback_evidence_incomplete` | 4 |
| `first_useful_work_retest_pending` | 1 |

## Narrowing rule

This scoreboard is the reviewer-facing view over the proof index. If delivery
proves a narrower stable claim than the rows above, downgrade the affected row
in the proof index and re-emit this scoreboard rather than inheriting an adjacent
green row. The proof index, scoreboard, companion doc, and upstream marketed-rows
doc move together in the same change set.

A row's `effective_claim` is derived from the most conservative applicable
dimension; a row cannot hold a stable claim when any applicable dimension narrows
it. Consumer surfaces must present the per-ecosystem or per-row effective claim
from the proof index rather than an aggregate that hides narrowed sub-rows.

## Consumer surface wiring status

All consumer surfaces are currently `wiring_pending`. Each surface should ingest
the canonical source listed in the proof index rather than maintaining bespoke
status text:

| Surface | Canonical source |
|---------|-----------------|
| Start Center | `start_center_packet` / `archetype_preflight_packet` |
| Migration center | `migration_center_packet` |
| Help/About | Row-specific packet (first-run, migration, archetype, warm-continuity) |
| Release notes | `onboarding_migration_proof_index:m4.stable` (this index) |
| Docs pack | `docs/m4/onboarding-and-migration-proof.md` |
| Support playbook | Row-specific packet |

Wiring is the natural next step for each surface owner; consumer wiring does not
narrow the core proof claims in this index.
