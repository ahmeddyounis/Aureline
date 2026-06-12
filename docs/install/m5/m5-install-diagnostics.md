# M5 install-and-update diagnostics

The M5 install-and-update diagnostics packet is the single inspectable object that explains, for
every M5-added artifact, **where it lives, who owns its updates, and what rollback target is still
valid** — without private build knowledge. Support engineers and enterprise operators read one
object instead of guessing from a version string or channel chip.

- Typed model: `aureline-install` crate, module `m5_install_diagnostics`
  ([`M5InstallUpdateDiagnostics`]).
- Canonical packet: `artifacts/install/m5/m5-install-diagnostics.json`
- Schema: `schemas/install/m5-install-diagnostics.schema.json`
- Reviewer artifact: `artifacts/install/m5/m5-install-diagnostics.md`
- Fixtures: `fixtures/install/m5/m5-install-diagnostics/`

## What the packet records

One `ArtifactDiagnosticRow` covers one M5 artifact family. The packet covers the artifact families
M5 adds beyond the primary app binary:

| Family | Covers |
| --- | --- |
| `desktop_app` | The first-party desktop application binary |
| `companion` | A companion surface paired to a host install |
| `marketplace_helper` | A marketplace or helper extension artifact |
| `local_model_runtime` | A local-model runtime and its on-disk weights |
| `portable_export` | A portable or exported state package |

Each row records, for that artifact:

- **install mode** — `system`, `user`, `portable`, `managed`, or `marketplace`;
- **channel** — the channel-and-ring it installs from;
- **updater owner** — `first_party_auto`, `managed_fleet`, `marketplace_host`,
  `os_package_manager`, `manual_user`, or `none`;
- **artifact roots, mutable-state roots, and policy roots** — each classified by role and
  sensitivity (see [Secrecy boundaries](#secrecy-boundaries));
- **last verification state and freshness** — `signed_verified` / `platform_trusted` /
  `self_signed` / `unverified`, and `current` / `aging` / `stale` / `never_verified`;
- **rollback target** — `available`, `available_bounded`, `expired`, or `missing`.

## The diagnostics gate

The support label an artifact may publish (`published_support`) is the weakest ceiling implied by
its observed states. It is computed, never asserted by hand:

```
published_support = declared_support
    .min(verification_state ceiling)
    .min(verification_freshness ceiling)
    .min(rollback_target ceiling)
    .min(governs_assurance)             // the governance lane's own published label
```

So an unverified binary, a stale verification, a missing rollback target, or a governance lane that
was itself narrowed all lower or withhold the published support automatically. The recorded
`narrow_reasons` and `recovery_path` are recomputed the same way and validated to match.

This is the rollout invariant the row exists to enforce: **an M5 topology with no current install
diagnostics and verification state cannot claim support.** Each artifact is pinned to the canonical
governance lane it draws verification truth from (`governs_lane`), and `governs_assurance` is
validated against the embedded `m5_install_and_portability_governance` matrix, so an artifact can
never publish support beyond the lane the governance gate already narrowed.

## Secrecy boundaries

Every root is classified, but no secret is dumped. A root with `sensitivity` `machine_protected` or
`secret_bearing` **must** carry `redacted: true`. The packet names *where* a credential, policy, or
machine-identity store is — it never carries token values, raw provider payloads, or machine-unique
protected material. The packet is metadata-only: every field is a typed state or an opaque ref.

## Troubleshooting drills

Each `DiagnosticDrill` replays one support incident and proves the diagnostics object detects it, so
support can reproduce the signal rather than guess:

| Incident | What it replays |
| --- | --- |
| `root_mismatch` | An artifact resolved under an unexpected root |
| `stale_verification` | An artifact's last verification went stale |
| `missing_rollback_target` | An artifact has no valid rollback target |
| `wrong_root_support` | Support inspected the wrong state root for an artifact |

## Consumer surfaces

Desktop, CLI, About, and support export each bind to this one packet via a
`DiagnosticsConsumerBinding`, ingest it, preserve its published support and recovery paths verbatim,
and narrow with it. An artifact narrowed here cannot read as supported on an About panel, a CLI
status line, or a support export. The export projection and support-export wrapper carry typed
states and opaque refs only.

[`M5InstallUpdateDiagnostics`]: ../../../crates/aureline-install/src/m5_install_diagnostics/mod.rs
