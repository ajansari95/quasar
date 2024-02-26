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

    #[test]
    fn verify_keplr_signature_works() {}
}
