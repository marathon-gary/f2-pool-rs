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
pub struct HashRateDistributionOrder {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Start timestamp
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    /// End timestamp, must be later than start timestamp
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    /// The distributor account name
    #[serde(rename = "distributor", skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    /// The recipient account name
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// Hashrate
    #[serde(rename = "hash_rate", skip_serializing_if = "Option::is_none")]
    pub hash_rate: Option<f64>,
    /// Return value if the order was ended early, otherwise 0
    #[serde(rename = "terminate_at", skip_serializing_if = "Option::is_none")]
    pub terminate_at: Option<i64>,
}

impl HashRateDistributionOrder {
    pub fn new() -> HashRateDistributionOrder {
        HashRateDistributionOrder {
            id: None,
            start_date: None,
            end_date: None,
            distributor: None,
            recipient: None,
            hash_rate: None,
            terminate_at: None,
        }
    }
}
