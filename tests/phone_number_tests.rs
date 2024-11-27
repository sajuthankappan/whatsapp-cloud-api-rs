use whatsapp_cloud_api::{models::CodeMethod, WhatsappClient, WhatsappError};

#[tokio::test]
async fn request_code_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let phone_number_id = std::env::var("WHATSAPP_PHONE_NUMBER_ID")
        .expect("Missing environment variable WHATSAPP_PHONE_NUMBER_ID");
    let client = WhatsappClient::new(&access_token, &phone_number_id);
    let response = client.request_code(CodeMethod::SMS, "en").await?;
    assert_eq!(response.success, Some(true));
    Ok(())
}

#[tokio::test]
async fn verify_code_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let phone_number_id = std::env::var("WHATSAPP_PHONE_NUMBER_ID")
        .expect("Missing environment variable WHATSAPP_PHONE_NUMBER_ID");
    let verification_code = std::env::var("WHATSAPP_VERIFICATION_CODE")
        .expect("Missing environment variable WHATSAPP_VERIFICATION_CODE");
    let client = WhatsappClient::new(&access_token, &phone_number_id);
    let response = client.verify_code(&verification_code).await?;
    assert_eq!(response.success, Some(true));
    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
