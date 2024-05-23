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
pub struct V2AssetsSettleModeSwitchPostRequest {
    /// Currency Type
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Settle mode
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// User
    #[serde(rename = "mining_user_name", skip_serializing_if = "Option::is_none")]
    pub mining_user_name: Option<String>,
    /// Active time
    #[serde(rename = "activated_at", skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<i64>,
}

impl V2AssetsSettleModeSwitchPostRequest {
    pub fn new() -> V2AssetsSettleModeSwitchPostRequest {
        V2AssetsSettleModeSwitchPostRequest {
            currency: None,
            mode: None,
            mining_user_name: None,
            activated_at: None,
        }
    }
}
