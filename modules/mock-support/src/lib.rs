#![cfg_attr(not(feature = "std"), no_std)]

pub mod collections;

pub mod primitives {
    pub type AccountId = u64;
    pub type Balance = u128;
}

pub mod accounts {
    use super::primitives::AccountId;

    pub const ALICE: AccountId = 1;
    pub const BOB: AccountId = 2;
    pub const CHARLIE: AccountId = 3;
    pub const EVE: AccountId = 4;
    pub const DAVE: AccountId = 5;
    pub const JASON: AccountId = 6;
    pub const JOHN: AccountId = 7;
}

pub mod consts {
    use super::primitives::Balance;

    pub const DOLLARS: Balance = 1_000_000_000_000_000_000;
}