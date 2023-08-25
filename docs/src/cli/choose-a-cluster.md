---
title: Connecting to a Cluster
---

See [Miraland Clusters](../clusters.md) for general information about the
available clusters.

## Configure the command-line tool

You can check what cluster the Miraland command-line tool (CLI) is currently targeting by
running the following command:

```bash
miraland config get
```

Use `miraland config set` command to target a particular cluster. After setting
a cluster target, any future subcommands will send/receive information from that
cluster.

For example to target the Devnet cluster, run:

```bash
miraland config set --url https://api.devnet-mln.miraland.top
```

## Ensure Versions Match

Though not strictly necessary, the CLI will generally work best when its version
matches the software version running on the cluster. To get the locally-installed
CLI version, run:

```bash
miraland --version
```

To get the cluster version, run:

```bash
miraland cluster-version
```

Ensure the local CLI version is greater than or equal to the cluster version.
