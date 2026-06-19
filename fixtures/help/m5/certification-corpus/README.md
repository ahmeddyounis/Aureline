# M5 Learnability Certification Corpus

## learnability_certification_corpus.json

The certification corpus for the M5 learnability certification packet. Every
claimed M5 feature-family onboarding row — notebook, request/API workspace,
database workspace, profiler/trace, docs/browser, preview, template/scaffold
(framework-pack), companion, and sync/offboarding — certifies its `guided_tour`,
`guided_exercise`, `progress_snapshot`, `educational_ai`, and `offline_mirror`
proof (the required core) plus the `learning_mode_profile` dimension on the
families that ship one (notebook, docs/browser, and template/scaffold).

The tenth row is the auto-narrowing drill: a profiler/trace row claims `certified`,
but its offline/mirror docs-pack evidence has aged outside its freshness window
(`offline_mirror` carries a `stale_expired` proof currency). Because a claimed row
may not outrun current proof, the row auto-narrows to an effective grade of
`uncertified`, records an `offline_mirror_continuity_lost` narrow trigger, and
carries a precise narrowed label rather than a generic provider error. Every other
row keeps current, reopenable proof for each dimension it certifies, so its
effective grade equals its claim.

The companion row is held read-only: its `mirror_served` flag agrees with its
subject, and its proof currency is `mirror_current`, which backs the mirror-served
row's claim but never a live one — a mirrored onboarding pack never reads as a live
local result. Each row keeps its canonical `family`, so unlike feature families are
never flattened into one synthetic onboarding claim. Each dimension certification
names a reopenable proof ref keyed by a non-display fingerprint distinct from the
ref.

The fixture validates against
`schemas/help/m5-learnability-cert-report.schema.json` and is byte-identical to the
checked support export at
`artifacts/m5/learnability/certification-report/support_export.json`.
