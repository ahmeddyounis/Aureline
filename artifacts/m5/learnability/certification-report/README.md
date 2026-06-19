# M5 Learnability Certification Report

`support_export.json` is the checked support export of the M5 learnability
certification packet (`LearnabilityCertificationPacket`). It is the canonical
artifact the Help/About, docs/migration, support, release-center, Start Center, and
AI-evidence surfaces ingest through
`aureline_learning::current_m5_learnability_certification_export`
instead of narrating learnability maturity by hand.

The packet certifies every claimed M5 feature-family onboarding row — notebook,
request/API workspace, database workspace, profiler/trace, docs/browser, preview,
template/scaffold (framework-pack), companion, and sync/offboarding — against the
command-backed tour, guided-exercise, user-owned progress, cited educational-AI,
and offline/mirror continuity model. Each row binds:

- a durable subject keyed by a canonical `family` and a `mirror_served` continuity
  flag, plus a non-display fingerprint distinct from its id;
- per-dimension certifications over `guided_tour`, `guided_exercise`,
  `progress_snapshot`, `educational_ai`, `offline_mirror`, and (for families that
  ship one) `learning_mode_profile`, each naming a proof currency and a reopenable
  proof ref;
- the row's guardrail state — tour steps command-backed, progress user-owned and
  private, educational AI cited, offline/mirror continuity disclosed, explain kept
  separate from do, and experts never trapped in tutorials;
- a claimed grade and an effective grade that ranks strictly below the claim when
  any dimension loses current proof.

The stale-offline-mirror profiler row is the auto-downgrade demonstration: its
`offline_mirror` proof aged outside its freshness window, so it auto-narrows from
`certified` to `uncertified` with an `offline_mirror_continuity_lost` trigger and a
precise narrowed label, while every other row's effective grade equals its claim.
The companion row is held read-only — its `mirror_served` flag agrees with its
subject and its proof currency is `mirror_current`, which backs the mirror-served
claim but never a live one, so a mirrored onboarding pack never reads as a live
local result.

`support_export.md` is the deterministic Markdown summary of the same packet. The
release-visible waiver-and-downgrade log derived from this packet lives at
`artifacts/m5/learnability/waiver-and-downgrade-log/support_export.md`.

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

The artifact validates against
[`schemas/help/m5-learnability-cert-report.schema.json`](../../../../schemas/help/m5-learnability-cert-report.schema.json)
and is byte-identical to the protected fixture at
[`fixtures/help/m5/certification-corpus/learnability_certification_corpus.json`](../../../../fixtures/help/m5/certification-corpus/learnability_certification_corpus.json).
The contract doc is
[`docs/m5/learnability-certification.md`](../../../../docs/m5/learnability-certification.md).
