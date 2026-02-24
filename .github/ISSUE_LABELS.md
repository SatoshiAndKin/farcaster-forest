# Issue Labeling System

This document defines the standard labels used in this repository to organize and prioritize work.

## Priority Labels

Use these to indicate urgency and importance:

- **P0 - Critical** 🔴
  - Blocks development or breaks production
  - Must be fixed immediately
  - Examples: Build failures, game crashes, security vulnerabilities

- **P1 - High** 🟠
  - Important features or significant bugs
  - Should be addressed in current sprint/milestone
  - Examples: Major gameplay issues, performance problems

- **P2 - Medium** 🟡
  - Standard priority for most work
  - Schedule for upcoming sprints
  - Examples: New features, minor bugs, improvements

- **P3 - Low** 🟢
  - Nice to have, not urgent
  - Backlog items for future consideration
  - Examples: Polish, minor optimizations, wishlist features

## Type Labels

Categorize the nature of the work:

- **bug** 🐛
  - Something isn't working correctly
  - Includes crashes, incorrect behavior, performance issues

- **feature** ✨
  - New functionality or enhancement
  - User-facing improvements

- **chore** 🔧
  - Maintenance tasks
  - Dependency updates, refactoring, tooling

- **documentation** 📚
  - Improvements to docs, comments, or README
  - Clarifications and examples

- **test** 🧪
  - Test coverage, test fixes, test infrastructure
  - CI/CD improvements

- **performance** ⚡
  - Optimization work
  - Frame rate, load times, memory usage

- **security** 🔒
  - Security vulnerabilities or improvements
  - Authentication, authorization, data protection

## Area Labels

Specify which part of the codebase is affected:

- **area: audio** 🔊
  - Spatial audio, bird calls, sound effects

- **area: birds** 🐦
  - Bird behavior, species, AI systems

- **area: scene** 🌳
  - Scene setup, day/night cycle, environment

- **area: mobile** 📱
  - iOS or Android specific issues

- **area: web** 🌐
  - WASM build or web-specific issues

- **area: ci/cd** 🚀
  - GitHub Actions, builds, releases

- **area: assets** 🎨
  - 3D models, textures, audio files

## Status Labels

Track the state of issues:

- **status: blocked** 🚫
  - Cannot proceed due to external dependency
  - Specify blocker in issue description

- **status: in-progress** 🔄
  - Currently being worked on
  - Assigned to someone

- **status: needs-review** 👀
  - PR created, awaiting review

- **status: help-wanted** 🙋
  - Community contributions welcome
  - Good for new contributors

## Platform Labels

For platform-specific issues:

- **platform: windows** 🪟
- **platform: linux** 🐧
- **platform: macos** 🍎
- **platform: ios** 📱
- **platform: android** 🤖
- **platform: web** 🌐

## Special Labels

- **good-first-issue** 🌱
  - Easy for new contributors
  - Well-defined scope

- **breaking-change** 💥
  - Requires version bump
  - May affect users

- **duplicate** 🔂
  - Already reported elsewhere
  - Reference original issue

## Usage Guidelines

### For Issue Creators
1. Start with a **type** label (bug, feature, etc.)
2. Add a **priority** label (P0-P3)
3. Add **area** labels as applicable
4. Add **platform** labels if platform-specific

### For Maintainers
- Use **status** labels to track progress
- Apply **good-first-issue** to help onboard contributors
- Mark **breaking-change** during design review
- Add **help-wanted** for community involvement

## Examples

**Critical bug:**
- Labels: `bug`, `P0`, `area: audio`, `platform: web`

**New feature request:**
- Labels: `feature`, `P2`, `area: birds`

**Performance improvement:**
- Labels: `performance`, `P1`, `area: scene`, `help-wanted`

**Documentation:**
- Labels: `documentation`, `P3`, `good-first-issue`

---

This labeling system enables:
- Efficient issue triage and prioritization
- Clear communication about work status
- Easy filtering for specific types of work
- Better organization for both humans and AI agents
