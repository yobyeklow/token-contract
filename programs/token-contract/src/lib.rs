use anchor_lang::prelude::*;

declare_id!("9NqDmt5LXu1FF1Xs8nmt6TFN8nShmM411xTH86o6mN1d");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
