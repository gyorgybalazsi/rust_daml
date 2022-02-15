#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod reset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to reset the ledger state. The goal here is to be able to reset the state in a way"]
    #[doc = " that's much faster compared to restarting the whole ledger application (be it a sandbox"]
    #[doc = " or the real ledger server)."]
    #[doc = ""]
    #[doc = " Note that *all* state present in the ledger implementation will be reset, most importantly"]
    #[doc = " including the ledger ID. This means that clients will have to re-fetch the ledger ID"]
    #[doc = " from the identity service after hitting this endpoint."]
    #[doc = ""]
    #[doc = " The semantics are as follows:"]
    #[doc = ""]
    #[doc = " * When the reset service returns the reset is initiated, but not completed;"]
    #[doc = " * While the reset is performed, the ledger will not accept new requests. In fact we guarantee"]
    #[doc = "   that ledger stops accepting new requests by the time the response to Reset is delivered;"]
    #[doc = " * In-flight requests might be aborted, we make no guarantees on when or how quickly this"]
    #[doc = "   happens;"]
    #[doc = " * The ledger might be unavailable for a period of time before the reset is complete."]
    #[doc = ""]
    #[doc = " Given the above, the recommended mode of operation for clients of the reset endpoint is to"]
    #[doc = " call it, then call the ledger identity endpoint in a retry loop that will tolerate a brief"]
    #[doc = " window when the ledger is down, and resume operation as soon as the new ledger ID is delivered."]
    #[doc = ""]
    #[doc = " Note that this service will be available on the sandbox and might be available in some other testing"]
    #[doc = " environments, but will *never* be available in production."]
    #[derive(Debug, Clone)]
    pub struct ResetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResetServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ResetServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ResetServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ResetServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Resets the ledger state. Note that loaded DARs won't be removed -- this only rolls back the"]
        #[doc = " ledger to genesis."]
        pub async fn reset(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.testing.ResetService/Reset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTimeRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTimeResponse {
    /// The current time according to the ledger server.
    #[prost(message, optional, tag = "1")]
    pub current_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTimeRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// MUST precisely match the current time as it's known to the ledger server.
    #[prost(message, optional, tag = "2")]
    pub current_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the client wants to set on the ledger.
    /// MUST be a point int time after ``current_time``.
    #[prost(message, optional, tag = "3")]
    pub new_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod time_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Optional service, exposed for testing static time scenarios."]
    #[derive(Debug, Clone)]
    pub struct TimeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TimeServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TimeServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TimeServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TimeServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Returns a stream of time updates."]
        #[doc = " Always returns at least one response, where the first one is the current time."]
        #[doc = " Subsequent responses are emitted whenever the ledger server's time is updated."]
        pub async fn get_time(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::GetTimeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.testing.TimeService/GetTime",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Allows clients to change the ledger's clock in an atomic get-and-set operation."]
        #[doc = " Errors:"]
        #[doc = " - ``INVALID_ARGUMENT``: if ``current_time`` is invalid (it MUST precisely match the current time as it's known to the ledger server)"]
        pub async fn set_time(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTimeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.testing.TimeService/SetTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
