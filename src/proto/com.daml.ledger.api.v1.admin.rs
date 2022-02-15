#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTimeModelRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTimeModelResponse {
    /// The current configuration generation. The generation is a monotonically increasing
    /// integer that is incremented on each change. Used when setting the time model.
    #[prost(int64, tag = "1")]
    pub configuration_generation: i64,
    /// The current ledger time model.
    #[prost(message, optional, tag = "2")]
    pub time_model: ::core::option::Option<TimeModel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTimeModelRequest {
    /// Submission identifier used for tracking the request and to reject
    /// duplicate submissions.
    /// Required.
    #[prost(string, tag = "1")]
    pub submission_id: ::prost::alloc::string::String,
    /// Deadline for the configuration change after which the change is rejected.
    #[prost(message, optional, tag = "2")]
    pub maximum_record_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current configuration generation which we're submitting the change against.
    /// This is used to perform a compare-and-swap of the configuration to
    /// safeguard against concurrent modifications.
    /// Required.
    #[prost(int64, tag = "3")]
    pub configuration_generation: i64,
    /// The new time model that replaces the current one.
    /// Required.
    #[prost(message, optional, tag = "4")]
    pub new_time_model: ::core::option::Option<TimeModel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTimeModelResponse {
    /// The configuration generation of the committed time model.
    #[prost(int64, tag = "1")]
    pub configuration_generation: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeModel {
    /// The expected average latency of a transaction, i.e., the average time
    /// from submitting the transaction to a \[[WriteService]\] and the transaction
    /// being assigned a record time.
    /// Required.
    #[prost(message, optional, tag = "4")]
    pub avg_transaction_latency: ::core::option::Option<::prost_types::Duration>,
    /// The minimimum skew between ledger time and record time: lt_TX >= rt_TX - minSkew
    /// Required.
    #[prost(message, optional, tag = "5")]
    pub min_skew: ::core::option::Option<::prost_types::Duration>,
    /// The maximum skew between ledger time and record time: lt_TX <= rt_TX + maxSkew
    /// Required.
    #[prost(message, optional, tag = "6")]
    pub max_skew: ::core::option::Option<::prost_types::Duration>,
}
#[doc = r" Generated client implementations."]
pub mod config_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Status: experimental interface, will change before it is deemed production"]
    #[doc = " ready"]
    #[doc = ""]
    #[doc = " The ledger configuration management service provides methods for the ledger administrator"]
    #[doc = " to change the current ledger configuration. The services provides methods to modify"]
    #[doc = " different aspects of the configuration."]
    #[derive(Debug, Clone)]
    pub struct ConfigManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConfigManagementServiceClient<tonic::transport::Channel> {
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
    impl<T> ConfigManagementServiceClient<T>
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
        ) -> ConfigManagementServiceClient<InterceptedService<T, F>>
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
            ConfigManagementServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Return the currently active time model and the current configuration generation."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn get_time_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTimeModelRequest>,
        ) -> Result<tonic::Response<super::GetTimeModelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.ConfigManagementService/GetTimeModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set the ledger time model."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``INVALID_ARGUMENT``: if arguments are invalid, or the provided configuration generation"]
        #[doc = "   does not match the current active configuration generation. The caller is expected"]
        #[doc = "   to retry by again fetching current time model using 'GetTimeModel', applying changes"]
        #[doc = "   and resubmitting."]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request times out. Note that a timed out request may"]
        #[doc = "   have still been committed to the ledger. Application should re-query the current"]
        #[doc = "   time model before retrying."]
        #[doc = " - ``FAILED_PRECONDITION``: if the request is rejected."]
        #[doc = " - ``UNIMPLEMENTED``: if this method is not supported by the backing ledger."]
        pub async fn set_time_model(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTimeModelRequest>,
        ) -> Result<tonic::Response<super::SetTimeModelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.ConfigManagementService/SetTimeModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnownPackagesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnownPackagesResponse {
    /// The details of all Daml-LF packages known to backing participant.
    /// Required
    #[prost(message, repeated, tag = "1")]
    pub package_details: ::prost::alloc::vec::Vec<PackageDetails>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageDetails {
    /// The identity of the Daml-LF package.
    /// Must be a valid PackageIdString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub package_id: ::prost::alloc::string::String,
    /// Size of the package in bytes.
    /// The size of the package is given by the size of the ``daml_lf``
    /// ArchivePayload. See further details in ``daml_lf.proto``.
    /// Required
    #[prost(uint64, tag = "2")]
    pub package_size: u64,
    /// Indicates since when the package is known to the backing participant.
    /// Required
    #[prost(message, optional, tag = "3")]
    pub known_since: ::core::option::Option<::prost_types::Timestamp>,
    /// Description provided by the backing participant describing where
    /// it got the package from.
    /// Optional
    #[prost(string, tag = "4")]
    pub source_description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadDarFileRequest {
    /// Contains a Daml archive DAR file, which in turn is a jar like zipped
    /// container for ``daml_lf`` archives. See further details in
    /// ``daml_lf.proto``.
    /// Required
    #[prost(bytes = "vec", tag = "1")]
    pub dar_file: ::prost::alloc::vec::Vec<u8>,
    /// Unique submission identifier.
    /// Optional, defaults to a random identifier.
    #[prost(string, tag = "2")]
    pub submission_id: ::prost::alloc::string::String,
}
/// An empty message that is received when the upload operation succeeded.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadDarFileResponse {}
#[doc = r" Generated client implementations."]
pub mod package_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Query the Daml-LF packages supported by the ledger participant and upload"]
    #[doc = " DAR files. We use 'backing participant' to refer to this specific participant"]
    #[doc = " in the methods of this API."]
    #[derive(Debug, Clone)]
    pub struct PackageManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PackageManagementServiceClient<tonic::transport::Channel> {
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
    impl<T> PackageManagementServiceClient<T>
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
        ) -> PackageManagementServiceClient<InterceptedService<T, F>>
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
            PackageManagementServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the details of all Daml-LF packages known to the backing participant."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn list_known_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKnownPackagesRequest>,
        ) -> Result<tonic::Response<super::ListKnownPackagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PackageManagementService/ListKnownPackages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Upload a DAR file to the backing participant."]
        #[doc = " Depending on the ledger implementation this might also make the package "]
        #[doc = " available on the whole ledger. This call might not be supported by some "]
        #[doc = " ledger implementations. Canton could be an example, where uploading a DAR"]
        #[doc = " is not sufficient to render it usable, it must be activated first."]
        #[doc = " This call may:"]
        #[doc = " - Succeed, if the package was successfully uploaded, or if the same package"]
        #[doc = "   was already uploaded before."]
        #[doc = " - Respond with a gRPC error"]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``UNIMPLEMENTED``: if DAR package uploading is not supported by the backing participant"]
        #[doc = " - ``INVALID_ARGUMENT``: if the DAR file is too big or malformed. The maximum supported size is implementation specific."]
        pub async fn upload_dar_file(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadDarFileRequest>,
        ) -> Result<tonic::Response<super::UploadDarFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PackageManagementService/UploadDarFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PruneRequest {
    /// Inclusive offset up to which the ledger is to be pruned.
    /// By default the following data is pruned:
    ///   1. All normal and divulged contracts that have been archived before
    ///   `prune_up_to`.
    ///   2. All transaction events and completions before `prune_up_to`
    #[prost(string, tag = "1")]
    pub prune_up_to: ::prost::alloc::string::String,
    /// Unique submission identifier.
    /// Optional, defaults to a random identifier, used for logging.
    #[prost(string, tag = "2")]
    pub submission_id: ::prost::alloc::string::String,
    /// Prune all immediately and retroactively divulged contracts created before `prune_up_to`
    /// independent of whether they were archived before `prune_up_to`. Useful to avoid leaking
    /// storage on participant nodes that can see a divulged contract but not its archival.
    ///
    /// Application developers SHOULD write their Daml applications
    /// such that they do not rely on divulged contracts; i.e., no warnings from
    /// using divulged contracts as inputs to transactions are emitted.
    ///
    /// Participant node operators SHOULD set the `prune_all_divulged_contracts` flag to avoid leaking
    /// storage due to accumulating unarchived divulged contracts PROVIDED that:
    ///   1. no application using this participant node relies on divulgence OR
    ///   2. divulged contracts on which applications rely have been re-divulged after the `prune_up_to` offset.
    #[prost(bool, tag = "3")]
    pub prune_all_divulged_contracts: bool,
}
/// Empty for now, but may contain fields in the future
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PruneResponse {}
#[doc = r" Generated client implementations."]
pub mod participant_pruning_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Prunes/truncates the \"oldest\" transactions from the participant (the participant Ledger Api Server plus any other"]
    #[doc = " participant-local state) by removing a portion of the ledger in such a way that the set of future, allowed"]
    #[doc = " commands are not affected."]
    #[doc = ""]
    #[doc = " This enables:"]
    #[doc = " 1. keeping the \"inactive\" portion of the ledger to a manageable size and"]
    #[doc = " 2. removing inactive state to honor the right to be forgotten."]
    #[derive(Debug, Clone)]
    pub struct ParticipantPruningServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParticipantPruningServiceClient<tonic::transport::Channel> {
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
    impl<T> ParticipantPruningServiceClient<T>
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
        ) -> ParticipantPruningServiceClient<InterceptedService<T, F>>
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
            ParticipantPruningServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Prune the ledger specifying the offset before and at which ledger transactions should be removed. Only returns when"]
        #[doc = " the potentially long-running prune request ends successfully or with one of the following errors:"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload, particularly the offset is malformed or missing"]
        #[doc = " - ``UNIMPLEMENTED``: if the participant is based on a ledger that has not implemented pruning"]
        #[doc = " - ``INTERNAL``: if the participant has encountered a failure and has potentially applied pruning partially. Such cases"]
        #[doc = "   warrant verifying the participant health before retrying the prune with the same (or a larger, valid) offset."]
        #[doc = "   Successful retries after such errors ensure that different components reach a consistent pruning state."]
        #[doc = " - ``FAILED_PRECONDITION``: if the participant is not yet able to prune at the specified offset."]
        #[doc = ""]
        pub async fn prune(
            &mut self,
            request: impl tonic::IntoRequest<super::PruneRequest>,
        ) -> Result<tonic::Response<super::PruneResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.ParticipantPruningService/Prune",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantIdRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParticipantIdResponse {
    /// Identifier of the participant, which SHOULD be globally unique.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    #[prost(string, tag = "1")]
    pub participant_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartiesRequest {
    /// The stable, unique identifier of the Daml parties.
    /// Must be valid PartyIdStrings (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "1")]
    pub parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartiesResponse {
    /// The details of the requested Daml parties by the participant, if known.
    /// The party details may not be in the same order as requested.
    /// Required
    #[prost(message, repeated, tag = "1")]
    pub party_details: ::prost::alloc::vec::Vec<PartyDetails>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnownPartiesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKnownPartiesResponse {
    /// The details of all Daml parties known by the participant.
    /// Required
    #[prost(message, repeated, tag = "1")]
    pub party_details: ::prost::alloc::vec::Vec<PartyDetails>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocatePartyRequest {
    /// A hint to the backing participant which party ID to allocate. It can be
    /// ignored.
    /// Must be a valid PartyIdString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "1")]
    pub party_id_hint: ::prost::alloc::string::String,
    /// Human-readable name of the party to be added to the participant. It doesn't
    /// have to be unique.
    /// Optional
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocatePartyResponse {
    #[prost(message, optional, tag = "1")]
    pub party_details: ::core::option::Option<PartyDetails>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartyDetails {
    /// The stable unique identifier of a Daml party.
    /// Must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub party: ::prost::alloc::string::String,
    /// Human readable name associated with the party. Caution, it might not be
    /// unique.
    /// Optional
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// true if party is hosted by the backing participant.
    /// Required
    #[prost(bool, tag = "3")]
    pub is_local: bool,
}
#[doc = r" Generated client implementations."]
pub mod party_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Inspect the party management state of a ledger participant and modify the"]
    #[doc = " parts that are modifiable. We use 'backing participant' to refer to this"]
    #[doc = " specific participant in the methods of this API."]
    #[derive(Debug, Clone)]
    pub struct PartyManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PartyManagementServiceClient<tonic::transport::Channel> {
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
    impl<T> PartyManagementServiceClient<T>
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
        ) -> PartyManagementServiceClient<InterceptedService<T, F>>
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
            PartyManagementServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Return the identifier of the backing participant."]
        #[doc = " All horizontally scaled replicas should return the same id."]
        #[doc = " daml-on-sql: returns an identifier supplied on command line at launch time"]
        #[doc = " daml-on-kv-ledger: as above"]
        #[doc = " canton: returns globally unique identifier of the backing participant"]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn get_participant_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParticipantIdRequest>,
        ) -> Result<tonic::Response<super::GetParticipantIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PartyManagementService/GetParticipantId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the party details of the given parties. Only known parties will be"]
        #[doc = " returned in the list."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn get_parties(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPartiesRequest>,
        ) -> Result<tonic::Response<super::GetPartiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PartyManagementService/GetParties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the parties known by the backing participant."]
        #[doc = " The list returned contains parties whose ledger access is facilitated by"]
        #[doc = " backing participant and the ones maintained elsewhere."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn list_known_parties(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKnownPartiesRequest>,
        ) -> Result<tonic::Response<super::ListKnownPartiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PartyManagementService/ListKnownParties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a new party to the set managed by the backing participant."]
        #[doc = " Caller specifies a party identifier suggestion, the actual identifier"]
        #[doc = " allocated might be different and is implementation specific."]
        #[doc = " This call may:"]
        #[doc = " - Succeed, in which case the actual allocated identifier is visible in"]
        #[doc = "   the response."]
        #[doc = " - Respond with a gRPC error"]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``UNIMPLEMENTED``: if synchronous party allocation is not supported by the backing participant"]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request times out"]
        #[doc = " - ``INVALID_ARGUMENT``: if the provided hint and/or display name is invalid on the given ledger (see below)."]
        #[doc = " daml-on-sql: suggestion's uniqueness is checked and call rejected if the identifier is already present"]
        #[doc = " daml-on-kv-ledger: suggestion's uniqueness is checked by the validators in"]
        #[doc = " the consensus layer and call rejected if the identifier is already present."]
        #[doc = " canton: completely different globally unique identifier is allocated."]
        #[doc = " Behind the scenes calls to an internal protocol are made. As that protocol"]
        #[doc = " is richer than the surface protocol, the arguments take implicit values"]
        #[doc = " The party identifier suggestion must be a valid party name. Party names are required to be non-empty US-ASCII strings built from letters, digits, space,"]
        #[doc = " colon, minus and underscore limited to 255 chars"]
        pub async fn allocate_party(
            &mut self,
            request: impl tonic::IntoRequest<super::AllocatePartyRequest>,
        ) -> Result<tonic::Response<super::AllocatePartyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.admin.PartyManagementService/AllocateParty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
