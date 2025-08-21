use anchor_lang::prelude::*;

declare_id!("GZED5mYAbd1j7dusXJPxwo2aUWHYXoiWKXzyBurWNQ2H");



pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod basic {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        budget: u64,
        start_time: i64,
        penalty_time: i64,
        terminate_time: i64,
    ) -> Result<()> {
        ctx.accounts.init_contract(ctx.accounts.company.key(), ctx.accounts.contractor.key(),budget, start_time, penalty_time, terminate_time, &ctx.bumps)?;
        ctx.accounts.deposit(budget)?;

        Ok(())
        }
}


