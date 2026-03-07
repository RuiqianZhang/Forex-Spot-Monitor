# Forex Spot Monitor

> [English Version](README.en.md) | [中文说明](README.md)

A powerful, resoure-friendly macOS menu bar widget for real-time monitoring of global forex, precious metals, US Dollar Index, crude oil, and other financial instruments.

This project is built on **Tauri v2 + Vue 3 + TailwindCSS + Rust**, offering ultra-low system resource footprint combined with a fluid, native desktop experience.

---

## ✨ Core Features

- **Blazing Fast Menu Bar Overview**: Real-time display of the latest quotes and changes for the currently selected instrument right in your macOS menu bar. Support one-click fast dropdown switching.
- **Dynamic Data-Driven Engine**: Both data source endpoints and instrument lists are driven by a local external JSON config. Flexibly scale and adapt to any nodes without touching code.
- **Modern Visual Configuration Center**: A built-in config window crafted with a "frosted glass" visual language. Supports visual CRUD of source nodes, custom HTTP Headers/Params/Body payloads, and ad-hoc connection testing.
- **High-Order Extraction Engine**:
  - Contains a **Universal `JSON_FIELDS` ruleset** to extract quoting data from any strictly formatted JSON object hierarchies.
  - Built-in native memory interception mechanism for high-performance slicing of specific sources (e.g., Sina Forex micro-strings).
- **Anti-Scraping Mock Strategy**: The underlying network initiator uses Rust TLS encapsulation, allowing flexible browser fingerprint impersonation (e.g., Impersonate Chrome) and bypassing strict SSL verification.
- **Human-Centric Interaction**: Auto-refreshes quotes every second, offers brute-force test request abortion, and features instant hot-swapping between English and Chinese UI.

---

## 🚀 Architecture

- **Backend Hub (Rust)**: Leverages `tauri`, `reqwest`, and `curl-cffi` to build ultra-lightweight network tunnels and caching queues for nanosecond-level memory interaction.
- **Frontend Panel (Vue 3)**: Combines `TailwindCSS` to render a modern configuration UI complete with immersive blurred background effects (`src/App.vue`).
- **Internationalization (TypeScript)**: Zero-reboot dynamic language distribution layer (`src/i18n.ts`).

---

## 🛠️ Setup and Development

Make sure you have installed the latest Node.js (v20+) and Rust toolchain on your system.

```bash
cd tauri-v2

# 1. Install frontend console dependencies
npm install

# 2. Launch Tauri Co-debug server with HMR enabled
npm run tauri dev
```

---

## 📦 Build for Release

If you wish to compile an independent, releasable macOS `.app` bundle or `.dmg` image, run the following command:

```bash
cd tauri-v2
npm run tauri build
```
The compiled artifacts will be saved under the `tauri-v2/src-tauri/target/release/bundle/` directory.

---

## ⚙️ Data Source & Parser Configuration Guide

One of the project's highlights is supporting almost all API-based (JSON or plain text) financial data services via its built-in configuration window (Click "Settings -> Edit Config..." from the menu bar) without altering any code.

### 1. Configure Providers (Data Sources)
A provider defines **how the app sends network requests** to a target server to retrieve raw data.

In the "Provider Configuration" tab, you can:
- **Define Base Request**: Enter the target `URL Template` (e.g., `https://api.example.com/price/{symbol}`, where `{symbol}` is automatically substituted during the request) and select the `GET/POST` method.
- **Custom HTTP Payload**: If the API requires authentication or specific payloads, expand the accordion panel below to supply `Headers`, `Params` (URL Query Params), or `Body` in standard JSON format.
- **Anti-Bot Evasion**: For strict APIs, enable `Impersonate Chrome` to inject TLS handshake fingerprints resembling a real browser, or disable `SSL Verify` to bypass non-standard certificate nodes.
- **Testing Sandbox**: Enter an existing symbol in the right-side test panel and click "Send". The app will instantly fetch the raw return (`Response Body`) and network status, helping you confidently establish availability.

### 2. Configure Parsers & Instruments
Once a "Provider" successfully yields data, the app needs to know **how to extract the desired price action** from the raw response body.

Switch to the "Parser Configuration" tab and manage your `Instruments` under the corresponding provider:
- **Map Instruments**: Set your preferred `Label` (which appears in the menu, e.g., `Gold`), precisely input the `Symbol` recognizable by the upstream API (e.g., `XAU/USD`), and group them.
- **Select Parser Engine (Parser Type)**:
  - **Built-in Fixed Parsers** (`sina_forex_text`, `swissquote_bbo`, etc.): Rust regex slicers tailor-made for historical mainstream sources. Use them directly for extremely high performance if your data matches these platforms.
  - **Universal JSON Engine** (`JSON_FIELDS`): If integrating a modern or niche API, as long as the target returns standard JSON, choose this engine.

#### 💡 Universal JSON Engine (`JSON_FIELDS`) Path Extraction Rules
When selecting `JSON_FIELDS`, you must explicitly define hierarchical paths to let the engine "follow the map". Paths use classic **dot notation**, for example, if the API returns:
```json
{
  "code": 200,
  "data": { "rates": { "last_price": 2730.55, "ask": 2730.60, "bid": 2730.50 } }
}
```
You only need to visually configure:
- **Price Path** -> Required, write `data.rates.last_price`
- **Bid/Ask Path** -> Optional, write `data.rates.bid` and `data.rates.ask`

**Special Syntax: Arrays and List Items**
For responses wrapped in arrays, like `{"quotes": [{"price": 100}]}`, you can directly use numeric indices in the path to enter the array:
- **Price Path** -> `quotes.0.price`

This realizes the "Zero-Code Integration" concept!

---

## 🔒 Configuration Persistence

All data models modified through the Visual Center are persistently encrypted, deserialized, and saved in your system-level application directory:
- 📂 `~/Library/Application Support/Forex Spot Monitor/data_sources.json`

If the file is accidentally deleted or corrupted, the project will automatically fall back and operate on a preset default downgrade list.
