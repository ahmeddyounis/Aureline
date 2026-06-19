# M5 Learnability Certification

This document is the contract for the M5 learnability certification: the
release-bearing gate that decides whether each claimed M5 feature-family
onboarding row may keep its learnability grade. It certifies learning-mode,
guided-exercise, progress-snapshot, educational-AI, and offline/mirror docs-pack
truth on every marketed onboarding surface, and auto-narrows any row that cannot
back its claim with current, reopenable proof.

The canonical packet is `LearnabilityCertificationPacket`, built by
`aureline_learning::seeded_m5_learnability_certification` and ingested by
Help/About, docs/migration packets, support export, the release center, the Start
Center, and AI evidence through
`aureline_learning::current_m5_learnability_certification_export` rather than by
re-narrating learnability status by hand.

## What a row certifies

Each `CertifiedLearnabilityRow` ties a durable `CertifiedLearnabilitySubject` —
keyed by an `M5LearningSurfaceFamily`, a mirror-served continuity flag, and a
non-display fingerprint distinct from its id — to per-dimension certifications over
the learnability evidence vocabulary:

| Dimension | Required core | What it proves |
|---|---|---|
| `guided_tour` | yes | Command-backed tour steps with stable target refs, prerequisites, and citations; no hidden mutating shortcut. |
| `guided_exercise` | yes | Guided-exercise rails with per-step success criteria and hint/reveal/reset/skip controls over a sandbox/reversible preference. |
| `progress_snapshot` | yes | User-owned, restart-safe progress with local-first storage, redacted export, and explicit reset disclosure. |
| `educational_ai` | yes | Educational-AI panels that cite repository truth (files, symbols, docs, examples, commands), with explain kept separate from do. |
| `offline_mirror` | yes | Offline / mirrored docs-pack continuity with explicit freshness, no dead links, never reading as live authoritative content. |
| `learning_mode_profile` | no | Opt-in learning-mode profile (tip intensity, jargon level, explain-versus-do posture) with reversible controls, on the families that ship one. |

Each `LearnabilityDimensionCertification` is **evidence-bound, not asserted**: it
names a `proof_currency` and, unless the proof is missing, a reopenable `proof_ref`
keyed by a non-display fingerprint. Certification review reopens the same tour /
exercise / progress / educational-AI / offline-mirror evidence object that backs
the grade.

## Proof currency and auto-narrowing

A grade only holds while current, reopenable proof backs it:

- `verified_current` / `cached_within_window` — current local proof; backs a live
  row's claim.
- `mirror_current` — current proof served from a disclosed offline / mirrored
  pack, read-only; backs a mirror-served row's claim but never a live one.
- `stale_expired` / `missing_proof` / `requires_review` — does not back any claim.

`CertifiedLearnabilityRow::needs_narrow` is true whenever a required-core
dimension is uncertified or any certified dimension lacks current proof. A narrowed
row must carry an effective grade strictly **below** its claim, a recorded
`narrow_trigger`, and a precise `narrowed_label` — never a generic non-answer such
as "unavailable" or "uncertified". There are **no manual waivers**: auto-narrowing
is the only mechanism by which a row sits below its claim, and every downgrade is
recorded in the release-visible waiver-and-downgrade log.

## Guardrails

`LearnabilityCertificationPacket::validate` refuses a packet that violates the
track invariant:

- tutorials, hints, and exercises stay command-backed and opt-in;
- explain stays separate from do — every teaching mutation routes through the same
  preview/approval model as ordinary work;
- progress stays user-owned and local-first, never widened into repo- or
  collaborator-visible telemetry;
- cached and offline/mirror packs stay explicit with disclosed freshness and never
  read as live authoritative content;
- educational AI cites repository truth rather than answering omnisciently or
  acting directly;
- experts are never trapped in tutorials.

Raw progress bodies, speaker notes, repository contents, provider payloads,
credentials, and raw docs-pack bytes never cross this boundary; the packet carries
only typed class tokens, booleans, opaque ids, fingerprint digests, and
redaction-aware reviewable labels.

## Seeded corpus

The seeded corpus certifies one onboarding row per claimed M5 feature family —
notebook, request/API workspace, database workspace, profiler/trace, docs/browser,
preview, template/scaffold (framework-pack), companion, and sync/offboarding — plus
two demonstration rows:

- the **companion** row is held read-only on a disclosed offline/mirror pack
  (`mirror_current` proof) and never reads as a live local result; it is claimed
  `provisionally_certified`;
- a second **profiler/trace** row is the auto-downgrade drill: its `offline_mirror`
  proof aged outside its freshness window, so it auto-narrows from `certified` to
  `uncertified` with an `offline_mirror_continuity_lost` trigger and a precise
  narrowed label, while every other row's effective grade equals its claim.

## Canonical references

- Schema: [`schemas/help/m5-learnability-cert-report.schema.json`](../../schemas/help/m5-learnability-cert-report.schema.json)
- Support export (JSON): [`artifacts/m5/learnability/certification-report/support_export.json`](../../artifacts/m5/learnability/certification-report/support_export.json)
- Markdown summary: [`artifacts/m5/learnability/certification-report/support_export.md`](../../artifacts/m5/learnability/certification-report/support_export.md)
- Waiver-and-downgrade log: [`artifacts/m5/learnability/waiver-and-downgrade-log/support_export.md`](../../artifacts/m5/learnability/waiver-and-downgrade-log/support_export.md)
- Protected fixture: [`fixtures/help/m5/certification-corpus/learnability_certification_corpus.json`](../../fixtures/help/m5/certification-corpus/learnability_certification_corpus.json)

## Regenerate

```bash
cargo run -p aureline-learning --bin aureline_learning_m5_learnability_certification -- support > \
  artifacts/m5/learnability/certification-report/support_export.json
cargo run -p aureline-learning --bin aureline_learning_m5_learnability_certification -- summary > \
  artifacts/m5/learnability/certification-report/support_export.md
cargo run -p aureline-learning --bin aureline_learning_m5_learnability_certification -- waiver > \
  artifacts/m5/learnability/waiver-and-downgrade-log/support_export.md
cp artifacts/m5/learnability/certification-report/support_export.json \
  fixtures/help/m5/certification-corpus/learnability_certification_corpus.json
```
