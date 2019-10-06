#![allow(non_snake_case)]

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::{QueryMeta, QueryOptions};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use async_trait::async_trait;

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Weights {
    Passing: u32,
    Warning: u32,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Node {
    ID: String,
    Node: String,
    Address: String,
    Datacenter: String,
    TaggedAddresses: HashMap<String, String>,
    Meta: HashMap<String, String>,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogService {
    ID: String,
    Node: String,
    Address: String,
    Datacenter: String,
    TaggedAddresses: HashMap<String, String>,
    NodeMeta: HashMap<String, String>,
    ServiceID: String,
    ServiceName: String,
    ServiceAddress: String,
    ServiceTags: Vec<String>,
    ServiceMeta: HashMap<String, String>,
    ServicePort: u32,
    ServiceWeights: Weights,
    ServiceEnableTagOverride: bool,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogNode {
    Node: Option<Node>,
    Services: HashMap<String, AgentService>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogRegistration {
    ID: String,
    Node: String,
    Address: String,
    TaggedAddresses: HashMap<String, String>,
    NodeMeta: HashMap<String, String>,
    Datacenter: String,
    Service: Option<AgentService>,
    Check: Option<AgentCheck>,
    SkipNodeUpdate: bool,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogDeregistration {
    Node: String,
    Address: String,
    Datacenter: String,
    ServiceID: String,
    CheckID: String,
}

#[async_trait]
pub trait Catalog {
    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, String>, QueryMeta)>;
}
