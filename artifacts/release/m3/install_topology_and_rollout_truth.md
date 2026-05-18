# Install Topology And Rollout Truth

This release packet summarizes the current beta install-profile, state-root,
repair/verify, uninstall, and rollout-ring evidence. It is generated from the
same rows consumed by the Rust validator and support projections.

## Source Evidence

| Evidence | Ref |
|---|---|
| Profile cards and rollout rows | `fixtures/install/m3/profile_cards_and_repair/profile_cards_packet.json` |
| Repair, verify, and uninstall diagnostics | `fixtures/install/m3/profile_cards_and_repair/repair_verify_uninstall_packet.json` |
| Exact-build diagnostics | `artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json` |
| Ring rollout packet | `artifacts/release/m3/ring_rollout/packet.json` |
| State-root audit | `artifacts/release/m3/state_root_audit.md` |

## Profile Rows

| Profile card | Mode | Channel | Updater owner | Binary root | Durable state truth | Rollback / uninstall |
|---|---|---|---|---|---|---|
| `card.windows.x86_64.per_user_installed.stable` | Per-user installed | Stable | User | Per-user profile program area | Stable configuration, recovery, and cache roots are channel-owned. | Previous same-channel build; user uninstall preserves state. |
| `card.windows.x86_64.side_by_side_preview.preview` | Side-by-side Preview | Preview | User | Per-user profile program area | Preview configuration, recovery, and cache roots are separate from Stable. | Previous same-channel build; user uninstall preserves state. |
| `card.windows.x86_64.portable.portable_stable` | Portable | Portable Stable | User | Portable directory | Only `state.portable_colocated_root.portable_stable` is durable. Machine-global integrations are suppressed. | Portable bundle replacement/removal; installed roots untouched. |
| `card.windows.x86_64.managed_deployed.stable` | Managed deployed | Stable | Managed fleet | Per-machine program area | User configuration/recovery/cache roots remain per-user; shared data and policy roots are admin-owned. | Last broad-compatible target; managed deprovision preserves local work. |

## Side-By-Side Import

`import_sheet:stable_to_preview_first_run` is the current cross-channel import
sheet. It gives users an explicit compare-before-apply path when Preview first
uses Stable state:

- import selected profile values as copies;
- keep recent work separate;
- review extension state manually;
- skip credential metadata and open a sign-in or repair path when needed;
- create `checkpoint:preview_profile_before_stable_import` before any apply;
- disclose state-root, file-association, and hidden shared-state assumptions.

This blocks last-writer-wins behavior and keeps Preview from silently
corrupting Stable state.

## Repair, Verify, And Uninstall

| Operation | Profile coverage | Result | Human diagnostics |
|---|---|---|---|
| `operation:managed.repair.health.success` | Enterprise managed + silent install | Success | Copyable install id, timestamps, state roots, repair transaction, support ref. |
| `operation:managed.verify.health.success` | Enterprise managed + silent install | Success | Exact-build, policy-root, and state-root verification summary. |
| `operation:managed.uninstall.success` | Enterprise managed + silent install | Success | Removes package markers and update state; preserves user configuration, recovery, and workspace files. |
| `operation:managed.verify.signature_failed` | Enterprise managed + silent install | Verify failed | Failure summary, signature reason, remediation pointer, rollback evidence, support ref. |

## Rollout Rows

| Ring | Owner | Promotion state | Rollback posture |
|---|---|---|---|
| Canary | `release-platform-admin` | Candidate ready | Roll back to previous same-channel build when exact-build or state-root evidence is missing. |
| Pilot | `release-platform-admin` | In progress | Roll back to the prior broad-compatible package set while preserving local work. |
| Broad | `release-manager` | Paused | Held until pilot evidence is green; rollback target remains the last broad-compatible set. |
| LTS | `enterprise-release-owner` | Held | Held until broad and offline rollback evidence are current; rollback target is the declared support floor. |

## Verification

```bash
cargo test -p aureline-install --test profile_cards_and_repair_beta
```

Current result: pass.
