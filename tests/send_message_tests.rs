use whatsapp_cloud_api::{
    models::{Message, Template, Text},
    WhatasppClient, WhatsappError,
};

#[tokio::test]
async fn send_text_message_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let text = Text::new("test message");
    let message = Message::from_text(&to, text);
    let client = WhatasppClient::new(&access_token);
    let response = client.send_message(&message).await?;
    assert_eq!(response.messages.len(), 1);
    Ok(())
}

#[tokio::test]
async fn send_message_template_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");
    let template_name = "hello_world";
    let language = "en_US";
    let template = Template::new(template_name, language);
    let message = Message::from_template(&to, template);
    let client = WhatasppClient::new(&access_token);
    let response = client.send_message(&message).await?;
    assert_eq!(response.messages.len(), 1);
    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
