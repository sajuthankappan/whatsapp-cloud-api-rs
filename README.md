# whatsapp &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs]
[Latest Version]: https://img.shields.io/crates/v/whatsapp.svg
[crates.io]: https://crates.io/crates/whatsapp
[Docs]: https://docs.rs/whatsapp/badge.svg
[docs.rs]: https://docs.rs/whatsapp

**WhatsApp Business Cloud API client for Rust**

## Migrating from `whatsapp-cloud-api`

This crate was previously published as [`whatsapp-cloud-api`](https://crates.io/crates/whatsapp-cloud-api) (up to 0.5.x). To migrate:

- In `Cargo.toml`, replace `whatsapp-cloud-api = "0.5"` with `whatsapp = "0.6"`
- In code, replace `whatsapp_cloud_api::` with `whatsapp::`

## Features

- Sending messages using Whatsapp Cloud API
- Get / Upload media
- Models to help processing incoming webhooks

## Supported Graph API Version
This crate is designed & tested for facebook graph api version v20.0. You may change the version using the set_verion() method. But, do it at your own risk :)

## Usage example

Send template based text message

```rust
let access_token = "<access_token>";
let phone_number_id = "<phone_number_id>";
let to = "<to>";
let template_name = "hello_world";
let language = "en_US";
let template = Template::new(template_name, language);
let message = Message::from_template(&to, template, None);
let client = WhatsappClient::new(&access_token, &phone_number_id);
client.send_message(&message).await?;
```

Send template based text message with parameters

```rust
let access_token = "<access_token>";
let phone_number_id = "<phone_number_id>";
let template_name = "sample_shipping_confirmation";
let language = "en_US";
let parameters = Vec::from([Parameter::from_text("3")]);
let components = Vec::from([Component::with_parameters("body", parameters)]);
let template = Template::with_components(template_name, language, components);
let message = Message::from_template(&to, template, None);
let client = WhatsappClient::new(&access_token, &phone_number_id);
let response = client.send_message(&message).await?;
```

Send text message (Note: This requires an user initial conversation)

```rust
let access_token = "<access_token>";
let phone_number_id = "<phone_number_id>";
let to = "<to>";
let text = Text::new("test message");
let message = Message::from_text(&to, text, None);
let client = WhatsappClient::new(&access_token, &phone_number_id);
client.send_message(&message).await?;
```


For more details, please see the [tests] folder

[tests]: https://github.com/sajuthankappan/whatsapp-cloud-api-rs/tree/main/tests
