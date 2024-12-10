NOTE: THIS IS JUST A ROUGH DRAFT 1.0 VERSION IT HAS NOT BEEN TESTED AND SHOULD NOT BE ASSUMED TO BE FUNCTIONAL AT THIS POINT

MintRewards NFT Reward Program
https://MintRewards.pw

A Solana program for managing NFT minting rewards with configurable distribution schedules.
Overview
This program manages a reward system for NFT minters where a portion of minting fees are collected into reward pools. On the 15th of each month, one NFT minter from the previous month is randomly selected to receive rewards. Winners can choose from four different distribution schedules:

1. Immediate Claim
Wallet Holder Receives: 50% of allocated reward instantly

50% instant payment to NFT holder wallet
25% burned (increasing token scarcity)
25% returned to Monthly Reward Pool which will be added to next monthly reward

2. 6 Month Distribution
Wallet Holder Receives 70% of their total allocated reward

Equal monthly payments over 6 months
15% burned
15% returned to Monthly Reward Pool

3. 12 Month Distribution
Wallet Holder Receives 85% of their total allocated reward

Equal quarterly payments over 12 months
7.5% burned
7.5% returned to Monthly Reward Pool

4. 18 Month Distribution
Wallet Holder Receives 100% of their total allocated reward

Two equal payments over 18 months
0% burned
0% returned to Monthly Reward Pool

Prerequisites

Rust toolchain
Solana CLI tools
Anchor Framework
Node.js and yarn
