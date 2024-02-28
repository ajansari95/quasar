use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, to_json_string, Addr, Binary, Coin, StdError, StdResult};
use serde::Serialize;

pub fn create_adr36_json_binary<T>(signer: Addr, data: &T) -> StdResult<Binary>
where
    T: Serialize,
{
    to_json_binary(&SortAlphabetically(Adr36Message::new(
        signer,
        serde_json_wasm::to_string(&data)
            .map_err(|err| StdError::serialize_err(std::any::type_name::<T>(), err.to_string()))?,
    )))
}

pub fn create_adr36_json_string<T>(signer: Addr, data: &T) -> StdResult<String>
where
    T: Serialize,
{
    to_json_string(&SortAlphabetically(Adr36Message::new(
        signer,
        serde_json_wasm::to_string(&data)
            .map_err(|err| StdError::serialize_err(std::any::type_name::<T>(), err.to_string()))?,
    )))
}

fn sort_alphabetically<T: Serialize, S: serde::Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let value = serde_cw_value::to_value(value).map_err(serde::ser::Error::custom)?;
    value.serialize(serializer)
}

#[cw_serde]
struct SortAlphabetically<T: Serialize>(#[serde(serialize_with = "sort_alphabetically")] T);

#[cw_serde]
struct Adr36Message {
    chain_id: String,
    account_number: String,
    sequence: String,
    fee: Fee,
    msgs: Vec<Msg>,
    memo: String,
}

impl Adr36Message {
    fn new(signer: Addr, data: String) -> Adr36Message {
        Adr36Message {
            chain_id: "".to_string(),
            account_number: "0".to_string(),
            sequence: "0".to_string(),
            fee: Fee::default(),
            msgs: vec![Msg::new(signer, data)],
            memo: "".to_string(),
        }
    }
}

#[cw_serde]
struct Fee {
    gas: String,
    amount: Vec<Coin>,
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            gas: "0".into(),
            amount: Default::default(),
        }
    }
}

#[cw_serde]
struct Msg {
    r#type: String,
    value: Value,
}

impl Msg {
    fn new(signer: Addr, data: String) -> Msg {
        Msg {
            r#type: "sign/MsgSignData".to_string(),
            value: Value::new(signer, data),
        }
    }
}

#[cw_serde]
struct Value {
    signer: Addr,
    data: String,
}

impl Value {
    fn new(signer: Addr, data: String) -> Value {
        Value { signer, data }
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, testing::mock_dependencies, to_json_binary, Api, BankMsg, CosmosMsg};
    use sha2::Sha256;
    use sha2::Digest;
    use base64::prelude::*;

    use super::*;

    #[test]
    fn verify_keplr_signature_works() {
        let keplr_msg = "eyJjaGFpbl9pZCI6IiIsImFjY291bnRfbnVtYmVyIjoiMCIsInNlcXVlbmNlIjoiMCIsImZlZSI6eyJnYXMiOiIwIiwiYW1vdW50IjpbXX0sIm1zZ3MiOlt7InR5cGUiOiJzaWduL01zZ1NpZ25EYXRhIiwidmFsdWUiOnsic2lnbmVyIjoie1wiYmFua1wiOntcInNlbmRcIjp7XCJ0b19hZGRyZXNzXCI6XCJvc21vMWNlMG56Zm01YTBqNXlnNDh4ejg4cXI0MzBjYWF4ZHJzd21rbW5uXCIsXCJhbW91bnRcIjpbe1wiZGVub21cIjpcInV0ZXN0XCIsXCJhbW91bnRcIjpcIjFcIn1dfX19IiwiZGF0YSI6ImIzTnRiekZqWlRCdWVtWnROV0V3YWpWNVp6UTRlSG80T0hGeU5ETXdZMkZoZUdSeWMzZHRhMjF1Ymc9PSJ9fV0sIm1lbW8iOiIifQ==";
        let keplr_signature = BASE64_STANDARD.decode("eTjfqodFUdl/YCwVuEJjVxEf3wENFolexSW7ro94Xhkk1Udm6BHZ7GV63uPk2vlVKUdlyqA1AM6atu20n70Hgg==").unwrap();

        let signer = "osmo1ce0nzfm5a0j5yg48xz88qr430caaxdrswmkmnn";
        let public_key = base64::decode("Av2Jj5fqKoUfPKSJfY3B8F6APIHPAkSDwzzsOIWWeQtn").unwrap();

        let bank_msg: CosmosMsg<BankMsg> = CosmosMsg::Bank(BankMsg::Send {
            to_address: signer.to_string(),
            amount: vec![coin(1, "utest")],
        });

        let adr_36_msg = create_adr36_json_binary(Addr::unchecked(signer), &to_json_binary(&bank_msg).unwrap()).unwrap();

        let message_hash = compute_sha256_hash(adr_36_msg.as_ref());

        let deps = mock_dependencies();

        let verified = deps.api
            .secp256k1_verify(message_hash.as_ref(), keplr_signature.as_ref(), public_key.as_ref()).unwrap();
        assert!(verified, "unable to verify")
    }

    fn compute_sha256_hash(message: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.finalize().to_vec()
    }
}
