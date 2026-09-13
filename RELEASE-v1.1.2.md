# Saesth v1.1.2

Saesth now supports Linux alongside Windows.

## Linux support

- Added keyboard and left/right mouse click sounds on Linux, under X11 and Wayland.
- Input devices are detected again when reconnected, and held keys do not trigger repeated sounds.
- Added a Wayland notice on the Setup page with the input permission command and instructions to log out and back in.
- Added `npm run build:appimage` to build on Linux with the `NO_STRIP` workaround and copy `saesth.AppImage` to the project root.

Linux input sounds require read access to the input devices. The Setup notice explains how to grant it on distributions using the `input` group.

## Interface

- Centralized the warning color palette in `App.css` and documented it in the design guide.

## Coming soon

Installation through an APT repository and the Arch User Repository (AUR), using tools such as `paru`, is planned for an upcoming release. These distribution channels are not available yet.
