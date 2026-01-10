# Setup

This file will guide you through setting up the dev environment for Horizon.

> [!CAUTION]
> We only support Linux as development environment. If you are using Windows you can use [WSL](https://learn.microsoft.com/en-us/windows/wsl/).

## Environment Configuration

We provide complete scripts for setting up the development environment.

After cloning the repository, switch to the project directory and run

```sh
./scripts/setup.sh
```

The script might fail if required software is not installed. To install all required software packages, run

```sh
./scripts/install.sh
```

A list with required software is provided below.

## Required Software

The following software is required to develop Horizon:

- [corepack](https://github.com/nodejs/corepack)
  - The packet manager manager.
- [pnpm](https://pnpm.io)
  - Our packet manager.
- [authbind](https://manpages.ubuntu.com/manpages/xenial/man1/authbind.1.html)
  - Used to allow the node process to bind to priviledged ports.
