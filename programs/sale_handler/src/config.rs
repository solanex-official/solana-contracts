use anchor_lang::prelude::*;

pub const MAX_CAP: u64            = 1_000_000_000_000_000; 
pub const MIN_CAP: u64            = 1_000_000_000;
pub const MAIN_INTEREST: u64      = 150_000_000;
pub const SECONDARY_INTEREST: u64 = 50_000_000;

pub const STEP_TAG: &[u8]           = b"STEP";
pub const PURCHASER_TAG: &[u8]      = b"PURCHASER";
pub const PARTNER_TAG: &[u8]        = b"PARTNER";
pub const BANK: &str                = "5rtu57yuSYYrqRe6VXJUAkZKU9RQpBiReuQ3CFKU2aCN";

pub const SOL_USD_PRICEFEED: &str   = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";
pub const FEED_ID: &str             = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const FEED_MAXIMUM_AGE: u64     = 3600; // 1 hour

pub const PRECISION: u32            = 9;
pub const STABLE_PRECISION: u32     = 3;
pub const USDT: &str                = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
pub const USDC: &str                = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub const SIGNATURE_SIGNER: &str     = "DfXfwqnkMMZHjdHJ2Ndhhty15n4okSXyKhrdYUKDNnUe";

const OWNERS: &[&str] = &["2xB43uGrEvZVoUqMYBKBPi6UzWXjavj1BuxjpVErE2gk", "DfXfwqnkMMZHjdHJ2Ndhhty15n4okSXyKhrdYUKDNnUe"];

pub fn only_owners(address: Pubkey) -> bool {
  return OWNERS.contains(&address.to_string().as_str());
}
