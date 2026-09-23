//! Offline tests for the request / response / webhook models (no network access).

use serde_json::{Value, json};
use whatsapp::models::{
    CodeMethod, Component, ComponentSubType, ComponentType, Context, Image, Interactive,
    InteractiveActionButton, InteractiveActionSection, InteractiveActionSectionRow, MediaResponse,
    Message, MessageResponse, MessageStatus, Parameter, Template, Text,
    webhooks::{NotificationMessageType, NotificationPayload, VerificationRequest},
};

fn to_json<T: serde::Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap()
}

// Outgoing requests

#[test]
fn text_message_serializes() {
    let message = Message::from_text("15551234567", Text::new("hello"), None);
    let json = to_json(&message);

    assert_eq!(json["messaging_product"], "whatsapp");
    assert_eq!(json["to"], "15551234567");
    assert_eq!(json["type"], "text");
    assert_eq!(json["text"]["body"], "hello");
}

#[test]
fn text_message_with_preview_url_and_context_serializes() {
    let message = Message::from_text(
        "15551234567",
        Text::with_preview_url("see https://example.com"),
        Some(Context::new("wamid.previous")),
    );
    let json = to_json(&message);

    assert_eq!(json["text"]["preview_url"], true);
    assert_eq!(json["context"], json!({ "message_id": "wamid.previous" }));
}

#[test]
fn template_message_serializes() {
    let message =
        Message::from_template("15551234567", Template::new("hello_world", "en_US"), None);
    let json = to_json(&message);

    assert_eq!(json["type"], "template");
    assert_eq!(json["template"]["name"], "hello_world");
    assert_eq!(
        json["template"]["language"],
        json!({ "policy": "deterministic", "code": "en_US" })
    );
}

#[test]
fn template_message_with_components_serializes() {
    let components = vec![
        Component::with_parameters(ComponentType::Body, vec![Parameter::from_text("3")]),
        Component::for_button(
            ComponentType::Button,
            ComponentSubType::QuickReply,
            vec![Parameter::from_text("payload")],
            0,
        ),
    ];
    let template = Template::with_components("sample_shipping_confirmation", "en_US", components);
    let json = to_json(&Message::from_template("15551234567", template, None));

    let components = &json["template"]["components"];
    assert_eq!(components[0]["type"], "body");
    assert_eq!(components[0]["parameters"][0]["type"], "text");
    assert_eq!(components[0]["parameters"][0]["text"], "3");
    assert_eq!(components[1]["type"], "button");
    assert_eq!(components[1]["sub_type"], "quick_reply");
    assert_eq!(components[1]["index"], 0);
}

#[test]
fn interactive_button_message_serializes() {
    let buttons = vec![
        InteractiveActionButton::new("Yes", "yes-id"),
        InteractiveActionButton::new("No", "no-id"),
    ];
    let interactive = Interactive::for_button(buttons, "Continue?");
    let json = to_json(&Message::from_interactive("15551234567", interactive, None));

    assert_eq!(json["type"], "interactive");
    let interactive = &json["interactive"];
    assert_eq!(interactive["type"], "button");
    assert_eq!(interactive["body"]["text"], "Continue?");
    assert_eq!(
        interactive["action"]["buttons"][0],
        json!({ "type": "reply", "reply": { "title": "Yes", "id": "yes-id" } })
    );
    assert_eq!(interactive["action"]["buttons"][1]["reply"]["id"], "no-id");
}

#[test]
fn interactive_list_message_serializes() {
    let rows = vec![
        InteractiveActionSectionRow::new("row-1", "First"),
        InteractiveActionSectionRow::with_description("row-2", "Second", "More detail"),
    ];
    let sections = vec![InteractiveActionSection::with_title(rows, "Section")];
    let interactive = Interactive::for_list("Choose", sections, "Pick one");
    let json = to_json(&Message::from_interactive("15551234567", interactive, None));

    let interactive = &json["interactive"];
    assert_eq!(interactive["type"], "list");
    assert_eq!(interactive["body"]["text"], "Pick one");
    assert_eq!(interactive["action"]["button"], "Choose");
    let section = &interactive["action"]["sections"][0];
    assert_eq!(section["title"], "Section");
    assert_eq!(section["rows"][0]["id"], "row-1");
    assert_eq!(section["rows"][0]["title"], "First");
    assert_eq!(section["rows"][1]["description"], "More detail");
}

