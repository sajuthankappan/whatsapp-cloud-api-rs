# whatsapp-cloud-api-rs

Whatsapp Cloud API Rust Client 

## Usage example

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

For more examples, please see the [tests] folder


[tests]: https://github.com/sajuthankappan/whatsapp-cloud-api-rs/tree/master/tests