# 🚀 Rust + WebAssembly Demo

This project demonstrates how to use Rust compiled to WebAssembly (WASM) for heavy computations (like recursive Fibonacci), and compare it with the same task written in JavaScript — all inside a browser.

## 📁 Folder Structure

```
wasm_demo/
├── Cargo.toml               # Rust project config
├── src/
│   └── lib.rs              # Rust code for Fibonacci

```

## 🛠️ How to Use This Project

Follow these steps to build, run, and test this Rust + WebAssembly demo in your browser.

### ✅ Step 1: Clone the Repository

```bash
git clone https://github.com/ParthNS1/rust-wasm-demo.git
cd rust-wasm-demo
```

### ✅ Step 2: Install Required Tools

**Install Rust** (if not already installed):
👉 [https://rustup.rs/](https://rustup.rs/)

Then install the WASM target and wasm-bindgen:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

### ✅ Step 3: Build for WebAssembly

Compile the project to a `.wasm` file:

```bash
cargo build --release --target wasm32-unknown-unknown
```

### ✅ Step 4: Generate JS Bindings and out/ Folder

Use wasm-bindgen to convert the raw `.wasm` file into a browser-usable module:

```bash
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_demo.wasm \\
  --out-dir out \\
  --target web
```

This creates the following inside `out/`:
- `wasm_demo_bg.wasm` (WebAssembly binary)
- `wasm_demo.js` (JavaScript loader)

### ✅ Step 5: Run in the Browser

Open `docs/index.html` in your browser.

Make sure the following files are in the same folder:

```
docs/
├── index.html
├── wasm_demo.js           ← copy from `out/`
└── wasm_demo_bg.wasm      ← copy from `out/`
```

## 🎯 What This Demo Shows

- **Performance Comparison**: Direct comparison between JavaScript and WebAssembly execution times
- **Heavy Computation**: Uses Naive recursive Fibonacci sum algorithm calculation as a CPU-intensive task
- **Browser Integration**: Shows how to load and use WASM modules in web browsers
- **Real-world Application**: Demonstrates practical use of Rust compiled to WebAssembly

## 🔧 Technologies Used

- **Rust**: Systems programming language
- **WebAssembly (WASM)**: Binary instruction format for web browsers
- **wasm-bindgen**: Tool for generating JavaScript bindings for Rust-generated WebAssembly
- **HTML/JavaScript**: Frontend for testing and comparison

## 📊 Expected Results

You should observe that the Rust/WASM implementation significantly outperforms the JavaScript version for computationally intensive tasks like recursive Fibonacci calculations.
