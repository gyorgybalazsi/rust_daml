/// Encodes values that the ledger accepts as command arguments and emits as contract arguments.
///
/// The values encoding use different four classes of strings as identifiers. Those classes are
/// defined as follow:
/// - NameStrings are strings that match the regexp ``\[A-Za-z\$_][A-Za-z0-9\$_\]*``.
/// - PackageIdStrings are strings that match the regexp ``[A-Za-z0-9\-_ ]+``.
/// - PartyIdStrings are strings that match the regexp ``[A-Za-z0-9:\-_ ]+``.
/// - LedgerStrings are strings that match the regexp ``[A-Za-z0-9#:\-_/ ]+``.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(
        oneof = "value::Sum",
        tags = "1, 2, 3, 4, 5, 6, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18"
    )]
    pub sum: ::core::option::Option<value::Sum>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        Record(super::Record),
        #[prost(message, tag = "2")]
        Variant(::prost::alloc::boxed::Box<super::Variant>),
        /// Identifier of an on-ledger contract. Commands which reference an unknown or already archived contract ID will fail.
        /// Must be a valid LedgerString.
        #[prost(string, tag = "3")]
        ContractId(::prost::alloc::string::String),
        /// Represents a homogeneous list of values.
        #[prost(message, tag = "4")]
        List(super::List),
        #[prost(sint64, tag = "5")]
        Int64(i64),
        /// A Numeric, that is a decimal value with precision 38 (at most 38 significant digits) and a
        /// scale between 0 and 37 (significant digits on the right of the decimal point).
        /// The field has to match the regex
        ///   \[+-\]?\d{1,38}(.\d{0,37})?
        /// and should be representable by a Numeric without loss of precision.
        #[prost(string, tag = "6")]
        Numeric(::prost::alloc::string::String),
        /// A string.
        #[prost(string, tag = "8")]
        Text(::prost::alloc::string::String),
        /// Microseconds since the UNIX epoch. Can go backwards. Fixed
        /// since the vast majority of values will be greater than
        /// 2^28, since currently the number of microseconds since the
        /// epoch is greater than that. Range: 0001-01-01T00:00:00Z to
        /// 9999-12-31T23:59:59.999999Z, so that we can convert to/from
        /// <https://www.ietf.org/rfc/rfc3339.txt>
        #[prost(sfixed64, tag = "9")]
        Timestamp(i64),
        /// An agent operating on the ledger.
        /// Must be a valid PartyIdString.
        #[prost(string, tag = "11")]
        Party(::prost::alloc::string::String),
        /// True or false.
        #[prost(bool, tag = "12")]
        Bool(bool),
        /// This value is used for example for choices that don't take any arguments.
        #[prost(message, tag = "13")]
        Unit(()),
        /// Days since the unix epoch. Can go backwards. Limited from
        /// 0001-01-01 to 9999-12-31, also to be compatible with
        /// <https://www.ietf.org/rfc/rfc3339.txt>
        #[prost(int32, tag = "14")]
        Date(i32),
        /// The Optional type, None or Some
        #[prost(message, tag = "15")]
        Optional(::prost::alloc::boxed::Box<super::Optional>),
        /// The Map type
        #[prost(message, tag = "16")]
        Map(super::Map),
        /// The Enum type
        #[prost(message, tag = "17")]
        Enum(super::Enum),
        /// The GenMap type
        #[prost(message, tag = "18")]
        GenMap(super::GenMap),
    }
}
/// Contains nested values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    /// Omitted from the transaction stream when verbose streaming is not enabled.
    /// Optional when submitting commands.
    #[prost(message, optional, tag = "1")]
    pub record_id: ::core::option::Option<Identifier>,
    /// The nested values of the record.
    /// Required
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<RecordField>,
}
/// A named nested value within a record.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordField {
    /// When reading a transaction stream, it's omitted if verbose streaming is not enabled.
    /// When submitting a commmand, it's optional:
    ///   - if all keys within a single record are present, the order in which fields appear does not matter. however, each key must appear exactly once.
    ///   - if any of the keys within a single record are omitted, the order of fields MUST match the order of declaration in the Daml template.
    /// Must be a valid NameString
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// A nested value of a record.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
/// Unique identifier of an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identifier {
    /// The identifier of the Daml package that contains the entity.
    /// Must be a valid PackageIdString.
    /// Required
    #[prost(string, tag = "1")]
    pub package_id: ::prost::alloc::string::String,
    // removed in favor of  ``module_name`` and ``entity_name``.
    /// The dot-separated module name of the identifier.
    /// Required
    #[prost(string, tag = "3")]
    pub module_name: ::prost::alloc::string::String,
    /// The dot-separated name of the entity (e.g. record, template, ...) within the module.
    /// Required
    #[prost(string, tag = "4")]
    pub entity_name: ::prost::alloc::string::String,
}
/// A value with alternative representations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variant {
    /// Omitted from the transaction stream when verbose streaming is not enabled.
    /// Optional when submitting commands.
    #[prost(message, optional, tag = "1")]
    pub variant_id: ::core::option::Option<Identifier>,
    /// Determines which of the Variant's alternatives is encoded in this message.
    /// Must be a valid NameString.
    /// Required
    #[prost(string, tag = "2")]
    pub constructor: ::prost::alloc::string::String,
    /// The value encoded within the Variant.
    /// Required
    #[prost(message, optional, boxed, tag = "3")]
    pub value: ::core::option::Option<::prost::alloc::boxed::Box<Value>>,
}
// // A builtin exception value
// message BuiltinException {
//
//   // Determines the kind of builtin exception: ArithmeticError, GeneralError etc
//   // Required
//   string tag = 1;
//
//   // The value encoded within the Variant.
//   // Required
//   Value value = 2;
// }

