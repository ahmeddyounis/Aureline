# Implement the First Justified Replay Backend with Recording-Mode Banner, Expiry, and Cost Posture

This document is the reviewer-facing landing page for the M5 justified replay backend lane.

## Scope

This lane governs how profiler and trace surfaces:

- show a recording-mode banner that makes recording state explicit (recording, not
  recording, recorded, expired, unsupported, or policy-blocked) so users never mistake
  a live session for a recorded one;
- present replay expiry information with retention class, expiry timestamp, freshness
  state, and policy posture so expired or stale captures never look current;
- surface cost posture with overhead class, storage band, and honest warnings so users
  know the impact before starting a recording;
- keep replay controls justified: replay is only offered when recording state, expiry,
  and cost posture all attest the capture is replayable.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/implement_the_first_justified_replay_backend_with_recording_mode_banner_expiry_and_cost_posture/`
- **Packet:** `artifacts/perf/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.json`
- **Schema:** `schemas/perf/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.schema.json`
- **Fixtures:** `fixtures/performance/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Recording-mode banner | Stable | Shows recording state, backend ref, allowed verbs, chronology support, reverse-step availability, and degraded-state labels so users never mistake a live session for a recorded one. |
| Replay expiry inspector | Stable | Shows retention class, expiry status, policy posture, and honest degraded-state warnings so expired or stale captures never look current. |
| Cost posture inspector | Stable | Shows overhead class, storage band, cost posture, and honest warnings so users know the impact before starting a recording. |
| Replay backend | Stable | Binds recording mode, expiry, and cost posture so replay controls are never shown without justification. |
| Export review | Preview | Redaction-safe export flows for replay evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for replay payloads is still under qualification. |

## Recording-Mode Banner Rows

Banner rows carry:

- `banner_id` — stable identifier;
- `recording_mode_state` — `recording`, `not_recording`, `recorded`, `expired`, `unsupported`, or `policy_blocked`;
- `backend_ref` — backend or runtime ref;
- `allowed_verbs` — verbs available in this recording mode;
- `chronology_support` — chronology support label;
- `reverse_step_available` — whether reverse step is possible;
- `reverse_step_unavailable_reason` — honest reason when reverse step is unavailable.

Every banner row MUST show its recording mode state and MUST show a degraded-state label
when applicable.

## Replay Expiry Rows

Expiry rows carry:

- `expiry_id` — stable identifier;
- `retention_class` — retention policy label;
- `expires_at` — expiry timestamp or null when pinned;
- `expiry_status` — `current`, `stale`, `expired`, `missing`, `pinned`, or `policy_blocked`;
- `policy_posture` — policy label.

Every expiry row MUST show its retention class, expiry status, and MUST warn on degraded
state when applicable.

## Replay Cost-Posture Rows

Cost-posture rows carry:

- `cost_id` — stable identifier;
- `cost_posture_class` — `low`, `moderate`, `high`, `extreme`, or `unknown`;
- `overhead_class` — overhead label;
- `storage_band` — storage impact label.

Every cost-posture row MUST show its cost posture class, MUST show an overhead warning
when cost is high or extreme, and MUST block automatic recording when cost is extreme.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Recording-mode banner rows MUST show recording mode state and degraded-state labels;
  missing truth labels trigger a validation violation.
- Replay expiry rows MUST show retention class, expiry status, and degraded warnings;
  missing behavior triggers a validation violation.
- Replay cost-posture rows MUST show cost posture class, overhead warnings when
  applicable, and auto-block when extreme; missing behavior triggers a validation
  violation.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Replay controls are hidden or disabled unless backed by an explicit recording-mode
  banner with declared state and allowed verbs.
- Expiry, artifact mismatch, unsupported recording, and missing source captures are
  explicit states.
- Cost posture is visible before recording starts; extreme cost blocks automatic
  recording without consent.
