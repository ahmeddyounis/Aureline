# Java and Kotlin daily-driver quality truth packet — reviewer artifact

This is the reviewer-facing artifact for the M4 stable Java and
Kotlin daily-driver quality truth packet covering the open/import,
navigate, edit, complete, refactor, run/test/debug, review, migrate,
and recover daily-loop steps with replacement-grade support,
Gradle/Maven build-workspace evidence (`settings.gradle` /
`settings.gradle.kts` / `build.gradle` / `build.gradle.kts` /
`gradle.properties` / `gradle/wrapper`; Maven `pom.xml` parent /
`<modules>` / BOM inheritance; `settings.xml` / `~/.m2/settings.xml`
resolution; `GRADLE_USER_HOME` / `MAVEN_OPTS` / `MAVEN_HOME` /
`JAVA_HOME` / `KOTLIN_HOME` environment resolution; repository /
mirror / proxy configuration), Spring Boot run/test/debug evidence
(`./gradlew bootRun`, `./gradlew bootBuildImage`,
`mvn spring-boot:run`, `gradle test` / `mvn test` with JUnit 5,
JUnit 4, Spock, and Kotest, Spring Boot DevTools hot-reload, JDWP /
JPDA attach and listen modes, remote-debug socket transport, Spring
Boot Actuator endpoints, `application.properties` /
`application.yml` profile selection), rename/navigation evidence
(`eclipse.jdt.ls` Java LSP and `kotlin-language-server` Kotlin LSP
symbol / rename / extract / inline / move, type-hierarchy,
call-hierarchy, find-references, source-roots / classpath /
module-path / kotlin source-set resolution, package rename across
`java/` and `kotlin/` source roots, JavaDoc / KDoc cross-link
navigation), framework-migration evidence, known limits, downgrade
automation, and evidence binding.

The contract lives at
`docs/languages/m4/stabilize-java-and-kotlin-daily-driver-quality-with.md`
and is replayed by
`crates/aureline-language/tests/java_and_kotlin_daily_driver_quality_truth_packet.rs`.

## Stable claim

For the governed language lane class
(`java_and_kotlin_daily_driver_lane`) the packet binds:

- at least one `daily_driver_quality` row (the lane's headline Java
  and Kotlin daily-driver qualification),
- a `daily_loop_step` row per certified step (open/import, navigate,
  edit, complete, refactor, run/test/debug, review, migrate,
  recover) when the lane claims `replacement_grade`,
- at least one `framework_pack` row certifying a Java or Kotlin
  framework pack (e.g., Spring Boot service archetype, Kotlin
  multi-module library, or Android Gradle module),
- at least one `migration_evidence` row (e.g., Spring Boot 2 →
  Spring Boot 3 migration, Java 8 → Java 17 LTS migration, Kotlin
  1.x → Kotlin 2.0 migration, or single-module → multi-module
  migration),
- at least one `archetype_repo_evidence` row certifying the
  archetype repos backing the daily loop,
- at least one `build_workspace_row` certifying the active
  Gradle/Maven build workspace contract,
- at least one `spring_boot_run_test_debug_row` certifying the
  Spring Boot run/test/debug surface,
- at least one `rename_navigation_row` certifying the JDT LSP /
  Kotlin LSP rename and navigation surface,
- a closed `support_class` (no surface pretends `replacement_grade`
  while a binding is unbound),
- a closed `daily_loop_step_class` (every replacement-grade lane
  covers the full daily loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark,
  build-workspace, Spring Boot run/test/debug, rename/navigation,
  or docs-disclosure),
- a closed `known_limit_class` (framework / language / archetype /
  migration / build-workspace / Spring Boot run/test/debug /
  rename/navigation subset, unsupported runtime target, beta
  capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework
  gap / unproven build-workspace / Spring Boot run/test/debug /
  rename/navigation, auto-demote on low confidence, auto-block on
  missing evidence, manual-only, or `none`),
- a closed `daily_driver_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `replacement_grade`, declares a non-`none_declared`
  known limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/java_and_kotlin_daily_driver_quality_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/java_and_kotlin_daily_driver_quality_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/java_and_kotlin_daily_driver_quality_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/java_and_kotlin_daily_driver_quality_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/java_and_kotlin_daily_driver_quality_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-java-and-kotlin-daily-driver-quality-with.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection              | Surface                              |
| ----------------------- | ------------------------------------ |
| `editor_language_pack`  | Editor language pack badge / hover   |
| `framework_pack_panel`  | Framework pack panel                 |
| `language_settings`     | Language settings / help surface     |
| `cli_headless`          | CLI/headless inspector               |
| `support_export`        | Support export bundle                |
| `release_proof_index`   | Release proof index entry            |
| `help_about`            | Help/About proof card                |
| `conformance_dashboard` | Conformance dashboard row            |

A projection that collapses any closed vocabulary, drops the packet
id, drops the lane class, row class, support class, daily-loop step,
known-limit, downgrade-automation, or evidence-class vocabulary, or
leaks raw private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `replacement_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a lane that claims `replacement_grade` daily-driver support is
  missing a certified `daily_loop_step` row for any of the nine
  required steps (open/import, navigate, edit, complete, refactor,
  run/test/debug, review, migrate, recover),
- a `daily_loop_step` row drops its daily-loop step binding,
- a non-`daily_loop_step` row binds a daily-loop step it cannot
  certify,
- a row narrowed below `replacement_grade` or with a non-default
  known limit / non-`none` downgrade automation drops its disclosure
  ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`JavaAndKotlinDailyDriverQualityTruthPacket::materialize` and then
read the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: [`schemas/language/java_and_kotlin_daily_driver_quality_truth.schema.json`](../../../schemas/language/java_and_kotlin_daily_driver_quality_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-java-and-kotlin-daily-driver-quality-with.md`](../../../docs/languages/m4/stabilize-java-and-kotlin-daily-driver-quality-with.md)
- Fixture corpus: [`fixtures/language/m4/java_and_kotlin_daily_driver_quality_truth_packet/`](../../../fixtures/language/m4/java_and_kotlin_daily_driver_quality_truth_packet/)
- Rust module: [`crates/aureline-language/src/java_and_kotlin_daily_driver_quality_truth_packet/mod.rs`](../../../crates/aureline-language/src/java_and_kotlin_daily_driver_quality_truth_packet/mod.rs)
