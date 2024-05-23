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
pub struct V2MiningUserWalletUpdatePost200Response {
    /// Results of the operation
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::ResultsEntry>>,
}

impl V2MiningUserWalletUpdatePost200Response {
    pub fn new() -> V2MiningUserWalletUpdatePost200Response {
        V2MiningUserWalletUpdatePost200Response {
            results: None,
        }
    }
}
