# M5 Content-Wording Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every governed
object present, the frozen vocabulary set intact, and the trust-review,
consumer-projection, and release-posture invariants satisfied — the difference is
which object is narrowed and why. They are minted from the same seed builder as the
canonical export by `aureline_shell_content_wording_matrix`.

## commercial_boundary_wording_held.json

The commercial-boundary wording object is held after a boundary-drift finding (the
claimed edition/hosting language no longer matches the actual deployment profile).
Held objects no longer carry a public claim, so the evidence requirement relaxes to
`recommended`, but the object stays present with its hosting-boundary, edition-label,
and client-scope vocabularies intact. The safety-critical UI string, glossary term,
action-label pattern, error/recovery block, AI copy guardrail, count/scope phrase
set, and content-ops artifact remain at their canonical qualifications. Demonstrates
that drifting commercial-boundary wording narrows to held rather than shipping
language that overstates the boundary.

## ai_copy_guardrail_narrowed.json

The AI copy guardrail is narrowed from Beta to Preview after an overclaim finding.
The object keeps all of its declared vocabularies and the `overclaim_detected`
downgrade trigger, so the overclaim is disclosed while the claim narrows.
Demonstrates that an overclaiming AI surface narrows the claim rather than shipping
copy that overstates confidence or autonomy.
