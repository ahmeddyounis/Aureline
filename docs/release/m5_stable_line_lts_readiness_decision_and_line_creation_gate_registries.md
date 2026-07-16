# M5 LTS-readiness-decision and line-creation-gate registries

This lane makes any future LTS promise evidence-bearing over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It requires a 91–180 day LTS-readiness decision
packet that proves backport discipline, rollback maturity, support-window posture, and mirror/air-gap continuity
before a stable line may present itself as LTS-grade, and it gates LTS line creation or LTS-style language on the
existence of a green decision packet rather than on age or enterprise demand alone. It records the
*LTS-readiness-decision* grammar (the decision packet published per candidate line — one typed decision section per
operating proof: a backport-branch-posture section, a correction-line-health section, a rollback-evidence section, a
support-window-posture section, a mirror/air-gap continuity-proof section, and an advisory/revocation-readiness
section — each bound to the named decision-forum outcome and the preserved rollback / support evidence snapshot that
justified it) and the *line-creation-gate* grammar (whether a candidate line is LTS committed on a green packet,
LTS blocked because its packet is missing or its evidence is stale, or LTS narrowed back to a plain stable posture,
naming the active gate reason) into registry resolvers that produce export-safe, honest projections, so
release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces resolve one
canonical LTS-readiness truth instead of restating a line's LTS posture by hand. The decision packet and the
creation gate are separated in runtime and serialized state: the published section, support-window proposal,
backport branch posture, correction-line health, rollback evidence, mirror/air-gap proof, and forum outcome live on
the decision-packet entry, while the resolved line identity, linked decision-packet reference, required-packet
reference, gate-scope state, narrowed-posture state, active gate reason, and last revision live on the
line-creation-gate entry, and a candidate's rollback / support posture stays preserved so LTS language never widens
ahead of the current decision packet.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-lts-readiness-decision.schema.json`](../../schemas/program/m5-lts-readiness-decision.schema.json)
  (reused from the frozen matrix — the LTS-readiness decision packet each decision section records)
  and
  [`schemas/program/m5-lts-line-creation-gate.schema.json`](../../schemas/program/m5-lts-line-creation-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first inspectable LTS-readiness
  decision packet — it demonstrates backport discipline, rollback evidence, support-window posture, mirror/air-gap
  proof, and forum outcome end to end for at least one candidate LTS line.
- **Narrowed fixtures:**
  `fixtures/release/m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries/`
  (`lts_readiness_decision_beta_narrowed.json`, `line_creation_gate_preview_narrowed.json`).

## Two registries

1. **LTS-readiness decision** (`resolve_lts_readiness_decision_entry`) — records one typed decision section per
   operating proof, per candidate line: the decision section and its canonical mode, the candidate line rows, the
   support-window proposal, backport branch posture, correction-line health, rollback evidence, mirror/air-gap
   continuity proof, and advisory/revocation readiness, and the named decision-forum outcome. A clean entry names a
   canonical registry token, a classified decision section, and a stable-line-protection role, covers the
   canonical / accessible / audit resolution forms, publishes a complete object, preserves its rollback / support
   posture before LTS language widens, and keeps a public-facing section's support-window / continuity claim matched
   to current rollback and support evidence. Otherwise it degrades honestly — a candidate widening its LTS language
   while its packet is unresolved, or a public-facing section running its support-window / continuity language ahead
   of the current evidence, degrades to
   `lts_readiness_decision_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason an LTS-widen-without-green-packet attempt must surface.
2. **Line-creation gate** (`resolve_line_creation_gate_entry`) — gates LTS line creation and LTS-style language on a
   green decision packet. A clean entry names a classified gate scope (LTS committed, LTS blocked because the packet
   is missing or stale, or LTS narrowed back to stable) and provides the complete line-identity / linked-decision /
   required-packet / gate-scope / narrowed-posture / active-reason / last-revision gate object; a gate that would
   keep LTS language ahead of the current packet, hide the gate, or let a missing packet masquerade as covered
   degrades to
   `line_creation_gate_runs_support_ahead_of_proof_or_drops_line_creation_gate`.

## Per-entry decision reference

The published section carries its canonical mode, and the resolver publishes the full decision object, so the
registry — never an LTS claim assumed to have been earned — is the single source of truth.
`lts_readiness_decision_object_is_complete` rejects an object missing any decision field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects LTS language widening while a candidate's packet
is unresolved or its support-window / continuity language running ahead of current evidence, and
`line_creation_gate_stays_honest` rejects a gate that has kept LTS language ahead of the current decision packet.

## Acceptance criteria (proven by resolved examples)

- **Any candidate LTS line has an inspectable decision packet showing backport discipline, rollback evidence,
  support-window posture, mirror/air-gap proof, and forum outcome.** Clean decision-packet entries cover the
  canonical backport-branch-posture / correction-line-health / rollback-evidence / support-window-posture /
  mirror-air-gap-proof / advisory-revocation-readiness sections and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  decision-packet entry published an incomplete object.
- **No LTS label or promise can widen without a green decision packet; blocked or missing packets force a narrower
  stable-line posture instead.** An LTS-widen-without-green-packet example and an unbound example degrade, a clean
  bounded decision-packet entry is present, and no clean entry is unbounded or unbound.
- **Release/help/support/public-proof consumers can explain why a line is or is not LTS-ready using packet-backed
  facts rather than generic enterprise-language placeholders.** Clean line-creation-gate entries cover the
  LTS-committed / LTS-blocked-missing-packet / LTS-narrowed-to-stable gate scopes with full resolution-form
  coverage while providing the complete gate object — the resolved line identity and the active gate reason — and a
  gate that would keep LTS language ahead of the current packet or drop the gate degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- lts-readiness-decision-table
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- fixture-lts-readiness-decision-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries -- fixture-line-creation-gate-preview-narrowed
```
