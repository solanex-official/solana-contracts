use anchor_lang::prelude::*;

#[error_code]
pub enum SaleHandler {
  #[msg("Unauthorized")]
  Unauthorized,
  #[msg("Signature verification failed.")]
  SignatureVerificationFailed,
  #[msg("SaleHandler already enabled")]
  SaleHandlerEnabled,
  #[msg("SaleHandler already disabled")]
  SaleHandlerDisabled,
  #[msg("SaleHandler not enabled")]
  SaleHandlerNotEnabled,
  #[msg("SaleHandler min cap larger than max cap")]
  SaleHandlerMinCapTooLarge,
  #[msg("SaleHandler min cap not reached")]
  SaleHandlerMinCapNotReached,
  #[msg("SaleHandler max cap exceeded")]
  SaleHandlerMaxCapExceeded,
  #[msg("SaleHandler main partner interest too large")]
  SaleHandlerMainPartnerInterestTooLarge,
  #[msg("SaleHandler secondary partner interest too large")]
  SaleHandlerSecondaryPartnerInterestTooLarge,
  #[msg("Step supply is too small")]
  StepSupplyTooSmall,
  #[msg("Step already enabled")]
  StepEnabled,
  #[msg("Step already disabled")]
  StepDisabled,
  #[msg("Step not enabled")]
  StepNotEnabled,
  #[msg("Step total supply exceeded")]
  StepSupplyExceeded,
  #[msg("Inactive step account")]
  InactiveStep,
  #[msg("Wrong price feed account")]
  WrongPriceFeedId,
  #[msg("Wrong stablecoin account")]
  WrongStablecoin,
  #[msg("Wrong bank account")]
  WrongBank,
  #[msg("Oracle price is down")]
  PriceIsDown,
  #[msg("Partner no funds")]
  PartnerNoFunds,
  #[msg("Expired signature")]
  ExpiredSignature,
  #[msg("Wrong Bonuses Lens")]
  WrongBonusesLens,
  #[msg("Wrong Bonuses Values")]
  WrongBonusesValues,
}