#[test]
fn image_message_by_link_serializes() {
    let image = Image::new("https://example.com/cat.jpg", Some("A cat".into()));
    let json = to_json(&Message::from_image("15551234567", image, None));

    assert_eq!(json["type"], "image");
    assert_eq!(json["image"]["link"], "https://example.com/cat.jpg");
    assert_eq!(json["image"]["caption"], "A cat");
}

#[test]
fn image_message_by_media_id_serializes() {
    let image = Image::for_id("1234567890", None);
    let json = to_json(&Message::from_image("15551234567", image, None));

    assert_eq!(json["image"]["id"], "1234567890");
}

#[test]
fn mark_as_read_serializes() {
    let json = to_json(&MessageStatus::for_read("wamid.abc"));

    assert_eq!(
        json,
        json!({ "messaging_product": "whatsapp", "message_id": "wamid.abc", "status": "read" })
    );
}

#[test]
fn code_method_serializes_uppercase() {
    assert_eq!(to_json(&CodeMethod::SMS), "SMS");
    assert_eq!(to_json(&CodeMethod::Voice), "VOICE");
}

// API responses

#[test]
fn message_response_deserializes() {
    let body = json!({
        "messaging_product": "whatsapp",
        "contacts": [{ "input": "+15551234567", "wa_id": "15551234567" }],
        "messages": [{ "id": "wamid.HBgLMTU1NTEyMzQ1NjcVAgARGBI", "message_status": "accepted" }]
    });
    let response: MessageResponse = serde_json::from_value(body).unwrap();

    assert_eq!(response.contacts[0].wa_id, "15551234567");
    assert_eq!(response.messages[0].id, "wamid.HBgLMTU1NTEyMzQ1NjcVAgARGBI");
    assert_eq!(
        response.messages[0].message_status.as_deref(),
        Some("accepted")
    );
}

#[test]
fn media_response_deserializes() {
    let body = json!({
        "messaging_product": "whatsapp",
        "url": "https://lookaside.fbsbx.com/whatsapp_business/attachments/?mid=1",
        "mime_type": "image/jpeg",
        "sha256": "abc123",
        "file_size": 303833,
        "id": "1234567890"
    });
    let media: MediaResponse = serde_json::from_value(body).unwrap();

    assert_eq!(media.mime_type, "image/jpeg");
    assert_eq!(media.file_size, 303833);
    assert_eq!(media.id, "1234567890");
}

// Webhooks

fn webhook(value: Value) -> NotificationPayload {
    serde_json::from_value(json!({
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "102290129340398",
            "changes": [{ "field": "messages", "value": value }]
        }]
    }))
    .unwrap()
}

fn incoming(message: Value) -> NotificationPayload {
    webhook(json!({
        "messaging_product": "whatsapp",
        "metadata": { "display_phone_number": "15550783881", "phone_number_id": "106540352242922" },
        "contacts": [{ "profile": { "name": "Sheena Nelson" }, "wa_id": "16505551234" }],
        "messages": [message]
    }))
}

#[test]
fn verification_request_deserializes() {
    let request: VerificationRequest = serde_json::from_value(json!({
        "hub.mode": "subscribe",
        "hub.verify_token": "my-token",
        "hub.challenge": "1158201444"
    }))
    .unwrap();

    assert_eq!(request.mode, "subscribe");
    assert_eq!(request.verify_token, "my-token");
    assert_eq!(request.challenge, "1158201444");
}

#[test]
fn webhook_text_message_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.HBgLMTY1MDM4Nzk0MzkVAgASGBQzQTRBNjU5OUFFRTAzODEwMTQ0RgA=",
        "timestamp": "1749416383",
        "type": "text",
        "text": { "body": "Does it come in another color?" }
    }));

    let value = &payload.entry[0].changes[0].value;
    assert_eq!(payload.object, "whatsapp_business_account");
    assert_eq!(value.metadata.phone_number_id, "106540352242922");
    let contact = &value.contacts.as_ref().unwrap()[0];
    assert_eq!(contact.wa_id, "16505551234");
    assert_eq!(contact.profile.name, "Sheena Nelson");

    let message = &value.messages.as_ref().unwrap()[0];
    assert_eq!(message.from, "16505551234");
    assert!(matches!(
        message.message_type,
        NotificationMessageType::Text
    ));
    assert_eq!(
        message.text.as_ref().unwrap().body,
        "Does it come in another color?"
    );
}

