---
title: Rust API
---

Miraland's Rust crates are [published to crates.io][crates.io] and can be found
[on docs.rs with the "miraland-" prefix][docs.rs].

[crates.io]: https://crates.io/search?q=miraland-
[docs.rs]: https://docs.rs/releases/search?query=miraland-

Some important crates:

- [`miraland-program`] &mdash; Imported by programs running on Miraland, compiled
  to BPF. This crate contains many fundamental data types and is re-exported from
  [`miraland-sdk`], which cannot be imported from a Miraland program.

- [`miraland-sdk`] &mdash; The basic off-chain SDK, it re-exports
  [`miraland-program`] and adds more APIs on top of that. Most Miraland programs
  that do not run on-chain will import this.

- [`miraland-client`] &mdash; For interacting with a Miraland node via the
  [JSON RPC API](jsonrpc-api).

- [`miraland-cli-config`] &mdash; Loading and saving the Miraland CLI configuration
  file.

- [`miraland-clap-utils`] &mdash; Routines for setting up a CLI, using [`clap`],
  as used by the main Miraland CLI. Includes functions for loading all types of
  signers supported by the CLI.

[`miraland-program`]: https://docs.rs/miraland-program
[`miraland-sdk`]: https://docs.rs/miraland-sdk
[`miraland-client`]: https://docs.rs/miraland-client
[`miraland-cli-config`]: https://docs.rs/miraland-cli-config
[`miraland-clap-utils`]: https://docs.rs/miraland-clap-utils
[`clap`]: https://docs.rs/clap
