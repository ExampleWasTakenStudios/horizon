# Git Flow

This document outlines the strategies and workflows used by the project to progress.

## Introduction

As the name of this document implies, Horizon uses Git Flow workflow as it was first described by Vincent Driessen in the blog post [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/).

## Branching Strategy

While we follow the core logic of Git Flow, we use modified naming conventions for brevity.

Both `dev` and `main` only accept squash commits.

#### Updating `dev` with releases from `main`

The `dev` branch receives all updates on main by merging `main` into `dev`. This means that a `release` or `hotfix` branch should never be merged into `dev`. Instead, they should be merged to `main` which would then be merged into `dev`.

##### Conflict Resolution Strategy: 
When merging `main` into `dev`, conflicts often occur in version files (e.g., `package.json`).
* **Version Numbers:** Accept **Current Change** (preserve the `dev` version, e.g., `v26.2.0-alpha`).
* **Code/Logic:** Resolve manually. Ensure hotfixes from `main` (Incoming) are combined with new features in `dev` (Current). **Do not** blindly accept "Current Changes" for code, or you will revert the hotfix.

### Long-Lived Branches

| Branch      | Standard Name | Horizon Name | Role                                                                   |
| ----------- | ------------- | ------------ | ---------------------------------------------------------------------- |
| **Master**  | `master`      | **`main`**   | Stores the official release history. **Production Ready.**             |
| **Develop** | `develop`     | **`dev`**    | Integration branch for features. Contains the latest development code. |

### Supporting Branches

These branches are short-lived and are eventually removed after merging.

#### Feature Branches (`feat/*`)

- **Branched From**: `dev`
- **Merges Into**: `dev`
- **Purpose**: New features for a future release.
- **Naming**: `feat/feature-name`

#### Bugfix Branches (`fix/*`)

- **Branched From:** `dev`
- **Merges Into:** `dev`
- **Purpose:** Fixing bugs found during development (non-production).
- **Naming:** `fix/bug-description`

#### Release Branches (`release/*`)

- **Branched From:** `dev`
- **Merges Into:** `main`
- **Purpose:** Preparation for a new production release (Alpha/Beta/RC phases).
- **Naming:** `release/vYY.MINOR.PATCH` (e.g., `release/v26.1.0`)

#### Hotfix Branches (`hotfix/*`)

- **Branched From:** `main`
- **Merges Into:** `main`
- **Purpose:** Critical patches for the version currently in production.
- **Naming:** `hotfix/vYY.MINOR.PATCH` (e.g., `hotfix/v26.1.1`)

> [!IMPORTANT]
> Release and Hotfix branches **MUST** be merged from the respective release or hotfix branch into `main` **FIRST** as the `dev` branch is updated by merging the `main` branch into it.
