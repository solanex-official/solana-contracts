use anchor_lang::prelude::*;

#[account]
pub struct Purchaser {
  purchased: u128,
}

impl Purchaser {
  pub const MAX_SIZE: usize = 16 + 1;

  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.purchased = 0;

    Ok(())
  }

  pub fn set_purchased(
    &mut self,
    amount: u128,
  ) -> Result<()> {
    self.purchased += amount;

    Ok(())
  }

  pub fn get_purchased(
    &mut self,
  ) -> u128 {
    self.purchased
  }
}
