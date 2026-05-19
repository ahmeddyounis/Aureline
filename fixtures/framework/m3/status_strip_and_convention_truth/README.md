# Framework status strip and convention-truth fixtures

Worked corpus for the framework support / status strip record and the
framework-object certainty record (route / component / service / entity
rows, convention-diagnostic rows, and generator / codemod / scaffold
previews).

See:

- [`/schemas/framework/framework_support_strip.schema.json`](../../../../schemas/framework/framework_support_strip.schema.json)
- [`/schemas/framework/framework_object_certainty.schema.json`](../../../../schemas/framework/framework_object_certainty.schema.json)
- [`/crates/aureline-framework/src/status_and_certainty/`](../../../../crates/aureline-framework/src/status_and_certainty/mod.rs)
- [`/artifacts/framework/m3/framework_certainty_and_generator_review.md`](../../../../artifacts/framework/m3/framework_certainty_and_generator_review.md)

Every case carries a `__fixture__` block declaring `expected_findings: []`
because the corpus is the seed of cases that validate clean. Negative
cases are exercised by the unit tests in
`crates/aureline-framework/src/status_and_certainty/tests.rs`.
