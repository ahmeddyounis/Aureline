# Fork review policy

This document is the operating policy for local forks, long-lived patch
stacks, and other deliberate upstream divergence on protected or
launch-critical paths. It narrows the generic fork language in the
dependency review policy into a concrete review bar the repository can
enforce before implementation accretes accidental ownership.

Companion artifacts:

- [`/docs/governance/dependency_review_policy.md`](./dependency_review_policy.md)
  — dependency and import admission workflow.
- [`/artifacts/governance/build_vs_buy_register.yaml`](../../artifacts/governance/build_vs_buy_register.yaml)
  — canonical domain register that names the posture, concerns, and
  exit strategy for each launch-critical subsystem.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — canonical dependency rows that carry the actual upstream selection.
- [`/artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml)
  — canonical import rows for copied, bundled, or mirrored bytes.

## 1. When fork review is required

Fork review is mandatory in the same change when Aureline:

1. selects a local fork for a protected-path dependency;
2. carries a long-lived upstream patch stack that is expected to survive
   beyond the immediate merge window;
3. changes copied or mirrored third-party bytes in a way that creates
   behavioral divergence from upstream rather than a pure subset,
   repack, or mirror-metadata update; or
4. depends on an upstream compatibility contract while knowingly
   changing that contract's semantics locally.

Purely temporary, merge-blocking patch carry may skip a separate fork
review only if all of the following are true:

- the patch is expected to disappear before the next release family;
- no protected-path behavior or compatibility contract is changed; and
- the dependency or import row still records the patch and its removal
  trigger in the same change.

If any of those statements stops being true, open full fork review
immediately.

## 2. Required traceability

Every reviewed fork or long-lived patch stack MUST be traceable in one
place for each concern:

- **Why this domain can tolerate divergence** lives in
  `build_vs_buy_register.yaml`.
- **Which upstream choice is actually affected** lives in
  `dependency_register.yaml` or `third_party_import_register.yaml`.
- **Why the architecture accepts the divergence** lives in an ADR when
  the fork touches a protected path or changes a compatibility claim.

The minimum repository links are:

1. a `build_vs_buy_refs` entry from the dependency row back to the
   relevant `domain.*` row in `build_vs_buy_register.yaml`;
2. a dependency or import row that records the upstream selection,
   license and provenance posture, and fork-or-replace trigger; and
3. an ADR for protected-path forks, protocol drifts, or other
   architecture-shaping divergence.

No protected-path fork may exist only in a branch name, patch file, or
chat explanation.

## 3. Required review fields

Protected-path forks and long-lived patch stacks MUST have all of the
following recorded before merge:

- **Named owner.** A human `owner_dri`, not just a team alias.
- **Upstream baseline.** The exact upstream project and version, tag, or
  commit the divergence starts from.
- **Reason upstreaming is not enough.** Specific technical or cadence
  reason, not a vague preference for local control.
- **Scope of divergence.** Which packages, generated assets, or
  compatibility surfaces differ from upstream.
- **Exit strategy.** How the divergence ends: re-upstream, replace,
  delete, or intentionally ratify into a stable local component.
- **Divergence review cadence.** At least once per release family, plus
  whenever upstream security, license, or compatibility posture changes
  materially.
- **Re-upstream plan.** Issue or PR reference when possible, owning
  person, next attempt window, and conditions that would make upstreaming
  unnecessary.
- **Rollback or replace trigger.** Concrete condition that reopens the
  selection or forces retirement of the fork.

The dependency or import row may already carry some of these fields. The
review is still incomplete if the combined artifact set does not cover
all of them.

## 4. Review cadence and escalation

Minimum cadence:

- every change that increases divergence from upstream;
- every release family while the divergence exists; and
- immediately when upstream maintainer health, licensing, security
  posture, or claimed compatibility changes materially.

Escalate to architecture review instead of silently carrying the fork
forward when any of the following happen:

- the divergence survives more than one release family;
- the fork changes a compatibility contract Aureline claims to preserve;
- the upstream becomes under-maintained or license-ambiguous; or
- no credible re-upstream or replacement path remains.

This policy is intentionally stricter on protected paths than on repo-only
tooling because hidden fork debt on those paths becomes product debt.

## 5. Re-upstream expectations

Protected-path forks default to upstream-first, not "carry forever until
someone notices." The re-upstream plan must answer:

- what exact upstream project should receive the change;
- whether an issue, discussion, or PR already exists;
- who owns the next upstreaming attempt;
- what date or release window triggers that attempt; and
- what result closes the plan: upstream accepted, local divergence
  deleted, or fork deliberately re-ratified with a fresh ADR.

If upstreaming is truly impossible, the exit strategy must explain why
replacement or explicit ownership is the right end state instead.

## 6. What does not count as a fork by itself

The following do not automatically require full fork review unless they
create behavioral divergence:

- pure mirroring with metadata-only changes;
- bundled subsets that do not change the upstream component's semantics;
- generated lockfiles or package manifests that simply record upstream
  selections; and
- short-lived patch carry removed before the next release family and
  before any compatibility or protected-path claim depends on it.

When in doubt, bias toward opening fork review. The cost of a few extra
lines in the canonical registers is lower than carrying hidden divergence
into a protected subsystem.
