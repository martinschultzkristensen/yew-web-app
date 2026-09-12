# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

danceOmatic (Cargo package `yew-app`, Tauri identifier `com.artfarm.danceomatic`) is a kiosk-style desktop
app for an interactive dance installation (https://artfarm.dk/repertoire/danceomatic). It runs fullscreen on
a touch-free keyboard interface: it loops a demo video, and the visitor cycles between choreography demos
with `W`/`S`, views info with `R`, and launches the full choreography video with `E`. `Ctrl+Shift+A` opens a
hidden admin panel for editing the on-machine config; `Q` restarts back to the intro loop.

Frontend and backend are two separate Cargo crates in one repo:
- **`src/`** — the Yew (Rust→WASM) frontend, built with Trunk.
- **`src-tauri/`** — the Tauri (native Rust) backend/shell that hosts the WASM frontend, exposes
  `#[tauri::command]`s the frontend calls via `invoke(...)`, and does all filesystem/network work.

## Commands

Dev and build always go through the Tauri CLI, which itself drives Trunk for the frontend (see
`src-tauri/tauri.conf.json`'s `beforeDevCommand`/`beforeBuildCommand: trunk serve`/`trunk build`). Don't run
`trunk` directly for normal iteration — `cargo tauri dev` does it for you and rebuilds on change.

```bash
npm run dev              # cargo tauri dev — full app (frontend + backend), hot-reloads on change
npm run build            # cargo tauri build — production bundle for the current host
```

Deployment target is a PC running Ubuntu (a `build:rpi` script for a Raspberry Pi target existed historically
but is no longer used).

Equivalent raw commands: `cargo tauri dev` / `cargo tauri build`. There is no lint/format/test setup
configured in this repo (no `#[test]` functions, no CI workflow) — verify changes by running the app.

Frontend-only type checking (fast, no Tauri window) is possible with `trunk build` from the repo root, since
`src/` is the default crate in the workspace `Cargo.toml`. Backend-only checking: `cargo check` inside
`src-tauri/`.

## Architecture

### Frontend (`src/`) — Yew, atomic-design component layout

`src/lib.rs` defines the `DanceOmatic` root component and the `Route` enum (yew-router). Routing is a
straight-line state machine driven by keypresses rather than links:

```
IntroScreen ("/") --x--> MainMenu ("/main-menu") --e--> ChoreoVideo ("/choreo-video")
      ^                        |--r--> AboutChoreo ("/about-choreo/:n")
      |                        |--q--> back to IntroScreen
      +-------- video ended ---+
```
`AdminPanel` ("/admin-panel") is reachable only via the global `Ctrl+Shift+A` listener registered in
`DanceOmatic`. `LoadScreenVideo` exists as a route but isn't wired into the keyboard flow above.

Components are organized under `src/components/` by atomic-design tier:
- `atoms/` — small stateless pieces (`dancer`, `dance_o_matic_logo`, `arrow_respnd_ui`, `use_focus_div` hook,
  `shared_props::AppConfigProps` — the `{ config: Rc<Config>, choreo_number }` props struct shared by every
  route-level component).
- `molecules/` — composed behavior: `video_list` (renders the active `<video>`, resolves its src — see
  below), `keydown_logic::get_toggle_key` (shared W/S video-cycling handler), `music_context` and
  `sound_effects` (see Audio below), `video_settings`, `btn_explainer_graphics`, `scollable_div`.
- `organisms/` — the route-level screens: `intro_screen`, `main_menu`, `choreo_videos`, `about_choreo`,
  `load_screen`, `admin_panel`.
- `data/` — `config.rs` (the `Config` struct deserialized from `config.toml`/backend, with helpers like
  `get_demo_videos()`/`load_choreo_videos()`/`load_dancers()` that convert config rows into the view-model
  types `VideoType`/`Video`/`DemoVideo`/`Dancer`), plus `choreography_data`, `image_imports`, `video_imports`.

State is passed top-down as `Rc<Config>` (fetched once in `DanceOmatic` via the `get_config` Tauri command,
then cloned into `AppConfigProps` for every route) rather than through a global store. Cross-cutting features
(audio) use Yew context providers instead: `MusicContextProvider` and `SoundEffectsProvider` both wrap the
whole app in `lib.rs` and expose `Callback`-based APIs (`start_music`/`stop_music`, `play_sound`) via
`use_context`.

**Video source resolution** (`molecules/video_list.rs`): a video's `url` from config is either a bundled
`static/...` path (used directly) or a `media/...`/`delivery/...` path from user-imported or
remotely-delivered content, which must be resolved through the backend's `resolve_video_path` command and
then through Tauri's `convertFileSrc` before it's usable in an HTML `<video src>`.

### Backend (`src-tauri/`)

`src-tauri/src/lib.rs` is the Tauri entrypoint (`run()`): registers plugins (`fs`, `dialog`, `log`), sets up
the app-data `media/` directory and external `config.toml` on first launch, registers the custom `media://`
URI scheme handler (byte-range-aware file serving, used to work around WebKitGTK on Linux not honoring range
requests on the frontend's static asset route), starts the local HTTP media server, and registers every
`#[tauri::command]` in `invoke_handler!`. `Config` (dancers/demo_videos/choreo_videos/intro_video/
loadscreen_video, TOML-backed) is defined here and mirrored by a near-identical struct in
`src/components/data/config.rs` on the frontend side — keep both in sync when the schema changes.

Key backend modules:
- **`commands.rs`** — most `#[tauri::command]`s: config load/reset (`get_config` — falls back from active
  machine-delivery config to `config.toml` if the delivery lookup fails), media import (`import_video`/
  `import_images`, used by the admin panel), path resolution (`resolve_media_path`/`resolve_video_path`,
  windows/android use the `media://` custom protocol, other platforms use the local HTTP server), file
  pickers (`select_video_file`/`select_img_file`), and the sound-effect byte cache (`AudioCache` /
  `get_audio_effect`, preloaded once at startup into `TauriState`).
- **`local_media_server.rs`** — spawns a dedicated OS thread with its own single-threaded Tokio runtime
  running an Axum server on `127.0.0.1:17847`, serving `/media` (the app-data media dir) and `/delivery` (the
  active machine-delivery deployment dir) as static file trees. Used instead of the `media://` protocol on
  platforms where that isn't viable.
- **`path_utils.rs`** — the two canonical on-disk locations: `external_config_path()` (`<config_dir>/<app
  name>/config.toml`) and `media_dir()` (`<data_dir>/<app name>/media`).
- **`supabase_sync.rs`** — talks to a Supabase backend (URL/key in `resources/supabase.toml`) for the
  "machine" pairing/update flow: authenticates a machine session, fetches a `MachineManifest`/
  `MachineDelivery` describing the choreography content that should be installed, and drives downloads.
- **`machine_delivery_store.rs`** — the local side of that flow: manages `machine-delivery/{staging,
  deployments,factory}` directories and `state.json`, stages a downloaded delivery, activates it (promoting
  staging → deployments and updating the active/previous pointers), and can build a `Config` from the active
  delivery's manifest instead of the bundled `config.toml`. `install_latest_machine_delivery_if_new` runs
  this whole check-download-activate sequence and is invoked both at startup and on a 10-minute timer in
  `lib.rs`'s `setup()`; if it installs a new delivery it sets `MACHINE_RESTART_PENDING`, which the frontend
  polls for (via `restart_if_machine_update_pending`) at safe points (e.g. every time the intro loop replays)
  to apply the update by restarting the app.

### Audio

Both `MusicContextProvider` (looping background music) and `SoundEffectsProvider` (one-shot SFX) fetch their
audio bytes from the backend's `get_audio_effect` command (not `<audio src=...>`) and decode them with the
Web Audio API (`AudioContext::decode_audio_data`) before playback. This is deliberate, not incidental: plain
`<audio>`/`<video src="static/...">` playback of these files fails under WebKitGTK on Linux because it
doesn't respond to range requests the way the frontend's static asset route needs — see the comment in
`music_context.rs`. Background music is started synchronously inside `intro_screen.rs`'s "x" keydown handler
(a real user-gesture callback, required by Linux/WebKitGTK's autoplay policy) and simply keeps playing across
navigation until something calls `stop_music` (leaving `MainMenu`/entering a choreo video, or `Q` to restart).

### Config and content model

There are two ways choreography/media content reaches the app, both producing the same `Config` shape:
1. **Bundled default**: `src-tauri/resources/config.toml`, copied to the external config path on first run
   and editable afterward through the admin panel (which calls back into `commands.rs`).
2. **Machine delivery**: content pushed from the Supabase backend and installed by
   `machine_delivery_store.rs`, taking priority over the external `config.toml` when present and valid
   (`get_config` tries this path first).

## Platform notes

- Built with Tauri v2. Linux is a first-class target (the kiosk runs on a PC running Ubuntu), and several
  of the choices above (the `media://` scheme, the local Axum media
  server, the Web-Audio-based audio path) exist specifically to work around WebKitGTK quirks on Linux —
  don't "simplify" those back to plain `<video src>`/`<audio src>` without checking Linux playback.
- `resources/supabase.toml` contains a *publishable* (anon) Supabase key — this is expected to be
  bundled in the app, not a secret to scrub.
