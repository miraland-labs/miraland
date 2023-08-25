---
title: "Program Examples"
---

## Helloworld

Hello World is a project that demonstrates how to use the Miraland Javascript API
and both Rust and C programs to build, deploy, and interact with programs on the
Miraland blockchain.

The project comprises of:

- An on-chain hello world program
- A client that can send a "hello" to an account and get back the number of
  times "hello" has been sent

### Build and Run

First fetch the latest version of the example code:

```bash
$ git clone https://github.com/miraland-labs/example-helloworld.git
$ cd example-helloworld
```

Next, follow the steps in the git repository's
[README](https://github.com/miraland-labs/example-helloworld/blob/master/README.md).

## Break

[Break](https://break.miraland.top/) is a React app that gives users a visceral
feeling for just how fast and high-performance the Miraland network really is. Can
you _break_ the Miraland blockchain? During a 15 second play-though, each click of
a button or keystroke sends a new transaction to the cluster. Smash the keyboard
as fast as you can and watch your transactions get finalized in real time while
the network takes it all in stride!

Break can be played on our Devnet, Testnet and Mainnet Beta networks. Plays are
free on Devnet and Testnet, where the session is funded by a network faucet. On
Mainnet Beta, users pay to play 0.08 MLN per game. The session account can be
funded by a local keystore wallet or by scanning a QR code from Trust Wallet to
transfer the tokens.

[Click here to play Break](https://break.miraland.top/)

### Build and Run

First fetch the latest version of the example code:

```bash
$ git clone https://github.com/miraland-labs/break.git
$ cd break
```

Next, follow the steps in the git repository's
[README](https://github.com/miraland-labs/break/blob/master/README.md).

## Language Specific

- [Rust](developing-rust.md#examples)
- [C](developing-c.md#examples)