#[test]
fn webhook_reply_with_context_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.reply",
        "timestamp": "1749416383",
        "context": { "from": "15550783881", "id": "wamid.original" },
        "type": "text",
        "text": { "body": "Yes" }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let context = message.context.as_ref().unwrap();
    assert_eq!(context.id.as_deref(), Some("wamid.original"));
    assert_eq!(context.from.as_deref(), Some("15550783881"));
    assert_eq!(context.forwarded, None);
}

#[test]
fn webhook_button_reply_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.button",
        "timestamp": "1749416383",
        "type": "interactive",
        "interactive": {
            "type": "button_reply",
            "button_reply": { "id": "yes-id", "title": "Yes" }
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert!(matches!(
        message.message_type,
        NotificationMessageType::Interactive
    ));
    let reply = message
        .interactive
        .as_ref()
        .unwrap()
        .button_reply
        .as_ref()
        .unwrap();
    assert_eq!(reply.id, "yes-id");
    assert_eq!(reply.title, "Yes");
}

#[test]
fn webhook_list_reply_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.list",
        "timestamp": "1749416383",
        "type": "interactive",
        "interactive": {
            "type": "list_reply",
            "list_reply": { "id": "row-2", "title": "Second", "description": "More detail" }
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let reply = message
        .interactive
        .as_ref()
        .unwrap()
        .list_reply
        .as_ref()
        .unwrap();
    assert_eq!(reply.id, "row-2");
    assert_eq!(reply.description.as_deref(), Some("More detail"));
}

#[test]
fn webhook_image_message_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.image",
        "timestamp": "1749416383",
        "type": "image",
        "image": {
            "caption": "This is a caption",
            "mime_type": "image/jpeg",
            "sha256": "NTRkYjI1ZjQ0ZTM1NWQ2ZWQyNjdhZDA1ZWQ5YzYwZTQ5ZDk2Mzc3NDc3MzM2NDRlODY1ZjA=",
            "id": "1003383421387256"
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let image = message.image.as_ref().unwrap();
    assert_eq!(image.id, "1003383421387256");
    assert_eq!(image.caption.as_deref(), Some("This is a caption"));
    assert_eq!(image.mime_type.as_deref(), Some("image/jpeg"));
}

#[test]
fn webhook_location_message_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.location",
        "timestamp": "1749416383",
        "type": "location",
        "location": {
            "latitude": 37.4845,
            "longitude": -122.1478,
            "name": "Philz Coffee",
            "address": "101 Forest Ave, Palo Alto, CA 94301"
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let location = message.location.as_ref().unwrap();
    assert_eq!(location.latitude, 37.4845);
    assert_eq!(location.longitude, -122.1478);
    assert_eq!(location.name.as_deref(), Some("Philz Coffee"));
}

#[test]
fn webhook_sent_status_deserializes() {
    let payload = webhook(json!({
        "messaging_product": "whatsapp",
        "metadata": { "display_phone_number": "15550783881", "phone_number_id": "106540352242922" },
        "statuses": [{
            "id": "wamid.sent",
            "status": "sent",
            "timestamp": "1750263773",
            "recipient_id": "16505551234",
            "conversation": {
                "id": "6ceb9d929c3c5bb3a01c6e5f4b3c5c8b",
                "expiration_timestamp": "1750350180",
                "origin": { "type": "service" }
            },
            "pricing": { "billable": true, "pricing_model": "PMP", "category": "service", "type": "regular" }
        }]
    }));

    let value = &payload.entry[0].changes[0].value;
    assert!(value.messages.is_none());
    let status = &value.statuses.as_ref().unwrap()[0];
    assert_eq!(status.id, "wamid.sent");
    assert_eq!(status.recipient_id, "16505551234");
    assert!(matches!(status.status, whatsapp::models::StatusCode::Sent));
    assert_eq!(
        status.conversation.as_ref().unwrap().origin.origin_type,
        "service"
    );
    assert_eq!(status.pricing.as_ref().unwrap().pricing_model, "PMP");
}

