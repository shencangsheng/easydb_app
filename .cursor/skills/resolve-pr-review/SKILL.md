---
name: resolve-pr-review
description: >-
  Resolve pull request code review comments end-to-end. Fetches inline review
  threads from a GitHub PR URL, verifies or implements fixes in the working
  branch, resolves threads, and posts a summary reply. Use when the user shares
  a PR link, asks to address/fix/resolve code review feedback, or mentions
  gemini-code-assist, Copilot, or Bugbot review comments on a pull request.
---

# Resolve PR Code Review

When the user provides a GitHub PR URL (or asks to resolve review on a PR), run this workflow end-to-end. Do not edit plan files if the user attached one — execute the workflow directly.

## Parse the PR

Extract `owner`, `repo`, and `number` from URLs like:

- `https://github.com/{owner}/{repo}/pull/{number}`
- `https://github.com/{owner}/{repo}/pull/{number}#discussion_r...`
- `https://github.com/{owner}/{repo}/pull/{number}#pullrequestreview-...`

Use `gh` for all GitHub operations. Require `gh` auth; if unauthenticated, stop and tell the user to run `gh auth login`.

## Workflow checklist

Copy and track progress:

```
- [ ] 1. Inspect PR state (branch, head commit, CI)
- [ ] 2. Fetch all inline review comments + unresolved threads
- [ ] 3. Build a per-comment checklist (file, suggestion, status)
- [ ] 4. Verify each item against current code; fix gaps with minimal diff
- [ ] 5. Run smoke checks (build / tests relevant to changed files)
- [ ] 6. Commit and push code fixes (if any)
- [ ] 7. Resolve all addressed review threads on GitHub
- [ ] 8. Post a PR conversation reply @-mentioning the reviewer bot (e.g. `@gemini-code-assist`)
```

## Step 1 — PR state

Run in parallel:

```bash
gh pr view {number} --repo {owner}/{repo} --json title,headRefName,baseRefName,headRefOid,state,commits
git status -sb
git branch --show-current
```

If not on the PR head branch, checkout it:

```bash
gh pr checkout {number} --repo {owner}/{repo}
```

## Step 2 — Fetch review feedback

**Inline comments** (suggestions, file/line context):

```bash
gh api repos/{owner}/{repo}/pulls/{number}/comments --paginate
```

**Unresolved threads** (preferred — includes resolved state):

```bash
gh api graphql -f query='
query {
  repository(owner: "{owner}", name: "{repo}") {
    pullRequest(number: {number}) {
      reviewThreads(first: 50) {
        nodes {
          id
          isResolved
          comments(first: 1) {
            nodes {
              body
              path
              line
              author { login }
            }
          }
        }
      }
    }
  }
}'
```

Filter to **unresolved** threads first. Skip bot noise (CI reports, dependency bots) unless the user asked to handle them. Prioritize human reviewers and code-assist bots (e.g. `gemini-code-assist`, `copilot-pull-request-reviewer`).

When reading API output, extract only: thread id, file path, line, comment body, author — do not dump full JSON into the reply.

## Step 3 — Build checklist

For each unresolved thread, record:

| # | File | Reviewer | Summary | Code status |
|---|------|----------|---------|-------------|
| 1 | `path/to/file` | author | one-line intent | pending / fixed / N/A |

Group related comments on the same file. If a later commit already addressed a comment but the thread is still open, mark **fixed** and plan to resolve without new code.

## Step 4 — Verify and fix

For each pending item:

1. Read the cited file at the relevant lines.
2. Compare current code to the reviewer's suggestion.
3. Apply the **smallest correct diff** that satisfies the feedback.
4. Match project conventions (existing components, error handling, naming).

**Do not** refactor unrelated code. **Do not** resolve a thread unless the fix is actually in the PR head commit (after push if you made changes).

If a comment is invalid or out of scope, skip the code change and explain why in the PR reply — do not resolve unless the user wants threads closed anyway.

## Step 5 — Smoke checks

