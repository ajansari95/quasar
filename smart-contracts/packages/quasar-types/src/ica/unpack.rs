use crate::{error::Error, ica::traits::Unpack};
/// a collection of pack implementations for foreign types to be used in our ica contracts
use cosmos_sdk_proto::Any;
use osmosis_std::types::osmosis::{
    gamm::v1beta1::{
        MsgExitSwapShareAmountInResponse, MsgJoinSwapExternAmountIn,
        MsgJoinSwapExternAmountInResponse,
    },
    lockup::MsgLockTokensResponse,
};
use prost::Message;

impl Unpack for MsgJoinSwapExternAmountIn {
    fn unpack(msg: Any) -> Result<Self, Error> {
        if msg.type_url != MsgJoinSwapExternAmountIn::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl {
                expected: MsgJoinSwapExternAmountIn::TYPE_URL.to_string(),
                actual: msg.type_url,
            });
        }
        let val: Self = Message::decode(msg.value.as_ref())?;
        Ok(val)
    }
}

impl Unpack for MsgJoinSwapExternAmountInResponse {
    // For some reason, MsgJoinSwapExternAmountInResponse's type url on Osmosis is the same as MsgJoinSwapExternAmountIn
    fn unpack(msg: Any) -> Result<Self, Error> {
        // the type url is intended to MsgJoinSwapExternAmountIn
        if msg.type_url != MsgJoinSwapExternAmountInResponse::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl {
                expected: MsgJoinSwapExternAmountInResponse::TYPE_URL.to_string(),
                actual: msg.type_url,
            });
        }
        let val: Self = Message::decode(msg.value.as_ref())?;
        Ok(val)
    }
}

impl Unpack for MsgLockTokensResponse {
    fn unpack(msg: Any) -> Result<Self, Error> {
        if msg.type_url != MsgLockTokensResponse::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl {
                expected: MsgLockTokensResponse::TYPE_URL.to_string(),
                actual: msg.type_url,
            });
        }
        let val: Self = Message::decode(msg.value.as_ref())?;
        Ok(val)
    }
}

impl Unpack for MsgExitSwapShareAmountInResponse {
    fn unpack(msg: Any) -> Result<Self, Error> {
        if msg.type_url != MsgExitSwapShareAmountInResponse::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl {
                expected: MsgExitSwapShareAmountInResponse::TYPE_URL.to_string(),
                actual: msg.type_url,
            });
        }
        let val: Self = Message::decode(msg.value.as_ref())?;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Binary;
    use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;

    use crate::ica::{packet::AckBody, traits::Unpack};

    #[test]
    fn unpack_lock_tokens_response() {
        // we take a raw response from Osmosis, decode it to an any, and then try to unpack the any
        let raw =
            Binary::from_base64("EkwKNy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dKb2luU3dhcEV4dGVybkFtb3VudEluUmVzcG9uc2USEQoPMTY1ODM0MjY1MjA1Mzgz").unwrap();
        let any = AckBody::from_bytes(raw.0.as_ref())
            .unwrap()
            .to_any()
            .unwrap();
        let resp = MsgJoinSwapExternAmountInResponse::unpack(any).unwrap();
        assert_eq!(resp.share_out_amount, 165834265205383_u128.to_string())
    }
}
