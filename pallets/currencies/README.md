# Currencies Module

## Overview

The currencies module provides a mixed currencies system, by configuring a
native currency which implements `BasicCurrencyExtended`, and a
multi-currency which implements `MultiCurrency`.

It also provides an adapter, to adapt `frame_support::traits::Currency`
implementations into `BasicCurrencyExtended`.

The currencies module provides functionality of both `MultiCurrencyExtended`
and `BasicCurrencyExtended`, via unified interfaces, and all calls would be
delegated to the underlying multi-currency and base currency system.
A native currency ID could be set by `Config::GetNativeCurrencyId`, to
identify the native currency.

### Implementations

The currencies module provides implementations for following traits.

- `MultiCurrency` - Abstraction over a fungible multi-currency system.
- `MultiCurrencyExtended` - Extended `MultiCurrency` with additional helper
  types and methods, like updating balance
by a given signed integer amount.

## Interface

### Dispatchable Functions

- `transfer` - Transfer some balance to another account, in a given
   currency.
- `transfer_native_currency` - Transfer some balance to another account, in
   native currency set in
 `Config::NativeCurrency`.
- `update_balance` - Update balance by signed integer amount, in a given
   currency, root origin required.