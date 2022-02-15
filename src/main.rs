pub mod api {
    tonic_include_protos::include_protos!("com.daml.ledger.api.v1");
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
