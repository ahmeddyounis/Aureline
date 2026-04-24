# .NET service or app

## Row binding

- Archetype row id: `archetype_row:dotnet_service_or_app`
- Archetype id: `dotnet_service_or_app`
- Initial support class: `experimental`
- Target support class: `supported`
- Inclusion target: `first_beta`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

.NET SDK with `dotnet` CLI, an ASP.NET Core service or comparable
app surface, xUnit or NUnit as the in-repo test runner. The row covers
service and app shapes that the `dotnet` CLI builds, restores, runs,
and tests directly.

## Required-mode rationale

- `local_only` — `dotnet restore`, `dotnet build`, `dotnet run`, and
  `dotnet test` cover the row's workflow set on a developer machine.
  The supported target deliberately stops short of remote-attach or
  container coverage.

## Evidence already on file

- Reference workspace: `reservation:fixtures/workspaces/reference/dotnet_service_archetype_seed.json`.
- Corpus scenarios: `reservation:archetype.dotnet_service_first_open`,
  `reservation:workflow.first_useful_edit_dotnet_service`.
- Design-partner input class: `sanitised_repo_admissible`.

## Open evidence questions

- Materialise the reservation slot for a synthetic seed workspace
  before any graduation step. The row currently has no inspectable
  reference workspace.
- Confirm the supported target is the right ceiling for first stable;
  promotion to certified requires a certified-archetype report and a
  release-evidence-owned claim-manifest row, which the row does not
  yet have.
- Decide whether the row covers .NET LTS only or LTS plus the current
  STS release, and how the support window is recorded on the
  resulting compatibility-report row.
