pub mod api {
    pub mod com {
        pub mod daml {
            pub mod ledger {
                pub mod api {
                    pub mod v1 {
                        include!("proto/com.daml.ledger.api.v1.rs");
                        pub mod testing {
                            include!("proto/com.daml.ledger.api.v1.testing.rs");
                        }

                        pub mod admin {
                            include!("proto/com.daml.ledger.api.v1.admin.rs");
                        }
                    }
                }
            }
        }
    }
    pub mod google {
        pub mod protobuf {
            include!("proto/google.protobuf.rs");
        }

        pub mod rpc {
            include!("proto/google.rpc.rs");
        }
    }
}

use api::com::daml::ledger::api::v1::ledger_identity_service_client::LedgerIdentityServiceClient;
use api::com::daml::ledger::api::v1::GetLedgerIdentityRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LedgerIdentityServiceClient::connect("http://localhost:6865").await?;

    let request = tonic::Request::new(GetLedgerIdentityRequest {});

    let response = client.get_ledger_identity(request).await?;

    println!("RESPONSE={}", response.into_inner().ledger_id);

    Ok(())
}
