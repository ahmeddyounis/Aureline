# Support-export benchmark fields

Support and enterprise-evaluation exports must be able to explain the benchmark
context behind a claim **without improvising their own wording**. They render the
redaction-safe projection of a claim publication entry through the
[publication-ingestion register](publication-ingestion-register.json); they never
restate a benchmark result in prose and never carry raw corpus, run, machine, or
provider material.

This document is the canonical reference for which fields a support or evaluation
export may carry and which fields may never cross the export boundary. It is the
human-readable companion to the `export_safe_fields` and `forbidden_fields`
tables in
[`publication-ingestion-register.json`](publication-ingestion-register.json), and
the validator `ci/check_publication_ingestion.py` enforces that every binding's
`disclosed_fields` is a subset of the export-safe set and disjoint from the
forbidden set.

## Export-safe fields

Each field below is projected verbatim from the claim publication entry in the
[shiproom benchmark-freshness ledger](shiproom-benchmark-freshness.json). The
support agent reads these values; they are stable governance ids and reviewable
sentences only.

| Field | What it carries |
| --- | --- |
| `entry_id` | The stable claim publication entry id the export points at. |
| `title` | The entry's reviewable one-line title. |
| `posture` | The entry's published claim posture. |
| `published_claim_ceiling` | The strongest claim the entry may make before narrowing. |
| `effective_claim` | The entry's narrowed effective claim after freshness and comparability rules apply. |
| `freshness_state` | The entry's computed freshness and comparability state. |
| `downgrade_label` | The entry's reviewable sentence explaining its current state. |
| `metric_refs` | The protected metric ids the entry covers. |
| `bound_corpus_revision` | The corpus-manifest revision the entry binds to. |
| `bound_hardware_class` | The reference hardware class the entry binds to. |
| `bound_hardware_profile_ref` | The reference hardware profile id; the raw machine label is withheld. |
| `bound_lab_image_ref` | The lab-image id the entry binds to. |
| `bound_lab_image_revision` | The lab-image revision the entry binds to. |
| `bound_threshold_version` | The protected-metrics threshold version the entry binds to. |
| `repro_pack_ref` | The reproducibility pack id a reviewer reruns or audits the claim against. |

A support export typically discloses the entry id, the posture and narrowed
effective claim, the freshness state, the downgrade label, the metric refs, the
bound revisions, and the reproducibility pack id. That is enough for support to
explain *what was claimed, on what corpus and hardware, how fresh it is, and how
to reproduce it* — entirely from the canonical entry.

## Forbidden fields

These never cross into any product or export surface. A binding that discloses
one of them fails the gate.

| Field | Why it is withheld |
| --- | --- |
| `raw_run_log` | Raw benchmark run logs stay in retained run metadata. |
| `raw_machine_label` | The captured machine's raw hostname or serial is withheld; only the reference hardware profile id is published. |
| `raw_provider_payload` | Raw provider request or response bodies never appear in a benchmark surface. |
| `raw_corpus_contents` | Raw corpus file contents stay in the corpus store. |
| `customer_or_repository_name` | Customer or repository names drawn from a corpus are never published. |
| `competitor_raw_payload` | A head-to-head competitor's raw capture payload is withheld; only the disclosed comparison fields are published. |
| `secret_material` | Credentials, tokens, and other secret material never appear in any surface. |
| `screenshot` | Screenshots of a run are not part of an export-safe benchmark projection. |

## Narrowed and quarantined claims

Because a support export renders the entry's `effective_claim`, `freshness_state`,
and `downgrade_label`, a narrowed or quarantined claim shows the same way in a
support export as it does in docs, help, and the About surface. A quarantined
entry exports as quarantined with its reason; it never exports its old ceiling.
Support never has to decide how to describe a stale claim — the entry already
carries the redaction-safe sentence.
