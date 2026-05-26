# java_and_kotlin_daily_driver_quality_truth_packet fixture corpus

Fixture corpus for the M4 stable Java and Kotlin daily-driver quality
truth packet
(`schemas/language/java_and_kotlin_daily_driver_quality_truth.schema.json`).

Each fixture is a `JavaAndKotlinDailyDriverQualityTruthPacketInput`
with an `expect` block that pins the materialized packet's promotion
state, finding count, lane and row-class token sets, support-class,
daily-loop step, known-limit, downgrade-automation, and
evidence-class tokens, and the support-export safety verdict. Tests
in
`crates/aureline-language/tests/java_and_kotlin_daily_driver_quality_truth_packet.rs`
load each case and assert that
`JavaAndKotlinDailyDriverQualityTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The Java and Kotlin daily-driver lane
  carries a `daily_driver_quality` row at `replacement_grade` plus
  every certified `daily_loop_step` row (open_or_import, navigate,
  edit, complete, refactor, run_test_debug, review, migrate,
  recover), framework-pack, migration-evidence, and archetype-repo
  rows, plus build-workspace (Gradle `settings.gradle` /
  `settings.gradle.kts` / `build.gradle` / `build.gradle.kts` /
  `gradle.properties` / `gradle/wrapper`; Maven `pom.xml` /
  `<modules>` / BOM inheritance; `settings.xml` /
  `~/.m2/settings.xml` resolution; `GRADLE_USER_HOME` /
  `MAVEN_OPTS` / `MAVEN_HOME` / `JAVA_HOME` / `KOTLIN_HOME`
  environment resolution), Spring Boot run/test/debug
  (`./gradlew bootRun`, `./gradlew bootBuildImage`,
  `mvn spring-boot:run`, `gradle test` / `mvn test` with JUnit 5,
  JUnit 4, Spock, Kotest, Spring Boot DevTools hot-reload, JDWP /
  JPDA attach and listen, remote-debug socket transport, Spring Boot
  Actuator endpoints, `application.properties` /
  `application.yml` profile selection), and rename/navigation
  (`eclipse.jdt.ls` Java LSP / `kotlin-language-server` Kotlin LSP
  symbol / rename / extract / inline / move, type-hierarchy,
  call-hierarchy, find-references, source-roots / classpath /
  module-path / kotlin source-set resolution, JavaDoc / KDoc
  cross-link navigation) rows. Every row binds support, known limit,
  downgrade automation, and evidence classes; narrowed rows carry
  their disclosure refs, and all eight required consumer projections
  preserve the packet verbatim.
- `replacement_grade_with_unbound_evidence_blocks_stable.json` — The
  Java and Kotlin daily-driver quality row claims `replacement_grade`
  while its evidence class is `evidence_unbound`; the packet blocks
  the stable claim because no archetype, fixture-repo, migration,
  build-workspace, Spring Boot run/test/debug, rename/navigation, or
  design-partner evidence backs the row.
- `missing_daily_loop_step_for_replacement_grade_blocks_stable.json`
  — The lane claims `replacement_grade` but the `recover` daily-loop
  step is missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  known-limit row narrows below replacement grade with
  `build_workspace_subset_only` but drops its disclosure ref; the
  packet blocks the stable claim.
- `projection_collapses_evidence_class_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the evidence-class
  vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A daily-driver row
  admits raw source bodies past the boundary; the packet blocks the
  stable claim because raw Java or Kotlin source bodies,
  `gradle-wrapper.properties` distribution sha checksums or jar
  contents, `.env` secrets, and ambient `GRADLE_USER_HOME` /
  Maven `settings.xml` credentials must never leak through the
  daily-driver boundary.
