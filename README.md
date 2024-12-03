NOTE: THIS IS JUST A ROUGH DRAFT 1.0 VERSION IT HAS NOT BEEN TESTED AND SHOULD NOT BE ASSUMED TO BE FUNCTIONAL AT THIS POINT

NFT Reward Program
A Solana program for managing NFT minting rewards with configurable distribution schedules.
Overview
This program manages a reward system for NFT minters where a portion of minting fees are collected into reward pools. On the 15th of each month, one NFT minter from the previous month is randomly selected to receive rewards. Winners can choose from four different distribution schedules:

1. Immediate Claim (50% Total Distribution)

50% instant payment
25% burned
25% returned to pool


2. Monthly Distribution (70% Total Distribution)

70% paid over 6 months
15% burned
15% returned to pool


3. Quarterly Distribution (85% Total Distribution)

85% paid over 12 months
7.5% burned
7.5% returned to pool


4. Yearly Distribution (100% Total Distribution)

100% paid in two installments over 18 months



Prerequisites

Rust toolchain
Solana CLI tools
Anchor Framework
Node.js and yarn
