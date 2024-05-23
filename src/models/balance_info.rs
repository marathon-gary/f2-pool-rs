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
pub struct BalanceInfo {
    /// Balance (before 00:00 UTC)
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Total payment
    #[serde(rename = "paid", skip_serializing_if = "Option::is_none")]
    pub paid: Option<f64>,
    /// Total mining revenues
    #[serde(rename = "total_income", skip_serializing_if = "Option::is_none")]
    pub total_income: Option<f64>,
    /// Yesterday’s revenues
    #[serde(rename = "yesterday_income", skip_serializing_if = "Option::is_none")]
    pub yesterday_income: Option<f64>,
    /// The estimated mining revenue from 00:00 UTC to now
    #[serde(rename = "estimated_today_income", skip_serializing_if = "Option::is_none")]
    pub estimated_today_income: Option<f64>,
}

impl BalanceInfo {
    pub fn new() -> BalanceInfo {
        BalanceInfo {
            balance: None,
            paid: None,
            total_income: None,
            yesterday_income: None,
            estimated_today_income: None,
        }
    }
}

