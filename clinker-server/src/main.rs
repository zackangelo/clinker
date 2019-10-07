use log::{debug,info};

use clinker_consul::ConsulClient;

pub struct ClinkerDest;

impl ClinkerDest {
    pub fn new() -> ClinkerDest {
        ClinkerDest
    }
}

use clinker_gen::linkerd;
use linkerd::destination::server::{Destination, DestinationServer};
use linkerd::destination::{DestinationProfile, GetDestination, update, Update, NoEndpoints, WeightedAddrSet, WeightedAddr};
use tonic::{Request, Response, transport::Server, Status};
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_timer::Interval;
use std::time::Duration;
use std::collections::HashMap;
// use std::net;


#[tonic::async_trait]
impl Destination for ClinkerDest {
    //type GetStream: Stream<Item = Result<super::Update, tonic::Status>>
    // + Send
    // + 'static;
    type GetStream = mpsc::Receiver<Result<Update, Status>>;

    async fn get(
        &self,
        request: Request<GetDestination>,
    ) -> Result<Response<Self::GetStream>, Status> {
        info!("get request: {:?}", request);

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let empty = Update {
                update: None, // Some(update::Update::NoEndpoints(NoEndpoints { exists: true })),
            };

            let au_greeter_tcp: ::std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();

            let au_greeter_addr = WeightedAddr {
                addr: Some((&au_greeter_tcp).into()),
                weight: 1,
                metric_labels: HashMap::new(),
                tls_identity: None,
                protocol_hint: None,
            };

            let us_greeter_tcp: ::std::net::SocketAddr = "127.0.0.1:50052".parse().unwrap();

            let us_greeter_addr = WeightedAddr {
                addr: Some((&us_greeter_tcp).into()),
                weight: 1,
                metric_labels: HashMap::new(),
                tls_identity: None,
                protocol_hint: None,
            };

            let greeter_addr_set = WeightedAddrSet {
                addrs: vec![au_greeter_addr, us_greeter_addr],
                metric_labels: HashMap::new(),
            };

            let greeter_update = Update {
                update: Some(update::Update::Add(greeter_addr_set)),
            };

            let mut timer = Interval::new_interval(Duration::new(5,0));
            let mut i: u32 = 0;

            loop {
                // info!("sending empty response for get_dest...");

                let update: Update = if i == 0 {
                    i += 1;
                    greeter_update.clone()
                } else {
                    empty.clone()
                };

                info!("sending update: {:?}", update);

                let send_result = tx.send(Ok(update.clone())).await;

                match send_result {
                    Ok(_) => timer.next().await.unwrap(),
                    Err(err) => {
                        debug!("error sending stream frame to update listener: {}", err);
                        break; //allow channel to close
                    }
                };
            }
        });

        Ok(Response::new(rx))
    }


    // type GetProfileStream: Stream<Item = Result<super::DestinationProfile, tonic::Status>>
    //         + Send
    //         + 'static;
    type GetProfileStream = mpsc::Receiver<Result<DestinationProfile, Status>>;

    async fn get_profile(
        &self,
        request: Request<GetDestination>,
    ) -> Result<Response<Self::GetProfileStream>, Status> {
        info!("get_profile request: {:?}", request);

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            info!("sending empty profile...");

            let profile = DestinationProfile {
                routes: Vec::new(),
                retry_budget: None,
                dst_overrides: Vec::new(),
            };

            tx.send(Ok(profile)).await.unwrap()
        });

        Ok(Response::new(rx))
        // unimplemented!()
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = ::env_logger::init();
    info!("clinker control plane gRPC server started!");

    // let client = ConsulClient::new();
    // let dcs = client.datacenters().await.unwrap();
    // println!("dcs = {:?}", dcs);

    // let services = client.services().await.unwrap();
    // println!("services = {:?}", services);
    let addr = "127.0.0.1:5000".parse().unwrap();
    let dest_svc = ClinkerDest::new();
    Server::builder()
        .serve(addr, DestinationServer::new(dest_svc))
        .await?;
    Ok(())
}

