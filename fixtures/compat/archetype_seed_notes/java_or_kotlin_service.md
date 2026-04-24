# Java or Kotlin service

## Row binding

- Archetype row id: `archetype_row:java_or_kotlin_service`
- Archetype id: `java_or_kotlin_service`
- Initial support class: `experimental`
- Target support class: `certified`
- Inclusion target: `first_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

Gradle or Maven build, Spring Boot or comparable JVM application,
JUnit or Kotest as the in-repo test runner. JDK selection and the
Gradle or Maven wrapper that the repo ships are part of the row's
trust posture; the wrapper-execution policy itself is shared with
other archetypes.

## Required-mode rationale

- `local_only` — JVM workflows are fully covered by an in-repo build
  and test on a developer machine. The row deliberately does not
  promise a devcontainer or remote-attach baseline at first stable.

## Evidence already on file

- Reference workspace: `reservation:fixtures/workspaces/reference/java_kotlin_service_archetype_seed.json`.
- Corpus scenarios: `reservation:archetype.java_kotlin_service_first_open`,
  `reservation:workflow.first_useful_edit_java_kotlin_service`.
- Design-partner input class: `sanitised_repo_admissible`.

## Open evidence questions

- Materialise the reservation slot for the synthetic seed workspace
  before any graduation step. The slot exists so the row can be cited
  by tooling without a renaming churn when the fixture lands.
- Decide whether the certified path requires both Gradle and Maven
  evidence, or whether one of them stays at supported.
- Capture the wrapper-trust posture (workspace trust, run-on-open
  prompts, audit log) for the certified row before promotion past
  supported.
