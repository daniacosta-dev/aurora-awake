# Aurora Awake

A lightweight desktop utility that prevents your computer from going to sleep by moving the mouse cursor at configurable intervals.

Built with Tauri v2 + Rust + React.

## Features

- Multiple movement patterns: Line, Square, Circle, ZigZag
- Configurable interval, distance, and animation duration
- Settings persist across sessions
- Lock controls while running to prevent accidental changes
- Strict Snap confinement (X11)

## Stack

| Layer | Technology |
|---|---|
| UI | React 19 + TypeScript |
| Backend | Rust (Tauri v2) |
| Mouse control | [enigo](https://github.com/enigo-rs/enigo) |
| Bundler | Vite 7 |

## Development

```bash
npm install
npm run tauri dev
```

**Requirements:** [Rust](https://rustup.rs) · [Node 22+](https://nodejs.org) · WebKit2GTK (`libwebkit2gtk-4.1-dev` on Debian/Ubuntu)

## Build

```bash
npm run tauri build
```

Generates `.AppImage`, `.deb`, and `.rpm` in `src-tauri/target/release/bundle/`.

## License

MIT

Made with ❤️ and Rust · MIT License · [Ko-fi](https://ko-fi.com/daniacostadev)
Created by [@daniacosta-dev](https://github.com/daniacosta-dev)