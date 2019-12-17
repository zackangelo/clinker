use log::{debug,info};
use clinker_consul::discovery_chain::*;
use crate::linkerd::destination::server::{Destination, DestinationServer};
use crate::linkerd::destination::{RetryBudget, Route as LinkerdRoute, DestinationProfile, GetDestination, update, Update, NoEndpoints, WeightedAddrSet, WeightedAddr};


struct ProfileState {
    pub routes: Vec<LinkerdRoute>,
}

impl ProfileState { }

struct ProfileBuilder {
    chain: DiscoveryChain,
}

impl ProfileBuilder {
    pub fn new(chain: DiscoveryChain) -> ProfileBuilder {
        ProfileBuilder {
            chain,
        }
    }

    /// Processes a resolver node in the chain. The resolver
    /// is a terminal node (i.e., the resolution process
    /// ends with this node).
    fn process_resolver_node(&self, state: &mut ProfileState, node: &ChainNode) {

    }

    fn process_splitter_node(&self, state: &mut ProfileState, node: &ChainNode) {

    }

    fn process_router_node(&self, state: &mut ProfileState, node: &ChainNode) {
        debug!("processing router node: {:?}", node);

        match &node.Routes {
            Some(routes) => {
                for route in routes {
                    debug!("processing route: {:?}", route);

                }
            },
            None => panic!("no routes in router node"),
        }
    }

    fn process_node(&self, state: &mut ProfileState, node: &ChainNode) {
        debug!("processing chain node: {:?}", node);

        match node.Type.as_str() {
            "resolver" => self.process_resolver_node(state, node),
            "splitter" => self.process_splitter_node(state, node),
            "router" => self.process_router_node(state, node),
            unk => panic!("unknown discovery chain node type: {}", unk),
        }
    }

    pub fn build_profile(&mut self) -> DestinationProfile {
        let start_node = self.chain.Nodes.get(&self.chain.StartNode).unwrap();
        let mut state = ProfileState {
            routes: Vec::new(),
        };

        self.process_node(&mut state, start_node);

        let routes = state.routes.clone();

        let retry_budget = RetryBudget {
            retry_ratio: 0.0,
            min_retries_per_second: 0,
            ttl: None,
        };

        DestinationProfile {
            routes,
            retry_budget: Some(retry_budget),
            dst_overrides: Vec::new(),
        }
    }
}

// {
//   "Chain": {
//     "ServiceName": "http-1",
//     "Namespace": "default",
//     "Datacenter": "dc1",
//     "Protocol": "http",
//     "StartNode": "router:http-1",
//     "Nodes": {
//       "resolver:backend-service-1.default.dc1": {
//         "Type": "resolver",
//         "Name": "backend-service-1.default.dc1",
//         "Resolver": {
//           "ConnectTimeout": "5s",
//           "Default": true,
//           "Target": "backend-service-1.default.dc1"
//         }
//       },
//       "resolver:backend-service-2.default.dc1": {
//         "Type": "resolver",
//         "Name": "backend-service-2.default.dc1",
//         "Resolver": {
//           "ConnectTimeout": "5s",
//           "Default": true,
//           "Target": "backend-service-2.default.dc1"
//         }
//       },
//       "resolver:http-1.default.dc1": {
//         "Type": "resolver",
//         "Name": "http-1.default.dc1",
//         "Resolver": {
//           "ConnectTimeout": "5s",
//           "Default": true,
//           "Target": "http-1.default.dc1"
//         }
//       },
//       "router:http-1": {
//         "Type": "router",
//         "Name": "http-1",
//         "Routes": [
//           {
//             "Definition": {
//               "Match": {
//                 "HTTP": {
//                   "Header": [
//                     {
//                       "Name": ":authority",
//                       "Prefix": "backend-service-1."
//                     }
//                   ]
//                 }
//               },
//               "Destination": {
//                 "Service": "backend-service-1"
//               }
//             },
//             "NextNode": "resolver:backend-service-1.default.dc1"
//           },
//           {
//             "Definition": {
//               "Match": {
//                 "HTTP": {
//                   "Header": [
//                     {
//                       "Name": ":authority",
//                       "Prefix": "backend-service-2."
//                     }
//                   ]
//                 }
//               },
//               "Destination": {
//                 "Service": "backend-service-2"
//               }
//             },
//             "NextNode": "resolver:backend-service-2.default.dc1"
//           },
//           {
//             "Definition": {
//               "Match": {
//                 "HTTP": {
//                   "PathPrefix": "/"
//                 }
//               },
//               "Destination": {
//                 "Service": "http-1"
//               }
//             },
//             "NextNode": "resolver:http-1.default.dc1"
//           }
//         ]
//       }
//     },
//     "Targets": {
//       "backend-service-1.default.dc1": {
//         "ID": "backend-service-1.default.dc1",
//         "Service": "backend-service-1",
//         "Namespace": "default",
//         "Datacenter": "dc1",
//         "MeshGateway": {},
//         "Subset": {},
//         "SNI": "backend-service-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
//         "Name": "backend-service-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
//       },
//       "backend-service-2.default.dc1": {
//         "ID": "backend-service-2.default.dc1",
//         "Service": "backend-service-2",
//         "Namespace": "default",
//         "Datacenter": "dc1",
//         "MeshGateway": {},
//         "Subset": {},
//         "SNI": "backend-service-2.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
//         "Name": "backend-service-2.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
//       },
//       "http-1.default.dc1": {
//         "ID": "http-1.default.dc1",
//         "Service": "http-1",
//         "Namespace": "default",
//         "Datacenter": "dc1",
//         "MeshGateway": {},
//         "Subset": {},
//         "SNI": "http-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul",
//         "Name": "http-1.default.dc1.internal.2d4d763e-9cb0-7b79-cfe7-e2f5992fe6df.consul"
//       }
//     }
//   }
// }