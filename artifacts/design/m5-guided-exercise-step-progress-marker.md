# Guided exercise steps and progress markers

- Packet: `m5-guided-exercise-step-progress-marker-controls:stable:0001`
- Surface: `M5 guided exercise steps and progress markers: target-object identity, observable success criteria, hint/reveal/reset/skip actions, sandbox-or-preview preference for mutating lessons, and privacy-bounded completed/remaining progress with resume/reset/export across claimed learnability lanes`
- Guided exercise steps: 6 (4 not yet completed)
- Progress markers: 6 (5 not complete)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Guided exercise steps

- **Open the review this lesson works on** — state `not_started`, mode `command_backed` → `pending`, target `file_location`
- **Make the requested change in the sandbox** — state `active`, mode `sandboxed_practice` → `in_progress`, target `surface_location`
- **Approve the review to finish the walkthrough** — state `passed`, mode `read_only_walkthrough` → `completed`, target `command_reference`
- **Fix the setting the checkpoint expects** — state `failed_retryable`, mode `checkpoint_gated` → `retryable`, target `file_location`
- **Replay the summary at your own pace** — state `replayable`, mode `self_paced` → `completed`, target `docs_anchor`
- **Practice freely in the playground** — state `sandboxed`, mode `no_hidden_apply` → `sandbox_practice`, target `surface_location`

## Progress markers

- **Onboarding lesson progress** — state `not_started`, ownership `local_only` → `unstarted`, 0/5 done
- **Review track progress** — state `in_progress`, ownership `user_owned_synced` → `underway`, 2/5 done
- **Exported lesson record** — state `completed`, ownership `exported_by_choice` → `complete`, 5/5 done
- **Workspace-shared exercise progress** — state `paused`, ownership `workspace_shared` → `interrupted`, 3/6 done
- **Reset practice progress** — state `reset`, ownership `cached_snapshot` → `interrupted`, 0/4 done
- **Offline lesson progress** — state `offline_local`, ownership `not_installed` → `offline_cached`, 1/3 done
