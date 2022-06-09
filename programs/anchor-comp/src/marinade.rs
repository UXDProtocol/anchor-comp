use anchor_lang::prelude::*;
use marinade_onchain_helper::{cpi_context_accounts::{MarinadeDeposit, MarinadeLiquidUnstake}, cpi_util};

pub fn deposit<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, MarinadeDeposit<'info>>,
    deposit_lamports: u64,
) -> Result<()> {
    let data = marinade_finance::instruction::Deposit {
        lamports: deposit_lamports,
    };
    cpi_util::invoke_signed(ctx, data)
}

pub fn liquid_unstake<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, MarinadeLiquidUnstake<'info>>,
    msol_liquid_unstake_amount: u64,
) -> Result<()> {
    let instruction_data = marinade_finance::instruction::LiquidUnstake {
        msol_amount: msol_liquid_unstake_amount,
    };
    cpi_util::invoke_signed(ctx, instruction_data)
}
