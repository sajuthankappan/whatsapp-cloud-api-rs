# whatsapp &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs] [![CI Status]][actions]
[Latest Version]: https://img.shields.io/crates/v/whatsapp.svg
[crates.io]: https://crates.io/crates/whatsapp
[Docs]: https://docs.rs/whatsapp/badge.svg
[docs.rs]: https://docs.rs/whatsapp
[CI Status]: https://github.com/sajuthankappan/whatsapp-rs/actions/workflows/ci.yml/badge.svg?branch=main
[actions]: https://github.com/sajuthankappan/whatsapp-rs/actions/workflows/ci.yml

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
This crate uses Facebook Graph API version v26.0 by default. You may change the version using the `set_version()` method. But, do it at your own risk :)

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

## Error handling

Errors returned by the Graph API are available as `WhatsappError::ApiError`, with Meta's error `code`, `error_subcode`, `error_data.details`, `fbtrace_id` and the HTTP `status`. See Meta's [error codes] reference.

```rust
use whatsapp::WhatsappError;

match client.send_message(&message).await {
    Ok(response) => println!("accepted {}", response.messages[0].id),
    // Access token expired or invalid
    Err(WhatsappError::ApiError(e)) if e.code == 190 => {
        eprintln!("refresh the access token: {e}");
    }
    // Any other error returned by the Graph API (bad parameters, rate limit, ...)
    Err(WhatsappError::ApiError(e)) => {
        eprintln!("HTTP {}: {e} (fbtrace_id: {:?})", e.status, e.fbtrace_id);
    }
    // Network / TLS errors, or an error response that is not from the Graph API
    Err(e) => eprintln!("request failed: {e}"),
}
```

`Ok` means the message was accepted, not delivered. Delivery failures (for example `131047` when more than 24 hours have passed since the user's last message) are reported later in a `failed` status webhook, available as `webhooks::Status::errors`.

[error codes]: https://developers.facebook.com/docs/whatsapp/cloud-api/support/error-codes

For more details, please see the [tests] folder

[tests]: https://github.com/sajuthankappan/whatsapp-rs/tree/main/tests

## Maintainer

Maintained by [Saju Thankappan](https://github.com/sajuthankappan), creator of [Smito One](https://smito.in).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
