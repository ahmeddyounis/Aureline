# Spring Boot Expert workflow pack truth packet — reviewer artifact

This is the reviewer-facing artifact for the M4 stable Spring Boot
Expert workflow pack truth packet covering the create, open, run,
test, debug, rename, and review loops with expert-grade support,
workflow loop coverage, Spring Boot target discovery evidence
(`./gradlew bootRun`, `./gradlew bootBuildImage`,
`./gradlew bootJar`, `./gradlew bootWar`, `mvn spring-boot:run`,
`mvn spring-boot:build-image`, `mvn spring-boot:repackage`, custom
Gradle/Maven targets, JUnit 5 / JUnit 4 / Spock / Kotest test tasks,
and JDWP / JPDA debug targets), Spring Boot config/property awareness
evidence (`application.properties`, `application.yml`,
profile-specific config, `@ConfigurationProperties`, `@Value`,
`@ConditionalOnProperty`, `@Profile`, `spring.config.import`,
`spring.profiles.active`, and auto-configuration imports resolution),
Spring Boot review-safe refactor evidence (DI/IoC-aware rename,
`@Component`/`@Service`/`@Repository`/`@Controller`/`@RestController`/
`@Configuration` bean lifecycle safety, `@Autowired`/
constructor-injection/`@Qualifier` resolution, AOP advice targets,
and bean rename across `@Bean(name)` references),
framework-migration evidence, known limits, downgrade automation, and
evidence binding. The contract lives at
`docs/languages/m4/stabilize-the-spring-boot-expert-workflow-pack-with.md`
and is replayed by
`crates/aureline-language/tests/spring_boot_expert_workflow_pack_truth_packet.rs`.

## Stable claim

For the governed workflow pack class
(`spring_boot_expert_workflow_pack`) the packet binds:

- at least one `pack_qualification` row (the pack's headline
  workflow-pack qualification),
- a `workflow_loop` row per certified step (create, open, run, test,
  debug, rename, review) when the pack claims `expert_grade`,
- at least one `framework_migration_row` certifying the Spring Boot
  2.x → Spring Boot 3.x migration archetype,
- at least one `archetype_repo_row` certifying the Java and Kotlin
  Spring Boot service archetypes,
- at least one `target_discovery_row` certifying the Spring Boot
  build/test/debug target discovery surface: `./gradlew bootRun`,
  `./gradlew bootBuildImage`, `./gradlew bootJar`,
  `./gradlew bootWar`, `mvn spring-boot:run`,
  `mvn spring-boot:build-image`, `mvn spring-boot:repackage`, custom
  Gradle/Maven targets, JUnit 5 / JUnit 4 / Spock / Kotest test
  tasks, and JDWP / JPDA debug targets discovered from
  `build.gradle(.kts)` / `pom.xml` / `settings.gradle(.kts)`,
- at least one `config_property_awareness_row` certifying the
  Spring Boot config/property surface: `application.properties`,
  `application.yml`, profile-specific
  `application-{profile}.properties` / `application-{profile}.yml`,
  `@ConfigurationProperties`, `@Value`, `@ConditionalOnProperty`,
  `@Profile`, `spring.config.import`, `spring.profiles.active`,
  `spring.profiles.include`, and auto-configuration imports
  resolution,
- at least one `review_safe_refactor_row` certifying the Spring
  Boot review-safe refactor surface: DI/IoC-aware rename,
  `@Component`/`@Service`/`@Repository`/`@Controller`/`@RestController`/
  `@Configuration` bean lifecycle safety, `@Autowired`/
  constructor-injection/`@Qualifier` resolution, Spring AOP advice
  targets, bean rename across `@Bean` / `@Bean(name)` /
  `@Component("name")` references, and rename-preview gating on
  beans whose binding cannot be statically proven,
- a closed `support_class` (no surface pretends `expert_grade` while
  a binding is unbound),
- a closed `workflow_loop_class` (every expert-grade pack covers the
  full workflow loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark,
  target-discovery, config-property-awareness, review-safe-refactor,
  or docs-disclosure),
- a closed `known_limit_class` (framework subset, language subset,
  archetype subset, migration subset, target-discovery subset,
  config-property subset, review-safe-refactor subset, unsupported
  runtime target, beta capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework
  gap / unproven target discovery / unproven config-property
  awareness / unproven review-safe refactor, auto-demote on low
  confidence, auto-block on missing evidence, manual-only, or
  `none`),
- a closed `workflow_pack_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `expert_grade`, declares a non-`none_declared` known
  limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/spring_boot_expert_workflow_pack_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/spring_boot_expert_workflow_pack_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/spring_boot_expert_workflow_pack_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/spring_boot_expert_workflow_pack_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/spring_boot_expert_workflow_pack_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-the-spring-boot-expert-workflow-pack-with.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection                    | Surface                              |
| ----------------------------- | ------------------------------------ |
| `editor_framework_pack_panel` | Editor framework pack panel          |
| `workflow_companion`          | Workflow companion / runner panel    |
| `framework_settings`          | Framework settings / help surface    |
| `cli_headless`                | CLI/headless inspector               |
| `support_export`              | Support export bundle                |
| `release_proof_index`         | Release proof index entry            |
| `help_about`                  | Help/About proof card                |
| `conformance_dashboard`       | Conformance dashboard row            |

A projection that collapses any closed vocabulary, drops the packet
id, drops the pack class, row class, support class, workflow-loop,
known-limit, downgrade-automation, or evidence-class vocabulary, or
leaks raw private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `expert_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a pack that claims `expert_grade` workflow-pack support is missing a
  certified `workflow_loop` row for any of the seven required steps
  (create, open, run, test, debug, rename, review),
- a `workflow_loop` row drops its workflow-loop step binding,
- a non-`workflow_loop` row binds a workflow-loop step it cannot
  certify,
- a row narrowed below `expert_grade` or with a non-default known
  limit / non-`none` downgrade automation drops its disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`SpringBootExpertWorkflowPackTruthPacket::materialize` and then read
the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: [`schemas/language/spring_boot_expert_workflow_pack_truth.schema.json`](../../../schemas/language/spring_boot_expert_workflow_pack_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-the-spring-boot-expert-workflow-pack-with.md`](../../../docs/languages/m4/stabilize-the-spring-boot-expert-workflow-pack-with.md)
- Fixture corpus: [`fixtures/language/m4/spring_boot_expert_workflow_pack_truth_packet/`](../../../fixtures/language/m4/spring_boot_expert_workflow_pack_truth_packet/)
- Rust module: [`crates/aureline-language/src/spring_boot_expert_workflow_pack_truth_packet/mod.rs`](../../../crates/aureline-language/src/spring_boot_expert_workflow_pack_truth_packet/mod.rs)