// pub mod client_gen {
//     #![allow(unused_variables, dead_code, missing_docs)]
//     use tonic::codegen::*;
//     pub struct DestinationClient<T> {
//         inner: tonic::client::Grpc<T>,
//     }
//     impl DestinationClient<tonic::transport::Channel> {
//         pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
//         where
//             D: std::convert::TryInto<tonic::transport::Endpoint>,
//             D::Error: Into<StdError>,
//         {
//             tonic::transport::Channel::builder()
//                 .build(dst)
//                 .map(|c| Self::new(c))
//         }
//     }
//     impl<T> DestinationClient<T>
//     where
//         T: tonic::client::GrpcService<tonic::body::BoxBody>,
//         T::ResponseBody: Body + HttpBody + Send + 'static,
//         T::Error: Into<StdError>,
//         <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
//         <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
//     {
//         pub fn new(inner: T) -> Self {
//             let inner = tonic::client::Grpc::new(inner);
//             Self { inner }
//         }
//         pub async fn ready(&mut self) -> Result<(), tonic::Status> {
//             self.inner.ready().await.map_err(|e| {
//                 tonic::Status::new(
//                     tonic::Code::Unknown,
//                     format!("Service was not ready: {}", e.into()),
//                 )
//             })
//         }
//         pub async fn get(
//             &mut self,
//             request: tonic::Request<super::GetDestination>,
//         ) -> Result<tonic::Response<tonic::codec::Streaming<super::Update>>, tonic::Status>
//         {
//             self.ready().await?;
//             let codec = tonic::codec::ProstCodec::new();
//             let path = http::uri::PathAndQuery::from_static(
//                 "/io.linkerd.proxy.destination.Destination/Get",
//             );
//             self.inner.server_streaming(request, path, codec).await
//         }
//         pub async fn get_profile(
//             &mut self,
//             request: tonic::Request<super::GetDestination>,
//         ) -> Result<
//             tonic::Response<tonic::codec::Streaming<super::DestinationProfile>>,
//             tonic::Status,
//         > {
//             self.ready().await?;
//             let codec = tonic::codec::ProstCodec::new();
//             let path = http::uri::PathAndQuery::from_static(
//                 "/io.linkerd.proxy.destination.Destination/GetProfile",
//             );
//             self.inner.server_streaming(request, path, codec).await
//         }
//     }
//     impl<T: Clone> Clone for DestinationClient<T> {
//         fn clone(&self) -> Self {
//             Self {
//                 inner: self.inner.clone(),
//             }
//         }
//     }
// }
// pub mod server {
//     #![allow(unused_variables, dead_code, missing_docs)]
//     use tonic::codegen::*;
//     #[async_trait]
//     pub trait Destination: Send + Sync + 'static {
//         type GetStream: Stream<Item = Result<super::Update, tonic::Status>> + Send + 'static;
//         async fn get(
//             &self,
//             request: tonic::Request<super::GetDestination>,
//         ) -> Result<tonic::Response<Self::GetStream>, tonic::Status>;
//         type GetProfileStream: Stream<Item = Result<super::DestinationProfile, tonic::Status>>
//             + Send
//             + 'static;
//         async fn get_profile(
//             &self,
//             request: tonic::Request<super::GetDestination>,
//         ) -> Result<tonic::Response<Self::GetProfileStream>, tonic::Status>;
//     }
//     #[derive(Clone, Debug)]
//     pub struct DestinationServer<T: Destination> {
//         inner: Arc<T>,
//     }
//     #[derive(Clone, Debug)]
//     #[doc(hidden)]
//     pub struct DestinationServerSvc<T: Destination> {
//         inner: Arc<T>,
//     }
//     impl<T: Destination> DestinationServer<T> {
//         pub fn new(inner: T) -> Self {
//             let inner = Arc::new(inner);
//             Self::from_shared(inner)
//         }
//         pub fn from_shared(inner: Arc<T>) -> Self {
//             Self { inner }
//         }
//     }
//     impl<T: Destination> DestinationServerSvc<T> {
//         pub fn new(inner: Arc<T>) -> Self {
//             Self { inner }
//         }
//     }
//     impl<T: Destination, R> Service<R> for DestinationServer<T> {
//         type Response = DestinationServerSvc<T>;
//         type Error = Never;
//         type Future = Ready<Result<Self::Response, Self::Error>>;
//         fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//             Poll::Ready(Ok(()))
//         }
//         fn call(&mut self, _: R) -> Self::Future {
//             ok(DestinationServerSvc::new(self.inner.clone()))
//         }
//     }
//     impl<T: Destination> Service<http::Request<HyperBody>> for DestinationServerSvc<T> {
//         type Response = http::Response<tonic::body::BoxBody>;
//         type Error = Never;
//         type Future = BoxFuture<Self::Response, Self::Error>;
//         fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//             Poll::Ready(Ok(()))
//         }
//         fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
//             let inner = self.inner.clone();
//             match req.uri().path() {
//                 "/io.linkerd.proxy.destination.Destination/Get" => {
//                     struct Get<T: Destination>(pub Arc<T>);
//                     impl<T: Destination>
//                         tonic::server::ServerStreamingService<super::GetDestination> for Get<T>
//                     {
//                         type Response = super::Update;
//                         type ResponseStream = T::GetStream;
//                         type Future =
//                             BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
//                         fn call(
//                             &mut self,
//                             request: tonic::Request<super::GetDestination>,
//                         ) -> Self::Future {
//                             let inner = self.0.clone();
//                             let fut = async move { inner.get(request).await };
//                             Box::pin(fut)
//                         }
//                     }
//                     let inner = self.inner.clone();
//                     let fut = async move {
//                         let method = Get(inner);
//                         let codec = tonic::codec::ProstCodec::new();
//                         let mut grpc = tonic::server::Grpc::new(codec);
//                         let res = grpc.server_streaming(method, req).await;
//                         Ok(res)
//                     };
//                     Box::pin(fut)
//                 }
//                 "/io.linkerd.proxy.destination.Destination/GetProfile" => {
//                     struct GetProfile<T: Destination>(pub Arc<T>);
//                     impl<T: Destination>
//                         tonic::server::ServerStreamingService<super::GetDestination>
//                         for GetProfile<T>
//                     {
//                         type Response = super::DestinationProfile;
//                         type ResponseStream = T::GetProfileStream;
//                         type Future =
//                             BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
//                         fn call(
//                             &mut self,
//                             request: tonic::Request<super::GetDestination>,
//                         ) -> Self::Future {
//                             let inner = self.0.clone();
//                             let fut = async move { inner.get_profile(request).await };
//                             Box::pin(fut)
//                         }
//                     }
//                     let inner = self.inner.clone();
//                     let fut = async move {
//                         let method = GetProfile(inner);
//                         let codec = tonic::codec::ProstCodec::new();
//                         let mut grpc = tonic::server::Grpc::new(codec);
//                         let res = grpc.server_streaming(method, req).await;
//                         Ok(res)
//                     };
//                     Box::pin(fut)
//                 }
//                 _ => unimplemented!(),
//             }
//         }
//     }
// }
