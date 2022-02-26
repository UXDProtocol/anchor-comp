# anchor-comp

Anchor wrapper for additional non mango program not part of the SPL library.

MangoMarketV3 Solana program.

Follows structure of <https://github.com/project-serum/anchor/blob/master/spl/src/dex.rs> and <https://github.com/project-serum/anchor/blob/master/spl/src/token.rs>

## Usage

```toml
[dependencies]
anchor-comp = { version = "0.1.0", git = "https://github.com/UXDProtocol/anchor-comp", features = ["no-entrypoint", "development"] }
# anchor-comp = { version = "0.1.0", git = "https://github.com/UXDProtocol/anchor-comp", features = ["no-entrypoint", "production"] }
```

## Example CPI from Rust Solana program

```rust
    use anchor_comp::mango_markets_v3;
    use anchor_comp::mango_markets_v3::MangoMarketV3;

    #[derive(Accounts)]
    pub struct MyInstruction<'info> {

        // ...

        pub mango_program: Program<'info, MangoMarketV3>,
    }

    pub fn my_instruction(
        ctx: Context<MyInstruction>,
    ) -> Result<()> {

        // ...

        // - [MangoMarkets CPI - Place perp order]
        mango_markets_v3::place_perp_order2(
            ctx.accounts
                .into_open_mango_short_perp_context()
                .with_signer(depository_pda_signer),
            taker_side,
            limit_price_lot.to_num(),
            max_base_quantity.to_num(),
            i64::MAX,
            0,
            OrderType::ImmediateOrCancel,
            false,
            None,
            10,
        )?;

        // ...
    }


    impl<'info> MyInstruction<'info> {

        // ...

        pub fn into_open_mango_short_perp_context(
            &self,
        ) -> CpiContext<'_, '_, '_, 'info, mango_markets_v3::PlacePerpOrder2<'info>> {
            let cpi_accounts = mango_markets_v3::PlacePerpOrder2 {
                mango_group: self.mango_group.to_account_info(),
                mango_account: self.depository_mango_account.to_account_info(),
                owner: self.depository.to_account_info(),
                mango_cache: self.mango_cache.to_account_info(),
                perp_market: self.mango_perp_market.to_account_info(),
                bids: self.mango_bids.to_account_info(),
                asks: self.mango_asks.to_account_info(),
                event_queue: self.mango_event_queue.to_account_info(),
            };
            let cpi_program = self.mango_program.to_account_info();
            CpiContext::new(cpi_program, cpi_accounts)
        }
        
        // ...
    }

```
