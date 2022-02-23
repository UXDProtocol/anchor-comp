use anchor_lang::prelude::*;
use mango::state::MAX_PAIRS;

mod mango_program_id {
    #[cfg(feature = "development")]
    solana_program::declare_id!("4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA");
    #[cfg(feature = "production")]
    solana_program::declare_id!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
}

#[derive(Clone)]
pub struct MangoMarketV3;

impl anchor_lang::Id for MangoMarketV3 {
    fn id() -> Pubkey {
        mango_program_id
    }
}

// impl anchor_lang::AccountDeserialize for Mango {
//     fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         Mango::try_deserialize_unchecked(buf)
//     }

//     fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self, ProgramError> {
//         Ok(Mango)
//     }
// }

pub fn create_mango_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMangoAccount<'info>>,
    account_num: u64,
) -> Result<()> {
    check_program_account(mango_program_id);
    let ix = mango::instruction::create_mango_account(
        &mango_program_id::ID,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.system_prog.key,
        ctx.accounts.payer.key,
        account_num,
    )?;
    solana_program::program::invoke(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
    )
    .map_err(Into::into)
}

pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    quantity: u64,
) -> Result<()> {
    check_program_account(ctx.accounts.program.key);
    let ix = mango::instruction::deposit(
        &mango_program_id::ID,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.root_bank.key,
        ctx.accounts.node_bank.key,
        ctx.accounts.vault.key,
        ctx.accounts.owner_token_account.key,
        quantity,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn withdraw<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
    quantity: u64,
    allow_borrow: bool
) -> Result<()> {
    check_program_account(ctx.accounts.program.key);
    let open_orders = ctx.remaining_accounts.get(0..MAX_PAIRS); // This could use something else
    let ix = mango::instruction::withdraw(
        &mango_program_id::ID,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.root_bank.key,
        ctx.accounts.node_bank.key,
        ctx.accounts.vault.key,
        ctx.accounts.token_account.key,
        ctx.accounts.signer.key,
        open_orders.map(|oo| oo.key),
        quantity,
        allow_borrow,
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct CreateMangoAccount<'info> {
    mango_group: AccountInfo<'info>,
    mango_account: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    system_prog: AccountInfo<'info>,
    payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    mango_group: AccountInfo<'info>,
    mango_account: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    mango_cache: AccountInfo<'info>,
    root_bank: AccountInfo<'info>,
    node_bank: AccountInfo<'info>,
    vault: AccountInfo<'info>,
    owner_token_account: AccountInfo<'info>,
}

/// To reference OpenOrders, add them to the accounts [0-MAX_PAIRS] of the
/// CpiContext's `remaining_accounts` Vec.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    mango_group: AccountInfo<'info>,
    mango_account: AccountInfo<'info>,
    owner: AccountInfo<'info>,
    mango_cache: AccountInfo<'info>,
    root_bank: AccountInfo<'info>,
    node_bank: AccountInfo<'info>,
    vault: AccountInfo<'info>,
    owner_token_account: AccountInfo<'info>,
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> ProgramResult {
    if !mango_program_id.eq(&mango_program_id::ID) {
        error!("Wrong program ID");
    }
    Ok(())
}
