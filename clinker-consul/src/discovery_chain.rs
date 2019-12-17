#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct DiscoveryChainResponse {
  pub Chain: DiscoveryChain,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct DiscoveryChain {
    pub ServiceName: String,
    pub Namespace: String,
    pub Datacenter: String,
    pub Protocol: String,
    pub StartNode: String,
    pub Nodes: HashMap<String,ChainNode>,
    pub Targets: HashMap<String,ChainTarget>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChainNode {
    pub Type: String,
    pub Name: String,
    pub Resolver: Option<ResolverNode>,
    pub Routes: Option<Vec<RouterNodeRoute>>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ChainTarget {
    pub ID: String,
    pub Service: String,
    pub Datacenter: String,
    pub SNI: String,
    pub Name: String,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ResolverNode {
    pub ConnectTimeout: String,
    pub Default: bool,
    pub Target: String,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Split {
    pub Weight: u8,
    pub NextNode: String,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouterNodeRoute {
    pub Definition: RouteDefinition,
    pub NextNode: String,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteDefinition {
    pub Match: RouteMatch,
    pub Destination: RouteDestination,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteMatch {
    pub HTTP: Option<RouteHttpMatch>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteHttpMatch {
    pub PathExact: Option<String>,
    pub PathPrefix: Option<String>,
    pub PathRegex: Option<String>,
    pub Header: Option<Vec<RouteHttpHeaderMatch>>,
    pub Methods: Option<Vec<String>>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteHttpHeaderMatch {
    pub Name: String,
    pub Present: Option<bool>,
    pub Exact: Option<String>,
    pub Prefix: Option<String>,
    pub Suffix: Option<String>,
    pub Regex: Option<String>,
    pub Invert: Option<bool>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteHttpQueryMatch {
    pub Name: String,
    pub Present: Option<bool>,
    pub Exact: Option<String>,
    pub Regex: Option<String>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct RouteDestination {
    pub Service: Option<String>,
    pub ServiceSubset: Option<String>,
    pub Namespace: Option<String>,
    pub PrefixRewrite: Option<String>,
    pub RequestTimeout: Option<String>,
    pub NumRetries: Option<u32>,
    pub RetryOnConnectFailure: Option<bool>,
    pub RetryOnStatusCodes: Option<Vec<u16>>,
}

/*
"splitter:web": {
    "Type": "splitter",
    "Name": "web",
    "Splits": [
        {
            "Weight": 90,
            "NextNode": "resolver:v1.web.default.dc2"
        },
        {
            "Weight": 10,
            "NextNode": "resolver:v2.web.default.dc2"
        }
    ]
}
*/
/*
{
  "Chain": {
    "ServiceName": "http-1",
    "Namespace": "default",
    "Datacenter": "dc1",
    "Protocol": "http",
    "StartNode": "router:http-1",
    "Nodes": {
      "resolver:backend-service-1.default.dc1": {
        "Type": "resolver",
        "Name": "backend-service-1.default.dc1",
        "Resolver": {
          "ConnectTimeout": "5s",
          "Default": true,
          "Target": "backend-service-1.default.dc1"
        }
      },
      "resolver:backend-service-2.default.dc1": {
        "Type": "resolver",
        "Name": "backend-service-2.default.dc1",
        "Resolver": {
          "ConnectTimeout": "5s",
          "Default": true,
          "Target": "backend-service-2.default.dc1"
        }
      },
      "resolver:http-1.default.dc1": {
        "Type": "resolver",
        "Name": "http-1.default.dc1",
        "Resolver": {
          "ConnectTimeout": "5s",
          "Default": true,
          "Target": "http-1.default.dc1"
        }
      },
      "router:http-1": {
        "Type": "router",
        "Name": "http-1",
        "Routes": [
          {
            "Definition": {
              "Match": {
                "HTTP": {
                  "Header": [
                    {
                      "Name": ":authority",
                      "Prefix": "backend-service-1."
                    }
                  ]
                }
              },
              "Destination": {
                "Service": "backend-service-1"
              }
            },
            "NextNode": "resolver:backend-service-1.default.dc1"
          },
          {
            "Definition": {
              "Match": {
                "HTTP": {
                  "Header": [
                    {
                      "Name": ":authority",
                      "Prefix": "backend-service-2."
                    }
                  ]
                }
              },
              "Destination": {
                "Service": "backend-service-2"
              }
            },
            "NextNode": "resolver:backend-service-2.default.dc1"
          },
          {
            "Definition": {
              "Match": {
                "HTTP": {
                  "PathPrefix": "/"
                }
              },
              "Destination": {
                "Service": "http-1"
              }
            },
            "NextNode": "resolver:http-1.default.dc1"
          }
        ]
      }
    },
    "Targets": {
      "backend-service-1.default.dc1": {
        "ID": "backend-service-1.default.dc1",
        "Service": "backend-service-1",
        "Namespace": "default",
        "Datacenter": "dc1",
        "MeshGateway": {},
        "Subset": {},
        "SNI": "backend-service-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
        "Name": "backend-service-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
      },
      "backend-service-2.default.dc1": {
        "ID": "backend-service-2.default.dc1",
        "Service": "backend-service-2",
        "Namespace": "default",
        "Datacenter": "dc1",
        "MeshGateway": {},
        "Subset": {},
        "SNI": "backend-service-2.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
        "Name": "backend-service-2.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
      },
      "http-1.default.dc1": {
        "ID": "http-1.default.dc1",
        "Service": "http-1",
        "Namespace": "default",
        "Datacenter": "dc1",
        "MeshGateway": {},
        "Subset": {},
        "SNI": "http-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
        "Name": "http-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
      }
    }
  }
}
*/