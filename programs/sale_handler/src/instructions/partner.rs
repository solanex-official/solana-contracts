use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use solana_program::sysvar::instructions::ID as IX_ID;
use crate::config::{ USDC, USDT, PARTNER_TAG };

use crate::events;
use crate::errors;
use crate::state::partner::*;

pub fn init_partner(
  ctx: Context<InitPartner>,
  main_interest: u64,
  secondary_interest: u64,
) -> Result<()> {
  let partner = &mut ctx.accounts.partner;
  partner.init(main_interest, secondary_interest)
}

pub fn set_partner_interest(
  ctx: Context<SetPartnerInterest>,
  main_interest: u64,
  secondary_interest: u64,
) -> Result<()> {
  let partner = &mut ctx.accounts.partner;
  partner.set_interest(main_interest, secondary_interest)
}

pub fn enable_partner(
  ctx: Context<SetPartnerEnabled>,
) -> Result<()> {
  let partner = &mut ctx.accounts.partner;
  partner.enable()
}

pub fn disable_partner(
  ctx: Context<SetPartnerDisabled>,
) -> Result<()> {
  let partner = &mut ctx.accounts.partner;
  partner.disable()
}

pub fn receive_sol(
  ctx: Context<ReceiveSol>,
  partner_code: String,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let partner = &mut ctx.accounts.partner;
  
  let sol_interest = partner.get_sol_reward();
  if sol_interest > 0 {
    partner.reset_sol_reward().unwrap();

    partner.sub_lamports(sol_interest).unwrap();
    payer.add_lamports(sol_interest).unwrap();

    emit!(events::ReceiveSol {
      partner: partner_code,
      amount: sol_interest,
    });
  }

  Ok(())
}

pub fn receive_usdc(
  ctx: Context<ReceiveUSDC>,
  partner_code: String,
) -> Result<()> {
  //let payer = &mut ctx.accounts.payer;
  let partner = &mut ctx.accounts.partner;
  
  let partner_ata = &ctx.accounts.partner_ata;
  let partner_pda_ata = &ctx.accounts.partner_pda_ata;
  let program = &ctx.accounts.token_program;

  let amount = partner.get_usdc_reward();
  if amount == 0 {
    return err!(errors::SaleHandler::PartnerNoFunds);
  }

  partner.reset_usdc_reward().unwrap();

  let bump = &[ctx.bumps.partner];
  let seeds: &[&[u8]] = &[PARTNER_TAG, b"_", partner_code.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: partner_pda_ata.to_account_info(),
    to: partner_ata.to_account_info(),
    authority: partner.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::ReceiveUsdc {
    partner: partner_code,
    amount: amount,
  });

  Ok(())
}

pub fn receive_usdt(
  ctx: Context<ReceiveUSDT>,
  partner_code: String,
) -> Result<()> {
  //let payer = &mut ctx.accounts.payer;
  let partner = &mut ctx.accounts.partner;
  
  let partner_ata = &ctx.accounts.partner_ata;
  let partner_pda_ata = &ctx.accounts.partner_pda_ata;
  let program = &ctx.accounts.token_program;

  let amount = partner.get_usdt_reward();
  if amount == 0 {
    return err!(errors::SaleHandler::PartnerNoFunds);
  }

  partner.reset_usdt_reward().unwrap();

  let bump = &[ctx.bumps.partner];
  let seeds: &[&[u8]] = &[PARTNER_TAG, b"_", partner_code.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: partner_pda_ata.to_account_info(),
    to: partner_ata.to_account_info(),
    authority: partner.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::ReceiveUsdt {
    partner: partner_code,
    amount: amount,
  });

  Ok(())
}

#[derive(Accounts)]
#[instruction(partner_code: String)]
pub struct InitPartner<'info> {
  #[account(
    init,
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
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPartnerInterest<'info> {
  #[account(mut)]
  pub partner: Account<'info, Partner>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPartnerEnabled<'info> {
  #[account(mut)]
  pub partner: Account<'info, Partner>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPartnerDisabled<'info> {
  #[account(mut)]
  pub partner: Account<'info, Partner>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(partner_code: String)]
pub struct ReceiveSol<'info> {
  #[account(
    mut,
    seeds = [
      PARTNER_TAG,
      b"_",
      partner_code.as_ref()
    ],
    bump
  )]
  pub partner: Account<'info, Partner>,

  #[account(address = IX_ID)]
  /// CHECK: we need this for sign
  pub ix_sysvar: AccountInfo<'info>,

  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(partner_code: String)]
pub struct ReceiveUSDC<'info> {
  #[account(
    mut,
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
    constraint = partner_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = partner_ata.owner == payer.key(),
  )]
  pub partner_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = partner_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = partner_pda_ata.owner == partner.key(),
  )]
  pub partner_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,

  #[account(address = IX_ID)]
  /// CHECK: we need this for sign
  pub ix_sysvar: AccountInfo<'info>,

  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(partner_code: String)]
pub struct ReceiveUSDT<'info> {
  #[account(
    mut,
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
    constraint = partner_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = partner_ata.owner == payer.key(),
  )]
  pub partner_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = partner_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = partner_pda_ata.owner == partner.key(),
  )]
  pub partner_pda_ata: Account<'info, TokenAccount>,
  pub token_program: Program<'info, Token>,

  #[account(address = IX_ID)]
  /// CHECK: we need this for sign
  pub ix_sysvar: AccountInfo<'info>,

  #[account(mut)]
  pub payer: Signer<'info>,
}
