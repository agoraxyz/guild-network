#![deny(clippy::all)]
#![deny(clippy::dbg_macro)]
#![deny(unused_crate_dependencies)]

use gn_client::{query, AccountId, Api};
use gn_common::filter::Guild as GuildFilter;
use gn_common::identity::Identity;
use gn_common::merkle::Proof;
use gn_common::{pad::pad_to_n_bytes, GuildName, RoleName};
use serde_wasm_bindgen::{from_value as deserialize_from_value, to_value as serialize_to_value};
use wasm_bindgen::prelude::*;

use std::str::FromStr;

const PAD_BYTES: usize = 32;

fn sanitize_name(name: String) -> Result<[u8; PAD_BYTES], JsValue> {
    if name.is_empty() || name.len() > PAD_BYTES {
        return Err(JsValue::from("invalid name length"));
    }
    Ok(pad_to_n_bytes::<PAD_BYTES, _>(&name))
}

#[wasm_bindgen(js_name = "queryMembers")]
pub async fn query_members(
    guild: String,
    role: Option<String>,
    url: String,
) -> Result<JsValue, JsValue> {
    let api = Api::from_url(&url)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    let guild_name = sanitize_name(guild)?;
    let role_name: Option<RoleName> = role.map(sanitize_name).transpose()?;
    let filter = GuildFilter {
        name: guild_name,
        role: role_name,
    };

    let members = query::members(api, &filter, 10)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    serialize_to_value(&members).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "queryGuilds")]
pub async fn query_guilds(guild: Option<String>, url: String) -> Result<JsValue, JsValue> {
    let api = Api::from_url(&url)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    let guild_name: Option<GuildName> = guild.map(sanitize_name).transpose()?;

    let guilds = query::guilds(api, guild_name, 10)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    serialize_to_value(&guilds).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "queryRequirements")]
pub async fn query_requirements(
    guild: String,
    role: String,
    url: String,
) -> Result<JsValue, JsValue> {
    let api = Api::from_url(&url)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    let guild_name = sanitize_name(guild)?;
    let role_name = sanitize_name(role)?;

    let requirements = query::filtered_requirements(api, guild_name, role_name)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;
    serialize_to_value(&requirements).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "queryUserIdentity")]
pub async fn query_user_identity(address: String, url: String) -> Result<JsValue, JsValue> {
    let id = AccountId::from_str(&address).map_err(|e| JsValue::from(e.to_string()))?;
    let api = Api::from_url(&url)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    let identities = query::user_identity(api, &id)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    serialize_to_value(&identities).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "queryAllowlist")]
pub async fn query_allowlist(guild: String, role: String, url: String) -> Result<JsValue, JsValue> {
    let api = Api::from_url(&url)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    let guild_name = sanitize_name(guild)?;
    let role_name = sanitize_name(role)?;

    let allowlist = query::allowlist(api, guild_name, role_name)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;
    serialize_to_value(&allowlist).map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "generateMerkleProof")]
pub fn generate_merkle_proof(
    list: JsValue,
    leaf_index: usize,
    id_index: u8,
) -> Result<JsValue, JsValue> {
    let allowlist: Vec<Identity> =
        deserialize_from_value(list).map_err(|e| JsValue::from(e.to_string()))?;
    serialize_to_value(&Proof::new(&allowlist, leaf_index, id_index))
        .map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "verificationMsg")]
pub async fn verification_msg(address: String) -> String {
    gn_common::utils::verification_msg(address)
}

#[cfg(test)]
mod test {
    use super::*;
    use gn_test_data::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn init_tracing() {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }

    #[wasm_bindgen_test]
    async fn test_query_chain() {
        init_tracing();

        let api = Api::from_url(URL).await.unwrap();

        let chain = api.rpc().system_chain().await.unwrap();

        assert_eq!(chain, "Development");
    }

    // NOTE these only work after the guild/join example
    // was successfully run
    #[cfg(feature = "queries")]
    mod queries {
        use super::*;
        use gn_client::{query::FilteredRequirements, AccountId};
        use gn_common::filter::{Filter, Logic as FilterLogic};
        use gn_common::identity::Identity;
        use gn_common::Guild;

