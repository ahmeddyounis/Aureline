# AI composer beta

The AI composer beta UI treats context, evidence, and spend as visible review
facts. The same refs must appear in composer pre-send review, the context
inspector, review workspace, docs/help, support export, and the headless CLI
projection.

## Composer

Before a user approves or executes an AI action, the composer exposes the
context snapshot ref, route snapshot ref, and context-state groups. Included
and pinned rows are shown as selected context; omitted, stale, and trimmed rows
remain visible with their reason token. The composer does not silently widen
context after review starts.

## Context Inspector

The inspector groups rows by source and state:

| State | UI obligation |
| --- | --- |
| `included` | Show the source class and stable identity ref. |
| `pinned` | Show that the user or policy pinned the row. |
| `omitted` | Keep the row visible with an omission reason. |
| `stale` | Block or require review until the stale row is resolved. |
| `trimmed` | Show that a bounded representation, not the raw source, was used. |

Rows must never imply that omitted, stale, or trimmed context was included
raw. Budget pressure and partial retrieval states remain labeled.

## Evidence And Spend

After the run, review surfaces show the evidence packet ref, approval-ticket
refs, tool-call lineage count, route receipt ref, spend receipt ref, cost
envelope token, cost visibility token, and charge-locus token. If evidence is
still pre-apply, the UI must not present the action as completed.

## Support Export

The support export and Markdown summary are metadata-only. They preserve refs
for context, retrieval, evidence, route, spend, approval, and tool lineage so a
support reviewer can trace what happened without seeing raw prompts, file
bodies, provider payloads, endpoint URLs, credentials, exact prices, or raw
token counts.

## Drift Rule

Any UI, docs/help, CLI, or support projection that drops required context-state
tokens, changes the operator-truth refs, hides partial retrieval state, or
shows spend copy that disagrees with the spend receipt is considered drift and
blocks promotion.
