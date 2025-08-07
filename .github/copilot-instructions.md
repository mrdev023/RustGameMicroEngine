# RustGameMicroEngine - GitHub Copilot Instructions

Always follow these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Project Overview
RustGameMicroEngine is a 3D graphics/game engine written in Rust using WGPU for cross-platform GPU graphics rendering, Winit for window management, and supporting both native desktop and WebAssembly targets. The engine renders 3D models with textures and lighting using modern graphics APIs.

## Working Effectively

### Essential Setup and Build Commands
Run these commands to bootstrap, build, and test the repository:

```bash
# Check Rust installation (should be 1.80+)
rustc --version && cargo --version

# Initial dependency check - takes 2-3 minutes on first run
cargo check
# Build times: First run ~2m 30s (dependency download), subsequent ~5s

# Debug build - takes ~40 seconds, NEVER CANCEL
cargo build
# NEVER CANCEL: Build takes 40 seconds. Set timeout to 60+ minutes for safety.

# Release build - takes ~90 seconds, NEVER CANCEL  
cargo build --release
# NEVER CANCEL: Release build takes 90 seconds. Set timeout to 60+ minutes for safety.

# Run tests - takes ~3 seconds
cargo test
# Test the render subcrate specifically
cargo test -p render
```

### Running the Application
```bash
# Run the 3D graphics application (native desktop)
cargo run
# Note: Requires display/GPU. May fail in headless environments.

# Check application compilation without running
cargo check
```

### Code Quality and Linting
```bash
# Format code (required before commits)
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run Clippy linting (has known warnings in current codebase)
cargo clippy --all-targets --all-features
# Note: Currently fails due to unused imports and dead code warnings.
# This is normal for this codebase - the engine has placeholder/future code.
```

### WebAssembly Build (CURRENTLY BROKEN)
```bash
# Install wasm-pack (first time only) - takes ~5 minutes
cargo install wasm-pack

# WASM build - CURRENTLY FAILS due to dependency version conflicts
wasm-pack build --target web --dev
# Known issue: wasm-bindgen version incompatibility with current Rust version
# Error: "older versions of the `wasm-bindgen` crate are incompatible with current versions of Rust; please update to `wasm-bindgen` v0.2.88"

# Alternative web build script (also fails with same issue)
./run_web.sh dev
```

## Build Times and Timeouts
- **CRITICAL**: Set timeouts of 60+ minutes for all build commands to prevent premature cancellation
- **NEVER CANCEL** any build or test command - let them complete naturally
- Initial dependency download: 2-3 minutes (only first time)
- Debug build: ~40 seconds  
- Release build: ~90 seconds
- Tests: <5 seconds
- Code formatting: <5 seconds
- Dependency check: <10 seconds

## Validation Steps
Always run these validation steps after making changes:

1. **Build validation**:
   ```bash
   cargo check    # Quick syntax/dependency check
   cargo build    # Full debug build
   cargo test     # Run all tests
   cargo fmt      # Format code
   ```

2. **Manual testing scenarios**:
   - Build and run the application: `cargo run`
   - Verify 3D graphics rendering (if display available)
   - Test resource loading (models and textures in `res/` directory)

3. **Before committing changes**:
   ```bash
   cargo fmt      # ALWAYS format code
   cargo build    # Ensure debug build works
   cargo test     # Ensure tests pass
   ```

## Repository Structure
```
├── src/                    # Main engine source code
│   ├── lib.rs             # Main engine library  
│   ├── main.rs            # Application entry point
│   ├── camera.rs          # Camera system
│   ├── model.rs           # 3D model loading
│   ├── texture.rs         # Texture management
│   ├── resources.rs       # Resource loading
│   └── render/            # Rendering system
├── crates/                # Sub-crates
│   └── render/            # Graphics rendering crate
├── res/                   # 3D models, textures, assets
│   ├── *.obj              # 3D models
│   ├── *.png, *.jpg       # Textures
│   └── *.mtl              # Material definitions
├── Cargo.toml             # Main project configuration
├── build.rs               # Build script (copies res/ to output)
├── index.html             # Web application HTML
└── run_web.sh             # Web build script
```

## Key Technologies and Dependencies
- **WGPU**: Cross-platform graphics API abstraction
- **Winit**: Cross-platform windowing
- **cgmath**: Mathematics for 3D graphics
- **tobj**: 3D model loading (.obj files)
- **image**: Texture loading (PNG, JPEG)
- **Tracy**: Performance profiling (native only)
- **wasm-bindgen**: WebAssembly bindings (web target)

## Common Tasks and Outputs

### Repository Root Listing
```
Cargo.lock
Cargo.toml  
README.md
build.rs
crates/
index.html
res/
run_web.sh
src/
```

### Sample Cargo.toml (Main Dependencies)
```toml
[dependencies]
render = { path = "crates/render" }
anyhow = "1.0"
bytemuck = { version = "1.13", features = ["derive"] }
cgmath = "0.18"
env_logger = "0.10"
pollster = "0.3.0"
log = "0.4"
rayon = "1.7"
tobj = { version = "3.2", features = ["async"]}
wgpu = { version = "0.15" }
winit = "0.28"
instant = "0.1"
image = { version = "0.24", default-features = false, features = ["png", "jpeg"] }
```

## Known Issues and Limitations
- **WebAssembly build fails** due to wasm-bindgen version compatibility issues
- **Clippy warnings**: Codebase has intentional unused code for future features
- **Graphics requirements**: Application requires GPU and display - cannot run headless
- **No CI/workflows**: Repository has no automated testing setup
- **Minimal test coverage**: Only basic tests exist in render crate

## Troubleshooting
- **Build fails with dependency errors**: Run `cargo clean` then `cargo build`
- **WASM build fails**: Known issue with dependency versions - document as broken
- **Graphics application won't start**: Requires display/GPU - expected in headless environments
- **Long build times**: Normal - NEVER CANCEL, always wait for completion

## Development Workflow
1. Make code changes in `src/` or `crates/render/src/`
2. Test changes: `cargo check && cargo build && cargo test`
3. Format code: `cargo fmt`
4. Run application to verify: `cargo run` (if display available)
5. For web changes: Note that WASM build is currently broken
6. Always document timing expectations when adding new build steps