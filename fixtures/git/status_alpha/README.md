# Git Status Alpha Fixtures

These fixtures describe protected launch-wedge states for the local Git service:
an attached branch with staged, unstaged, and untracked changes; a detached HEAD;
and a plain folder with no repository. The tests build temporary repositories
from these fixture descriptions and assert that the service emits one canonical
snapshot plus shared shell, activity-center, and review projections.
