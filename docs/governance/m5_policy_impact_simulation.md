# M5 Policy-Impact Simulation (Pre-Apply Preview)

This page is the human-readable companion to the canonical M5 policy-impact
simulation packet emitted by `aureline-records` and mirrored at
`artifacts/governance/m5_policy_impact_simulation.md`. A frozen example of the
packet lives at
`fixtures/governance/m5_policy_impact_simulation/canonical_packet.yaml` and its
shape is validated by
`schemas/governance/m5_policy_impact_simulation.schema.json`.

## Why this packet exists

The M5 surfaces add policy-bearing rows — AI evidence, provider-linked work
items, companion and incident packets, managed sync/offboarding records,
browser handoff manifests, and richer support/export surfaces. A policy change
on these rows can change what a delete or export does, defer it, redact it, or
shorten how long an artifact survives. Discovering that *after* publish means
users discover breakage; admins need to see the consequences *before*.

This packet makes a proposed policy change previewable. For each governed
family it compares the current policy against a draft and shows:

- **what changes** — which delete/export actions land on a different outcome;
- **what stays unchanged** — the actions a draft leaves exactly as they are;
- **which saved objects it affects** — the impacted objects, named with the
  same record classes the runtime surfaces use; and
- **how expiry changes runtime behavior** — expiry effects and downgrade paths
  are part of the simulation, not an after-effect of publishing.

It is metadata-only and carries no credential bodies or raw provider payloads.

## Object model

Each governed family contributes one simulation row binding:

- **Action diffs** — for both `delete` and `export`, the current outcome, the
  draft outcome, a `changed` flag, and a plain-language effect summary. The
  `changed` flag always agrees with whether the two outcomes differ.
- **Impacted objects** — the saved objects the draft would touch, each carrying
  the object ref, the governing record class, the producer record kinds that
  materialize it, whether a managed copy exists, and an optional scope note.
- **Expiry effect** — the effect class (`unchanged`, `extended`, `shortened`,
  `introduced`, `removed`), the current and draft expiry rules, the runtime
  consequence, and when the change takes effect. A changing rule always states
  its runtime consequence.
- **Downgrade path** — the downgrade class, the before/after behavior, and a
  flag confirming the downgrade is visible before publish (always true).
- **Draft policy epoch** — a draft is always a new policy epoch; it never
  reuses the current one.

## Outcome vocabulary

Action diffs reuse the same outcome vocabulary the records lifecycle and the
runtime hold/retention surfaces use:

`requested`, `queued`, `completed`, `partial`, `blocked_by_hold`,
`policy_retained`, `outside_platform_scope`, `manual_local_capture_required`,
`not_found`, and `omitted_by_redaction`.

## Required contract

- Every family covers both the `delete` and `export` actions, and the
  `changed`/`unchanged` partition is exhaustive.
- Each action diff's `changed` flag agrees with whether the current and draft
  outcomes differ.
- A changing expiry effect states its runtime consequence; a downgrade path
  states its before/after behavior and is visible before publish.
- Each row's record class is the canonical class for its family, so the
  simulation and the runtime surface it previews refer to the same objects.
- A local-only family's draft never claims a managed (remote) hold, export, or
  delete.
- A draft always advances the policy epoch.
- The machine-readable `impact_summary` roll-up agrees with the rows.

## Governed families

| Family | Delete (current → draft) | Export (current → draft) | Expiry effect | Downgrade |
| --- | --- | --- | --- | --- |
| AI evidence packet | `policy_retained` → `completed` | `completed` (unchanged) | shortened | none |
| Provider-linked work item | `not_found` (unchanged) | `outside_platform_scope` (unchanged) | shortened | none |
| Companion continuity packet | `blocked_by_hold` (unchanged) | `completed` → `omitted_by_redaction` | unchanged | export → omitted-by-redaction |
| Incident support packet | `completed` → `policy_retained` | `omitted_by_redaction` (unchanged) | extended | completed → policy-retained |
| Sync mirror ledger | `blocked_by_hold` (unchanged) | `manual_local_capture_required` → `partial` | unchanged | none |
| Browser handoff manifest | `completed` (unchanged) | `outside_platform_scope` (unchanged) | introduced | none |
| Offboarding record | `policy_retained` → `completed` | `partial` (unchanged) | shortened | none |
| Support export packet | `completed` (unchanged) | `completed` → `omitted_by_redaction` | unchanged | export → omitted-by-redaction |

These illustrative rows are exactly the ones frozen in the canonical fixture;
they exercise changed and unchanged actions, every expiry effect class used by
the lane, and both downgrade paths, while keeping the two local-only families
(provider-linked work item, browser handoff manifest) from claiming any managed
control in the draft.

## Consumers

- `aureline-records::m5_policy_simulation` is the authoritative producer and
  validator, and exposes product, CLI/headless, and support/export projections
  plus a flattened machine-readable changed-action export.
- `aureline-records::m5_records_policy` is the runtime hold/retention surface
  whose object identities (governed family, record class, operation outcome)
  the simulation reuses, so the preview and the surface it affects refer to the
  same objects.
- `aureline-policy::m5_exception_expiry` is the policy companion that gates the
  simulated changes with time-bounded, actor-scoped exceptions revalidated on
  drift.
- `aureline-support::m5_records_policy_governance` is the first support
  consumer; it embeds the simulation packet alongside the runtime hold/retention
  and exception/expiry packets, exposes the metadata-only support/export
  projection, and proves the simulation covers the same families and resolves
  its exception references against live, bounded policy exceptions.
