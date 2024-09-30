use instructions::*;
use anchor_lang::prelude::*;

pub mod config;
pub mod signature;
pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;

declare_id!("EtWFjjyscJFt29nJp5vJdQmQ3usqY1dKmQnTbbNNPqGY");

#[program]
pub mod sale_handler {

use signature::check_sign;

  use super::*;

  pub fn init(
    ctx: Context<InitSaleHandler>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::init_sale_handler(ctx)
  }

  pub fn set_sale_handler_cap(
    ctx: Context<SetSaleHandlerCap>,
    max_cap: u64,
    min_cap: u64,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::set_sale_handler_cap(ctx, max_cap, min_cap)
  }

  pub fn set_sale_handler_partner_interest(
    ctx: Context<SetSaleHandlerInterest>,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::set_sale_handler_interest(ctx, main_interest, secondary_interest)
  }

  pub fn set_sale_handler_purchase_bonus(
    ctx: Context<SetSaleHandlerBonus>,
    thresholds: Vec<u64>,
    percents: Vec<u64>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::set_sale_handler_purchase_bonus(ctx, thresholds, percents)
  }

  pub fn enable_sale_handler(
    ctx: Context<SetSaleHandlerEnabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::enable_sale_handler(ctx)
  }

  pub fn disable_sale_handler(
    ctx: Context<SetSaleHandlerDisabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::sale_handler::disable_sale_handler(ctx)
  }

  pub fn purchase_with_sol(
    ctx: Context<PurchaseSol>,
    partner_code: String,
    amount: u64,
  ) -> Result<()> {
    instructions::sale_handler::purchase_with_sol(ctx, partner_code, amount)
  }

  pub fn purchase_with_usdc(
    ctx: Context<PurchaseUSDC>,
    partner_code: String,
    amount: u64,
  ) -> Result<()> {
    instructions::sale_handler::purchase_with_usdc(ctx, partner_code, amount)
  }

  pub fn purchase_with_usdt(
    ctx: Context<PurchaseUSDT>,
    partner_code: String,
    amount: u64,
  ) -> Result<()> {
    instructions::sale_handler::purchase_with_usdt(ctx, partner_code, amount)
  }

  pub fn init_step(
    ctx: Context<InitStep>,
    id: i16,
    price: u64,
    total_supply: u128,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::step::init_step(ctx, id, price, total_supply)
  }

  pub fn set_step_price(
    ctx: Context<SetStepPrice>,
    price: u64,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::step::set_step_price(ctx, price)
  }

  pub fn set_step_supply(
    ctx: Context<SetStepSupply>,
    total_supply: u128,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::step::set_step_supply(ctx, total_supply)
  }

  pub fn enable_step(
    ctx: Context<SetStepEnabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::step::enable_step(ctx)
  }

  pub fn disable_step(
    ctx: Context<SetStepDisabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::step::disable_step(ctx)
  }

  pub fn init_partner(
    ctx: Context<InitPartner>,
    _partner_code: String,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::partner::init_partner(ctx, main_interest, secondary_interest)
  }

  pub fn set_partner_interest(
    ctx: Context<SetPartnerInterest>,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::partner::set_partner_interest(ctx, main_interest, secondary_interest)
  }

  pub fn enable_partner(
    ctx: Context<SetPartnerEnabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::partner::enable_partner(ctx)
  }

  pub fn disable_partner(
    ctx: Context<SetPartnerDisabled>,
  ) -> Result<()> {
    if !config::only_owners(ctx.accounts.payer.key()) {
      return err!(errors::SaleHandler::Unauthorized);
    }

    instructions::partner::disable_partner(ctx)
  }

  pub fn receive_sol(
    ctx: Context<ReceiveSol>,
    partner: String,
    deadline: u128,
    sig: [u8; 64],
    idx: u32,
  ) -> Result<()> {
    check_sign(idx, &partner, &ctx.accounts.payer, sig, &ctx.accounts.ix_sysvar, deadline).unwrap();
    instructions::partner::receive_sol(ctx, partner)
  }

  pub fn receive_usdc(
    ctx: Context<ReceiveUSDC>,
    partner: String,
    deadline: u128,
    sig: [u8; 64],
    idx: u32,
  ) -> Result<()> {
    check_sign(idx, &partner, &ctx.accounts.payer, sig, &ctx.accounts.ix_sysvar, deadline).unwrap();
    instructions::partner::receive_usdc(ctx, partner)
  }

  pub fn receive_usdt(
    ctx: Context<ReceiveUSDT>,
    partner: String,
    deadline: u128,
    sig: [u8; 64],
    idx: u32,
  ) -> Result<()> {
    check_sign(idx, &partner, &ctx.accounts.payer, sig, &ctx.accounts.ix_sysvar, deadline).unwrap();
    instructions::partner::receive_usdt(ctx, partner)
  }
}
