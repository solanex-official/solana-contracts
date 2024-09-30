use anchor_lang::prelude::*;

#[event]
pub struct PurchaseWithSol {
  pub step: i16,
  pub purchaser: Pubkey,
  pub partner: String,
  pub usd_equivalent: u128,
  pub sol_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct PurchaseWithUsdt {
  pub step: i16,
  pub purchaser: Pubkey,
  pub partner: String,
  pub usd_equivalent: u128,
  pub usdt_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct PurchaseWithUsdc {
  pub step: i16,
  pub purchaser: Pubkey,
  pub partner: String,
  pub usd_equivalent: u128,
  pub usdc_amount: u64,
  pub token_amount: u128,
}

#[event]
pub struct ReceiveSol {
  pub partner: String,
  pub amount: u64,
}

#[event]
pub struct ReceiveUsdt {
  pub partner: String,
  pub amount: u64,
}

#[event]
pub struct ReceiveUsdc {
  pub partner: String,
  pub amount: u64,
}
