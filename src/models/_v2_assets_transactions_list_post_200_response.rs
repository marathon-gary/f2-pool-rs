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
pub struct V2AssetsTransactionsListPost200Response {
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<models::TransactionItem>>,
}

impl V2AssetsTransactionsListPost200Response {
    pub fn new() -> V2AssetsTransactionsListPost200Response {
        V2AssetsTransactionsListPost200Response {
            transactions: None,
        }
    }
}

