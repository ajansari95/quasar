#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    const ACCOUNTS_NUMBER: u64 = 10;
    const ACCOUNTS_INITIAL_BALANCE: u128 = 1_000_000_000_000;
    const DENOM_BASE: &str = "uatom";
    const DENOM_QUOTE: &str = "uosmo";

    proptest! {
        ///
        #[test]
        fn test_complete_works(
            (deposit_amounts) in (0usize..100).prop_flat_map(|size|
                (
                    // to avoid overflows, we limit the amounts to u64. also force amounts & bond_ids to be >= 1
                    vec(any::<u64>().prop_map(|x| (x as u128).max(1)), size..=size),
                )
            ),
        ) {
            // Creating test core
            let (app, contract_address, _cl_pool_id, _admin) = default_init();
            let wasm = Wasm::new(&app);

            // Creating test vars
            let mut accounts_shares_balance: HashMap<String, u128> = HashMap::new();

            // TODO: Create a fixed number of accounts using app.init_accounts() function from test-tube, and assign a fixed initial balance for all of them
            let accounts = app
                .init_accounts(&[
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_BASE),
                    Coin::new(ACCOUNTS_INITIAL_BALANCE, DENOM_QUOTE),
                ], ACCOUNTS_NUMBER)
                .unwrap();

            // TODO: Make random deposit, random in number of deposits, and random amount in info.funds too
            // We could generate a random Vec of Uint128 that we order ASC in order to know which user,
            // how much he deposited, to check later that each user shares should be < or > than the previous one? Or how do we test that the balance is ok?
            for deposit_amount in deposit_amounts {
                // Generate random number between 0 and len(accounts) to pick one of the addresses to execute the deposit
                let account_index = proptest::num::u128::ANY.between(0, ACCOUNTS_NUMBER - 1); // TODO check is working

                // TODO: Get current pool position to know asset0 and asset1 as /osmosis.concentratedliquidity.v1beta1.FullPositionBreakdown
                let (amount0, amount1) = (1000u128, 1000u128); // mocked

                let deposit = wasm.execute(
                    contract_address.as_str(),
                    &ExecuteMsg::ExactDeposit { recipient: None }, // TODO: Make recipient random
                    &[Coin::new(amount0, DENOM_BASE), Coin::new(amount1, DENOM_QUOTE)],
                    &accounts[account_index],
                ).unwrap();

                // TODO: Get shares_amount from deposit_resp
                let deposit_resp: MsgCreatePositionResponse = deposit.data.try_into()?;
                let liquidity_created = deposit_resp.liquidity_created?;

                // TODO: Update map to track account history and make further assertions
                let mut current_shares_amount = accounts_shares_balance.get(&accounts[account_index].address());
                accounts_shares_balance.insert(
                    accounts[account_index].address(),
                    current_shares_amount.unwrap_or(&0u128).checked_add(liquidity_created),
                )
            }

            // TODO: multi-query foreach user created previously
            for account in accounts {
                let shares: UserBalanceResponse = wasm
                    .query(
                        contract_address.as_str(),
                        &QueryMsg::VaultExtension(ExtensionQueryMsg::Balances(
                            crate::msg::UserBalanceQueryMsg::UserLockedBalance {
                                user: account.address(),
                            },
                        )),
                    )
                    .unwrap();
                assert!(!shares.balance.is_zero());
            }

            // TODO: Here some users should withdraw entirely and partially before the update range.

            // TODO: Update ranges, make more deposits, make more withdrawals.

            // TODO: Make random withdraws foreach user, consider making some users withdrawing the full stake while others a minor percentage between 1% and 99%.

            // TODO: Update ranges, make more deposits, make more withdrawals
        }
    }
}
