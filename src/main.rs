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
use tonic::transport::{Certificate, ClientTlsConfig, Endpoint};
use tonic::Request;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let ledger_id = &args[2];
    let jwt = &args[3];
    let tls_cert_file = &args[4];

    let pem = tokio::fs::read(tls_cert_file).await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("daml.app");

    let uri = format!("https://{}.daml.app:443", ledger_id);

    let endpoint = Endpoint::from_shared(uri)?
        .tls_config(tls)?
        .connect()
        .await?;

    let mut token = String::from("Bearer ");
    token.push_str(jwt);

    let mut ledger_id_client = LedgerIdentityServiceClient::with_interceptor(endpoint, move |mut req: Request<()>| {
        // adding token to request.
                req.metadata_mut().insert(
                    "authorization",
                    tonic::metadata::MetadataValue::from_str(&token).unwrap(),
                );
                Ok(req)
            });

    let ledger_id_request = tonic::Request::new(GetLedgerIdentityRequest {});

    let ledger_id_response = ledger_id_client.get_ledger_identity(ledger_id_request).await?;

    println!("Ledger id = {}", ledger_id_response.into_inner().ledger_id);

    Ok(())
}
