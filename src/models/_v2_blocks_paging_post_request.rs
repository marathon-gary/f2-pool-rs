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
pub struct V2BlocksPagingPostRequest {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "pagesize", skip_serializing_if = "Option::is_none")]
    pub pagesize: Option<i32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl V2BlocksPagingPostRequest {
    pub fn new() -> V2BlocksPagingPostRequest {
        V2BlocksPagingPostRequest {
            page: None,
            pagesize: None,
            currency: None,
        }
    }
}
