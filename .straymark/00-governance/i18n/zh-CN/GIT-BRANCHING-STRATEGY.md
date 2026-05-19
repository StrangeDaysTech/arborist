# Git 分支策略

> **本项目 Git 工作流的参考文档。**
> 快速规则请参阅 CLAUDE.md 或 GEMINI.md 中的 Git 操作部分。

---

## 关键规则

**绝对不要直接提交到 `main` 分支。**

所有变更必须通过功能/修复分支和 Pull Request 进行。这确保了代码审查、CI 验证和清晰的提交历史。

---

## 分支命名约定

| 前缀 | 用途 | 示例 |
| -------- | --------- | --------- |
| `feature/` | 新功能或增强 | `feature/export-excel` |
| `feat/` | feature 的别名 | `feat/folio-c5` |
| `fix/` | 缺陷修复 | `fix/report-form-tests` |
| `hotfix/` | 紧急生产修复 | `hotfix/auth-bypass` |
| `docs/` | 仅文档变更 | `docs/api-reference` |
| `refactor/` | 代码重构（无行为变更） | `refactor/catalog-service` |
| `test/` | 仅测试变更 | `test/bunit-coverage` |

---

## 工作流

1. **开始工作前**，确认当前分支：

   ```bash
   git branch --show-current
   ```

2. **从最新的 `main` 创建分支**：

   ```bash
   git checkout main
   git pull origin main
   git checkout -b fix/descriptive-name
   ```

3. **按照约定式提交格式进行提交**

4. **推送并创建 PR**：

   ```bash
   git push -u origin fix/descriptive-name
   gh pr create --title "fix: description" --body "..."
   ```

5. **CI 通过后通过 PR 合并** - 绝不直接推送到 `main`

---

## 约定式提交

在提交信息中使用语义化前缀：

| 前缀 | 使用场景 |
| -------- | ---------- |
| `feat:` | 新功能 |
| `fix:` | 缺陷修复 |
| `docs:` | 仅文档 |
| `test:` | 添加或修复测试 |
| `refactor:` | 无行为变更的代码修改 |
| `chore:` | 维护、依赖、配置 |
| `perf:` | 性能改进 |

---

## 恢复：如果意外提交到 Main

如果有提交被错误地提交到了 `main`，而它们应该在一个分支上：

```bash
# 用这些提交创建一个新分支
git branch fix/accidental-commits

# 将 main 重置为与远程一致
git reset --hard origin/main

# 切换到新分支并推送
git checkout fix/accidental-commits
git push -u origin fix/accidental-commits
```

---

*StrayMark v1.0.0 | 最后更新：2025-01-30*
*[Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*
