use std::collections::HashMap;

use async_trait::async_trait;

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Weights {
    Passing: u32,
    Warning: u32,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogNode {
    Node: Option<Node>,
    Services: HashMap<String, AgentService>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogDeregistration {
    Node: String,
    Address: String,
    Datacenter: String,
    ServiceID: String,
    CheckID: String,
}

#[async_trait]
pub trait Catalog {
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)>;
    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
}

#[async_trait]
impl Catalog for Client {
    /// https://www.consul.io/api/catalog.html#register-entity
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/session/create",
            Some(reg),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#deregister-entity
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/catalog/deregister",
            Some(dereg),
            &self.config,
            HashMap::new(),
            q,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#list-datacenters
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
        .await
    }

    /// https://www.consul.io/api/catalog.html#list-nodes
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)> {
        get("/v1/catalog/nodes", &self.config, HashMap::new(), q).await
    }

    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q).await
    }
}
