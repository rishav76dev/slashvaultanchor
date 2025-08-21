use anchor_lang::prelude::*;

#[account]
pub struct Contract {
  pub bump: u8,
  pub company: Pubkey,
  pub contractor: Pubkey,
  pub budget: u64,
  pub start_time: i64,
  pub penalty_time: i64,
  pub terminate_time: i64,
}

impl Space for Contract {
    const INIT_SPACE: usize = 8   // discriminator
        + 1   // bump
        + 32  // company pubkey
        + 32  // contractor pubkey
        + 8   // budget
        + 8   // start_time
        + 8   // penalty_time
        + 8   // terminate_time
        + 8;  // padding for future fields
}
