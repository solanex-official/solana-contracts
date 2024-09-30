use anchor_lang::{
  prelude::*,
  solana_program::{ program::invoke, system_instruction::transfer },
};
use std::str::FromStr;
use anchor_spl::token::{ self, Token, TokenAccount, Transfer as SplTransfer };
use pyth_solana_receiver_sdk::price_update::{ get_feed_id_from_hex, PriceUpdateV2 };

use crate::errors;
use crate::events;
use crate::state::sale_handler::SaleHandler;
use crate::state::step::Step;
use crate::state::partner::Partner;
use crate::state::purchaser::Purchaser;

use crate::config::{
  SOL_USD_PRICEFEED, BANK, USDC, USDT,
  PRECISION, STABLE_PRECISION, PARTNER_TAG,
  PURCHASER_TAG, FEED_MAXIMUM_AGE, FEED_ID,
};

pub fn init_sale_handler(
  ctx: Context<InitSaleHandler>,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.init()
}

pub fn set_sale_handler_cap(
  ctx: Context<SetSaleHandlerCap>,
  max_cap: u64,
  min_cap: u64,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_cap(max_cap, min_cap)
}

pub fn set_sale_handler_interest(
  ctx: Context<SetSaleHandlerInterest>,
  main_interest: u64,
  secondary_interest: u64,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_interest(main_interest, secondary_interest)
}

pub fn set_sale_handler_purchase_bonus(
  ctx: Context<SetSaleHandlerBonus>,
  thresholds: Vec<u64>,
  percents: Vec<u64>,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_bonus(thresholds, percents)
}

pub fn enable_sale_handler(
  ctx: Context<SetSaleHandlerEnabled>,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_enable()
}

pub fn disable_sale_handler(
  ctx: Context<SetSaleHandlerDisabled>,
) -> Result<()> {
  let sale_handler = &mut ctx.accounts.sale_handler;
  sale_handler.set_disable()
}

pub fn purchase_with_sol(
  ctx: Context<PurchaseSol>,
  partner_code: String,
  amount: u64,
) -> Result<()> {
  let to_account_infos = &mut ctx.accounts.to_account_infos();
  let payer = &mut ctx.accounts.payer;
  let sale_handler = &mut ctx.accounts.sale_handler;
  let step = &mut ctx.accounts.step;
  let purchaser = &mut ctx.accounts.purchaser;
  let partner = &mut ctx.accounts.partner;
  let price_update = &ctx.accounts.price_update;
  let bank_info = &mut ctx.accounts.bank_info;

  if !sale_handler.is_enabled() {
    return err!(errors::SaleHandler::SaleHandlerNotEnabled);
  }

  if !step.is_enabled() {
    return err!(errors::SaleHandler::StepNotEnabled);
  }

  if sale_handler.get_step() != step.get_id() {
    return err!(errors::SaleHandler::InactiveStep);
  }

  if Pubkey::from_str(BANK) != Ok(bank_info.key()){
    return Err(error!(errors::SaleHandler::WrongBank))
  };

  if Pubkey::from_str(SOL_USD_PRICEFEED) != Ok(price_update.key()){
    return Err(error!(errors::SaleHandler::WrongPriceFeedId))
  };
  
  let (price, expo) = get_price(price_update).unwrap();
  let usd_amount = u128::from(amount) * price / 10u128.pow(expo);
  let mut token_amount = usd_amount * 10u128.pow(PRECISION) / u128::from(step.get_price());
  let bonus = sale_handler.calculate_bonus(usd_amount, token_amount);

  if sale_handler.get_max_cap() < usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMaxCapExceeded);
  }

  if sale_handler.get_min_cap() > usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMinCapNotReached);
  }

  if step.get_total_sold() + token_amount + bonus > step.get_total_supply() {
    return err!(errors::SaleHandler::StepSupplyExceeded);
  }
  
  let (partner_sol_reward, partner_token_reward) = get_interest(sale_handler, &partner_code, partner, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if partner_sol_reward > 0 {
    to_amount = to_amount - partner_sol_reward;
  }

  let instruction = &transfer(&payer.key(), &bank_info.key(), to_amount);
  invoke(instruction, to_account_infos).unwrap();

  if partner_sol_reward > 0 {
    let instruction = &transfer(&payer.key(), &partner.key(), partner_sol_reward);
    invoke(instruction, to_account_infos).unwrap();
  }

  token_amount += bonus;

  // Updating sale_handler details
  sale_handler.set_total_sold(token_amount).unwrap();

  // Updating step details
  step.set_total_sold(token_amount).unwrap();

  // Updating purchaser details
  purchaser.set_purchased(token_amount).unwrap();

  // Updating partner details
  if !partner_code.is_empty() {
    partner.set_sol_reward(partner_sol_reward).unwrap();
    partner.set_token_reward(partner_token_reward).unwrap();
  };

  emit!(events::PurchaseWithSol {
    step: step.get_id(),
    purchaser: payer.key(),
    partner: partner_code,
    usd_equivalent: usd_amount,
    sol_amount: amount,
    token_amount: token_amount,
  });
  Ok(())
}

