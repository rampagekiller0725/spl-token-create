use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::{token::{TokenAccount, Mint, Token}};

// use anchor_spl::token::Transfer;
use {
    anchor_spl::{
        associated_token,
        token,
    },
};

declare_id!("64xhQ9y8CcQ1PVWYkLuZ9uR5RXCTZj8WmSe11ra8nJ34");


#[program]
pub mod patreon {
    use super::*;


pub fn transfer_nft(ctx: Context<TransferNft>)->ProgramResult{
        
    msg!("Creating token account...");
    msg!("Token Address: {}", &ctx.accounts.buyer_token_account.key());
    

    //Its depends on cross program invocation for more info please refer - https://solanacookbook.com/references/programs.html#how-to-do-cross-program-invocation
    associated_token::create(
        CpiContext::new( 
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                // rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Transferring NFT or SPL TOKEN...");
    msg!("Owner Token Address: {}", &ctx.accounts.owner_token_account.key());    
    msg!("User or Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority:ctx.accounts.token_holder.to_account_info(), //ctx.accounts.seller.to_account_info()
            }
        ),
        1
    )?;
    
    msg!("Transferred successfully.");

    Ok(())
}
}
#[derive(Accounts)]
pub struct TransferNft<'info>{
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    buyer: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    seller: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub token_holder:Signer<'info,>, //signer
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>, //UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    // pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[account]
#[derive(Default)]
pub struct State {
    bump: u8,
    amount: u64,
}