# Changelog

## [Unreleased]


## [0.4.0] - 2024-08-01

### Added

-  Methods Message::from_text_with_context() and Message::from_template_with_context() for replying to messages
-  Models to help processing incoming webhooks

### Changed

- **Breaking:** Message is now enum instead of struct, to provide stronger typig. If you are directly using Message struct (instead of using from_text & from_template), this will require some code changes.

## [0.3.1] - 2023-11-23

_Last release before changelog_

