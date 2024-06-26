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
pub struct V2RevenueDistributionInfoPost200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::RevenueDistribution>>,
}

impl V2RevenueDistributionInfoPost200Response {
    pub fn new() -> V2RevenueDistributionInfoPost200Response {
        V2RevenueDistributionInfoPost200Response {
            data: None,
        }
    }
}