/// A value with finite set of alternative representations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Enum {
    /// Omitted from the transaction stream when verbose streaming is not enabled.
    /// Optional when submitting commands.
    #[prost(message, optional, tag = "1")]
    pub enum_id: ::core::option::Option<Identifier>,
    /// Determines which of the Variant's alternatives is encoded in this message.
    /// Must be a valid NameString.
    /// Required
    #[prost(string, tag = "2")]
    pub constructor: ::prost::alloc::string::String,
}
/// A homogenous collection of values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    /// The elements must all be of the same concrete value type.
    /// Optional
    #[prost(message, repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<Value>,
}
/// Corresponds to Java's Optional type, Scala's Option, and Haskell's Maybe.
/// The reason why we need to wrap this in an additional ``message`` is that we
/// need to be able to encode the ``None`` case in the ``Value`` oneof.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Optional {
    /// optional
    #[prost(message, optional, boxed, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::boxed::Box<Value>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Map {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<map::Entry>,
}
/// Nested message and enum types in `Map`.
pub mod map {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<super::Value>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenMap {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<gen_map::Entry>,
}
/// Nested message and enum types in `GenMap`.
pub mod gen_map {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(message, optional, tag = "1")]
        pub key: ::core::option::Option<super::Value>,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<super::Value>,
    }
}
/// An event in the flat transaction stream can either be the creation
/// or the archiving of a contract.
///
/// In the transaction service the events are restricted to the events
/// visible for the parties specified in the transaction filter. Each
/// event message type below contains a ``witness_parties`` field which
/// indicates the subset of the requested parties that can see the event
/// in question. In the flat transaction stream you'll only receive events
/// that have witnesses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(oneof = "event::Event", tags = "1, 3")]
    pub event: ::core::option::Option<event::Event>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "1")]
        Created(super::CreatedEvent),
        #[prost(message, tag = "3")]
        Archived(super::ArchivedEvent),
    }
}
/// Records that a contract has been created, and choices may now be exercised on it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatedEvent {
    /// The ID of this particular event.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub event_id: ::prost::alloc::string::String,
    /// The ID of the created contract.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub contract_id: ::prost::alloc::string::String,
    /// The template of the created contract.
    /// Required
    #[prost(message, optional, tag = "3")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The key of the created contract, if defined.
    /// Optional
    #[prost(message, optional, tag = "7")]
    pub contract_key: ::core::option::Option<Value>,
    /// The arguments that have been used to create the contract.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub create_arguments: ::core::option::Option<Record>,
    /// The parties that are notified of this event. When a ``CreatedEvent``
    /// is returned as part of a transaction tree, this will include all
    /// the parties specified in the ``TransactionFilter`` that are informees
    /// of the event. If served as part of a flat transaction those will
    /// be limited to all parties specified in the ``TransactionFilter`` that
    /// are stakeholders of the contract (i.e. either signatories or observers).
    /// Required
    #[prost(string, repeated, tag = "5")]
    pub witness_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The signatories for this contract as specified by the template.
    /// Required
    #[prost(string, repeated, tag = "8")]
    pub signatories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The observers for this contract as specified explicitly by the template or implicitly as choice controllers.
    /// This field never contains parties that are signatories.
    /// Required
    #[prost(string, repeated, tag = "9")]
    pub observers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The agreement text of the contract.
    /// We use StringValue to properly reflect optionality on the wire for backwards compatibility.
    /// This is necessary since the empty string is an acceptable (and in fact the default) agreement
    /// text, but also the default string in protobuf.
    /// This means a newer client works with an older sandbox seamlessly.
    /// Optional
    #[prost(message, optional, tag = "6")]
    pub agreement_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Records that a contract has been archived, and choices may no longer be exercised on it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchivedEvent {
    /// The ID of this particular event.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub event_id: ::prost::alloc::string::String,
    /// The ID of the archived contract.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub contract_id: ::prost::alloc::string::String,
    /// The template of the archived contract.
    /// Required
    #[prost(message, optional, tag = "3")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The parties that are notified of this event. For ``ArchivedEvent``s,
    /// these are the intersection of the stakeholders of the contract in
    /// question and the parties specified in the ``TransactionFilter``. The
    /// stakeholders are the union of the signatories and the observers of
    /// the contract.
    /// Each one of its elements must be a valid PartyIdString (as described
    /// in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "4")]
    pub witness_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Records that a choice has been exercised on a target contract.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExercisedEvent {
    /// The ID of this particular event.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub event_id: ::prost::alloc::string::String,
    /// The ID of the target contract.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub contract_id: ::prost::alloc::string::String,
    /// The template of the target contract.
    /// Required
    #[prost(message, optional, tag = "3")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The choice that's been exercised on the target contract.
    /// Must be a valid NameString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "5")]
    pub choice: ::prost::alloc::string::String,
    /// The argument the choice was made with.
    /// Required
    #[prost(message, optional, tag = "6")]
    pub choice_argument: ::core::option::Option<Value>,
    /// The parties that made the choice.
    /// Each element must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "7")]
    pub acting_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, the target contract may no longer be exercised.
    /// Required
    #[prost(bool, tag = "8")]
    pub consuming: bool,
    /// The parties that are notified of this event. The witnesses of an exercise
    /// node will depend on whether the exercise was consuming or not.
    /// If consuming, the witnesses are the union of the stakeholders and
    /// the actors.
    /// If not consuming, the witnesses are the union of the signatories and
    /// the actors. Note that the actors might not necessarily be observers
    /// and thus signatories. This is the case when the controllers of a
    /// choice are specified using "flexible controllers", using the
    /// ``choice ... controller`` syntax, and said controllers are not
    /// explicitly marked as observers.
    /// Each element must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "10")]
    pub witness_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// References to further events in the same transaction that appeared as a result of this ``ExercisedEvent``.
    /// It contains only the immediate children of this event, not all members of the subtree rooted at this node.
    /// Each element must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, repeated, tag = "11")]
    pub child_event_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The result of exercising the choice
    /// Required
    #[prost(message, optional, tag = "12")]
    pub exercise_result: ::core::option::Option<Value>,
}
/// Used for filtering Transaction and Active Contract Set streams.
/// Determines which on-ledger events will be served to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionFilter {
    /// Keys of the map determine which parties' on-ledger transactions are being queried.
    /// Values of the map determine which events are disclosed in the stream per party.
    /// At the minimum, a party needs to set an empty Filters message to receive any events.
    /// Each key must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(map = "string, message", tag = "1")]
    pub filters_by_party: ::std::collections::HashMap<::prost::alloc::string::String, Filters>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filters {
    /// If not set, no filters will be applied.
    /// Optional
    #[prost(message, optional, tag = "1")]
    pub inclusive: ::core::option::Option<InclusiveFilters>,
}
/// If no internal fields are set, no filters will be applied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InclusiveFilters {
    /// A collection of templates.
    /// SHOULD NOT contain duplicates.
    /// Required
    #[prost(message, repeated, tag = "1")]
    pub template_ids: ::prost::alloc::vec::Vec<Identifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractsRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// Templates to include in the served snapshot, per party.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub filter: ::core::option::Option<TransactionFilter>,
    /// If enabled, values served over the API will contain more information than strictly necessary to interpret the data.
    /// In particular, setting the verbose flag to true triggers the ledger to include labels for record fields.
    /// Optional
    #[prost(bool, tag = "3")]
    pub verbose: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveContractsResponse {
    /// Included in the last message.
    /// The client should start consuming the transactions endpoint with this offset.
    /// The format of this field is described in ``ledger_offset.proto``.
    /// Required
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    /// The workflow that created the contracts.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "2")]
    pub workflow_id: ::prost::alloc::string::String,
    /// The list of contracts that were introduced by the workflow with ``workflow_id`` at the offset.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(message, repeated, tag = "3")]
    pub active_contracts: ::prost::alloc::vec::Vec<CreatedEvent>,
}
#[doc = r" Generated client implementations."]
pub mod active_contracts_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to initialize themselves according to a fairly recent state of the ledger without reading through all transactions that were committed since the ledger's creation."]
    #[derive(Debug, Clone)]
    pub struct ActiveContractsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActiveContractsServiceClient<tonic::transport::Channel> {
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
    impl<T> ActiveContractsServiceClient<T>
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
        ) -> ActiveContractsServiceClient<InterceptedService<T, F>>
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
            ActiveContractsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns a stream of the latest snapshot of active contracts."]
        #[doc = " If there are no active contracts, the stream returns a single GetActiveContractsResponse message with the offset at which the snapshot has been taken."]
        #[doc = " Clients SHOULD use the offset in the last GetActiveContractsResponse message to continue streaming transactions with the transaction service."]
        #[doc = " Clients SHOULD NOT assume that the set of active contracts they receive reflects the state at the ledger end."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (filters by party cannot be empty)"]
        pub async fn get_active_contracts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActiveContractsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetActiveContractsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.ActiveContractsService/GetActiveContracts",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// A completion represents the status of a submitted command on the ledger: it can be successful or failed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Completion {
    /// The ID of the succeeded or failed command.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub command_id: ::prost::alloc::string::String,
    /// Identifies the exact type of the error.
    /// For example, malformed or double spend transactions will result in a ``INVALID_ARGUMENT`` status.
    /// Transactions with invalid time time windows (which may be valid at a later date) will result in an ``ABORTED`` error.
    /// Optional
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::super::super::google::rpc::Status>,
    /// The transaction_id of the transaction that resulted from the command with command_id.
    /// Only set for successfully executed commands.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "3")]
    pub transaction_id: ::prost::alloc::string::String,
    /// The application ID that was used for the submission, as described in ``commands.proto``.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional for historic completions where this data is not available.
    #[prost(string, tag = "4")]
    pub application_id: ::prost::alloc::string::String,
    /// The set of parties on whose behalf the commands were executed.
    /// Contains the union of ``party`` and ``act_as`` from ``commands.proto``.
    /// The order of the parties need not be the same as in the submission.
    /// Each element must be a valid PartyIdString (as described in ``value.proto``).
    /// Optional for historic completions where this data is not available.
    #[prost(string, repeated, tag = "5")]
    pub act_as: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The submission ID this completion refers to, as described in ``commands.proto``.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "6")]
    pub submission_id: ::prost::alloc::string::String,
    /// The actual deduplication window used for the submission, which is derived from
    /// ``Commands.deduplication_period``. The ledger may convert the deduplication period into other
    /// descriptions and extend the period in implementation-specified ways.
    ///
    /// Used to audit the deduplication guarantee described in ``commands.proto``.
    ///
    /// Optional; the deduplication guarantee applies even if the completion omits this field.
    #[prost(oneof = "completion::DeduplicationPeriod", tags = "8, 9")]
    pub deduplication_period: ::core::option::Option<completion::DeduplicationPeriod>,
}
/// Nested message and enum types in `Completion`.
pub mod completion {
    /// The actual deduplication window used for the submission, which is derived from
    /// ``Commands.deduplication_period``. The ledger may convert the deduplication period into other
    /// descriptions and extend the period in implementation-specified ways.
    ///
    /// Used to audit the deduplication guarantee described in ``commands.proto``.
    ///
    /// Optional; the deduplication guarantee applies even if the completion omits this field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeduplicationPeriod {
        /// Specifies the start of the deduplication period by a completion stream offset.
        ///
        /// Must be a valid LedgerString (as described in ``value.proto``).
        #[prost(string, tag = "8")]
        DeduplicationOffset(::prost::alloc::string::String),
        /// Specifies the length of the deduplication period.
        /// It is measured in record time of completions.
        ///
        /// Must be non-negative.
        #[prost(message, tag = "9")]
        DeduplicationDuration(::prost_types::Duration),
    }
}
/// Describes a specific point on the ledger.
///
/// The Ledger API endpoints that take offsets allow to specify portions
/// of the ledger that are relevant for the client to read.
///
/// Offsets returned by the Ledger API can be used as-is (e.g.
/// to keep track of processed transactions and provide a restart
/// point to use in case of need).
///
/// The format of absolute offsets is opaque to the client: no
/// client-side transformation of an offset is guaranteed
/// to return a meaningful offset.
///
/// The server implementation ensures internally that offsets
/// are lexicographically comparable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerOffset {
    #[prost(oneof = "ledger_offset::Value", tags = "1, 2")]
    pub value: ::core::option::Option<ledger_offset::Value>,
}
/// Nested message and enum types in `LedgerOffset`.
pub mod ledger_offset {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LedgerBoundary {
        /// Refers to the first transaction.
        LedgerBegin = 0,
        /// Refers to the currently last transaction, which is a moving target.
        LedgerEnd = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The format of this string is specific to the ledger and opaque to the client.
        #[prost(string, tag = "1")]
        Absolute(::prost::alloc::string::String),
        #[prost(enumeration = "LedgerBoundary", tag = "2")]
        Boundary(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionStreamRequest {
    /// Must correspond to the ledger id reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// Only completions of commands submitted with the same application_id will be visible in the stream.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub application_id: ::prost::alloc::string::String,
    /// Non-empty list of parties whose data should be included.
    /// Only completions of commands for which at least one of the ``act_as`` parties is in the given set of parties
    /// will be visible in the stream.
    /// Must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "3")]
    pub parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This field indicates the minimum offset for completions. This can be used to resume an earlier completion stream.
    /// This offset is exclusive: the response will only contain commands whose offset is strictly greater than this.
    /// Optional, if not set the ledger uses the current ledger end offset instead.
    #[prost(message, optional, tag = "4")]
    pub offset: ::core::option::Option<LedgerOffset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionStreamResponse {
    /// This checkpoint may be used to restart consumption.  The
    /// checkpoint is after any completions in this response.
    /// Optional
    #[prost(message, optional, tag = "1")]
    pub checkpoint: ::core::option::Option<Checkpoint>,
    /// If set, one or more completions.
    #[prost(message, repeated, tag = "2")]
    pub completions: ::prost::alloc::vec::Vec<Completion>,
}
/// Checkpoints may be used to:
///
/// * detect time out of commands.
/// * provide an offset which can be used to restart consumption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checkpoint {
    /// All commands with a maximum record time below this value MUST be considered lost if their completion has not arrived before this checkpoint.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub record_time: ::core::option::Option<::prost_types::Timestamp>,
    /// May be used in a subsequent CompletionStreamRequest to resume the consumption of this stream at a later time.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<LedgerOffset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionEndRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionEndResponse {
    /// This offset can be used in a CompletionStreamRequest message.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub offset: ::core::option::Option<LedgerOffset>,
}
#[doc = r" Generated client implementations."]
pub mod command_completion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to observe the status of their submissions."]
    #[doc = " Commands may be submitted via the Command Submission Service."]
    #[doc = " The on-ledger effects of their submissions are disclosed by the Transaction Service."]
    #[doc = ""]
    #[doc = " Commands may fail in 2 distinct manners:"]
    #[doc = ""]
    #[doc = " 1. Failure communicated synchronously in the gRPC error of the submission."]
    #[doc = " 2. Failure communicated asynchronously in a Completion, see ``completion.proto``."]
    #[doc = ""]
    #[doc = " Note that not only successfully submitted commands MAY produce a completion event. For example, the participant MAY"]
    #[doc = " choose to produce a completion event for a rejection of a duplicate command."]
    #[doc = ""]
    #[doc = " Clients that do not receive a successful completion about their submission MUST NOT assume that it was successful."]
    #[doc = " Clients SHOULD subscribe to the CompletionStream before starting to submit commands to prevent race conditions."]
    #[derive(Debug, Clone)]
    pub struct CommandCompletionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommandCompletionServiceClient<tonic::transport::Channel> {
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
    impl<T> CommandCompletionServiceClient<T>
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
        ) -> CommandCompletionServiceClient<InterceptedService<T, F>>
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
            CommandCompletionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Subscribe to command completion events."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        #[doc = " - ``FAILED_PRECONDITION``: if the ledger has been pruned after the subscription start offset"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``OUT_OF_RANGE``: if the absolute offset is after the end of the ledger"]
        pub async fn completion_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CompletionStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CompletionStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandCompletionService/CompletionStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Returns the offset after the latest completion."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        pub async fn completion_end(
            &mut self,
            request: impl tonic::IntoRequest<super::CompletionEndRequest>,
        ) -> Result<tonic::Response<super::CompletionEndResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandCompletionService/CompletionEnd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A composite command that groups multiple commands together.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commands {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// Identifier of the on-ledger workflow that this command is a part of.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "2")]
    pub workflow_id: ::prost::alloc::string::String,
    /// Uniquely identifies the application (or its part) that issued the command. This is used in tracing
    /// across different components and to let applications subscribe to their own submissions only.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "3")]
    pub application_id: ::prost::alloc::string::String,
    /// Uniquely identifies the command.
    /// The triple (application_id, party + act_as, command_id) constitutes the change ID for the intended ledger change,
    /// where party + act_as is interpreted as a set of party names.
    /// The change ID can be used for matching the intended ledger changes with all their completions.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "4")]
    pub command_id: ::prost::alloc::string::String,
    /// Party on whose behalf the command should be executed.
    /// If ledger API authorization is enabled, then the authorization metadata must authorize the sender of the request
    /// to act on behalf of the given party.
    /// Must be a valid PartyIdString (as described in ``value.proto``).
    /// Deprecated in favor of the ``act_as`` field. If both are set, then the effective list of parties on whose
    /// behalf the command should be executed is the union of all parties listed in ``party`` and ``act_as``.
    /// Optional
    #[prost(string, tag = "5")]
    pub party: ::prost::alloc::string::String,
    /// Individual elements of this atomic command. Must be non-empty.
    /// Required
    #[prost(message, repeated, tag = "8")]
    pub commands: ::prost::alloc::vec::Vec<Command>,
    /// Lower bound for the ledger time assigned to the resulting transaction.
    /// Note: The ledger time of a transaction is assigned as part of command interpretation.
    /// Use this property if you expect that command interpretation will take a considerate amount of time, such that by
    /// the time the resulting transaction is sequenced, its assigned ledger time is not valid anymore.
    /// Must not be set at the same time as min_ledger_time_rel.
    /// Optional
    #[prost(message, optional, tag = "10")]
    pub min_ledger_time_abs: ::core::option::Option<::prost_types::Timestamp>,
    /// Same as min_ledger_time_abs, but specified as a duration, starting from the time the command is received by the server.
    /// Must not be set at the same time as min_ledger_time_abs.
    /// Optional
    #[prost(message, optional, tag = "11")]
    pub min_ledger_time_rel: ::core::option::Option<::prost_types::Duration>,
    /// Set of parties on whose behalf the command should be executed.
    /// If ledger API authorization is enabled, then the authorization metadata must authorize the sender of the request
    /// to act on behalf of each of the given parties.
    /// This field supersedes the ``party`` field. The effective set of parties on whose behalf the command
    /// should be executed is the union of all parties listed in ``party`` and ``act_as``, which must be non-empty.
    /// Each element must be a valid PartyIdString (as described in ``value.proto``).
    /// Optional
    #[prost(string, repeated, tag = "12")]
    pub act_as: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Set of parties on whose behalf (in addition to all parties listed in ``act_as``) contracts can be retrieved.
    /// This affects Daml operations such as ``fetch``, ``fetchByKey``, ``lookupByKey``, ``exercise``, and ``exerciseByKey``.
    /// Note: A participant node of a Daml network can host multiple parties. Each contract present on the participant
    /// node is only visible to a subset of these parties. A command can only use contracts that are visible to at least
    /// one of the parties in ``act_as`` or ``read_as``. This visibility check is independent from the Daml authorization
    /// rules for fetch operations.
    /// If ledger API authorization is enabled, then the authorization metadata must authorize the sender of the request
    /// to read contract data on behalf of each of the given parties.
    /// Optional
    #[prost(string, repeated, tag = "13")]
    pub read_as: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A unique identifier to distinguish completions for different submissions with the same change ID.
    /// Typically a random UUID. Applications are expected to use a different UUID for each retry of a submission
    /// with the same change ID.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    ///
    /// If omitted, the participant or the committer may set a value of their choice.
    /// Optional
    #[prost(string, tag = "14")]
    pub submission_id: ::prost::alloc::string::String,
    /// Specifies the deduplication period for the change ID.
    /// If omitted, the participant will assume the configured maximum deduplication time (see
    /// ``ledger_configuration_service.proto``).
    #[prost(oneof = "commands::DeduplicationPeriod", tags = "9, 15")]
    pub deduplication_period: ::core::option::Option<commands::DeduplicationPeriod>,
}
/// Nested message and enum types in `Commands`.
pub mod commands {
    /// Specifies the deduplication period for the change ID.
    /// If omitted, the participant will assume the configured maximum deduplication time (see
    /// ``ledger_configuration_service.proto``).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeduplicationPeriod {
        /// Specifies the length of the deduplication period.
        /// Same semantics apply as for `deduplication_duration`.
        ///
        /// Must be non-negative. Must not exceed the maximum deduplication time (see
        /// ``ledger_configuration_service.proto``).
        #[prost(message, tag = "9")]
        DeduplicationTime(::prost_types::Duration),
        /// Specifies the length of the deduplication period.
        /// It is interpreted relative to the local clock at some point during the submission's processing.
        ///
        /// Must be non-negative. Must not exceed the maximum deduplication time (see
        /// ``ledger_configuration_service.proto``).
        #[prost(message, tag = "15")]
        DeduplicationDuration(::prost_types::Duration),
    }
}
/// A command can either create a new contract or exercise a choice on an existing contract.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(oneof = "command::Command", tags = "1, 2, 4, 3")]
    pub command: ::core::option::Option<command::Command>,
}
/// Nested message and enum types in `Command`.
pub mod command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "1")]
        Create(super::CreateCommand),
        #[prost(message, tag = "2")]
        Exercise(super::ExerciseCommand),
        #[prost(message, tag = "4")]
        ExerciseByKey(super::ExerciseByKeyCommand),
        #[prost(message, tag = "3")]
        CreateAndExercise(super::CreateAndExerciseCommand),
    }
}
/// Create a new contract instance based on a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommand {
    /// The template of contract the client wants to create.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The arguments required for creating a contract from this template.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub create_arguments: ::core::option::Option<Record>,
}
/// Exercise a choice on an existing contract.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExerciseCommand {
    /// The template of contract the client wants to exercise.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The ID of the contract the client wants to exercise upon.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub contract_id: ::prost::alloc::string::String,
    /// The name of the choice the client wants to exercise.
    /// Must be a valid NameString (as described in ``value.proto``)
    /// Required
    #[prost(string, tag = "3")]
    pub choice: ::prost::alloc::string::String,
    /// The argument for this choice.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub choice_argument: ::core::option::Option<Value>,
}
/// Exercise a choice on an existing contract specified by its key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExerciseByKeyCommand {
    /// The template of contract the client wants to exercise.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The key of the contract the client wants to exercise upon.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub contract_key: ::core::option::Option<Value>,
    /// The name of the choice the client wants to exercise.
    /// Must be a valid NameString (as described in ``value.proto``)
    /// Required
    #[prost(string, tag = "3")]
    pub choice: ::prost::alloc::string::String,
    /// The argument for this choice.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub choice_argument: ::core::option::Option<Value>,
}
/// Create a contract and exercise a choice on it in the same transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAndExerciseCommand {
    /// The template of the contract the client wants to create.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub template_id: ::core::option::Option<Identifier>,
    /// The arguments required for creating a contract from this template.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub create_arguments: ::core::option::Option<Record>,
    /// The name of the choice the client wants to exercise.
    /// Must be a valid NameString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "3")]
    pub choice: ::prost::alloc::string::String,
    /// The argument for this choice.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub choice_argument: ::core::option::Option<Value>,
}
/// Complete view of an on-ledger transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionTree {
    /// Assigned by the server. Useful for correlating logs.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    /// The ID of the command which resulted in this transaction. Missing for everyone except the submitting party.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "2")]
    pub command_id: ::prost::alloc::string::String,
    /// The workflow ID used in command submission. Only set if the ``workflow_id`` for the command was set.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "3")]
    pub workflow_id: ::prost::alloc::string::String,
    /// Ledger effective time.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub effective_at: ::core::option::Option<::prost_types::Timestamp>,
    /// The absolute offset. The format of this field is described in ``ledger_offset.proto``.
    /// Required
    #[prost(string, tag = "6")]
    pub offset: ::prost::alloc::string::String,
    /// Changes to the ledger that were caused by this transaction. Nodes of the transaction tree.
    /// Each key be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(map = "string, message", tag = "7")]
    pub events_by_id: ::std::collections::HashMap<::prost::alloc::string::String, TreeEvent>,
    /// Roots of the transaction tree.
    /// Each element must be a valid LedgerString (as describe in ``value.proto``).
    /// The elements are in the same order as the commands in the
    /// corresponding Commands object that triggered this transaction.
    /// Required
    #[prost(string, repeated, tag = "8")]
    pub root_event_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Each tree event message type below contains a ``witness_parties`` field which
