# M5 Test-Intelligence Qualification Matrix Fixtures

## support_export_row_downgrades_on_unidentified_session_attempt.json

An auto-downgrade drill fixture for the test-intelligence qualification matrix.
Every claimed M5 test-intelligence surface — framework test explorer, notebook
test cells, AI test generation, review test panel, CI import overlay, coverage
surface, flaky/quarantine board, snapshot/golden review, and support/export
projection — carries its test-item identity class, discovery-snapshot class,
selection-object class, session-attempt class, verdict projection class,
selection-object contract, triage-packet contract, and proposal descriptors.

The support/export projection row claims `beta`, but its session-attempt class is
not yet identified (`session_attempt_class` is absent). Because a claimed row may
not outrun identified evidence, the row auto-downgrades to `effective` `held`,
records an `unidentified_session_attempt` downgrade trigger, and carries a precise
degraded label rather than a generic provider error. Every other row identifies a
durable test item plus its discovery, selection, and session objects, so its
effective qualification equals its claim.

The notebook row keeps a `partial_visible_discovery` snapshot visible rather than
hiding uncovered scope; the CI import overlay carries an `imported_read_only` item
identity, `provider_imported_discovery`, and an `imported_ci_session`, and keeps
imported results from reading as live local truth; the flaky/quarantine board
keeps a `muted` test visible, filterable, exportable, and carrying renewal/expiry
semantics; and the AI test-generation and snapshot/golden review rows render every
proposal behind a reviewable diff and an explicit apply step. Every selection
object survives rediscovery by stable identity and never captures display names
only.

The fixture validates against
`schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json`
and is byte-identical to the checked support export at
`artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/support_export.json`.
