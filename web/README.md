# chrome2moz — Web UI

A WebAssembly-powered web interface for converting Chrome MV3 extensions to Firefox-compatible format. Features a distinctive amber/forge-inspired dark theme with refined typography and atmospheric design.

## Features

- **Drag & Drop Upload**: Simply drag your Chrome extension ZIP file onto the interface
- **Real-time Analysis**: Get instant feedback on compatibility issues before conversion
- **One-Click Conversion**: Convert your extension with a single click
- **Detailed Reports**: View comprehensive analysis of incompatibilities and fixes grouped by category
- **Keyboard Shortcut Conflicts**: Detect and resolve conflicts with 60+ Firefox shortcuts
- **Instant Download**: Download the converted Firefox extension immediately
- **Distinctive Design**: Warm amber aesthetic, noise texture, ember glow atmosphere, and DM Serif Display typography

## Building the Web UI

### Prerequisites

1. **Rust and Cargo**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack**:
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

### Build Instructions

1. Build the WebAssembly module:
   ```bash
   ./build-wasm.sh
   ```

   This will:
   - Compile the Rust code to WebAssembly
   - Generate JavaScript bindings
   - Output everything to `web/pkg/`

2. Serve the web application:

   **Option 1 — Python:**
   ```bash
   cd web
   python3 -m http.server 8080
   ```

   **Option 2 — Node.js:**
   ```bash
   npx http-server web -p 8080 -c-1
   ```

   **Option 3 — Rust miniserve:**
   ```bash
   cargo install miniserve
   miniserve web -p 8080
   ```

3. Open your browser to `http://localhost:8080`

## Usage

1. **Upload Extension**: Click or drag & drop your Chrome extension ZIP file
2. **Review Analysis**: Check the compatibility analysis results with collapsible sections
3. **Resolve Shortcuts**: Optionally choose alternative keyboard shortcuts for Firefox
4. **Convert**: Click "Convert Extension" to transform it for Firefox
5. **Download**: Download the converted `.xpi` or `.zip` file
6. **Test in Firefox**:
   - Open Firefox
   - Go to `about:debugging#/runtime/this-firefox`
   - Click "Load Temporary Add-on"
   - Select the downloaded file

## Technical Details

### Architecture

The web UI consists of three main components:

1. **WebAssembly Module** (`src/wasm.rs`):
   - Rust code compiled to WASM
   - Handles all conversion logic
   - Provides: `analyze_extension_zip()`, `convert_extension_zip()`, `analyze_keyboard_shortcuts()`

2. **Frontend UI** (`web/`):
   - `index.html`: Semantic HTML5 structure with ARIA support
   - `styles.css`: Responsive amber-themed design with noise texture and ember glow
   - `app.js`: JavaScript logic, WASM interface, collapsible sections, shortcut selectors

3. **Build System**:
   - `build-wasm.sh`: Defensive Bash script with `set -Eeuo pipefail`, trap cleanup, structured logging

### File Structure

```
web/
├── README.md          # This file
├── index.html         # Main HTML page (DM Serif Display + Inter fonts)
├── styles.css         # Amber/forge dark theme with noise & gradient overlays
├── app.js            # Frontend JavaScript (WASM bridge, collapsible UI)
└── pkg/              # Generated WASM files (after build)
    ├── chrome2moz.js
    ├── chrome2moz_bg.wasm
    └── ...
```

## Design

The interface uses a **forge/ember** aesthetic:
- **Colors**: Warm amber accents (`#f59e0b`, `#d97706`) against deep charcoal (`#0c0a09`)
- **Typography**: DM Serif Display for headings, Inter for body text
- **Atmosphere**: Noise texture overlay, radial gradient glows, subtle border gradients
- **Motion**: Smooth cubic-bezier transitions, glow effects on hover, rotating logo mark

## Browser Compatibility

The web UI requires a modern browser with WebAssembly support:

- ✅ Chrome/Edge 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Opera 44+

## Development

### Rebuilding After Changes

If you modify the Rust code, rebuild the WASM module:

```bash
./build-wasm.sh
```

Then refresh your browser (hard refresh: Ctrl+Shift+R or Cmd+Shift+R).

### Debugging

1. **Browser Console**: Check for JavaScript errors or WASM loading issues
2. **Network Tab**: Verify that WASM files are loading correctly
3. **Rust Panics**: Will appear in the browser console (thanks to `console_error_panic_hook`)

### Code Quality

```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

## Deployment

To deploy the web UI to a static hosting service:

1. Build the WASM module:
   ```bash
   ./build-wasm.sh
   ```

2. Deploy the entire `web/` directory to your hosting service:
   - GitHub Pages
   - Netlify
   - Vercel
   - Cloudflare Pages
   - Any static hosting service

3. Ensure the server serves `.wasm` files with the correct MIME type:
   ```
   application/wasm
   ```

## Limitations

- Maximum file size depends on browser memory limits (typically 100–200 MB)
- Processing happens client-side, so complex extensions may take longer
- No server-side storage — all data stays in your browser

## Troubleshooting

### "Failed to load WASM module"

- Ensure you've run `./build-wasm.sh` first
- Check that `web/pkg/` directory exists and contains `.wasm` files
- Verify your web server is serving `.wasm` files correctly
- Try a hard refresh (Ctrl+Shift+R)

### "Conversion failed"

- Check browser console for detailed error messages
- Ensure the uploaded file is a valid Chrome extension ZIP
- Verify the ZIP contains a valid `manifest.json`

### Build Errors

- Ensure Rust and wasm-pack are installed correctly
- Try updating wasm-pack: `cargo install wasm-pack --force`
- Check that all dependencies in `Cargo.toml` are available

## Contributing

Contributions are welcome! To work on the web UI:

1. Make changes to `web/*.html/css/js` or `src/wasm.rs`
2. Rebuild: `./build-wasm.sh`
3. Test locally using a local web server
4. Run quality checks: `cargo clippy && cargo test`
5. Submit a pull request

## License

MIT License — See main project LICENSE file
