
# DEX Module

## Overview

Built-in decentralized exchange modules in Curio-parachain, the swap
mechanism refers to the design of Uniswap V2. In addition to being used for
trading, DEX also participates in CDP liquidation, which is faster than
liquidation by auction when the liquidity is sufficient. And providing
market making liquidity for DEX will also receive stable currency as
additional reward for its participation in the CDP liquidation.
## Interface

### Dispatchable Functions
- `swap_with_exact_supply` - Trading with DEX, swap with exact supply amount.
- `swap_with_exact_target` - Trading with DEX, swap with exact target amount.
- `add_liquidity` - Add liquidity to Enabled trading pair.
- `add_provision` - Add provision to Provisioning trading pair. If succeed, will record the provision, but shares issuing will happen after the trading pair convert to Enabled status.
- `claim_dex_share` - Claim dex share for founders who have participated in trading pair provision.
- `remove_liquidity` - Remove liquidity from specific liquidity pool in the form of burning shares, and withdrawing currencies in trading pairs from liquidity pool in proportion, and withdraw liquidity incentive interest.
- `list_provisioning` - List a new provisioning trading pair.
- `update_provisioning_parameters` - List a new trading pair, trading pair will become Enabled status after provision process.
- `end_provisioning` - Enable a Provisioning trading pair if meet the condition.
- `enable_trading_pair` - Enable a trading pair if the status of trading pair is `Disabled`, or `Provisioning` without any accumulated provision, enable it directly.
- `disable_trading_pair` - Disable a `Enabled` trading pair.
- `refund_provision` - Refund provision if the provision has already aborted.
- `abort_provisioning` - Abort provision when it's don't meet the target and expired.