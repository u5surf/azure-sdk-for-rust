use azure_core::HttpClient;
/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
use azure_cosmos::prelude::*;
use azure_cosmos::stored_procedure::Parameters;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = {
        let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
        azure_cosmos::client_builder::build_default_client(&account, authorization_token)?
            .with_http_client(http_client)
            .build()
    };

    let ret = client
        .into_database_client(&database)
        .into_collection_client(&collection)
        .into_stored_procedure_client("test_proc")
        .execute_stored_procedure()
        .with_parameters(Parameters::new().push("Robert")?)
        .execute::<serde_json::Value>()
        .await?;

    println!("Response object:\n{:#?}", ret);
    println!("Response as JSON:\n{}", ret.payload.to_string());

    Ok(())
}