#[test]
fn webhook_failed_status_deserializes() {
    let payload = webhook(json!({
        "messaging_product": "whatsapp",
        "metadata": { "display_phone_number": "15550783881", "phone_number_id": "106540352242922" },
        "statuses": [{
            "id": "wamid.failed",
            "status": "failed",
            "timestamp": "1750263773",
            "recipient_id": "16505551234",
            "errors": [{
                "code": 131047,
                "title": "Re-engagement message",
                "message": "Re-engagement message",
                "error_data": {
                    "details": "Message failed to send because more than 24 hours have passed since the customer last replied to this number."
                }
            }]
        }]
    }));

    let status = &payload.entry[0].changes[0].value.statuses.as_ref().unwrap()[0];
    assert!(matches!(
        status.status,
        whatsapp::models::StatusCode::Failed
    ));
    let error = &status.errors.as_ref().unwrap()[0];
    assert_eq!(error.code, 131047);
    assert_eq!(error.title, "Re-engagement message");
}

#[test]
fn webhook_reaction_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.reaction",
        "timestamp": "1749416383",
        "type": "reaction",
        "reaction": { "message_id": "wamid.original", "emoji": "\u{1F44D}" }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert!(matches!(
        message.message_type,
        NotificationMessageType::Reaction
    ));
    let reaction = message.reaction.as_ref().unwrap();
    assert_eq!(reaction.message_id, "wamid.original");
    assert_eq!(reaction.emoji.as_deref(), Some("\u{1F44D}"));
}

#[test]
fn webhook_removed_reaction_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.reaction",
        "timestamp": "1749416383",
        "type": "reaction",
        "reaction": { "message_id": "wamid.original" }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert_eq!(message.reaction.as_ref().unwrap().emoji, None);
}

#[test]
fn webhook_shared_contacts_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.contacts",
        "timestamp": "1749416383",
        "type": "contacts",
        "contacts": [{
            "name": { "formatted_name": "Barbara Johnson", "first_name": "Barbara", "last_name": "Johnson" },
            "phones": [{ "phone": "+1 (940) 555-1234", "wa_id": "19405551234", "type": "CELL" }]
        }]
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert!(matches!(
        message.message_type,
        NotificationMessageType::Contacts
    ));
    let contact = &message.contacts.as_ref().unwrap()[0];
    assert_eq!(contact.name.formatted_name, "Barbara Johnson");
    assert_eq!(contact.name.first_name.as_deref(), Some("Barbara"));
    let phone = &contact.phones.as_ref().unwrap()[0];
    assert_eq!(phone.phone.as_deref(), Some("+1 (940) 555-1234"));
    assert_eq!(phone.wa_id.as_deref(), Some("19405551234"));
    assert_eq!(phone.phone_type.as_deref(), Some("CELL"));
}

#[test]
fn webhook_request_welcome_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.welcome",
        "timestamp": "1749416383",
        "type": "request_welcome"
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert!(matches!(
        message.message_type,
        NotificationMessageType::RequestWelcome
    ));
}

#[test]
fn webhook_unmodelled_message_type_deserializes_as_unknown() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.future",
        "timestamp": "1749416383",
        "type": "some_future_type",
        "some_future_type": { "anything": true }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    assert!(matches!(
        message.message_type,
        NotificationMessageType::Unknown
    ));
}

#[test]
fn webhook_video_without_filename_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.video",
        "timestamp": "1749416383",
        "type": "video",
        "video": {
            "caption": "Look",
            "mime_type": "video/mp4",
            "sha256": "Ygo0I7ONQEYDBVB2g1xMlx4sP2vYz3tBC0Gb3N3SNLk=",
            "id": "1308624870840587"
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let video = message.video.as_ref().unwrap();
    assert_eq!(video.id, "1308624870840587");
    assert_eq!(video.filename, None);
    assert_eq!(video.caption.as_deref(), Some("Look"));
}

#[test]
fn webhook_system_user_changed_number_deserializes() {
    let payload = incoming(json!({
        "from": "16505551234",
        "id": "wamid.system",
        "timestamp": "1749416383",
        "type": "system",
        "system": {
            "body": "Sheena changed from 16505551234 to 16505559999",
            "new_wa_id": "16505559999",
            "type": "user_changed_number"
        }
    }));

    let message = &payload.entry[0].changes[0].value.messages.as_ref().unwrap()[0];
    let system = message.system.as_ref().unwrap();
    assert_eq!(system.system_type.as_deref(), Some("user_changed_number"));
    assert_eq!(system.new_wa_id.as_deref(), Some("16505559999"));
    assert_eq!(system.identity, None);
}
