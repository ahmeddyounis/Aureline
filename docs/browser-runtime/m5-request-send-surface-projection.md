# M5 browser-runtime request send projection

Browser-runtime request panels reuse the same request-editor header,
environment picker, variable-resolution inspector, and auth sheet contracts as
the desktop request workspace. The browser-runtime fixture
`fixtures/ui/m5-request-data-components/request_editor_header.json` is the
first narrowed consumer: it keeps `execution_origin=browser_runtime`, exposes
the managed-policy environment layer, shows device-code auth posture, blocks
the primary send behind mutation review, and references the same variable
inspector and auth sheet used by support and release proof.

The browser-runtime projection may reduce capability to current-context replay,
inspect-only, or mutation-review-required. It may not hide target identity,
origin, auth storage mode, auth scheme, secret source class, token lifetime,
handoff state, run/cancel state, last-run summary, or redaction/export scope.
Default exports remain metadata-only, redacted-preview, or secret-handle-ref;
raw secrets, tokens, cookies, browser storage values, and verification codes are
not exported.
