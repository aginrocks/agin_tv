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
pub struct MovieWatchProviders200ResponseResultsEc {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Option<Vec<models::MovieWatchProviders200ResponseResultsEcBuyInner>>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::MovieWatchProviders200ResponseResultsArFlatrateInner>>,
    #[serde(rename = "rent", skip_serializing_if = "Option::is_none")]
    pub rent: Option<Vec<models::MovieWatchProviders200ResponseResultsEcBuyInner>>,
}

impl MovieWatchProviders200ResponseResultsEc {
    pub fn new() -> MovieWatchProviders200ResponseResultsEc {
        MovieWatchProviders200ResponseResultsEc {
            link: None,
            buy: None,
            flatrate: None,
            rent: None,
        }
    }
}

