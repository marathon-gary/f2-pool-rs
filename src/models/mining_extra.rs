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
pub struct MiningExtra {
    /// Date of mining
    #[serde(rename = "mining_date", skip_serializing_if = "Option::is_none")]
    pub mining_date: Option<i64>,
    /// Date of settlement
    #[serde(rename = "settle_date", skip_serializing_if = "Option::is_none")]
    pub settle_date: Option<i64>,
    /// PPS revenues
    #[serde(rename = "pps", skip_serializing_if = "Option::is_none")]
    pub pps: Option<f64>,
    /// The pool fee of the PPS part
    #[serde(rename = "pps_fee_rate", skip_serializing_if = "Option::is_none")]
    pub pps_fee_rate: Option<f32>,
    /// Reward from transaction fees
    #[serde(rename = "tx_fee", skip_serializing_if = "Option::is_none")]
    pub tx_fee: Option<f64>,
    /// The pool fee of the reward from transaction fees
    #[serde(rename = "tx_fee_rate", skip_serializing_if = "Option::is_none")]
    pub tx_fee_rate: Option<f32>,
    /// The average hashrate on the day of mining
    #[serde(rename = "hash_rate", skip_serializing_if = "Option::is_none")]
    pub hash_rate: Option<f64>,
}

impl MiningExtra {
    pub fn new() -> MiningExtra {
        MiningExtra {
            mining_date: None,
            settle_date: None,
            pps: None,
            pps_fee_rate: None,
            tx_fee: None,
            tx_fee_rate: None,
            hash_rate: None,
        }
    }
}

