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
pub struct V2RevenueDistributionDeletePostRequest {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "distributor", skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
}

impl V2RevenueDistributionDeletePostRequest {
    pub fn new() -> V2RevenueDistributionDeletePostRequest {
        V2RevenueDistributionDeletePostRequest {
            id: None,
            distributor: None,
        }
    }
}

