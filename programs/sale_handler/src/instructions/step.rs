use anchor_lang::prelude::*;
use crate::state::step::Step;
use crate::state::sale_handler::SaleHandler;

use crate::config::STEP_TAG;

pub fn init_step(
  ctx: Context<InitStep>,
  id: i16,
  price: u64,
  total_supply: u128,
) -> Result<()> {
  let step = &mut ctx.accounts.step;
  step.init(id, price, total_supply)
}

pub fn set_step_price(
  ctx: Context<SetStepPrice>,
  price: u64,
) -> Result<()> {
  let step = &mut ctx.accounts.step;
  step.set_price(price)
}

pub fn set_step_supply(
  ctx: Context<SetStepSupply>,
  total_supply: u128
) -> Result<()> {
  let step = &mut ctx.accounts.step;
  step.set_total_supply(total_supply)
}

pub fn enable_step(
  ctx: Context<SetStepEnabled>,
) -> Result<()> {
  let step = &mut ctx.accounts.step;
  step.set_enable().unwrap();

  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_step(step.get_id())
}

pub fn disable_step(
  ctx: Context<SetStepDisabled>,
) -> Result<()> {
  let step = &mut ctx.accounts.step;
  step.set_disable()
}

#[derive(Accounts)]
#[instruction(id: i16)]
pub struct InitStep<'info> {
  #[account(
    init,
    payer = payer,
    space = 8 + Step::MAX_SIZE,
    seeds = [
      STEP_TAG,
      b"_",
      &id.to_le_bytes()
    ],
    bump,
  )]
  pub step: Account<'info, Step>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct SetStepPrice<'info> {
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(total_supply: u128)]
pub struct SetStepSupply<'info> {
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetStepEnabled<'info> {
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetStepDisabled<'info> {
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}