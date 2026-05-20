# Command-truth and palette-authority conformance (beta)

This is the beta contract for the M3 command-truth and palette-authority proof
lane. It turns the one-command-graph promise into current, regression-gated
evidence instead of trusting generated help: every claimed beta command is
validated against the way the palette, keybindings, menus/buttons, CLI/headless,
AI, recipe, voice, and browser-companion surfaces actually reach it.

## Boundary under proof

- Runtime model: [`aureline_commands::CommandAuthorityScenarioRecord`](../../../crates/aureline-commands/src/authority.rs)
- Canonical descriptor schema: [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
- Conformance-result schema: [`/schemas/commands/command_conformance_result.schema.json`](../../../schemas/commands/command_conformance_result.schema.json)

A `CommandAuthorityScenarioRecord` bundles one canonical command descriptor with
the enablement decision, preview/approval posture, and authority every claimed
invocation surface reports, plus the invocation-lineage join a support export
must be able to reconstruct. It does not execute commands; it composes the
surfaces and lineage of one command into a single validated, support-safe
artifact so the palette, keybinding, CLI, AI, recipe, voice, and
browser-companion surfaces can be proven to project from the *same* descriptor.

## Corpus and harness

- Corpus: `fixtures/commands/m3/command_truth_and_authority/`
- Harness: [`crates/aureline-qe/src/command_truth_authority/`](../../../crates/aureline-qe/src/command_truth_authority/)
- Replay: `cargo test -p aureline-qe --test command_truth_authority_conformance`
- Regenerate: `python3 tools/regenerate_command_truth_authority_corpus.py --write`

`manifest.json` is authoritative. Positive drills MUST parse, validate, project,
and match **every** `expected_*` field in the manifest. Negative drills MUST FAIL
validation with an error whose message contains `expected_failure_substring`. The
fixtures carry only the scenario records and a `$schema`/`__fixture__` prelude —
they do **not** restate the expectations, so there is exactly one place to read
and audit the pinned truth.

## What the corpus proves

### Cross-surface authority parity

The same canonical command keeps **one** enablement decision, **one**
preview/approval posture, and **one** result contract across menu/button,
keybinding, palette, CLI/headless, AI, recipe, voice, and browser-companion
surfaces. The full-cross-surface drill exercises all eight surfaces for a
reversible command; the high-risk drill proves a structured-diff preview and an
explicit approval stay enforced on every surface (and that the command stays off
the AI tool surface because its descriptor is not AI-callable); the
disabled-with-reason drill proves every surface agrees on the same denial and
disabled-reason code.

### No authority widening or requirement suppression

No invocation surface may widen authority or suppress a preview or approval
requirement relative to the canonical descriptor. The validator rejects:

| Regression | Rejected with |
| --- | --- |
| AI tool reaching a not-AI-callable command | `surface ai_tool widens authority` |
| CLI/headless reaching a non-`headless_safe` command | `surface cli_headless widens authority` |
| A surface declaring a weaker preview class | `suppresses the preview requirement` |
| A surface declaring a weaker approval posture | `suppresses the approval requirement` |
| Surfaces disagreeing on enablement | `diverges from the canonical enablement decision` |

### Automation-label honesty and alias canonicalization

Automation labels (`macro_safe`, `recipe_safe`, `headless_safe`, `ui_only`,
`approval_required`) stay truthful: an `approval_required` label must match the
descriptor's approval posture, and a `ui_only` command may neither advertise an
automation label nor be reached from a CLI/headless, recipe, or AI surface.
Migration aliases stay generated from the canonical descriptor record — every
alias must resolve to the canonical command id, so a deprecated alias never
becomes a shadow command.

### Invocation-lineage reconstruction

A support export reconstructs invocation lineage from a command id through the
result packet, evidence ref, notification/activity row, and rollback handle. A
durable command that actually applied a mutation must join a reversible rollback
handle; a denied invocation changed nothing and joins no rollback handle but
still reconstructs through its denial evidence and activity row.

### Machine-readable lifecycle and automation metadata

Stable, beta, LTS-facing, and deprecated commands must carry machine-readable
lifecycle and automation metadata (automation labels, category and
discoverability refs, and invocation/result schema refs). A stable command that
drops its automation metadata is rejected before a beta row hardens, so docs/help
and migration aliases cannot fall back to hand-maintained shadow data.

## Published evidence

- Parity report: [`artifacts/ux/m3/command_truth_and_authority_parity_report.md`](../../../artifacts/ux/m3/command_truth_and_authority_parity_report.md) — one row per claimed command plus the negative guards.
- Release evidence packet: [`artifacts/release/m3/command_invocation_evidence_packet.json`](../../../artifacts/release/m3/command_invocation_evidence_packet.json) — machine-readable per-command projection with the reconstructable lineage chain.

The conformance suite asserts that both artifacts cover every drill id and the
corpus id, so the published parity and evidence truth cannot drift from the
corpus.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw secrets, private keys, credentials, raw local paths, hostnames,
command lines, logs, and source content never appear. The runner scans each
fixture for forbidden raw-content tokens before validation. Removing any positive
or negative drill without a replacement is a breaking contract change for the
`commands.command_truth_and_authority.beta` corpus.
