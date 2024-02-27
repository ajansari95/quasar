#[cfg(test)]
mod tests {
    use crate::{
        admin::execute::AdminExecuteMsg,
        incentives::{execute::IncentivesExecuteMsg, CoinVec},
        msg::ExecuteMsg,
        state::ClaimAccount,
        test_tube::initialize::initialize::default_init,
    };
    use cosmwasm_std::{Addr, Coin};
    use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
    use osmosis_test_tube::{
        osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
        Account, Bank, Module, Wasm,
    };

    const MNEMONIC: &str = "market inquiry reward way sting diet double beef accuse help crisp circle leaf connect elder bench wrong dust push essence wise flip devote about";

    #[test]
    #[ignore]
    fn merkle_complete_cycle_works() {
        let (app, contract, admin) = default_init(1000000000000000000u128);
        let bank = Bank::new(&app);
        let wasm = Wasm::new(&app);

        // This test has been generated using those proofs.

        // osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt900000000ugauge
        // osmo1dplhdl5uch280hlcy3mdt207yf4vz3hkzpsysf9000000000ugauge
        // osmo1htv3nrpc69w5hnnxhh9pdpmfa7w5zvh3cst7cy90000000000ugauge
        // osmo1hr5u2ak5k6dydhpu5q348w5mw8fk69sj24x47l900000000000ugauge
        // osmo149pjhcf2sgwpmwqyknkkav4w2ywjjfc58egh6m9000000000000ugauge
        // osmo1n65qu2feqk8g0fcte2wtph3k3x55qsu720qyvx90000000000000ugauge
        // osmo18aduztzax34vtyhdqcarc455kn7xpldtdxn75s900000000000000ugauge
        // osmo1uctm0pn6fdwdlt48c6x9h29nwhg6jsr409q35m9000000000900000ugauge
        // osmo16xetz07p6v2laa7fqxfvtq2jx99g4yphaugp4490000000009000000ugauge
        // osmo1aemevl2sxymzmjxkaal4u8959qmxgpek4r7u6w900000000090000000ugauge

        // ClaimAccounts related to above ToVerifies
        let claim_accounts: Vec<ClaimAccount> = vec![
            // osmo1cn2t4zha4ukq42u2q8x0zgyp60hp5gy54a2wxt
            ClaimAccount {
                proof: vec![
                    base64::decode("R6J/QIhrqN4KxMa7ZhaCm/6J7ibT7HHcw9KKRV4ML0k=").unwrap(),
                    base64::decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=").unwrap(),
                    base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
                    base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
                ],
                coins: CoinVec::from(vec![Coin::new(900000000, "ugauge")]),
            },
            // osmo1dplhdl5uch280hlcy3mdt207yf4vz3hkzpsysf
            ClaimAccount {
                proof: vec![
                    base64::decode("H0z7k6nOssp7mOQRtw1G0+cXcE8sZgoJOKu+gtoiNxQ=").unwrap(),
                    base64::decode("B2Tu7/SQT48JJTAv+8KncPIgSVMSx08IhNN3Fxm2iBo=").unwrap(),
                    base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
                    base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
                ],
                coins: CoinVec::from(vec![Coin::new(9000000000, "ugauge")]),
            },
            // osmo1htv3nrpc69w5hnnxhh9pdpmfa7w5zvh3cst7cy
            ClaimAccount {
                proof: vec![
                    base64::decode("vD1tJykyse83cMvZ5UXg3DqiNo8k/7JSrAsjcvWBNo0=").unwrap(),
                    base64::decode("93xy7Gd10nfD/lOODtmTjdCtdzWtgH0PvltTA5NYTdI=").unwrap(),
                    base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
                    base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
                ],
                coins: CoinVec::from(vec![Coin::new(90000000000, "ugauge")]),
            },
            // osmo1hr5u2ak5k6dydhpu5q348w5mw8fk69sj24x47l
            ClaimAccount {
                proof: vec![
                    base64::decode("bNyohDWEFc/xq0krIvPCa/IbW2OlFGZdGBooXe4bZxs=").unwrap(),
                    base64::decode("93xy7Gd10nfD/lOODtmTjdCtdzWtgH0PvltTA5NYTdI=").unwrap(),
                    base64::decode("rWczQYIqxQMn6Kuglth0Z2gq8YysvEUqwt5VO8iYkZI=").unwrap(),
                    base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
                ],
                coins: CoinVec::from(vec![Coin::new(900000000000, "ugauge")]),
            },
            // osmo149pjhcf2sgwpmwqyknkkav4w2ywjjfc58egh6m
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("8sw/P8aUt7HEp+L9V+0tyU/jrq3vYLlD8LU8Isef5jI=").unwrap(),
            //         base64::decode("8sw/P8aUt7HEp+L9V+0tyU/jrq3vYLlD8LU8Isef5jI=").unwrap(),
            //         base64::decode("ZOGfLiJbP9PN8rCEEFhN7dZwxbsv6v9YfcEzxzLRAw4=").unwrap(),
            //         base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(9000000000000, "ugauge")]),
            // },
            // // osmo1n65qu2feqk8g0fcte2wtph3k3x55qsu720qyvx
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("gO2AiC7p/fhc9xg3Wj2RJcxdebRUDJ3oeg5007YGj/A=").unwrap(),
            //         base64::decode("8sw/P8aUt7HEp+L9V+0tyU/jrq3vYLlD8LU8Isef5jI=").unwrap(),
            //         base64::decode("ZOGfLiJbP9PN8rCEEFhN7dZwxbsv6v9YfcEzxzLRAw4=").unwrap(),
            //         base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(90000000000000, "ugauge")]),
            // },
            // // osmo18aduztzax34vtyhdqcarc455kn7xpldtdxn75s
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("rTOwCJwFbt6qXhSrDyZFBp4n1T6vcPR9SbKA2OwNCC0=").unwrap(),
            //         base64::decode("gO2AiC7p/fhc9xg3Wj2RJcxdebRUDJ3oeg5007YGj/A=").unwrap(),
            //         base64::decode("ZOGfLiJbP9PN8rCEEFhN7dZwxbsv6v9YfcEzxzLRAw4=").unwrap(),
            //         base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(900000000000000, "ugauge")]),
            // },
            // // osmo1uctm0pn6fdwdlt48c6x9h29nwhg6jsr409q35m
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("peaAc4Hd1OkOgKJwPwSniVtkf1hUDZPw0kkm4ElkOrs=").unwrap(),
            //         base64::decode("gO2AiC7p/fhc9xg3Wj2RJcxdebRUDJ3oeg5007YGj/A=").unwrap(),
            //         base64::decode("ZOGfLiJbP9PN8rCEEFhN7dZwxbsv6v9YfcEzxzLRAw4=").unwrap(),
            //         base64::decode("0ykV7dikL6TIBXAzwDZ21InNZdTIvT9S9sxgtZtA4gw=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(9000000000900000, "ugauge")]),
            // },
            // // osmo16xetz07p6v2laa7fqxfvtq2jx99g4yphaugp44
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("2fyEYqzL3pMwNrDUv7MJA+kZVdymnhxDJHy+Wv6xfEg=").unwrap(),
            //         base64::decode("rTOwCJwFbt6qXhSrDyZFBp4n1T6vcPR9SbKA2OwNCC0=").unwrap(),
            //         base64::decode("oGvDZTBGUuvTwMOaXZVKN1aZhPa0PhkI0AudEmfwsw4=").unwrap(),
            //         base64::decode("mHNu4BFMV6jir7MzwBbXp5aDc8QPZFaztVEVUgjBb2U=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(90000000009000000, "ugauge")]),
            // },
            // // osmo1aemevl2sxymzmjxkaal4u8959qmxgpek4r7u6w
            // ClaimAccount {
            //     proof: vec![
            //         base64::decode("Gh/gMVxgiP9XMEFhgBOhx76+AAgSy8FzCDFCGWLVTCo=").unwrap(),
            //         base64::decode("rTOwCJwFbt6qXhSrDyZFBp4n1T6vcPR9SbKA2OwNCC0=").unwrap(),
            //         base64::decode("oGvDZTBGUuvTwMOaXZVKN1aZhPa0PhkI0AudEmfwsw4=").unwrap(),
            //         base64::decode("mHNu4BFMV6jir7MzwBbXp5aDc8QPZFaztVEVUgjBb2U=").unwrap(),
            //     ],
            //     coins: CoinVec::from(vec![Coin::new(900000000090000000, "ugauge")]),
            // },
        ];
        // Create as many acocunt from mnemonic as ClaimAccounts
        let accounts = app
            .init_accounts_from_mnemonic(
                &[Coin::new(100_000_000_000, "uosmo")],
                MNEMONIC,
                claim_accounts.len() as u64,
            )
            .unwrap();

        // Fund the incentives contract as admin
        let _ = bank.send(
            MsgSend {
                from_address: admin.address().to_string(),
                to_address: contract.to_string(),
                amount: vec![OsmoCoin {
                    amount: "1000000000000000000".to_string(), // 1_000_000_000_000.000000 GAUGE (1T $GAUGE)
                    denom: "ugauge".to_string(),
                }],
            },
            &admin,
        );

        // Assert initial balance on gauge contract
        let contract_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: contract.to_string(),
                denom: "ugauge".to_string(),
            })
            .unwrap();
        assert_eq!(
            contract_balance.balance.unwrap().amount,
            "1000000000000000000".to_string()
        );

        // Execute AdminMsg::UpdateAdmin
        let new_admin = app
            .init_account(&[Coin::new(1_000_000_000, "uosmo")])
            .unwrap();
        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateAdmin {
                    new_admin: new_admin.address(),
                }),
                &[],
                &admin,
            )
            .unwrap();

        // TODO: Assert admin changed and queriable

        // AdminMsg::UpdateMerkleRoot
        let merkle_root: &str = "0hGvbH+l9pdPgOmJY6wZuwjsrvtPsuslgTURavrUP6I=";
        let _ = wasm
            .execute(
                contract.as_str(),
                &ExecuteMsg::AdminMsg(AdminExecuteMsg::UpdateMerkleRoot {
                    new_root: merkle_root.to_string(),
                }),
                &[],
                &new_admin,
            )
            .unwrap();

        // Execute IncentivesMsg::Claim
        for (index, claim_account) in claim_accounts.iter().enumerate() {
            println!(
                "{:?} {:?} {:?}",
                accounts.get(index).unwrap().address().to_string(),
                index,
                claim_accounts.len()
            );
            let mut proof_hashes: Vec<[u8; 32]> = Vec::new();
            for proof in &claim_account.proof {
                if proof.len() == 32 {
                    let mut arr = [0u8; 32];
                    arr.copy_from_slice(&proof);
                    proof_hashes.push(arr);
                } else {
                    eprintln!("Error: Hash is not 32 bytes.");
                }
            }

            // Execute claim for the current user
            let _ = wasm
                .execute(
                    contract.as_str(),
                    &ExecuteMsg::IncentivesMsg(IncentivesExecuteMsg::Claim {
                        address: accounts.get(index).unwrap().address().to_string(),
                        coins: claim_account.coins.clone(),
                        proof_hashes,
                        leaf_index: index,
                        total_leaves_count: 10usize,
                        destination_address: Addr::unchecked(accounts.get(index).unwrap().address()),
                        // total_leaves_count: claim_accounts.len(), // TODO: reimplement this with all 10 users claiming
                    }),
                    &[],
                    &accounts.get(index).unwrap(),
                )
                .unwrap();

            // Assert bank send occurred
            let address_balance = bank
                .query_balance(&QueryBalanceRequest {
                    address: accounts.get(index).unwrap().address().to_string(),
                    denom: "ugauge".to_string(),
                })
                .unwrap();
            assert_eq!(
                address_balance.balance.unwrap().amount,
                claim_account
                    .coins
                    .coins()
                    .get(0)
                    .unwrap()
                    .amount
                    .to_string()
            );
        }

        // Assert final balance on gauge contract
        // Positive due to precision loss
        let contract_balance = bank
            .query_balance(&QueryBalanceRequest {
                address: contract.to_string(),
                denom: "ugauge".to_string(),
            })
            .unwrap();
        assert_eq!(
            contract_balance.balance.unwrap().amount,
            // "100000".to_string() // TODO: reimplement this with all 10 users claiming
            "999999000100000000".to_string()
        );
    }
}
