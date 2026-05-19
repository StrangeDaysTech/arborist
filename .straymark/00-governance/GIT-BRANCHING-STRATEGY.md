# Git Branching Strategy

> **Reference document for Git workflow in this project.**
> For quick rules, see CLAUDE.md or GEMINI.md section on Git Operations.

---

## Critical Rule

**NEVER commit directly to `main` branch.**

All changes must go through feature/fix branches and Pull Requests. This ensures code review, CI validation, and a clean commit history.

---

## Branch Naming Convention

| Prefix | Purpose | Example |
| -------- | --------- | --------- |
| `feature/` | New features or enhancements | `feature/export-excel` |
| `feat/` | Alias for feature | `feat/folio-c5` |
| `fix/` | Bug fixes | `fix/report-form-tests` |
| `hotfix/` | Urgent production fixes | `hotfix/auth-bypass` |
| `docs/` | Documentation-only changes | `docs/api-reference` |
| `refactor/` | Code refactoring (no behavior change) | `refactor/catalog-service` |
| `test/` | Test-only changes | `test/bunit-coverage` |

---

## Workflow

1. **Before starting work**, verify current branch:

   ```bash
   git branch --show-current
   ```

2. **Create branch** from updated `main`:

   ```bash
   git checkout main
   git pull origin main
   git checkout -b fix/descriptive-name
   ```

3. **Make commits** following conventional commit format

4. **Push and create PR**:

   ```bash
   git push -u origin fix/descriptive-name
   gh pr create --title "fix: description" --body "..."
   ```

5. **Merge via PR** after CI passes - Never push directly to `main`

---

## Conventional Commits

Use semantic prefixes in commit messages:

| Prefix | Use Case |
| -------- | ---------- |
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation only |
| `test:` | Adding or fixing tests |
| `refactor:` | Code change without behavior change |
| `chore:` | Maintenance, dependencies, config |
| `perf:` | Performance improvement |

---

## Recovery: If You Accidentally Commit to Main

If commits were made to `main` that should be on a branch:

```bash
# Create a new branch with the commits
git branch fix/accidental-commits

# Reset main to match origin
git reset --hard origin/main

# Switch to the new branch and push
git checkout fix/accidental-commits
git push -u origin fix/accidental-commits
```

---

*DevTrail v1.0.0 | Last updated: 2025-01-30*
*[Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*
