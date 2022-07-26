use anchor_lang::prelude::*;

// Anchor init gives a placeholder, so this is changed
// depending on the deployment target and found by running
// solana address -k target/deploy/{project-name}-keypair.json
declare_id!("F2yGB1QSPjuaqquyWRqE9g1fmYdugupd8iukUVNp4ocA");

// macro- sharable extraction of code at syntax level
#[program]
// module - a structure to organize code and control scope
pub mod learning_solana {
    use super::*;

    // initialization function, which is public to be called
    // from the outside to run the program.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // get the accomplishments
        let accomplishments_account = &mut ctx.accounts.accomplishments;
        // initialize the variables (a requirement)
        accomplishments_account.cadet_mastery = vec!["".to_string()];
        Ok(())
    }

    // function to add Solana to the collection of cadet mastered skills.
    pub fn add_mastery(ctx: Context<AddMastery>) -> Result<()> {
        let account = &mut ctx.accounts.accomplishments;
        // FIXME: This overrides/sets the vector to Solana only everytime.
        // In the future, we probably want to `push` a skill, maybe have
        // an input that allows the user to select or enter the skill.
        account.cadet_mastery = vec!["Solana".to_string()];
        Ok(())
    }
}

// implements an Accounts deserializer on the given struct
// aka process user addresses and accounts
// and attach certain variables to the context
#[derive(Accounts)]
// 'info is a Rust lifetime and way to pass variables
pub struct Initialize<'info> {
    // an Anchor program annotation,
    // attribute implements the Owner trait (user)
    // initalizing a new account where the data will be stored
    // space is set to allocate 9000 bytes of space for the account
    #[account(init, payer = user, space = 9000)]
    // store the accomplishments in the new data account
    pub accomplishments: Account<'info, Accomplishments>,
    #[account(mut)]
    // defines the payer of the data account
    pub user: Signer<'info>,
    // required element to create Solana data
    pub system_program: Program<'info, System>,
}

// data needed in the AddMastery Context
// has access to a mutable reference to accomplishments so it can be changed
#[derive(Accounts)]
pub struct AddMastery<'info> {
  #[account(mut)]
  pub accomplishments: Account<'info, Accomplishments>,
}

// account struct https://book.anchor-lang.com/anchor_in_depth/the_accounts_struct.html
#[account]
// Struct is pretty straightforward
pub struct Accomplishments {
    pub cadet_mastery: Vec<String>,
}