pub fn purchase_with_usdc(
  ctx: Context<PurchaseUSDC>,
  partner_code: String,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let sale_handler = &mut ctx.accounts.sale_handler;
  let step = &mut ctx.accounts.step;
  let purchaser = &mut ctx.accounts.purchaser;
  let partner = &mut ctx.accounts.partner;

  let purchaser_ata = &ctx.accounts.purchaser_ata;
  let bank_ata = &ctx.accounts.bank_ata;
  let partner_pda_ata = &ctx.accounts.partner_pda_ata;
  let token_program = &ctx.accounts.token_program;

  if !sale_handler.is_enabled() {
    return err!(errors::SaleHandler::SaleHandlerNotEnabled);
  }

  if !step.is_enabled() {
    return err!(errors::SaleHandler::StepNotEnabled);
  }

  if sale_handler.get_step() != step.get_id() {
    return err!(errors::SaleHandler::InactiveStep);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let mut token_amount = usd_amount * 10u128.pow(PRECISION) / u128::from(step.get_price());
  let bonus = sale_handler.calculate_bonus(usd_amount, token_amount);

  if sale_handler.get_max_cap() < usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMaxCapExceeded);
  }

  if sale_handler.get_min_cap() > usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMinCapNotReached);
  }

  if step.get_total_sold() + token_amount + bonus > step.get_total_supply() {
    return err!(errors::SaleHandler::StepSupplyExceeded);
  }

  let (partner_usdc_reward, partner_token_reward) = get_interest(sale_handler, &partner_code, partner, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if partner_usdc_reward > 0 {
    to_amount = to_amount - partner_usdc_reward;
  }

  let cpi_accounts = SplTransfer {
    from: purchaser_ata.to_account_info(),
    to: bank_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if partner_usdc_reward > 0 {
    let cpi_accounts = SplTransfer {
      from: purchaser_ata.to_account_info(),
      to: partner_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), partner_usdc_reward).unwrap();
  }

  token_amount += bonus;

  // Updating sale_handler details
  sale_handler.set_total_sold(token_amount).unwrap();

  // Updating step details
  step.set_total_sold(token_amount).unwrap();

  // Updating purchaser details
  purchaser.set_purchased(token_amount).unwrap();

  // Updating partner details
  if !partner_code.is_empty() {
    partner.set_usdc_reward(partner_usdc_reward).unwrap();
    partner.set_token_reward(partner_token_reward).unwrap();
  };

  emit!(events::PurchaseWithUsdc {
    step: step.get_id(),
    purchaser: payer.key(),
    partner: partner_code,
    usd_equivalent: usd_amount,
    usdc_amount: amount,
    token_amount: token_amount,
  });

  Ok(())
}

pub fn purchase_with_usdt(
  ctx: Context<PurchaseUSDT>,
  partner_code: String,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let sale_handler = &mut ctx.accounts.sale_handler;
  let step = &mut ctx.accounts.step;
  let purchaser = &mut ctx.accounts.purchaser;
  let partner = &mut ctx.accounts.partner;

  let purchaser_ata = &ctx.accounts.purchaser_ata;
  let bank_ata = &ctx.accounts.bank_ata;
  let partner_pda_ata = &ctx.accounts.partner_pda_ata;
  let token_program = &ctx.accounts.token_program;

  if !sale_handler.is_enabled() {
    return err!(errors::SaleHandler::SaleHandlerNotEnabled);
  }

  if !step.is_enabled() {
    return err!(errors::SaleHandler::StepNotEnabled);
  }

  if sale_handler.get_step() != step.get_id() {
    return err!(errors::SaleHandler::InactiveStep);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let mut token_amount = usd_amount * 10u128.pow (PRECISION) / u128::from(step.get_price());
  let bonus = sale_handler.calculate_bonus(usd_amount, token_amount);

  if sale_handler.get_max_cap() < usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMaxCapExceeded);
  }

  if sale_handler.get_min_cap() > usd_amount {
    return err!(errors::SaleHandler::SaleHandlerMinCapNotReached);
  }

  if step.get_total_sold() + token_amount + bonus > step.get_total_supply() {
    return err!(errors::SaleHandler::StepSupplyExceeded);
  }

  let (partner_usdt_reward, partner_token_reward) = get_interest(sale_handler, &partner_code, partner, amount, token_amount).unwrap();
  let mut to_amount = amount;
  if partner_usdt_reward > 0 {
    to_amount = to_amount - partner_usdt_reward;
  }

  let cpi_accounts = SplTransfer {
    from: purchaser_ata.to_account_info(),
    to: bank_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = token_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if partner_usdt_reward > 0 {
    let cpi_accounts = SplTransfer {
      from: purchaser_ata.to_account_info(),
      to: partner_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), partner_usdt_reward).unwrap();
  }

  token_amount += bonus;

  // Updating sale_handler details
  sale_handler.set_total_sold(token_amount).unwrap();

  // Updating step details
  step.set_total_sold(token_amount).unwrap();

  // Updating purchaser details
  purchaser.set_purchased(token_amount).unwrap();

  // Updating partner details
  if !partner_code.is_empty() {
    partner.set_usdt_reward(partner_usdt_reward).unwrap();
    partner.set_token_reward(partner_token_reward).unwrap();
  };

  emit!(events::PurchaseWithUsdt {
    step: step.get_id(),
    purchaser: payer.key(),
    partner: partner_code,
    usd_equivalent: usd_amount,
    usdt_amount: amount,
    token_amount: token_amount,
  });

  Ok(())
}

