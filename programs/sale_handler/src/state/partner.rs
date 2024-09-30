use anchor_lang::prelude::*;

#[account]
pub struct Partner {
  main_interest: u64,
  secondary_interest: u64,

  sol_reward: u64,
  usdt_reward: u64,
  usdc_reward: u64,
  token_reward: u128,

  enabled: bool,
}

impl Partner {
  pub const MAX_SIZE: usize = (5 * 8) + 16 + 1 + 3;

  pub fn init(
    &mut self,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    self.main_interest = main_interest;
    self.secondary_interest = secondary_interest;

    self.sol_reward = 0;
    self.usdt_reward = 0;
    self.usdc_reward = 0;
    self.token_reward = 0;

    self.enabled = true;

    Ok(())
  }

  pub fn set_interest(
    &mut self,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    self.main_interest = main_interest;
    self.secondary_interest = secondary_interest;

    Ok(())
  }

  pub fn set_sol_reward(
    &mut self,
    amount: u64,
  ) -> Result<()> {
    self.sol_reward += amount;

    Ok(())
  }

  pub fn reset_sol_reward(
    &mut self,
  ) -> Result<()> {
    self.sol_reward = 0;

    Ok(())
  }

  pub fn set_usdt_reward(
    &mut self,
    amount: u64,
  ) -> Result<()> {
    self.usdt_reward += amount;

    Ok(())
  }

  pub fn reset_usdt_reward(
    &mut self,
  ) -> Result<()> {
    self.usdt_reward = 0;

    Ok(())
  }

  pub fn set_usdc_reward(
    &mut self,
    amount: u64,
  ) -> Result<()> {
    self.usdc_reward += amount;

    Ok(())
  }

  pub fn reset_usdc_reward(
    &mut self,
  ) -> Result<()> {
    self.usdc_reward = 0;

    Ok(())
  }

  pub fn set_token_reward(
    &mut self,
    amount: u128,
  ) -> Result<()> {
    self.token_reward += amount;

    Ok(())
  }

  pub fn get_interest(
    &mut self,
  ) -> (u64, u64) {
    (self.main_interest, self.secondary_interest)
  }

  pub fn get_sol_reward(
    &mut self,
  ) -> u64 {
    self.sol_reward
  }

  pub fn get_usdt_reward(
    &mut self,
  ) -> u64 {
    self.usdt_reward
  }

  pub fn get_usdc_reward(
    &mut self,
  ) -> u64 {
    self.usdc_reward
  }

  pub fn get_token_reward(
    &mut self,
  ) -> u128 {
    self.token_reward
  }

  pub fn enable(
    &mut self,
  ) -> Result<()> {
    self.enabled = true;

    Ok(())
  }

  pub fn disable(
    &mut self,
  ) -> Result<()> {
    self.enabled = false;

    Ok(())
  }
}
