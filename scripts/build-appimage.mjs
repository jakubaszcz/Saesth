import {spawnSync} from "node:child_process";
import {chmodSync, copyFileSync, readdirSync, statSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
import {fileURLToPath} from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

if (process.platform !== "linux") {
    console.error("Build the AppImage on Linux (for example, on CachyOS).");
    process.exit(1);
}

// Use a known output directory, including when Cargo has a custom configuration.
const targetDir = resolve(root, process.env.CARGO_TARGET_DIR || "src-tauri/target");
const startedAt = Date.now();
const result = spawnSync("npm", ["run", "tauri", "--", "build", "--bundles", "appimage"], {
    cwd: root,
    stdio: "inherit",
    env: {...process.env, NO_STRIP: "1", CARGO_TARGET_DIR: targetDir},
});

if (result.error || result.status !== 0) {
    console.error("AppImage build failed; no file was copied to the project root.");
    if (result.error) console.error(result.error.message);
    process.exit(result.status || 1);
}

try {
    // Cargo may use a configured build target, adding a target-triple directory.
    const metadata = spawnSync("cargo", ["metadata", "--no-deps", "--format-version", "1",
        "--manifest-path", "src-tauri/Cargo.toml"], {
        cwd: root,
        encoding: "utf8",
        env: {...process.env, CARGO_TARGET_DIR: targetDir},
    });
    if (metadata.error || metadata.status !== 0) {
        throw new Error(metadata.error?.message || metadata.stderr || "Cannot locate Cargo output.");
    }
    const output = JSON.parse(metadata.stdout).target_directory;
    const candidates = [output, ...readdirSync(output, {withFileTypes: true})
        .filter((entry) => entry.isDirectory())
        .map((entry) => join(output, entry.name))];
    const images = candidates.flatMap((directory) => {
        const bundleDir = join(directory, "release", "bundle", "appimage");
        let entries;
        try {
            entries = readdirSync(bundleDir);
        } catch (error) {
            if (error.code === "ENOENT") return [];
            throw error;
        }
        return entries.filter((name) => name.endsWith(".AppImage"))
            .map((name) => join(bundleDir, name))
            .filter((path) => {
                const info = statSync(path);
                return info.isFile() && info.mtimeMs >= startedAt - 2000;
            });
    });
    if (images.length !== 1) {
        throw new Error(`Expected one newly built AppImage, found ${images.length}. Check ${output}.`);
    }
    const destination = join(root, "saesth.AppImage");
    copyFileSync(images[0], destination);
    chmodSync(destination, 0o755);
    console.log(`\nAppImage ready: ${destination}`);
} catch (error) {
    console.error(`Could not copy the AppImage: ${error.message}`);
    process.exit(1);
}