pub fn get_price(price_update: &Account<PriceUpdateV2>)
  -> Result<(u128, u32)>
{
  let feed_id = &get_feed_id_from_hex(FEED_ID)?;
  let current_price = price_update.get_price_no_older_than(
      &Clock::get()?,
      FEED_MAXIMUM_AGE,
      feed_id,
  ).unwrap();
  let price = u64::try_from(current_price.price).unwrap();
  let expo = u32::try_from(-current_price.exponent).unwrap();
  Ok((u128::from(price), expo))
}

pub fn get_price_test(_price_update: &AccountInfo)
  -> Result<(u128, u32)>
{
  Ok((144000000000, 9))
}

pub fn get_interest(
  sale_handler: &mut Account<SaleHandler>,
  partner_code: &str,
  partner: &mut Account<Partner>,
  amount: u64,
  token_amount: u128,
)
  -> Result<(u64, u128)>
{
  if partner_code.is_empty() {
    return Ok((0, 0));
  };

  let (sale_handler_main_interest, sale_handler_secondary_interest) = sale_handler.get_interest();
  let (partner_main_interest, partner_secondary_interest) = partner.get_interest();

  let main_interest = u64::max(sale_handler_main_interest, partner_main_interest);
  let secondary_interest = u64::max(sale_handler_secondary_interest, partner_secondary_interest);

  let amount = amount * main_interest / 10u64.pow(PRECISION);
  let reward_token_amount = token_amount * u128::from(secondary_interest) / 10u128.pow(PRECISION);

  Ok((amount, reward_token_amount))
}

#[derive(Accounts)]
pub struct InitSaleHandler<'info> {
  #[account(
    init,
    payer = payer,
    space = 8 + SaleHandler::MAX_SIZE,
    seeds = [],
    bump,
  )]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(max_cap: u64, min_cap: u64)]
pub struct SetSaleHandlerCap<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(main_interest: u64, secondary_interest: u64)]
pub struct SetSaleHandlerInterest<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(threshold: u64, percent: u64)]
pub struct SetSaleHandlerBonus<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSaleHandlerEnabled<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetSaleHandlerDisabled<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(partner_code: String, amount: u64)]
pub struct PurchaseSol<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Purchaser::MAX_SIZE,
    seeds = [
      PURCHASER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub purchaser: Account<'info, Purchaser>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Partner::MAX_SIZE,
    seeds = [
      PARTNER_TAG,
      b"_",
      partner_code.as_ref()
    ],
    bump
  )]
  pub partner: Account<'info, Partner>,
  /// CHECK: price oracle
  pub price_update: Account<'info, PriceUpdateV2>,
  #[account(mut)]
  /// CHECK: bank info
  pub bank_info: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(partner_code: String, amount: u64)]
pub struct PurchaseUSDC<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Purchaser::MAX_SIZE,
    seeds = [
      PURCHASER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub purchaser: Account<'info, Purchaser>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Partner::MAX_SIZE,
    seeds = [
      PARTNER_TAG,
      b"_",
      partner_code.as_ref()
    ],
    bump
  )]
  pub partner: Account<'info, Partner>,
  #[account(
    mut,
    constraint = purchaser_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = purchaser_ata.owner == payer.key(),
  )]
  pub purchaser_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = bank_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = bank_ata.owner == BANK.parse::<Pubkey>().unwrap(),
  )]
  pub bank_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = partner_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = partner_pda_ata.owner == partner.key(),
  )]
  pub partner_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(partner_code: String, amount: u64)]
pub struct PurchaseUSDT<'info> {
  #[account(mut)]
  pub sale_handler: Account<'info, SaleHandler>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub step: Account<'info, Step>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Purchaser::MAX_SIZE,
    seeds = [
      PURCHASER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub purchaser: Account<'info, Purchaser>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Partner::MAX_SIZE,
    seeds = [
      PARTNER_TAG,
      b"_",
      partner_code.as_ref()
    ],
    bump
  )]
  pub partner: Account<'info, Partner>,
  #[account(
    mut,
    constraint = purchaser_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = purchaser_ata.owner == payer.key(),
  )]
  pub purchaser_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = bank_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = bank_ata.owner == BANK.parse::<Pubkey>().unwrap(),
  )]
  pub bank_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = partner_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = partner_pda_ata.owner == partner.key(),
  )]
  pub partner_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