/// indicates the subset of the requested parties that can see the event
/// in question.
///
/// Note that transaction trees might contain events with
/// _no_ witness parties, which were included simply because they were
/// children of events which have witnesses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeEvent {
    #[prost(oneof = "tree_event::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<tree_event::Kind>,
}
/// Nested message and enum types in `TreeEvent`.
pub mod tree_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Created(super::CreatedEvent),
        #[prost(message, tag = "2")]
        Exercised(super::ExercisedEvent),
    }
}
/// Filtered view of an on-ledger transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Assigned by the server. Useful for correlating logs.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    /// The ID of the command which resulted in this transaction. Missing for everyone except the submitting party.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "2")]
    pub command_id: ::prost::alloc::string::String,
    /// The workflow ID used in command submission.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Optional
    #[prost(string, tag = "3")]
    pub workflow_id: ::prost::alloc::string::String,
    /// Ledger effective time.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(message, optional, tag = "4")]
    pub effective_at: ::core::option::Option<::prost_types::Timestamp>,
    /// The collection of events.
    /// Only contains ``CreatedEvent`` or ``ArchivedEvent``.
    /// Required
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// The absolute offset. The format of this field is described in ``ledger_offset.proto``.
    /// Required
    #[prost(string, tag = "6")]
    pub offset: ::prost::alloc::string::String,
}
/// These commands are atomic, and will become transactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitAndWaitRequest {
    /// The commands to be submitted.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub commands: ::core::option::Option<Commands>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitAndWaitForTransactionIdResponse {
    /// The id of the transaction that resulted from the submitted command.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitAndWaitForTransactionResponse {
    /// The flat transaction that resulted from the submitted command.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitAndWaitForTransactionTreeResponse {
    /// The transaction tree that resulted from the submitted command.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<TransactionTree>,
}
#[doc = r" Generated client implementations."]
pub mod command_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Command Service is able to correlate submitted commands with completion data, identify timeouts, and return contextual"]
    #[doc = " information with each tracking result. This supports the implementation of stateless clients."]
    #[doc = ""]
    #[doc = " Note that submitted commands generally produce completion events as well, even in case a command gets rejected."]
    #[doc = " For example, the participant MAY choose to produce a completion event for a rejection of a duplicate command."]
    #[derive(Debug, Clone)]
    pub struct CommandServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommandServiceClient<tonic::transport::Channel> {
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
    impl<T> CommandServiceClient<T>
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
        ) -> CommandServiceClient<InterceptedService<T, F>>
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
            CommandServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Submits a single composite command and waits for its result."]
        #[doc = " Propagates the gRPC error of failed submissions including Daml interpretation errors."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or if a resource is missing (e.g. contract key)"]
        #[doc = " due to for example contention on resources"]
        #[doc = " - ``ALREADY_EXISTS`` if a resource is duplicated (e.g. contract key)"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``ABORTED``: if the number of in-flight commands reached the maximum (if a limit is configured)"]
        #[doc = " - ``FAILED_PRECONDITION``: on consistency errors (e.g. the contract key has changed since the submission)"]
        #[doc = " or if an interpretation error occurred"]
        #[doc = " - ``UNAVAILABLE``: if the participant is not yet ready to submit commands or if the service has been shut down."]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request failed to receive its completion within the predefined timeout."]
        pub async fn submit_and_wait(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitAndWaitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandService/SubmitAndWait",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submits a single composite command, waits for its result, and returns the transaction id."]
        #[doc = " Propagates the gRPC error of failed submissions including Daml interpretation errors."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or if a resource is missing (e.g. contract key)"]
        #[doc = " due to for example contention on resources"]
        #[doc = " - ``ALREADY_EXISTS`` if a resource is duplicated (e.g. contract key)"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``ABORTED``: if the number of in-flight commands reached the maximum (if a limit is configured)"]
        #[doc = " - ``FAILED_PRECONDITION``: on consistency errors (e.g. the contract key has changed since the submission)"]
        #[doc = " or if an interpretation error occurred"]
        #[doc = " - ``UNAVAILABLE``: if the participant is not yet ready to submit commands or if the service has been shut down."]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request failed to receive its completion within the predefined timeout."]
        pub async fn submit_and_wait_for_transaction_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitAndWaitRequest>,
        ) -> Result<tonic::Response<super::SubmitAndWaitForTransactionIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandService/SubmitAndWaitForTransactionId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submits a single composite command, waits for its result, and returns the transaction."]
        #[doc = " Propagates the gRPC error of failed submissions including Daml interpretation errors."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or if a resource is missing (e.g. contract key)"]
        #[doc = " due to for example contention on resources"]
        #[doc = " - ``ALREADY_EXISTS`` if a resource is duplicated (e.g. contract key)"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``ABORTED``: if the number of in-flight commands reached the maximum (if a limit is configured)"]
        #[doc = " - ``FAILED_PRECONDITION``: on consistency errors (e.g. the contract key has changed since the submission)"]
        #[doc = " or if an interpretation error occurred"]
        #[doc = " - ``UNAVAILABLE``: if the participant is not yet ready to submit commands or if the service has been shut down."]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request failed to receive its completion within the predefined timeout."]
        pub async fn submit_and_wait_for_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitAndWaitRequest>,
        ) -> Result<tonic::Response<super::SubmitAndWaitForTransactionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandService/SubmitAndWaitForTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Submits a single composite command, waits for its result, and returns the transaction tree."]
        #[doc = " Propagates the gRPC error of failed submissions including Daml interpretation errors."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or if a resource is missing (e.g. contract key)"]
        #[doc = " due to for example contention on resources"]
        #[doc = " - ``ALREADY_EXISTS`` if a resource is duplicated (e.g. contract key)"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``ABORTED``: if the number of in-flight commands reached the maximum (if a limit is configured)"]
        #[doc = " - ``FAILED_PRECONDITION``: on consistency errors (e.g. the contract key has changed since the submission)"]
        #[doc = " or if an interpretation error occurred"]
        #[doc = " - ``UNAVAILABLE``: if the participant is not yet ready to submit commands or if the service has been shut down."]
        #[doc = " - ``DEADLINE_EXCEEDED``: if the request failed to receive its completion within the predefined timeout."]
        pub async fn submit_and_wait_for_transaction_tree(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitAndWaitRequest>,
        ) -> Result<tonic::Response<super::SubmitAndWaitForTransactionTreeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandService/SubmitAndWaitForTransactionTree",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The submitted commands will be processed atomically in a single transaction. Moreover, each ``Command`` in ``commands`` will be executed in the order specified by the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitRequest {
    /// The commands to be submitted in a single transaction.
    /// Required
    #[prost(message, optional, tag = "1")]
    pub commands: ::core::option::Option<Commands>,
}
#[doc = r" Generated client implementations."]
pub mod command_submission_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to attempt advancing the ledger's state by submitting commands."]
    #[doc = " The final states of their submissions are disclosed by the Command Completion Service."]
    #[doc = " The on-ledger effects of their submissions are disclosed by the Transaction Service."]
    #[doc = ""]
    #[doc = " Commands may fail in 2 distinct manners:"]
    #[doc = ""]
    #[doc = " 1. Failure communicated synchronously in the gRPC error of the submission."]
    #[doc = " 2. Failure communicated asynchronously in a Completion, see ``completion.proto``."]
    #[doc = ""]
    #[doc = " Note that not only successfully submitted commands MAY produce a completion event. For example, the participant MAY"]
    #[doc = " choose to produce a completion event for a rejection of a duplicate command."]
    #[doc = ""]
    #[doc = " Clients that do not receive a successful completion about their submission MUST NOT assume that it was successful."]
    #[doc = " Clients SHOULD subscribe to the CompletionStream before starting to submit commands to prevent race conditions."]
    #[derive(Debug, Clone)]
    pub struct CommandSubmissionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommandSubmissionServiceClient<tonic::transport::Channel> {
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
    impl<T> CommandSubmissionServiceClient<T>
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
        ) -> CommandSubmissionServiceClient<InterceptedService<T, F>>
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
            CommandSubmissionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Submit a single composite command."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or if a resource is missing (e.g. contract key)"]
        #[doc = " due to for example contention on resources"]
        #[doc = " - ``ALREADY_EXISTS`` if a resource is duplicated (e.g. contract key)"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields"]
        #[doc = " - ``ABORTED``: if the number of in-flight commands reached the maximum (if a limit is configured)"]
        #[doc = " - ``FAILED_PRECONDITION``: on consistency errors (e.g. the contract key has changed since the submission)"]
        #[doc = " or if an interpretation error occurred"]
        #[doc = " - ``UNAVAILABLE``: if the participant is not yet ready to submit commands or if the service has been shut down."]
        pub async fn submit(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.CommandSubmissionService/Submit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
} //
  //IMPORTANT: in contrast to other parts of the Ledger API, only json-wire backwards
  //compatibility guarantees are given for the messages in this file.

