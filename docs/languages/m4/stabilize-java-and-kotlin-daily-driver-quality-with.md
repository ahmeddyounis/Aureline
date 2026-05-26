# Stabilize Java and Kotlin daily-driver quality with Gradle/Maven import, Spring Boot run/debug/test, and rename/navigation truth — stable contract

Status: Stable lane proof for Java and Kotlin replacement-grade
daily-driver quality.

This document is the reviewer-facing contract for the stable Java and
Kotlin daily-driver quality truth packet. The packet is the single
source of truth that the editor language pack, framework pack panel,
language settings/help, CLI/headless inspector, support export,
release proof index, Help/About proof card, and the conformance
dashboard all read; surfaces MUST NOT mint local copies or paraphrase
daily-driver posture.

The packet pins Java and Kotlin daily-driver quality across three
intertwined truths beyond the bare daily loop:

1. The **Gradle/Maven build workspace truth** — every row that
   crosses the JVM build boundary (Gradle `settings.gradle` /
   `settings.gradle.kts` / `build.gradle` / `build.gradle.kts` /
   `gradle.properties` / `gradle/wrapper/gradle-wrapper.properties`
   plus included builds, composite builds, and multi-project
   subprojects; Maven `pom.xml` parent / `<modules>` / BOM
   inheritance, `settings.xml` / `~/.m2/settings.xml` resolution,
   `GRADLE_USER_HOME` / `MAVEN_OPTS` / `MAVEN_HOME` / `JAVA_HOME` /
   `KOTLIN_HOME` environment resolution, and repository / mirror /
   proxy configuration) binds a dedicated `build_workspace_row` and a
   disclosure ref so the daily-driver row never confuses one
   workspace layout for another.
2. The **Spring Boot run/debug/test parity** — every row that
   certifies the run/test/debug step on Java or Kotlin archetype
   repos binds a dedicated `spring_boot_run_test_debug_row`
   (`./gradlew bootRun` / `./gradlew bootBuildImage` / Maven
   `mvn spring-boot:run`; `gradle test` / `mvn test` with JUnit 5,
   JUnit 4, Spock, and Kotest runners; Spring Boot DevTools
   hot-reload; JDWP / JPDA attach and listen modes; remote-debug
   socket transport; Spring Boot Actuator endpoints; and
   `application.properties` / `application.yml` profile selection) so
   a beta-grade capability sample cannot masquerade as a
   replacement-grade Spring Boot daily-driver surface.
3. The **rename/navigation parity** — every row that certifies
   symbol navigation, completion, or refactor on Java or Kotlin
   archetype repos binds a dedicated `rename_navigation_row`
   (`eclipse.jdt.ls` Java LSP and `kotlin-language-server` Kotlin
   LSP symbol / rename / extract / inline / move; type-hierarchy,
   call-hierarchy, and find-references; source-roots / classpath /
   module-path / kotlin source-set resolution; package rename across
   both `java/` and `kotlin/` source roots; JavaDoc and KDoc
   cross-link navigation) so large-workspace rename and navigation
   posture is never inferred from a tiny single-module sample.

## What the packet asserts

For each governed *language lane × daily-driver row* the packet
asserts:

1. The **language lane class** — currently
   `java_and_kotlin_daily_driver_lane`. Every certified packet MUST
   carry at least one row for each required lane.
2. The **daily-driver row class** — one of `daily_driver_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `build_workspace_row`,
   `spring_boot_run_test_debug_row`, `rename_navigation_row`,
   `unsupported_gap`, `known_limit`, or `downgrade_automation`. A
   `daily_loop_step` row MUST bind a real daily-loop step; no other
   row class is permitted to bind one.
3. The **support class** — one of `replacement_grade`,
   `daily_driver_below_replacement`, `beta_grade_only`,
   `preview_only`, `unsupported`, or `support_unbound`. The validator
   refuses to certify a row that claims `replacement_grade` while any
   binding is unbound (support, known limit, downgrade automation, or
   evidence).
4. The **daily-loop step class** — one of `open_or_import`,
   `navigate`, `edit`, `complete`, `refactor`, `run_test_debug`,
   `review`, `migrate`, `recover`, or `not_applicable`. A lane that
   claims `replacement_grade` daily-driver support MUST cover every
   certified daily-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `build_workspace_evidence`,
   `spring_boot_run_test_debug_evidence`,
   `rename_navigation_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is `evidence_unbound`
   is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `build_workspace_subset_only`,
   `spring_boot_run_test_debug_subset_only`,
   `rename_navigation_subset_only`, `unsupported_runtime_target`,
   `beta_capability_sample_only`, or `limit_unbound`. A row whose
   known limit is `limit_unbound` is refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`,
   `auto_narrow_on_framework_gap`,
   `auto_narrow_on_unproven_build_workspace`,
   `auto_narrow_on_unproven_spring_boot_run_test_debug`,
   `auto_narrow_on_unproven_rename_navigation`,
   `auto_demote_on_low_confidence`,
   `auto_block_on_missing_evidence`, `manual_only_pending_review`,
   or `automation_unbound`. A row whose automation is
   `automation_unbound` is refused.
8. The **daily-driver confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `replacement_grade` at `low_confidence` is narrowed below stable
   until evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the daily-driver claim.
10. The **disclosure ref** — every row that is not
    `replacement_grade`, that declares a non-`none_declared` known
    limit, or that binds a non-`none` downgrade automation MUST carry
    a repo-relative disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one
of those booleans to false. The packet never admits raw Java or
Kotlin source bodies, `gradle-wrapper.properties` distribution sha
checksums or jar contents, `.env` secrets, ambient
`GRADLE_USER_HOME` / `MAVEN_OPTS` / Maven `settings.xml` credentials,
or provider payloads.

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

- Schema: `schemas/language/java_and_kotlin_daily_driver_quality_truth.schema.json`
- Reviewer artifact: `artifacts/language/m4/stabilize-java-and-kotlin-daily-driver-quality-with.md`
- Checked-in packet: `artifacts/language/m4/java_and_kotlin_daily_driver_quality_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/java_and_kotlin_daily_driver_quality_truth_packet/`
- Rust module: `crates/aureline-language/src/java_and_kotlin_daily_driver_quality_truth_packet/mod.rs`
- Replay tests: `crates/aureline-language/tests/java_and_kotlin_daily_driver_quality_truth_packet.rs`
