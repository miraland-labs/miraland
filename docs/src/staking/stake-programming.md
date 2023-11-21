---
title: Stake Programming
---

To maximize stake distribution, decentralization, and censorship resistance on
the Miraland network, staking can be performed programmatically. The team
and community have developed several on-chain and off-chain programs to make
stakes easier to manage.

#### Stake-o-matic aka Auto-delegation Bots

This off-chain program manages a large population of validators staked by a
central authority. The Miraland Foundation uses an auto-delegation bot to regularly delegate its
stake to "non-delinquent" validators that meet specified performance requirements.

#### Stake Pools

This on-chain program pools together MLN to be staked by a manager, allowing MLN
holders to stake and earn rewards without managing stakes.
Users deposit MLN in exchange for Solarti tokens (staking derivatives) that represent their ownership in the stake pool. The pool
manager stakes deposited MLN according to their strategy, perhaps using a variant
of an auto-delegation bot as described above. As stakes earn rewards, the pool and pool tokens
grow proportionally in value. Finally, pool token holders can send Solarti tokens
back to the stake pool to redeem MLN, thereby participating in decentralization with much
less work required. More information can be found at the
[SPL stake pool documentation](https://spl.miraland.top/stake-pool).
