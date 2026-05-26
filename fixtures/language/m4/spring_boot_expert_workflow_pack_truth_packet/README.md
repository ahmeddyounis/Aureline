# spring_boot_expert_workflow_pack_truth_packet fixture corpus

Fixture corpus for the M4 stable Spring Boot Expert workflow pack
truth packet
(`schemas/language/spring_boot_expert_workflow_pack_truth.schema.json`).

Each fixture is a `SpringBootExpertWorkflowPackTruthPacketInput` with
an `expect` block that pins the materialized packet's promotion state,
finding count, pack and row-class token sets, support-class,
workflow-loop, known-limit, downgrade-automation, and evidence-class
tokens, and the support-export safety verdict. Tests in
`crates/aureline-language/tests/spring_boot_expert_workflow_pack_truth_packet.rs`
load each case and assert that
`SpringBootExpertWorkflowPackTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The Spring Boot Expert workflow pack
  carries a pack-qualification row at `expert_grade` plus every
  certified workflow-loop row (create, open, run, test, debug, rename,
  review), a framework-migration row certifying the Spring Boot 2.x →
  Spring Boot 3.x migration archetype, an archetype-repo row covering
  certified Java and Kotlin Spring Boot service archetypes, a
  target-discovery row certifying `./gradlew bootRun`,
  `./gradlew bootBuildImage`, `./gradlew bootJar`,
  `./gradlew bootWar`, `mvn spring-boot:run`,
  `mvn spring-boot:build-image`, `mvn spring-boot:repackage`, custom
  Gradle/Maven targets, JUnit 5 / JUnit 4 / Spock / Kotest test tasks,
  and JDWP / JPDA debug targets discovered from `build.gradle(.kts)`
  / `pom.xml` / `settings.gradle(.kts)`, a config/property awareness
  row certifying `application.properties`, `application.yml`,
  profile-specific config, `@ConfigurationProperties`, `@Value`,
  `@ConditionalOnProperty`, `@Profile`, `spring.config.import`,
  `spring.profiles.active`, and auto-configuration imports
  resolution, and a review-safe refactor row certifying DI/IoC-aware
  rename, `@Component`/`@Service`/`@Repository`/`@Controller`/
  `@RestController`/`@Configuration` bean lifecycle safety,
  `@Autowired`/constructor-injection/`@Qualifier` resolution, AOP
  advice targets, and bean rename across `@Bean(name)` references.
  Every row binds support, known limit, downgrade automation, and
  evidence classes; narrowed rows carry their disclosure refs, and
  all eight required consumer projections preserve the packet
  verbatim.
- `expert_grade_with_unbound_evidence_blocks_stable.json` — The pack
  qualification row claims `expert_grade` while its evidence class is
  `evidence_unbound`; the packet blocks the stable claim because no
  archetype, fixture-repo, migration, target-discovery,
  config-property awareness, review-safe refactor, or design-partner
  evidence backs the row.
- `missing_workflow_loop_for_expert_grade_blocks_stable.json` — The
  pack claims `expert_grade` but the `rename` workflow-loop row is
  missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  known-limit row narrows below expert grade with
  `review_safe_refactor_subset_only` but drops its disclosure ref;
  the packet blocks the stable claim.
- `projection_collapses_workflow_loop_vocabulary_blocks_stable.json`
  — The `conformance_dashboard` consumer projection drops the
  workflow-loop vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A workflow-pack row
  admits raw source bodies past the boundary; the packet blocks the
  stable claim because raw material must never leak through the
  workflow-pack boundary.
