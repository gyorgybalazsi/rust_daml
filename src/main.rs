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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pem = tokio::fs::read("./isrgrootx1.pem").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("daml.app");

    let endpoint = Endpoint::from_static("https://iiv3knqfxltkcayz.daml.app:443")
        .tls_config(tls)?
        .connect()
        .await?;

    let token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6ImRhYmwtZDBmODg0NjctZjU4NC00NzhkLWEyOTctOWNlYjg2NGExNmNkIn0.eyJpc3MiOiJodWIuZGFtbC5jb20vbG9naW4iLCJzdWIiOiJhdXRoMHw2MTkyOWIzMjRlMmJmNTAwNmZlNTJmNjAiLCJleHAiOjE2NDUyNzMwMjUsImp0aSI6IjgzNDJiNGQ1LWVkNzctNDk4ZC05YTNiLTY4NmJmM2I4NGMyOSIsImh0dHBzOi8vZGFtbC5jb20vbGVkZ2VyLWFwaSI6eyJhY3RBcyI6WyJsZWRnZXItcGFydHktMTJlMTc3ZjYtNzI5My00ODliLTkyOGQtZTkxNzJhMDgzODlmIl0sImFwcGxpY2F0aW9uSWQiOiJkYW1saHViIiwibGVkZ2VySWQiOiJpaXYza25xZnhsdGtjYXl6IiwicmVhZEFzIjpbImxlZGdlci1wYXJ0eS0xMmUxNzdmNi03MjkzLTQ4OWItOTI4ZC1lOTE3MmEwODM4OWYiXX0sImxlZGdlcklkIjoiaWl2M2tucWZ4bHRrY2F5eiIsIm93bmVyIjoidXNlci1ncmFudC1lYmFhYmRiMy03M2YwLTQ5MTYtOWNiZi0wMzhhODM5MjIzOTkiLCJwYXJ0eSI6ImxlZGdlci1wYXJ0eS0xMmUxNzdmNi03MjkzLTQ4OWItOTI4ZC1lOTE3MmEwODM4OWYiLCJwYXJ0eU5hbWUiOiJBbGljZSJ9.VKL-87hTsoWNG_FrEuEvigbudTG_iGpfv43NkXE2N8boAHitETFK1_5CI_RO7RpRcPLxoAzO2tG-ZNjN26n5WgaVFeG5MFkNAckdGA8LIjJ7Vu0Kkzvb_f4oP_eK6OEHwV6FBal0hhP6uQ2Tdw3L9sjhwwZMoZnGPCkO8S4LVAU";

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
