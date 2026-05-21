# Docs/help browser content beta corpus

Frozen corpus that promotes the in-product docs/help browser **content** surface
from incidental coverage to beta. The docs browser already projects source /
version / freshness / client-scope / browser-handoff rows from a docs pack
(`crates/aureline-shell/src/docs_browser/`), and the cross-surface
`truth_wiring` module already binds the `DocsBrowser` surface to the same release
truth the migration center, Help/About, and service-health surfaces read. This
corpus proves the docs content rows are themselves wired to that release truth
instead of asserting a private, more optimistic story.

Each `*.json` case is a `DocsBrowserRowCard` projection (the existing render-ready
type — no parallel content type is forked) plus a small expectation envelope. The
case proves a docs entry:

- **cites the current release truth** — the same `claim_manifest_ref` /
  `compatibility_report_ref` and the `docs_site` channel the `DocsBrowser` surface
  binding resolves to, and a docs claim row that the binding actually selected;
- **does not over-claim freshness** — a docs entry can be as fresh as the release
  truth's docs freshness badge (`warm_cached`) or more conservative, never more
  confident (an `authoritative_live` claim over the release truth fails);
- **does not over-claim the build match** — only an `exact_build_match` entry that
  cites the running build identity counts as version-wired; any drift state is a
  degrade;
- **labels degraded / stale entries** — when the freshness row is degraded the
  entry must carry an explicit snapshot-age label rather than silently downgrading;
- **shares the boundary / visibility vocabulary** — identity mode, trust state,
  source class, version state, and the per-entry contract state are all drawn from
  the same closed vocabularies the sibling public-truth surfaces use.

## Cases

| Case | Source | Version | Freshness | Contract state |
|---|---|---|---|---|
| `project_docs_release_current.json` | project docs | exact build match | warm cached | `ready` |
| `mirrored_docs_stale_labeled.json` | mirrored official docs | incompatible drift | stale (labeled) | `stale` |
| `vendor_docs_degraded_labeled.json` | vendor / provider docs | compatible minor drift | degraded cached (labeled) | `degraded` |

## What replays the corpus

`crates/aureline-shell/tests/docs_browser_content_beta.rs` deserializes each case
directly into a `DocsBrowserRowCard`, reads the live `DocsBrowser` surface binding
from `seeded_truth_wiring_report()`, and asserts:

- the corpus `release_truth_binding` snapshot equals the live binding
  (`claim_manifest_ref`, `compatibility_report_ref`, `required_channel_id`,
  `service_contract_state`, `freshness_state_tokens`, `evidence_stale`,
  `claim_downgraded`, and the selected docs claim rows), and the release docs
  freshness badge equals what the service-health beta surface projects for those
  rows — so the corpus tracks live release truth instead of a hand-edited copy;
- every case is wired to that binding, labels its degraded freshness, and resolves
  to the contract state it declares;
- the corpus covers the `ready`, `stale`, and `degraded` contract states.

The test also drives the two acceptance failures directly: an entry that claims
freshness fresher than release truth fails, and a degraded entry that drops its
freshness label fails — so neither an inconsistent entry nor an unlabeled stale
entry can pass silently.

## Drift defense

`ci/check_beta_docs_browser_content.py` re-derives the freshness, version, source,
identity-mode, trust-state, and contract-state vocabularies from
`crates/aureline-shell/src/docs_browser/state.rs` and `truth_wiring.rs`, re-derives
the `DocsBrowser` release-truth binding straight from
`artifacts/release/m3/claim_manifest.json` and
`artifacts/compat/m3/compatibility_report.json`, and fails closed if the manifest
mirrors, the declared binding, or any case label drift. The corpus is pure JSON, so
the gate needs no Rust toolchain.
