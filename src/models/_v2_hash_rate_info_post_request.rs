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
pub struct V2HashRateInfoPostRequest {
    /// Account name (Only one of mining_user_name and address should be used)
    #[serde(rename = "mining_user_name", skip_serializing_if = "Option::is_none")]
    pub mining_user_name: Option<String>,
    /// Wallet address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Currency type
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl V2HashRateInfoPostRequest {
    pub fn new() -> V2HashRateInfoPostRequest {
        V2HashRateInfoPostRequest {
            mining_user_name: None,
            address: None,
            currency: None,
        }
    }
}

