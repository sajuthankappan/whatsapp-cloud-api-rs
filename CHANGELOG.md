# Changelog

## [Unreleased]

## [0.4.6] - 2024-02-04

### Added

- add mark_message_as_read method for WhatsappClient


## [0.4.5] - 2024-02-04

### Changed

- Make message_status optional in CreatedMessage struct


## [0.4.4] - 2024-02-01

### Changed

- Add message_status to CreatedMessage struct
- status string (delieverd, read, sent) is now an enum


## [0.4.3] - 2024-01-29

### Changed

- **Breaking:** conversation and pricing in Status are now optional


## [0.4.2] - 2024-01-29

### Added

- Additional model structs to help processing incoming webhooks

### Changed

- **Breaking:** contacts and messages in Value are now optional


## [0.4.1] - 2024-01-09

### Added

- Additional model structs to help processing incoming webhooks
- Contact profile to incoming webhook messages


### Changed

- **Breaking:** Message is now back as struct :)
- **Breaking:** Message::from_text() and Message::from_template() have an additional parameter context, for replying to messages. For normal messages, pass it as None.


## [0.4.0] - 2024-01-08

### Added

-  Methods Message::from_text_with_context() and Message::from_template_with_context() for replying to messages
-  Models to help processing incoming webhooks

### Changed

- **Breaking:** Message is now enum instead of struct, to provide stronger typig. If you are directly using Message struct (instead of using from_text & from_template), this will require some code changes.

## [0.3.1] - 2023-11-23

_Last release before changelog_

