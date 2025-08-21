use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::Contract;

#[derive(Accounts)]
pub struct Initialize<'info> {

  #[account(mut)]
  pub company: Signer<'info>,

  pub contractor: UncheckedAccount<'info>,

  #[account(
    init,
    payer = company,
    space = Contract::INIT_SPACE,
    seeds = [b"contract", company.key().as_ref(), contractor.key().as_ref()],
    bump
  )]
  pub contract: Account<'info, Contract>,

  #[account(
    init,
    payer = company,
    seeds = [b"vault", contract.key().as_ref()],
    bump,
    space = 8
  )]
  pub vault: SystemAccount<'info>,

  pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info> {
    pub fn init_contract(
        &mut self,
        company: Pubkey,
        contractor: Pubkey,
        budget: u64,
        start_time: i64,
        penalty_time: i64,
        terminate_time: i64,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.contract.set_inner(Contract {
        bump: bumps.contract,
        company,
        contractor,
        budget,
        start_time,
        penalty_time,
        terminate_time,
    });

        // transfer budget into vault
        Ok(())
    }

    pub fn deposit(&mut self, budget: u64) -> Result<()> {
      let cpi_program = self.system_program.to_account_info();
      let cpi_accounts = Transfer{
        from: self.company.to_account_info(),
        to: self.vault.to_account_info(),
      };
      let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, budget)?;
        Ok(())
      }

}
