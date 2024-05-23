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
pub struct WorkerMiningInfo {
    #[serde(rename = "hash_rate_info", skip_serializing_if = "Option::is_none")]
    pub hash_rate_info: Option<Box<models::HashRateInfo>>,
    /// The time of the last submitted share from the worker
    #[serde(rename = "last_share_at", skip_serializing_if = "Option::is_none")]
    pub last_share_at: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::WorkerStatus>,
    /// The public IP address
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl WorkerMiningInfo {
    pub fn new() -> WorkerMiningInfo {
        WorkerMiningInfo {
            hash_rate_info: None,
            last_share_at: None,
            status: None,
            host: None,
        }
    }
}

