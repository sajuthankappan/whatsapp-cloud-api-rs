# whatsapp-cloud-api &emsp; [![Latest Version]][crates.io] [![Docs]][docs.rs]
[Latest Version]: https://img.shields.io/crates/v/whatsapp-cloud-api.svg
[crates.io]: https://crates.io/crates/whatsapp-cloud-api
[Docs]: https://docs.rs/whatsapp-cloud-api/badge.svg
[docs.rs]: https://docs.rs/whatsapp-cloud-api

**RWhatsapp Cloud API Rust Client**

## Usage example

Send templatge based text message

```rust
let access_token = "<access_token>";
let to = "<to>";
let template_name = "hello_world";
let language = "en_US";
let template = Template::new(template_name, language);
let message = Message::from_template(&to, template);
let client = WhatsppClient::new(&access_token);
client.send_message(&message).await?;
```

Send text message (Note: This requires an user initial conversation)
```rust
let access_token = "<access_token>";
let to = "<to>";
let text = Text::new("test message");
let message = Message::from_text(&to, text);
let client = WhatasppClient::new(&access_token);
client.send_message(&message).await?;
```

For more examples, please see the [tests] folder

[tests]: https://github.com/sajuthankappan/whatsapp-cloud-api-rs/tree/master/tests