# Project Doctor probe descriptor and finding card cases

These fixtures seed the Project Doctor probe-semantics and finding-card
contracts. Each scenario carries one `doctor_probe_descriptor_record`
(against `schemas/support/doctor_probe.schema.json`) and one
`doctor_finding_card_record` (against
`schemas/support/doctor_finding_card.schema.json`). The pair makes the
read-only-by-default rule, the closed probe-class taxonomy, and the
desktop-versus-headless parity contract concrete.

| Scenario | Probe class | Invocation policy | Card posture |
|---|---|---|---|
| Missing toolchain | `read_only_inspection` | `automatic` | `read_only_diagnosis` |
| Proxy or CA failure | `environment_check` | `with_user_consent` | `read_only_diagnosis` |
| Extension regression | `read_only_inspection` | `automatic` | `read_only_diagnosis` |
| Schema drift (cache rebuild preview) | `repair_preview` | `with_user_consent` | `preview_only_no_apply` |
| Local-history corruption | `unsafe_or_unsupported` | `never_without_explicit_invocation` | `refusing_unsupported` |
| Remote-target mismatch (simulation) | `simulation` | `automatic_inferring_only` | `read_only_diagnosis` |

The manifest pins the closed enums each row must use and lists the
reviewer-facing assertions every case must satisfy.

## How the fixtures pair up

Each scenario has two files:

- `probe_<scenario>_<probe-class>.yaml` — the probe descriptor.
- `finding_card_<scenario>.yaml` — the finding card the descriptor
  produces (or refuses to produce).

The card's `probe_descriptor_ref` points back at the descriptor file
verbatim so a reviewer can audit both halves of the contract from one
case.

## Why these six scenarios

The cases cover the six examples named by the contract spec:

- missing toolchain (proves a read-only probe stays inside metadata
  and environment-adjacent data classes);
- proxy or CA failure (proves an environment-check probe requires
  single-step user consent and is unavailable offline);
- extension regression (proves Doctor diagnoses a crash loop without
  quarantining the extension itself; the repair runs through the
  reviewed repair-transaction path);
- schema drift (proves a repair-preview probe materialises a local
  preview manifest only and never applies a mutation);
- local-history corruption (proves Doctor labels and refuses unsafe
  repair surfaces rather than offering them under a diagnosis label);
- remote-target mismatch (proves a simulation probe labels its
  diagnosis as "simulating" rather than "proven", and that the
  offline parity gap is rendered as "planned" rather than dropped).

## Headless parity rule

Every descriptor and card carries exactly four `headless_parity_rows`
(one per support context: `desktop`, `cli_headless`, `remote_managed`,
`offline_local`). A reviewer can read the four rows side-by-side to
see exactly which UI affordances are suppressed, which fields the
machine-readable output emits, which capability rows are not yet
implemented, and which exit class the headless renderer returns. A
parity gap is therefore visible per row instead of implied by
omission.