/// See the feature message definitions for descriptions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentalFeatures {
    #[prost(message, optional, tag = "1")]
    pub self_service_error_codes: ::core::option::Option<ExperimentalSelfServiceErrorCodes>,
}
/// GRPC self-service error codes are returned by the Ledger API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentalSelfServiceErrorCodes {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerConfigurationRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerConfigurationResponse {
    /// The latest ledger configuration.
    #[prost(message, optional, tag = "1")]
    pub ledger_configuration: ::core::option::Option<LedgerConfiguration>,
}
/// LedgerConfiguration contains parameters of the ledger instance that may be useful to clients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerConfiguration {
    /// If a command submission specifies a deduplication period of length up to ``max_deduplication_time``,
    /// the submission SHOULD not be rejected with ``NOT_FOUND`` because the deduplication period start is too early.
    /// The deduplication period is measured on a local clock of the participant or Daml ledger,
    /// and therefore subject to clock skews and clock drifts.
    /// Command submissions with longer periods MAY get accepted though.
    #[prost(message, optional, tag = "3")]
    pub max_deduplication_time: ::core::option::Option<::prost_types::Duration>,
}
#[doc = r" Generated client implementations."]
pub mod ledger_configuration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " LedgerConfigurationService allows clients to subscribe to changes of the ledger configuration."]
    #[derive(Debug, Clone)]
    pub struct LedgerConfigurationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LedgerConfigurationServiceClient<tonic::transport::Channel> {
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
    impl<T> LedgerConfigurationServiceClient<T>
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
        ) -> LedgerConfigurationServiceClient<InterceptedService<T, F>>
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
            LedgerConfigurationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the latest configuration as the first response, and publishes configuration updates in the same stream."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        pub async fn get_ledger_configuration(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLedgerConfigurationRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetLedgerConfigurationResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.LedgerConfigurationService/GetLedgerConfiguration",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerIdentityRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerIdentityResponse {
    /// The ID of the ledger exposed by the server.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ledger_identity_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to verify that the server they are communicating with exposes the ledger they wish to operate on."]
    #[doc = " Note that every ledger has a unique ID."]
    #[derive(Debug, Clone)]
    pub struct LedgerIdentityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LedgerIdentityServiceClient<tonic::transport::Channel> {
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
    impl<T> LedgerIdentityServiceClient<T>
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
        ) -> LedgerIdentityServiceClient<InterceptedService<T, F>>
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
            LedgerIdentityServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Clients may call this RPC to return the identifier of the ledger they are connected to."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        pub async fn get_ledger_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLedgerIdentityRequest>,
        ) -> Result<tonic::Response<super::GetLedgerIdentityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.LedgerIdentityService/GetLedgerIdentity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesResponse {
    /// The IDs of all Daml-LF packages supported by the server.
    /// Each element must be a valid PackageIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "1")]
    pub package_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// The ID of the requested package.
    /// Must be a valid PackageIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub package_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageResponse {
    /// The hash function we use to calculate the hash.
    /// Required
    #[prost(enumeration = "HashFunction", tag = "1")]
    pub hash_function: i32,
    /// Contains a ``daml_lf`` ArchivePayload. See further details in ``daml_lf.proto``.
    /// Required
    #[prost(bytes = "vec", tag = "2")]
    pub archive_payload: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the archive payload, can also used as a ``package_id``.
    /// Must be a valid PackageIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageStatusRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// The ID of the requested package.
    /// Must be a valid PackageIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub package_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageStatusResponse {
    /// The status of the package.
    #[prost(enumeration = "PackageStatus", tag = "1")]
    pub package_status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageStatus {
    /// The server is not aware of such a package.
    Unknown = 0,
    /// The server is able to execute Daml commands operating on this package.
    Registered = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HashFunction {
    Sha256 = 0,
}
#[doc = r" Generated client implementations."]
pub mod package_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to query the Daml-LF packages that are supported by the server."]
    #[derive(Debug, Clone)]
    pub struct PackageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PackageServiceClient<tonic::transport::Channel> {
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
    impl<T> PackageServiceClient<T>
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
        ) -> PackageServiceClient<InterceptedService<T, F>>
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
            PackageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the identifiers of all supported packages."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        pub async fn list_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPackagesRequest>,
        ) -> Result<tonic::Response<super::ListPackagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.PackageService/ListPackages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the contents of a single package."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the requested package is unknown"]
        pub async fn get_package(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPackageRequest>,
        ) -> Result<tonic::Response<super::GetPackageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.PackageService/GetPackage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the status of a single package."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the requested package is unknown"]
        pub async fn get_package_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPackageStatusRequest>,
        ) -> Result<tonic::Response<super::GetPackageStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.PackageService/GetPackageStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// Beginning of the requested ledger section.
    /// This offset is exclusive: the response will only contain transactions whose offset is strictly greater than this.
    /// Required
    #[prost(message, optional, tag = "2")]
    pub begin: ::core::option::Option<LedgerOffset>,
    /// End of the requested ledger section.
    /// This offset is inclusive: the response will only contain transactions whose offset is less than or equal to this.
    /// Optional, if not set, the stream will not terminate.
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<LedgerOffset>,
    /// Requesting parties with template filters.
    /// Template filters must be empty for GetTransactionTrees requests.
    /// Required
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<TransactionFilter>,
    /// If enabled, values served over the API will contain more information than strictly necessary to interpret the data.
    /// In particular, setting the verbose flag to true triggers the ledger to include labels for record fields.
    /// Optional
    #[prost(bool, tag = "5")]
    pub verbose: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsResponse {
    /// The list of transactions that matches the filter in GetTransactionsRequest for the GetTransactions method.
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionTreesResponse {
    /// The list of transaction trees that matches the filter in ``GetTransactionsRequest`` for the ``GetTransactionTrees`` method.
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionTree>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionByEventIdRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// The ID of a particular event.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    /// The parties whose events the client expects to see.
    /// Events that are not visible for the parties in this collection will not be present in the response.
    /// Each element must be a valid PartyIdString (as described in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "3")]
    pub requesting_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionByIdRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
    /// The ID of a particular transaction.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "2")]
    pub transaction_id: ::prost::alloc::string::String,
    /// The parties whose events the client expects to see.
    /// Events that are not visible for the parties in this collection will not be present in the response.
    /// Each element be a valid PartyIdString (as describe in ``value.proto``).
    /// Required
    #[prost(string, repeated, tag = "3")]
    pub requesting_parties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<TransactionTree>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFlatTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerEndRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as describe in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerEndResponse {
    /// The absolute offset of the current ledger end.
    #[prost(message, optional, tag = "1")]
    pub offset: ::core::option::Option<LedgerOffset>,
}
#[doc = r" Generated client implementations."]
pub mod transaction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to read transactions from the ledger."]
    #[derive(Debug, Clone)]
    pub struct TransactionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TransactionServiceClient<tonic::transport::Channel> {
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
    impl<T> TransactionServiceClient<T>
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
        ) -> TransactionServiceClient<InterceptedService<T, F>>
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
            TransactionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Read the ledger's filtered transaction stream for a set of parties."]
        #[doc = " Lists only creates and archives, but not other events."]
        #[doc = " Omits all events on transient contracts, i.e., contracts that were both created and archived in the same transaction."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if ``before`` is not before ``end``)"]
        #[doc = " - ``FAILED_PRECONDITION``: if the ledger has been pruned after the subscription start offset"]
        #[doc = " - ``OUT_OF_RANGE``: if the ``begin`` parameter value is not before the end of the ledger"]
        pub async fn get_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetTransactionsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetTransactions",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Read the ledger's complete transaction tree stream for a set of parties."]
        #[doc = " The stream can be filtered only by parties, but not templates (template filter must be empty)."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if ``before`` is not before ``end``)"]
        #[doc = " - ``FAILED_PRECONDITION``: if the ledger has been pruned after the subscription start offset"]
        #[doc = " - ``OUT_OF_RANGE``: if the ``begin`` parameter value is not before the end of the ledger"]
        pub async fn get_transaction_trees(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GetTransactionTreesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetTransactionTrees",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Lookup a transaction tree by the ID of an event that appears within it."]
        #[doc = " For looking up a transaction instead of a transaction tree, please see GetFlatTransactionByEventId"]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or no such transaction exists"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if requesting parties are invalid or empty)"]
        pub async fn get_transaction_by_event_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByEventIdRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetTransactionByEventId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup a transaction tree by its ID."]
        #[doc = " For looking up a transaction instead of a transaction tree, please see GetFlatTransactionById"]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or no such transaction exists"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if requesting parties are invalid or empty)"]
        pub async fn get_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByIdRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetTransactionById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup a transaction by the ID of an event that appears within it."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or no such transaction exists"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if requesting parties are invalid or empty)"]
        pub async fn get_flat_transaction_by_event_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByEventIdRequest>,
        ) -> Result<tonic::Response<super::GetFlatTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetFlatTransactionByEventId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup a transaction by its ID."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id or no such transaction exists"]
        #[doc = " - ``INVALID_ARGUMENT``: if the payload is malformed or is missing required fields (e.g. if requesting parties are invalid or empty)"]
        pub async fn get_flat_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionByIdRequest>,
        ) -> Result<tonic::Response<super::GetFlatTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetFlatTransactionById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the current ledger end."]
        #[doc = " Subscriptions started with the returned offset will serve transactions created after this RPC was called."]
        #[doc = " Errors:"]
        #[doc = " - ``UNAUTHENTICATED``: if the request does not include a valid access token"]
        #[doc = " - ``PERMISSION_DENIED``: if the claims in the token are insufficient to perform a given operation"]
        #[doc = " - ``NOT_FOUND``: if the request does not include a valid ledger id"]
        pub async fn get_ledger_end(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLedgerEndRequest>,
        ) -> Result<tonic::Response<super::GetLedgerEndResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.TransactionService/GetLedgerEnd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerApiVersionRequest {
    /// Must correspond to the ledger ID reported by the Ledger Identification Service.
    /// Must be a valid LedgerString (as described in ``value.proto``).
    /// Required
    #[prost(string, tag = "1")]
    pub ledger_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLedgerApiVersionResponse {
    /// The version of the ledger API.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The features supported by this Ledger API endpoint.
    ///
    /// Daml applications CAN use the feature descriptor on top of
    /// version constraints on the Ledger API version to determine
    /// whether a given Ledger API endpoint supports the features
    /// required to run the application.
    ///
    /// See the feature descriptions themselves for the relation between
    /// Ledger API versions and feature presence.
    #[prost(message, optional, tag = "2")]
    pub features: ::core::option::Option<FeaturesDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeaturesDescriptor {
    /// Features under development or features that are used
    /// for ledger implementation testing purposes only.
    ///
    /// Daml applications SHOULD not depend on these in production.
    #[prost(message, optional, tag = "1")]
    pub experimental: ::core::option::Option<ExperimentalFeatures>,
}
#[doc = r" Generated client implementations."]
pub mod version_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allows clients to retrieve information about the ledger API version"]
    #[derive(Debug, Clone)]
    pub struct VersionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VersionServiceClient<tonic::transport::Channel> {
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
    impl<T> VersionServiceClient<T>
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
        ) -> VersionServiceClient<InterceptedService<T, F>>
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
            VersionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Read the Ledger API version"]
        pub async fn get_ledger_api_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLedgerApiVersionRequest>,
        ) -> Result<tonic::Response<super::GetLedgerApiVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/com.daml.ledger.api.v1.VersionService/GetLedgerApiVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
