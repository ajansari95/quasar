import { StdSignDoc, OfflineAminoSigner, makeSignDoc } from '@cosmjs/amino';
import { MockKeplr } from "@keplr-wallet/provider-mock";
import { Buffer } from "buffer/";
import { makeADR36AminoSignDoc, serializeSignDoc } from "@keplr-wallet/cosmos"
import { toBase64, toUtf8 } from "@cosmjs/encoding"


const getBankMsgAndSignature = async (): Promise<any> => {

  const chainId = "";

  const keplr = new MockKeplr(
    () => {
      throw new Error("Not implemented");
    },
    [
      {
        chainId,
        bech32Config: {
          bech32PrefixAccAddr: "osmo",
        },
      },
    ],
    "diary match wagon soccer worth planet sea stumble thought post easily want"
  );

  const offlineSigner = keplr.getOfflineSignerOnlyAmino(chainId);

  const signer = (await offlineSigner.getAccounts())[0].address;

  console.log("signer:", signer)

  const msg = {
    "bank":
      {"send":
        {
          "to_address":"osmo1ce0nzfm5a0j5yg48xz88qr430caaxdrswmkmnn",
          "amount":[
            {
              "denom":"utest",
              "amount":"1"
            }
          ]
        }
      }
    };

  const adr36Msg = makeADR36AminoSignDoc(JSON.stringify(msg), signer);
  const serializedDoc = await serializeSignDoc(adr36Msg);

    keplr.signAmino
  // This function is what keplr and leap do to sign an adr36 signdoc in the browser
  let signature = await offlineSigner.signAmino(
    signer,
    serializedDoc,
  );

  return {
    adr36Msg,
    signature
  }
}

(async () => {
  let data = await getBankMsgAndSignature();
  console.log("message:", toBase64(toUtf8(JSON.stringify(data.adr36Msg))));
  console.log("pubkey:", JSON.stringify(data.signature.signature.pub_key));
  console.log("signature:", data.signature.signature.signature);
  console.log("signature:", toBase64(toUtf8(JSON.stringify(data.signature.signature.signature))));
})();
