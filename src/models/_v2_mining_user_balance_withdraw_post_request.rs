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
pub struct V2MiningUserBalanceWithdrawPostRequest {
    /// Account name
    #[serde(rename = "mining_user_name", skip_serializing_if = "Option::is_none")]
    pub mining_user_name: Option<String>,
    /// Currency type
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl V2MiningUserBalanceWithdrawPostRequest {
    pub fn new() -> V2MiningUserBalanceWithdrawPostRequest {
        V2MiningUserBalanceWithdrawPostRequest {
            mining_user_name: None,
            currency: None,
        }
    }
}

