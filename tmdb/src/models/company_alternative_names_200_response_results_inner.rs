/*
 * TMDB API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyAlternativeNames200ResponseResultsInner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CompanyAlternativeNames200ResponseResultsInner {
    pub fn new() -> CompanyAlternativeNames200ResponseResultsInner {
        CompanyAlternativeNames200ResponseResultsInner {
            name: None,
            r#type: None,
        }
    }
}

