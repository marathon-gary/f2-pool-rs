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
pub struct V2HashRateDistributionTerminatePostRequest {
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
}

impl V2HashRateDistributionTerminatePostRequest {
    pub fn new() -> V2HashRateDistributionTerminatePostRequest {
        V2HashRateDistributionTerminatePostRequest {
            order_id: None,
        }
    }
}
