---
id: task-0039
title: Clean up compiler warnings with cargo clippy
status: To Do
assignee: []
created_date: '2026-01-16 21:28'
updated_date: '2026-01-16 21:30'
labels:
  - quality
  - clippy
  - phase7
dependencies:
  - task-0047
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Eliminate all compiler warnings and clippy lints. Warnings indicate potential bugs, non-idiomatic code, or future compatibility issues. A clean build with no warnings demonstrates code quality and prevents warning fatigue where real issues get lost in noise.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 cargo build produces zero warnings
- [ ] #2 cargo clippy produces zero warnings (with default lint level)
- [ ] #3 No #[allow(unused)] attributes hiding legitimate issues
- [ ] #4 No dead code remaining in the codebase
- [ ] #5 All public items have documentation (no missing_docs warnings when enabled)
- [ ] #6 CI check for warnings is added to justfile
<!-- AC:END -->
