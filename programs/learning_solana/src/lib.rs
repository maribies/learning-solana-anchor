use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// macro- sharable extraction of code at syntax level
#[program]
// module - a structure to organize code and control scope
pub mod learning_solana {
    use super::*;

    // initialization function, which is public to be called
    // from the outside to run the program.
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
