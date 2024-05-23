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
pub struct V2HashRateHistoryPostRequest {
    /// Account name (Only one of mining_user_name and address should be used)
    #[serde(rename = "mining_user_name", skip_serializing_if = "Option::is_none")]
    pub mining_user_name: Option<String>,
    /// Wallet address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Currency type
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The seconds of interval, it should be divisible by 10 minutes
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// The seconds of duration, it is up to 30 days
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

impl V2HashRateHistoryPostRequest {
    pub fn new() -> V2HashRateHistoryPostRequest {
        V2HashRateHistoryPostRequest {
            mining_user_name: None,
            address: None,
            currency: None,
            interval: None,
            duration: None,
        }
    }
}
