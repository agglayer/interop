---
name: design
description: >
  Required alongside brainstorming for all design work in this
  repository. Overrides spec and plan output paths to
  docs/knowledge-base/src/ai-agents/ instead of the superpowers
  defaults. Load this skill whenever you load brainstorming.
---

This skill provides project-local overrides
for the superpowers `brainstorming` and `writing-plans` skills.
Load it **alongside** brainstorming, not instead of it.

## What this skill changes

The only thing this skill does is redirect output paths.
Without it, specs and plans land in `docs/superpowers/`
(the superpowers default), which is the wrong location
for this repository.

## Output location

Write specs and plans under `docs/knowledge-base/src/ai-agents/`
instead of the superpowers default (`docs/superpowers/`):

- **Specs** →
  `docs/knowledge-base/src/ai-agents/specs/YYYY-MM-DD-<topic>-design.md`
- **Plans** →
  `docs/knowledge-base/src/ai-agents/plans/YYYY-MM-DD-<topic>-plan.md`

These paths override the brainstorming and writing-plans defaults.
Create the directories if they do not exist.
