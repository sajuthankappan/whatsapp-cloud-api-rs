# whatsapp-cloud-api &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs]
[Latest Version]: https://img.shields.io/crates/v/whatsapp-cloud-api.svg
[crates.io]: https://crates.io/crates/whatsapp-cloud-api
[Docs]: https://docs.rs/whatsapp-cloud-api/badge.svg
[docs.rs]: https://docs.rs/whatsapp-cloud-api

**Whatsapp Cloud API Rust Client**

> [!WARNING]
> **This crate has moved to [`whatsapp`](https://crates.io/crates/whatsapp).** `whatsapp-cloud-api` will not receive further updates.
>
> To migrate, replace `whatsapp-cloud-api = "0.5"` with `whatsapp = "0.6"` in `Cargo.toml`, and `whatsapp_cloud_api::` with `whatsapp::` in code.

## Features

- Sending messages using Whatsapp Cloud API
- Get / Upload media
- Models to help processing incoming webhooks

## Supported Graph API Version
This crate uses Facebook Graph API version v26.0 by default (v20.0 expires on 2026-09-24). You may change the version using the `set_version()` method. But, do it at your own risk :)

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

[tests]: https://github.com/sajuthankappan/whatsapp-cloud-api-rs/tree/master/tests
