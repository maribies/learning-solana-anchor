use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod learning_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// account struct https://book.anchor-lang.com/anchor_in_depth/the_accounts_struct.html
#[account]
// Struct is pretty straightforward
pub struct Accomplishments {
    pub cadet_mastery: Vec<String>,
}
