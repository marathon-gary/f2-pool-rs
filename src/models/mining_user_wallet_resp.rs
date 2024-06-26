/*
 * F2 Pool API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MiningUserWalletResp {
    /// Account name
    #[serde(rename = "mining_user_name", skip_serializing_if = "Option::is_none")]
    pub mining_user_name: Option<String>,
    /// 
    #[serde(rename = "wallet_result", skip_serializing_if = "Option::is_none")]
    pub wallet_result: Option<Vec<models::WalletResp>>,
}

impl MiningUserWalletResp {
    pub fn new() -> MiningUserWalletResp {
        MiningUserWalletResp {
            mining_user_name: None,
            wallet_result: None,
        }
    }
}

