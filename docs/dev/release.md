# Release
This document describes how Horizon is released. This includes stable release as well as pre-releases.

It also covers the versioning used by Horizon.

## Versioning
Horizon uses a modified version of the [semver](https://semver.org/) principle for its releases that follows the format **`YY.MINOR.PATCH`**. This style of versioning is also known as `CalVer`. As there is no official specification for CalVer, we use the semver spec as reference for precedence etc.

- **Major (`YY`):** The last two digits of the release year (e.g., `26` for 2026).
- **Minor:** Incremented for new features or significant changes within the year.
- **Patch:** Incremented for bug fixes and maintenance.

> [!NOTE]
> Breaking changes are usually handled gracefully. E.g. config files will be updated to new standards, versions, etc. along with proper deprecation notices.

## Pre-Releases
Horizon pre-releases are also released under the semver standard.

We use three categories to identify the nature of a pre-release:

- `alpha`
- `beta`
- `release candidate (rc)`

All pre-releases are identified by the appropriate suffix in the version string.

`alpha` releases are not always required, so a release chain may not include `alpha` releases. It will however, ***always*** include at least one `beta` and one `rc` release before a `stable` release is published.

## Releases
Horizon is released in continuous order. That means there are not LTS releases or similar.

Once the decision has been made to release the current feature set as a new release, a release branch (see [git-flow.md](./git-flow.md)) is created. This marks the end of the feature contribution for that release and the start of the release procedure outlined below.

There are up to four but at least three stages leading up to a release:

### 1. Alpha
Depending on the complexity of the additions, developers may elect to start an alpha phase to indicate that, while features are generally considered working, bugs are very likely.

As this does not apply to all releases, the alpha stage may be skipped.

### 2. Beta
The beta stage indicates releases that are generally considered working but may still contain bugs.

The beta stage is mandatory for all releases and is entered into as soon as the release branch has been created, unless it has been decided that there would be an alpha stage.

If there was an alpha stage, the beta stage follows it.

### 3. Release Candidate
Release candidates (RCs) are beta versions that are considered stable enough for a stable release. The goal of the RCs is to give the software a final grace period during which bugs may be fixed without requiring a full release cycle.

The Release Candidate stage follows the beta stage and is mandatory for all releases.

### 4. Stable Release
The stable release is a full production release. The software is considered stable enough to work in full production environments.

### *Special Case: Hotfix*
Hotfixes address critical bugs (e.g., security vulnerabilities or startup crashes) that cannot wait for a full release cycle.

#### Procedure
Unlike standard releases, hotfixes bypass the Alpha/Beta/RC phases.
1. A hotfix is branched directly from the `main` stable branch.
2. It undergoes a focused review and test cycle.
3. It is released immediately as a `PATCH` version increment (e.g., `26.2.4` -> `26.2.5`).
