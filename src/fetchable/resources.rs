use serde::{Deserialize, Serialize};

use super::Fetchable;
use crate::entities::ResourceType;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource<R: ResourceType> {
    pub id: R::Id,
    pub text: String,
    pub dept: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceList<R: ResourceType> {
    pub total: u64,
    #[serde(bound(deserialize = "R: ResourceType"))]
    pub results: Vec<Resource<R>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceListRequest<R: ResourceType> {
    pub my_resources: bool,
    pub search_term: String,
    pub page_size: u64,
    pub page_number: u64,
    pub res_type: R,
}

impl<R> Fetchable for ResourceList<R>
where
    R: ResourceType,
{
    type Request = ResourceListRequest<R>;

    const METHOD_NAME: &'static str = "ReadResourceListItems";
}
