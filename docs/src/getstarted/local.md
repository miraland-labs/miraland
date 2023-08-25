---
title: "Local Development Quickstart"
description: "This quickstart guide will demonstrate how to quickly install and setup your local Miraland development environment."
keywords:
  - rust
  - cargo
  - toml
  - program
  - tutorial
  - intro to miraland development
  - blockchain developer
  - blockchain tutorial
  - web3 developer
---

This quickstart guide will demonstrate how to quickly install and setup your local development environment, getting you ready to start developing and deploying Miraland programs to the blockchain.

## What you will learn

- How to install the Miraland CLI locally
- How to setup a localhost Miraland cluster/validator
- How to create a Miraland wallet for developing
- How to airdrop MLN tokens for your wallet

## Install the Miraland CLI

To interact with the Miraland clusters from your terminal, install the [Miraland CLI tool suite](./../cli/install-miraland-cli-tools) on your local system:

```bash
sh -c "$(curl -sSfL https://release.miraland.top/stable/install)"
```

## Setup a localhost blockchain cluster

The Miraland CLI comes with the [test validator](./../developing/test-validator.md) built in. This command line tool will allow you to run a full blockchain cluster on your machine.

```bash
miraland-test-validator
```

> **PRO TIP:**
> Run the Miraland test validator in a new/separate terminal window that will remain open. The command line program must remain running for your localhost cluster to remain online and ready for action.

Configure your Miraland CLI to use your localhost validator for all your future terminal commands:

```bash
miraland config set --url localhost
```

At any time, you can view your current Miraland CLI configuration settings:

```bash
miraland config get
```

## Create a file system wallet

To deploy a program with Miraland CLI, you will need a Miraland wallet with MLN tokens to pay for the cost of transactions.

Let's create a simple file system wallet for testing:

```bash
miraland-keygen new
```

By default, the `miraland-keygen` command will create a new file system wallet located at `~/.config/miraland/id.json`. You can manually specify the output file location using the `--outfile /path` option.

> **NOTE:**
> If you already have a file system wallet saved at the default location, this command will **NOT** override it (unless you explicitly force override using the `--force` flag).

#### Set your new wallet as default

With your new file system wallet created, you must tell the Miraland CLI to use this wallet to deploy and take ownership of your on chain program:

```bash
miraland config set -k ~/.config/miraland/id.json
```

## Airdrop MLN tokens to your wallet

Once your new wallet is set as the default, you can request a free airdrop of MLN tokens to it:

```bash
miraland airdrop 2
```

> **NOTE:**
> The `miraland airdrop` command has a limit of how many MLN tokens can be requested _per airdrop_ for each cluster (localhost, testnet, or devent). If your airdrop transaction fails, lower your airdrop request quantity and try again.

You can check your current wallet's MLN balance any time:

```bash
miraland balance
```

## Next steps

See the links below to learn more about writing Rust based Miraland programs:

- [Create and deploy a Miraland Rust program](./rust.md)
- [Overview of writing Miraland programs](../developing/on-chain-programs/overview)
