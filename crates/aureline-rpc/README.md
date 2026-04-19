# aureline-rpc

## Purpose
Transport layer for the supervisor and service fabric: framing, request/response
surface, deadline/cancellation plumbing, and trace-context propagation for
every cross-process call.

## Protected-path status
Protected. Reliability, deadline honesty, and trace-context integrity sit on
this crate.

## Allowed dependencies
- May depend on `aureline-telemetry` for trace propagation.
- Must not depend on `aureline-render`, `aureline-buffer`, `aureline-vfs`,
  `aureline-text`, or `aureline-shell-spike`.

## Canonical owner path
`crates/aureline-rpc/`

## Work packages
- WP-13 (Release engineering and governance)
