fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("TIP_OUT_DIR", "src/proto");
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto")
        .compile(
            &[ // the list of protos
                "com/daml/ledger/api/v1/admin/config_management_service.proto",
                "com/daml/ledger/api/v1/admin/package_management_service.proto",
                "com/daml/ledger/api/v1/admin/participant_pruning_service.proto",
                "com/daml/ledger/api/v1/admin/party_management_service.proto",
                "com/daml/ledger/api/v1/testing/reset_service.proto",
                "com/daml/ledger/api/v1/testing/time_service.proto",
                "com/daml/ledger/api/v1/active_contracts_service.proto",
                "com/daml/ledger/api/v1/command_completion_service.proto",
                "com/daml/ledger/api/v1/command_service.proto",
                "com/daml/ledger/api/v1/command_submission_service.proto",
                "com/daml/ledger/api/v1/commands.proto",
                "com/daml/ledger/api/v1/completion.proto",
                "com/daml/ledger/api/v1/event.proto",
                "com/daml/ledger/api/v1/experimental_features.proto",
                "com/daml/ledger/api/v1/ledger_configuration_service.proto",
                "com/daml/ledger/api/v1/ledger_identity_service.proto",
                "com/daml/ledger/api/v1/ledger_offset.proto",
                "com/daml/ledger/api/v1/package_service.proto",
                "com/daml/ledger/api/v1/transaction.proto",
                "com/daml/ledger/api/v1/transaction_filter.proto",
                "com/daml/ledger/api/v1/transaction_service.proto",
                "com/daml/ledger/api/v1/value.proto",
                "com/daml/ledger/api/v1/version_service.proto",
            ],
            &["."], // specify the root location to search proto dependencies
        )?;

    Ok(())
}