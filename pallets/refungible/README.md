# Refungible Pallet

 The Refungible pallet provides functionality for handling refungible collections and tokens.

## Overview

 The Refungible pallet provides functions for:

 - RFT collection creation and removal
 - Minting and burning of RFT tokens
 - Partition and repartition of RFT tokens
 - Retrieving number of pieces of RFT token
 - Retrieving account balances
 - Transfering RFT token pieces
 - Burning RFT token pieces
 - Setting and checking allowance for RFT tokens

### Terminology

 - **RFT token:** Non fungible token that was partitioned to pieces. If an account owns all
   of the RFT token pieces than it owns the RFT token and can repartition it.

 - **RFT Collection:** A collection of RFT tokens. All RFT tokens are part of a collection.
   Each collection has its own settings and set of permissions.

 - **RFT token piece:** A fungible part of an RFT token.

 - **Balance:** RFT token pieces owned by an account

 - **Allowance:** Maximum number of RFT token pieces that one account is allowed to
   transfer from the balance of another account

 - **Burning:** The process of “deleting” a token from a collection or removing token pieces from
   an account balance.

## Interface

### Dispatchable Functions

 - `init_collection` - Create RFT collection. RFT collection can be configured to allow or deny access for
   some accounts.
 - `destroy_collection` - Destroy exising RFT collection. There should be no tokens in the collection.
 - `burn` - Burn some amount of RFT token pieces owned by account. Burns the RFT token if no pieces left.
 - `transfer` - Transfer some amount of RFT token pieces. Transfers should be enabled for RFT collection.
   Nests the RFT token if RFT token pieces are sent to another token.
 - `create_item` - Mint RFT token in collection. Sender should have permission to mint tokens.
 - `set_allowance` - Set allowance for another account to transfer balance from sender's account.
 - `repartition` - Repartition token to selected number of pieces. Sender should own all existing pieces.

## Assumptions

 * Total number of pieces for one token shouldn't exceed `collection_primitives::MAX_REFUNGIBLE_PIECES`.
 * Total number of tokens of all types shouldn't be greater than `collection_primitives::MAX_TOKEN_PREFIX_LENGTH`.
 * Sender should be in collection's allow list and in whitelist to perform operations on tokens.