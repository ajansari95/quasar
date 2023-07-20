import prompts from "prompts";
import {seed_liquidity_from_alice, simple_test_bond, simple_test_claim, simple_test_unbond} from "./vault/src/driver";

async function main() {
  let response = await prompts({
    type: "text",
    name: "vaultAddress",
    message: "Enter the vault address",
  });
  console.log("vault addr:", response.vaultAddress);

  // TODO put prompt here to ask if yes or not liquidity from alice, if yes do it otherwise not
  let liquidityResponse = await prompts({
    type: 'confirm',
    name: 'seedLiquidity',
    message: 'Do you want to seed liquidity from Alice?',
  });

  if (liquidityResponse.seedLiquidity) {
    await seed_liquidity_from_alice(response.vaultAddress);
  }

  // TODO put prompt here to just wait and ask if continue
  let bondResponse = await prompts({
    type: 'confirm',
    name: 'continue',
    message: 'Do you want to continue to test bond?',
  });

  if (bondResponse.continue) {
    await simple_test_bond(response.vaultAddress);
  }

  // TODO put prompt here to just wait and ask if continue
  let unbondResponse = await prompts({
    type: 'confirm',
    name: 'continue',
    message: 'Do you want to continue to test unbond?',
  });

  if (unbondResponse.continue) {
    await simple_test_unbond(response.vaultAddress);
  }

  // TODO put prompt here to just wait and ask if continue
  let claimResponse = await prompts({
    type: 'confirm',
    name: 'continue',
    message: 'Do you want to continue to test claim?',
  });

  if (claimResponse.continue) {
    await simple_test_claim(response.vaultAddress);
  }
}


main();
