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
pub struct HashRateDistributionSettlement {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<models::HashRateDistributionOrder>>,
    /// Note that this is the number of hashes, not the hashrate
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<f64>,
    /// Revenue
    #[serde(rename = "income", skip_serializing_if = "Option::is_none")]
    pub income: Option<f64>,
    /// Settlement timestamp of the hashrate distribution
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl HashRateDistributionSettlement {
    pub fn new() -> HashRateDistributionSettlement {
        HashRateDistributionSettlement {
            id: None,
            order: None,
            hashes: None,
            income: None,
            timestamp: None,
        }
    }
}