Run checks proportional to touched files:

```bash
# Rust changes
cd src-tauri && cargo check

# Frontend changes
npm run build
# or: npx tsc --noEmit
```

Fix any failures introduced by your changes before committing.

## Step 6 — Commit and push (only if code changed)

Follow the repo's commit message style. One focused commit is enough:

```bash
git add <relevant files>
git commit -m "$(cat <<'EOF'
fix: address PR review feedback on <short topic>

EOF
)"
git push
```

Skip commit/push if no code changes were needed (threads-only resolution).

## Step 7 — Resolve threads

For each thread that is **fixed in the PR head**, run:

```bash
gh api graphql -f query='
mutation {
  resolveReviewThread(input: {threadId: "{thread_id}"}) {
    thread { isResolved }
  }
}'
```

Resolve **all** addressed threads, including ones fixed in an earlier commit on the branch.

Verify all threads are resolved:

```bash
gh api graphql -f query='...'  # same query as Step 2; confirm isResolved: true
```

## Step 8 — PR reply

Post a concise summary on the PR conversation. **Always @-mention the reviewer bot** that left the inline comments (e.g. `@gemini-code-assist`, `@copilot-pull-request-reviewer`). Detect the reviewer login from Step 2 thread data (`author.login`); if multiple bots reviewed, mention each one once at the top.

**Standard format** (follow PR #15 as the template):

```bash
gh pr comment {number} --repo {owner}/{repo} --body "$(cat <<'EOF'
Thanks @gemini-code-assist for the review! All {N} inline comments have been addressed in commit `<sha>` (`<commit subject>`):

1. **`file.ts`**: brief what changed
2. **`other.rs`**: brief what changed

All review threads are now marked as resolved.
EOF
)"
```

Rules for the reply body:

- **Opening line**: `Thanks @<reviewer-bot> for the review!` — required when the review came from a bot; use the exact GitHub login (e.g. `gemini-code-assist`, not the display name).
- **Commit reference**: include short SHA and commit subject on the same line as the item count.
- **Item list**: numbered, one entry per resolved thread; wrap file paths in backticks.
- **Closing line**: `All review threads are now marked as resolved.`
- Use the user's language (Chinese or English) for the change descriptions, but keep the `@mention` and structural phrases consistent with the template above.

## Decision rules

| Situation | Action |
|-----------|--------|
| Comment already fixed in branch | Resolve thread only; no new commit |
| Comment partially addressed | Finish the fix, then resolve |
| Conflicting review comments | Ask the user which approach to take |
| Comment on outdated/deleted lines | Verify fix exists elsewhere; resolve if satisfied |
| Requires design choice | Ask before implementing |

## Example trigger

User: `https://github.com/shencangsheng/easydb_app/pull/15#pullrequestreview-4433989260 把 review 都解决掉`

Agent should: parse PR 15 → fetch gemini-code-assist threads → verify/fix → resolve threads → reply on PR with `@gemini-code-assist` mention.

**Reference reply** (PR #15):

```
Thanks @gemini-code-assist for the review! All 7 inline comments have been addressed in commit `4f9e8d1` (`refactor: enhance notebook components for saved queries`):

1. **`db_utils.rs`** — `insert_saved_query` now opens a single DB connection and reuses it for `execute` + `last_insert_rowid`.
2. **`notebook-left-saved-queries.tsx`** — Search filter uses HeroUI `Input` with `placeholder`, `size="sm"`, and `variant="bordered"`.
...

All review threads are now marked as resolved.
```

## Hard rules

- Never force-push to `main`/`master`.
- Never skip pre-commit hooks unless the user explicitly asks.
- Only commit when there are actual code changes (or the user explicitly asks to commit).
- Resolve a thread only when the feedback is genuinely addressed in the PR head.
- Keep replies and diffs scoped to review feedback — no drive-by refactors.
- **Always @-mention the reviewing bot** in the PR summary comment (`@gemini-code-assist`, etc.); a reply without the mention is incomplete.
