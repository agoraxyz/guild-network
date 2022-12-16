use gn_client::{AccountId, SubstrateAddress};
use js_sys::{Error, Function, JsString, Object, Promise};
use serde_wasm_bindgen::from_value as deserialize_from_value;
use wasm_bindgen::prelude::JsValue;
use web_sys::{console, window};

use std::str::FromStr;

fn get_sign_request(msg: &[u8], address: &SubstrateAddress) -> Result<js_sys::Object, Error> {
    let sign_request_param = js_sys::Object::new();

    js_sys::Reflect::set(
        &sign_request_param,
        &JsString::from("address"),
        &JsString::from(address.to_string()),
    )?;
    js_sys::Reflect::set(
        &sign_request_param,
        &JsString::from("data"),
        &JsString::from(hex::encode(msg)),
    )?;

    Ok(sign_request_param)
}

pub struct WasmSigner {
    lib: Object,
    account_id: AccountId,
    address: SubstrateAddress,
}

impl WasmSigner {
    pub async fn new() -> Result<Self, Error> {
        let window = window().expect("Failed to access window object");

        let injected_web3 = window
            .get("injectedWeb3")
            .expect("Failed to access window.injectedWeb3");

        let polkadot_js: JsValue =
            js_sys::Reflect::get(&injected_web3, &JsString::from("polkadot-js"))?;

        let enable: Function =
            js_sys::Reflect::get(&polkadot_js, &JsString::from("enable"))?.into();

        let lib: Object =
            wasm_bindgen_futures::JsFuture::from(Promise::from(enable.call0(&JsValue::NULL)?))
                .await?
                .into();

        let addresses: js_sys::Array = wasm_bindgen_futures::JsFuture::from(Promise::from(
            js_sys::Function::from(js_sys::Reflect::get(
                &js_sys::Reflect::get(&lib, &JsString::from("accounts"))?,
                &JsString::from("get"),
            )?)
            .call0(&JsValue::NULL)?,
        ))
        .await?
        .into();

        let name: String = js_sys::Reflect::get(&addresses.at(0), &JsString::from("name"))?
            .as_string()
            .expect("Failed to cast addresses[0] to String");
        let address: String = js_sys::Reflect::get(&addresses.at(0), &JsString::from("address"))?
            .as_string()
            .expect("Failed to cast addresses[0] to String");

        console::log_1(&format!("Hello {}! ({})", name, address).into());

        let account_id = AccountId::from_str(&address).expect("invalid address");
        let address = SubstrateAddress::from(account_id.clone());

        Ok(Self {
            lib,
            account_id,
            address,
        })
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn address(&self) -> &SubstrateAddress {
        &self.address
    }

    pub async fn sign(&self, signer_payload: &[u8]) -> Result<[u8; 64], JsValue> {
        let signer = js_sys::Reflect::get(&self.lib, &JsString::from("signer"))?;
        let sign_raw: Function = js_sys::Reflect::get(&signer, &JsString::from("signRaw"))?.into();

        let sign_payload = get_sign_request(&signer_payload, self.address())?;

        let signature_js = sign_raw.call1(&JsValue::NULL, &sign_payload)?;

        let signature: String =
            deserialize_from_value(signature_js).map_err(|e| JsValue::from(e.to_string()))?;
        let mut signature_bytes = [0u8; 64];
        hex::decode_to_slice(&signature, &mut signature_bytes)
            .map_err(|e| JsValue::from(e.to_string()))?;

        Ok(signature_bytes)
    }
}