        #[wasm_bindgen_test]
        async fn test_query_members() {
            let guild = "myguild".to_string();
            let role = None;
            let members_js = query_members(guild, role, URL.to_string()).await.unwrap();
            let members_vec: Vec<AccountId> = deserialize_from_value(members_js).unwrap();

            assert_eq!(members_vec.len(), N_TEST_ACCOUNTS);

            let guild = "mysecondguild".to_string();
            let role = Some("myrole".to_string());
            let members_js = query_members(guild, role, URL.to_string()).await.unwrap();
            let members_vec: Vec<AccountId> = deserialize_from_value(members_js).unwrap();

            assert_eq!(members_vec.len(), N_TEST_ACCOUNTS / 2);
        }

        #[wasm_bindgen_test]
        async fn test_query_guilds() {
            let guilds_js = query_guilds(None, URL.to_string()).await.unwrap();
            let guilds: Vec<Guild<AccountId>> = deserialize_from_value(guilds_js).unwrap();

            assert!(guilds.len() == 2);
            for guild in &guilds {
                assert_eq!(guild.roles[0], pad_to_n_bytes::<PAD_BYTES, _>("myrole"));
                assert_eq!(
                    guild.roles[1],
                    pad_to_n_bytes::<PAD_BYTES, _>("mysecondrole")
                );
            }
        }

        #[wasm_bindgen_test]
        async fn test_query_requirements() {
            let guild_name = "myguild".to_string();
            let role_name = "myrole".to_string();
            let requirements_js = query_requirements(guild_name, role_name, URL.to_string())
                .await
                .unwrap();
            let requirements: FilteredRequirements =
                deserialize_from_value(requirements_js).unwrap();
            assert!(requirements.filter.is_none());
            assert!(requirements.requirements.is_none());

            let guild_name = "myguild".to_string();
            let role_name = "mysecondrole".to_string();
            let requirements_js =
                query_requirements(guild_name.clone(), role_name.clone(), URL.to_string())
                    .await
                    .unwrap();
            let requirements: FilteredRequirements =
                deserialize_from_value(requirements_js).unwrap();
            let root = gn_client::H256::from_str(
                "0xf6bace20645fc288795dc16cf6780d755772ba7fbe8815d78d911023ff3c8f5b",
            )
            .unwrap();
            assert_eq!(
                requirements.filter,
                Some(Filter::Allowlist(
                    root.0,
                    FilterLogic::And,
                    N_TEST_ACCOUNTS as u32
                ))
            );
            assert!(requirements.requirements.is_none());

            let allowlist_js = query_allowlist(guild_name, role_name, URL.to_string())
                .await
                .unwrap();
            let allowlist = deserialize_from_value::<Option<Vec<Identity>>>(allowlist_js)
                .unwrap()
                .unwrap();
            assert_eq!(allowlist.len(), N_TEST_ACCOUNTS);
        }

        #[wasm_bindgen_test]
        async fn test_query_user_identity() {
            use gn_common::utils::matches_variant;
            let account_id = AccountId::from_str(TEST_ADDRESS).unwrap();

            let members_js = query_members("myguild".to_string(), None, URL.to_string())
                .await
                .unwrap();
            let members_vec: Vec<AccountId> = deserialize_from_value(members_js).unwrap();
            assert!(members_vec.contains(&account_id));

            let identities_js = query_user_identity(TEST_ADDRESS.to_string(), URL.to_string())
                .await
                .unwrap();

            let identities: Vec<Identity> = deserialize_from_value(identities_js).unwrap();

            assert_eq!(identities.len(), 2);
            assert!(matches_variant(
                identities.get(0).unwrap(),
                &Identity::Address20([0u8; 20])
            ));
            assert!(matches_variant(
                identities.get(1).unwrap(),
                &Identity::Other([0u8; 64])
            ));
        }

        #[wasm_bindgen_test]
        async fn test_generate_proof() {
            let guild_name = "myguild".to_string();
            let role_name = "mysecondrole".to_string();
            let allowlist_js = query_allowlist(guild_name, role_name, URL.to_string())
                .await
                .unwrap();
            let proof_js = generate_merkle_proof(allowlist_js, 7, 0).unwrap();
            let proof: Proof = deserialize_from_value(proof_js).unwrap();
            assert_eq!(proof.path.len(), 4);
            assert_eq!(proof.id_index, 0);
        }
    }
}
