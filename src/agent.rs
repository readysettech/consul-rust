use std::collections::HashMap;

use async_trait::async_trait;

use crate::errors::Result;
use crate::request::{get, put};
use crate::Client;

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentCheck {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentMember {
    pub Name: String,
    pub Addr: String,
    pub Port: u16,
    pub Tags: HashMap<String, String>,
    pub pubStatus: usize,
    pub ProtocolMin: u8,
    pub ProtocolMax: u8,
    pub ProtocolCur: u8,
    pub DelegateMin: u8,
    pub DelegateMax: u8,
    pub DelegateCur: u8,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentService {
    pub ID: String,
    pub Service: String,
    pub Tags: Option<Vec<String>>,
    pub Port: u16,
    pub Address: String,
    pub EnableTagOverride: bool,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

//I haven't implemetned https://www.consul.io/api/agent.html#read-configuration
//I haven't implemetned https://www.consul.io/api/agent.html#stream-logs
#[async_trait]
pub trait Agent {
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>>;
    async fn members(&self, wan: bool) -> Result<AgentMember>;
    async fn reload(&self) -> Result<()>;
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()>;
    async fn join(&self, address: &str, wan: bool) -> Result<()>;
    async fn leave(&self) -> Result<()>;
    async fn force_leave(&self) -> Result<()>;
}

#[async_trait]
impl Agent for Client {
    /// https://www.consul.io/api/agent/check.html#list-checks
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>> {
        get("/v1/agent/checks", &self.config, HashMap::new(), None)
            .await
            .map(|x| x.0)
    }
    /// https://www.consul.io/api/agent.html#list-members
    async fn members(&self, wan: bool) -> Result<AgentMember> {
        let mut params = HashMap::new();
        if wan {
            params.insert(String::from("wan"), String::from("1"));
        }
        get("/v1/agent/members", &self.config, params, None)
            .await
            .map(|x| x.0)
    }
    /// https://www.consul.io/api/agent.html#reload-agent
    async fn reload(&self) -> Result<()> {
        put(
            "/v1/agent/reload",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#reload-agent
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable {
            String::from("true")
        } else {
            String::from("false")
        };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        put(
            "/v1/agent/maintenance",
            None as Option<&()>,
            &self.config,
            params,
            None,
        )
        .await
        .map(|x| x.0)
    }
    ///https://www.consul.io/api/agent.html#join-agent
    async fn join(&self, address: &str, wan: bool) -> Result<()> {
        let mut params = HashMap::new();

        if wan {
            params.insert(String::from("wan"), String::from("true"));
        }
        let path = format!("/v1/agent/join/{}", address);
        put(&path, None as Option<&()>, &self.config, params, None)
            .await
            .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#graceful-leave-and-shutdown
    async fn leave(&self) -> Result<()> {
        put(
            "/v1/agent/leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }

    ///https://www.consul.io/api/agent.html#force-leave-and-shutdown
    async fn force_leave(&self) -> Result<()> {
        put(
            "/v1/agent/force-leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .await
        .map(|x| x.0)
    }
}
