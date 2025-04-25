use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, MintTo,Transfer};

declare_id!("9NqDmt5LXu1FF1Xs8nmt6TFN8nShmM411xTH86o6mN1d");

#[program]
pub mod token_contract {
    use super::*;

    pub fn mint_token(ctx:Context<MintToken>)-> Result<()>{
        let cpi_account = MintTo{
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program,cpi_account);
        token::mint_to(cpi_ctx,10)?;
        Ok(())
    }
    pub fn transfer_token(ctx:Context<TransferToken>)->Result<()>{
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to:ctx.accounts.to.to_account_info(),
            authority:ctx.accounts.signer.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program,transfer_instruction);
        anchor_spl::token::transfer(cpi_ctx,5)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
     /// CHECK: This is the token that we want to mint
     #[account(mut)]
     pub mint: UncheckedAccount<'info>,
     pub token_program: Program<'info, Token>,
     /// CHECK: This is the token account that we want to mint tokens to
     #[account(mut)]
     pub token_account: UncheckedAccount<'info>,
     /// CHECK: the authority of the mint account
     #[account(mut)]
     pub payer: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct TransferToken<'info>{
    pub token_program : Program<'info,Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from:UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to:AccountInfo<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
}