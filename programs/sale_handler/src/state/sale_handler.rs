use anchor_lang::prelude::*;
use crate::errors;
use crate::config::{ MAIN_INTEREST, MAX_CAP, MIN_CAP, PRECISION, SECONDARY_INTEREST };

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Status {
  None,
  Enabled,
  Disabled,
}

#[account]
pub struct SaleHandler {
  max_cap: u64,
  min_cap: u64,
  main_interest: u64,
  secondary_interest: u64,
  total_sold: u128,
  step: i16,
  status: Status,
  enabled: bool,
  // NOTE: unforturantelly unable to use array of objects
  bonus_percents: Vec<u64>,
  bonus_thresholds: Vec<u64>,
}

impl SaleHandler {
  pub const MAX_SIZE: usize = (4 * 8) + 16 + 2 + 1 + 2 + 1 + 2 * (8 * 10 + 24);

  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.step = -1;
    self.max_cap = MAX_CAP;
    self.min_cap = MIN_CAP;
    self.main_interest = MAIN_INTEREST;
    self.secondary_interest = SECONDARY_INTEREST;
    self.total_sold = 0;
    self.status = Status::None;
    self.enabled = true;

    self.bonus_thresholds = Vec::new();
    self.bonus_percents = Vec::new();

    Ok(())
  }

  pub fn set_cap(
    &mut self,
    max_cap: u64,
    min_cap: u64,
  ) -> Result<()> {
    if max_cap < min_cap {
      return err!(errors::SaleHandler::SaleHandlerMinCapTooLarge);
    }

    self.max_cap = max_cap;
    self.min_cap = min_cap;

    Ok(())
  }

  pub fn set_bonus(
    &mut self,
    thresholds: Vec<u64>,
    percents: Vec<u64>,
  ) -> Result<()> {
    if thresholds.len() != percents.len() {
      return err!(errors::SaleHandler::WrongBonusesLens);
    }

    for idx in 0..percents.len() {
      if idx == 0 {
        if percents[0] == 0 || thresholds[0] == 0 {
          return err!(errors::SaleHandler::WrongBonusesValues);
        }
      } else {
        if percents[idx -1] >= percents[idx] || thresholds[idx -1] >= thresholds[idx] {
          return err!(errors::SaleHandler::WrongBonusesValues);
        }
      }
    }

    self.bonus_thresholds = thresholds;
    self.bonus_percents = percents;

    Ok(())
  }

  pub fn set_interest(
    &mut self,
    main_interest: u64,
    secondary_interest: u64,
  ) -> Result<()> {
    if main_interest > 1000_000_000 {
      return err!(errors::SaleHandler::SaleHandlerMainPartnerInterestTooLarge);
    }

    if secondary_interest > 1000_000_000 {
      return err!(errors::SaleHandler::SaleHandlerSecondaryPartnerInterestTooLarge);
    }

    self.main_interest = main_interest;
    self.secondary_interest = secondary_interest;

    Ok(())
  }

  pub fn set_enable(
    &mut self,
  ) -> Result<()> {
    if self.status == Status::Enabled || self.status == Status::Disabled {
      return err!(errors::SaleHandler::SaleHandlerEnabled);
    }

    self.status = Status::Enabled;

    Ok(())
  }

  pub fn set_disable(
    &mut self,
  ) -> Result<()> {
    if self.status != Status::Enabled {
      return err!(errors::SaleHandler::SaleHandlerDisabled);
    }

    self.status = Status::Disabled;

    Ok(())
  }

  pub fn set_step(
    &mut self,
    step: i16,
  ) -> Result<()> {
    self.step = step;

    Ok(())
  }

  pub fn set_total_sold(
    &mut self,
    total_sold: u128,
  ) -> Result<()> {
    self.total_sold += total_sold;

    Ok(())
  }

  pub fn get_step(
    &self,
  ) -> i16 {
    self.step
  }

  pub fn get_max_cap(
    &self,
  ) -> u128 {
    u128::from(self.max_cap)
  }

  pub fn get_min_cap(
    &self,
  ) -> u128 {
    u128::from(self.min_cap)
  }

  pub fn get_total_sold(
    &self,
  ) -> u128 {
    self.total_sold
  }

  pub fn get_interest(
    &mut self,
  ) -> (u64, u64) {
    (self.main_interest, self.secondary_interest)
  }

  pub fn is_enabled(
    &self,
  ) -> bool {
    self.status == Status::Enabled
  }

  pub fn calculate_bonus(
    &mut self,
    usd_amount: u128,
    token_amount: u128,
  ) -> u128 {
    if self.bonus_percents.len() == 0 {
      return 0;
    }
    
    let mut target: i32 = -1;

    for idx in 0..self.bonus_percents.len() {
      if usd_amount >= self.bonus_thresholds[idx].into() {
        target = idx as i32;
      } else {
        break;
      }
    }

    if target == -1 {
      return 0;
    }

    return token_amount * self.bonus_percents[target as usize] as u128 / 10u128.pow(PRECISION);
  }
}
