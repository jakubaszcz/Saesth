# Saesth — Work Aesthetically

Saesth is a lightweight Windows application designed to make your workspace feel calmer and more enjoyable.

Create your own atmosphere with relaxing ambient sounds such as rain and other natural environments, while keeping them alongside the audio you already listen to.

Saesth also makes your interactions with your computer feel more satisfying. Keyboard presses, releases, and mouse interactions can produce soft, responsive sounds, making every action feel a little more alive.

Saesth isn't just an ambient sound player. It's a tool designed to turn your workspace into your own peaceful environment.

**Discover Saesth:** https://www.saesth.com/

![Saesth](https://www.saesth.com/og-image.png)

## Features

### Ambient sounds
Mix relaxing natural sounds and create an atmosphere that fits the way you work.

### Responsive input sounds
Give your keyboard and mouse their own satisfying sound feedback. Each interaction can subtly react and vary while you work.

### Lightweight
Saesth is built with Rust and designed to stay quietly in the background without getting in your way.

### Calm interface
The interface is designed around the same philosophy as the sounds themselves: simple, calm, and distraction-free.

### Shareable sound packs
Create packs containing custom sounds and configurations, then share them with other Saesth users.

> **Status:** Upcoming

## Download

Download the latest version of Saesth from:

https://www.saesth.com/

## License

Saesth is **source-available software**.

The source code is publicly accessible for transparency and contributions, but this does not grant permission to redistribute, sell, or create independent versions of Saesth.

You may:

- View the source code.
- Contribute to the official Saesth project.

You may not, without explicit permission:

- Redistribute Saesth or substantial parts of its source code.
- Sell Saesth or its source code.
- Create and distribute your own version of Saesth.
- Use the source code as the basis of another commercial or publicly distributed product.

See the `LICENSE` file for the complete terms.
## Building the Windows MSI

On Windows, install Node.js, Rust (MSVC), and the Visual Studio C++ build tools required by Tauri, then run:

```sh
npm ci
npm run build:msi
```

Alternatively, run `create.bat`. The script rebuilds both the frontend and Rust executable and stops if the build fails. The MSI is written to `src-tauri/target/release/bundle/msi/` (unless a custom Cargo target directory is configured). Upload the generated `.msi` to your download host; no separate `dist` or sounds folder is required.

Windows packaging options live in `src-tauri/tauri.windows.conf.json`, merged automatically with the shared configuration. Tauri's maintained WiX template replaces the old standalone `msi-installer.wxs` and manual harvesting. WebView2 is downloaded during installation only when needed, so internet access is required on machines without the runtime. The MSI is unsigned unless code signing is configured separately.

The release version comes from `tauri.conf.json`; keep it in sync with `Cargo.toml` and `package.json`. Keep the WiX upgrade code stable across releases. The retired manual installer declared version `1.2.0`, whereas this project is `0.1.0`: users who installed that old package must uninstall it before installing this lower version, or wait for a release with a higher version.

See the [Tauri Windows installer guide](https://v2.tauri.app/distribute/windows-installer/) for build prerequisites and signing options. Linux and macOS packages must be built and tested separately on their respective platforms.

### Pack asset permissions

The shared asset protocol scope starts empty. During Rust setup, only `PATHS.packs` and `PATHS.packs_cache` are authorized recursively using `asset_protocol_scope().allow_directory`. These are the same paths resolved by `directories::ProjectDirs` for pack storage on Windows, macOS and Linux. No Windows-specific paths or broad application-data access are needed, and existing packs stay in their current location. If storage paths change, these permissions follow automatically.
