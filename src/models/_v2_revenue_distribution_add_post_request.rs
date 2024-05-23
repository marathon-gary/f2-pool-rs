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
pub struct V2RevenueDistributionAddPostRequest {
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "distributor", skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "proportion", skip_serializing_if = "Option::is_none")]
    pub proportion: Option<f64>,
}

impl V2RevenueDistributionAddPostRequest {
    pub fn new() -> V2RevenueDistributionAddPostRequest {
        V2RevenueDistributionAddPostRequest {
            currency: None,
            distributor: None,
            recipient: None,
            description: None,
            proportion: None,
        }
    }
}
