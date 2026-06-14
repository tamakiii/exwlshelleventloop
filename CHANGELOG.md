# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-06-07
### Added
- Feat: sessionlockev handles the `ext_session_lock_v1` `locked`/`finished` events instead of ignoring them: they are delivered as `DispatchMessage::Locked`/`DispatchMessage::Finished`, `WindowState::is_locked()`/`lock_finished()` expose the lock state, an unlock requested before `locked` is deferred until the compositor answers the lock request (avoiding the `invalid_unlock` protocol error), and after `finished` the lock object is destroyed the way the protocol requires (`destroy` vs `unlock_and_destroy`) before the event loop stops.
- Feat: iced_sessionlock delivers `locked` to the application as a message through the new `FromLockedInfo` trait (the `to_session_message` macro now also generates a `Locked` variant), and reports `finished` by returning the new `Error::LockFinished` from `run()`.
- Feat: xdg-activation (`xdg_activation_v1`) support. layershellev binds the global when available and exposes `WindowState::request_activation_token()` (token delivered as `DispatchMessage::ActivationTokenDone`, with the serial of the most recent keyboard/pointer/touch input attached) and `WindowState::activate_with_token()`. iced_layershell exposes the matching `LayerShellCustomAction::ActivationTokenRequest` with an `ActivationTokenSender` reply channel, and `to_layer_message` generates an `ActivationTokenRequest` variant plus a `request_activation_token` helper for both single and multi mode. This lets launchers and notification daemons pass `XDG_ACTIVATION_TOKEN` to spawned clients so the compositor hands them focus; the `application_launcher` example does this now.
### Changed (breaking)
- iced_sessionlock applications must implement `FromLockedInfo` for their message type (automatic when using `to_session_message`).
- Feat: `IcedNewPopupSettings`/`NewPopUpSettings` take explicit parent surface plus full xdg_positioner controls (`anchor_rect`, `anchor`, `gravity`, `constraint_adjustment`) instead of a single `position`.
- Popups now positioned by the compositor relative to their parent surface, auto-flip/slide near screen edges, can be nested, and are dismissed via an xdg_popup grab. 
### Removed (breaking)
- Removed `MenuDirection`, `IcedNewMenuSettings`, the `NewMenu` action, and `menu_open`. 
- Removed the now-unused public getters `WindowState::mouse_position()` and `WindowManager::get_alias()`

## [0.18.1] - 2026-05-09
- Fix: handle wl_pointer::Event::AxisValue120 for scrolling (#376) by @ibrahimduran
- Fix: (Message, Instant) cannot try_into to LayerShellCustomActionWithId

[0.18.1]: https://github.com/waycrate/exwlshelleventloop/compare/v0.18.1...v0.18.0

## [0.18.0] - 2026-05-01

### Changed
- Feat: Sctk refactor in #368
- Fix: high cpu usage in #364 by @Magniquck
- Feat: add iced_exwlshell library
- Feat: add client side decoration control for XDG windows (#375) by @httpsworldview

[0.18.0]: https://github.com/waycrate/exwlshelleventloop/compare/v0.18.0...v0.17.1

## [0.18.0-beta4] - 2026-04-22

### Changed
- Feat: Sctk refactor in #368
- Fix: high cpu usage in #364 by @Magniquck
- Feat: add iced_exwlshell library
- Chore: republish

[0.18.0-beta4]: https://github.com/waycrate/exwlshelleventloop/compare/v0.18.0-beta4...v0.17.1

## [0.18.0-beta3] - 2026-04-22

### Changed
- Feat: Sctk refactor in #368
- Fix: high cpu usage in #364 by @Magniquck
- Feat: add iced_exwlshell library
- Chore: republish

[0.18.0-beta3]: https://github.com/waycrate/exwlshelleventloop/compare/v0.18.0-beta3...v0.17.1

## [0.18.0-beta2] - 2026-04-22

### Changed
- Feat: Sctk refactor in #368
- Fix: high cpu usage in #364 by @Magniquck
- Feat: add iced_exwlshell library

[0.18.0-beta2]: https://github.com/waycrate/exwlshelleventloop/compare/v0.18.0-beta2...v0.17.1

## [0.17.1] - 2026-03-25

### Changed
- Fixed: background cannot run

[0.17.1]: https://github.com/waycrate/exwlshelleventloop/compare/v0.17.1...v0.17.0

## [0.17.0] - 2026-03-25

### Changed

- Fixed: single program never exits after switching to tty
- Fixed: mouse and pointer dead after switching to tty
- refactor: use winit keyboard types (#357) by @bitbloxhub

### NOTE:
It should be breaking change because the waycrate_xkbkeycode is rewritten. I forgot that.

[0.17.0]: https://github.com/waycrate/exwlshelleventloop/compare/v0.17.0...v0.16.0

## [0.16.1] - 2026-03-25

### Changed

- Fixed: single program never exits after switching to tty
- Fixed: mouse and pointer dead after switching to tty

### Others
Maybe it is wired that layershell program must die, but seems that it is needed.

[0.16.1]: https://github.com/waycrate/exwlshelleventloop/compare/v0.16.1...v0.16.0

## [0.16.0] - 2026-03-18

### Changed

- Allow changing KeyboardInteractivity at runtime by @danhandrea
- feat: make with_connection accept function
- feat(layershellev): replace use_last_output with output_option in NewInputPanelSettings (#355) by @fortime
- fix: do not panic in eventloop run, and return the error to the top

[0.16.0]: https://github.com/waycrate/exwlshelleventloop/compare/v0.16.0...v0.15.1
