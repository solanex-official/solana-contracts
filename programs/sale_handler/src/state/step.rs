use anchor_lang::prelude::*;
use crate::errors;

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Status {
  None,
  Enabled,
  Disabled,
}

#[account]
pub struct Step {
  id: i16,
  price: u64,
  total_sold: u128,
  total_supply: u128,
  status: Status,
}

impl Step {
  pub const MAX_SIZE: usize = 2 + 8 + (2 * 16) + (32 + 1) + 2;

  pub fn init(
    &mut self,
    id: i16,
    price: u64,
    total_supply: u128,
  ) -> Result<()> {
    self.id = id;
    self.price = price;
    self.total_supply = total_supply;
    self.total_sold = 0;
    self.status = Status::None;

    Ok(())
  }

  pub fn set_price(
    &mut self,
    price: u64,
  ) -> Result<()> {
    if self.status == Status::Enabled || self.status == Status::Disabled {
      return err!(errors::SaleHandler::StepEnabled);
    }

    self.price = price;

    Ok(())
  }

  pub fn set_total_supply(
    &mut self,
    total_supply: u128,
  ) -> Result<()> {
    if self.total_sold > total_supply {
      return err!(errors::SaleHandler::StepSupplyTooSmall);
    }

    self.total_supply = total_supply;

    Ok(())
  }

  pub fn set_enable(
    &mut self,
  ) -> Result<()> {
    if self.status == Status::Enabled || self.status == Status::Disabled {
      return err!(errors::SaleHandler::StepEnabled);
    }

    self.status = Status::Enabled;

    Ok(())
  }

  pub fn set_disable(
    &mut self,
  ) -> Result<()> {
    if self.status != Status::Enabled {
      return err!(errors::SaleHandler::StepDisabled);
    }

    self.status = Status::Disabled;

    Ok(())
  }

  pub fn set_total_sold(
    &mut self,
    total_sold: u128,
  ) -> Result<()> {
    self.total_sold += total_sold;

    Ok(())
  }

  pub fn get_id(
    &mut self,
  ) -> i16 {
    self.id
  }

  pub fn get_price(
    &mut self,
  ) -> u64 {
    self.price
  }

  pub fn get_total_sold(
    &mut self,
  ) -> u128 {
    self.total_sold
  }

  pub fn get_total_supply(
    &mut self,
  ) -> u128 {
    self.total_supply
  }

  pub fn is_enabled(
    &self,
  ) -> bool {
    self.status == Status::Enabled
  }
}
