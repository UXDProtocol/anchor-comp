use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;

pub fn sync_native<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, SyncNative<'info>>
)-> Result<()> {
    let ix = &spl_token::instruction::sync_native(
        ctx.accounts.token_program.key,
        &ctx.accounts.account.key(),
    )?;
    invoke_signed(
        ix,
        &[ctx.accounts.account.clone()],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SyncNative<'info> {
    /// CHECK: token program CPI
    pub account: AccountInfo<'info>,
    /// CHECK: token program CPI
    pub token_program: AccountInfo<'info>,
}