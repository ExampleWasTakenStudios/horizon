# Release
This document describes how Horizon is released. This includes stable release as well as pre-releases.

It also covers the versioning used by Horizon.

## Versioning
Horizon uses the [semver](https://semver.org/) principle for its releases.

As Horizon it not a library used within other projects, but instead is intended to run independently on a users machine, the concept of breaking changes does not apply to it. Because of that, the `MAJOR` version of the semver string, indicates the latter two digits of the year the version was released in. 

The meanings of `MINOR` and `PATCH` are unchanged.

Version `x.2.4` released in 2026 would therefore be `26.2.4`.

## Pre-Releases
Horizon pre-releases are also released under the semver standard.

We use four categories to identify the nature of a pre-release:

- `alpha`
- `beta`
- `release candidate (rc)`
- `stable`

All pre-releases are identified by the appropriate suffix in the version string.

`alpha` releases are not always required, so a release chain may not include `alpha` releases. It will however, ***always*** include at least one `beta` and one `rc` release before a `stable` release is published.


