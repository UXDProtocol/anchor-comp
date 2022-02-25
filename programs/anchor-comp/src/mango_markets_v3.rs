use anchor_lang::prelude::*;
use mango::matching::OrderType;
use mango::matching::Side;
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
        mango_program_id::ID
    }
}

pub fn create_mango_account<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMangoAccount<'info>>,
    account_num: u64,
) -> Result<()> {
    check_program_account(ctx.program.key)?;
    let ix = mango::instruction::create_mango_account(
        &mango_program_id::ID,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.system_prog.key,
        ctx.accounts.payer.key,
        account_num,
    )?;
    solana_program::program::invoke(&ix, &ToAccountInfos::to_account_infos(&ctx))
        .map_err(Into::into)
}

pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
    quantity: u64,
) -> Result<()> {
    check_program_account(ctx.program.key)?;
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
    allow_borrow: bool,
) -> Result<()> {
    check_program_account(ctx.program.key)?;
    let remaining_accounts_iter = ctx.remaining_accounts.iter();
    let mut open_orders = vec![Pubkey::default(); MAX_PAIRS];
    remaining_accounts_iter.for_each(|ai| open_orders.push(*ai.key));
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
        open_orders.as_slice(),
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

pub fn place_perp_order2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, PlacePerpOrder2<'info>>,
    side: Side,
    price: i64,
    max_base_quantity: i64,
    max_quote_quantity: i64,
    client_order_id: u64,
    order_type: OrderType,
    reduce_only: bool,
    // Send 0 if you want to ignore time in force
    expiry_timestamp: Option<u64>,
    // maximum number of FillEvents before terminating
    limit: u8,
) -> Result<()> {
    check_program_account(ctx.program.key)?;
    let mut remaining_accounts_iter = ctx.remaining_accounts.iter();
    let referral = remaining_accounts_iter.next();
    let mut open_orders = vec![Pubkey::default(); MAX_PAIRS];
    remaining_accounts_iter.for_each(|ai| open_orders.push(*ai.key));
    let ix = mango::instruction::place_perp_order2(
        &mango_program_id::ID,
        ctx.accounts.mango_group.key,
        ctx.accounts.mango_account.key,
        ctx.accounts.owner.key,
        ctx.accounts.mango_cache.key,
        ctx.accounts.perp_market.key,
        ctx.accounts.bids.key,
        ctx.accounts.asks.key,
        ctx.accounts.event_queue.key,
        referral.map(|r| r.key),
        open_orders.as_slice(),
        side,
        price,
        max_base_quantity,
        max_quote_quantity,
        client_order_id,
        order_type,
        reduce_only,
        expiry_timestamp,
        limit,
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
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub system_prog: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub root_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub node_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub vault: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner_token_account: AccountInfo<'info>,
}

/// To reference OpenOrders, add them to the accounts [0-MAX_PAIRS] of the
/// CpiContext's `remaining_accounts` Vec.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub root_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub node_bank: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub vault: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub token_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub signer: AccountInfo<'info>,
}

/// To reference OpenOrders, add them to the accounts [0-MAX_PAIRS] of the
/// CpiContext's `remaining_accounts` Vec.
#[derive(Accounts)]
pub struct PlacePerpOrder2<'info> {
    /// CHECK: Mango CPI
    pub mango_group: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_account: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub owner: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub perp_market: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub bids: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub asks: AccountInfo<'info>,
    /// CHECK: Mango CPI
    pub event_queue: AccountInfo<'info>,
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(mango_program_id: &Pubkey) -> Result<()> {
    if !mango_program_id.eq(&mango_program_id::ID) {
        error!(ErrorCode::InvalidProgramId);
    }
    Ok(())
}
