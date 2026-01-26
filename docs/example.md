# nitrate — Semantic Map

**Purpose:** A high-performance, hybrid-architecture video engine targeting 8K/60fps playback with physics-based film simulation.

## Legend

`[CRITICAL]` Architectural linchpin; modification carries high risk.
`[UNSAFE]` Contains unsafe Rust blocks; requires strict manual memory/lifetime management.
`[SPIKE]` Temporary or experimental code validating the architecture.
`[WIP]` Skeleton code or incomplete implementation.
`[PROVEN]` Validated by spike; architecture confirmed working.

## Layer 0 — Config

`nitrate/crates/*/Cargo.toml`
Defines the workspace dependency graph. Exists to enforce the strict separation between the Platform Abstraction Layer, the Math/Color core, and the Application logic.

`nitrate/slopchop.toml`
Configuration for project-specific tooling. Exists to standardize build or linting parameters across the workspace.

## Layer 1 — Core

`nitrate/crates/nitrate-core/src/lib.rs`
Defines protocol-agnostic types like `FrameId` and `PixelFormat` shared by all crates. Exists to decouple the Decoder (Producer) from the Compositor (Consumer) so they agree on data structures without direct dependencies.
→ Exports: `PixelFormat`, `PlaneDesc`, `FrameId`, `Error`

## Layer 2 — Platform (PAL)

`nitrate/crates/nitrate-pal/src/lib.rs`
Re-exports backend-specific implementations under a unified API. Exists to allow upper layers to use `ImportedSurface` or `SyncTier` without binding to specific Vulkan, Metal, or DX12 types.

`nitrate/crates/nitrate-pal/src/surface.rs` `[CRITICAL]`
Defines `ImportedSurface` as a container for platform-specific handles (DMA-BUF, Shared Handles) and color metadata. Exists as the physical "handoff" contract passed from the Decoder thread to the Render thread.
→ Exports: `ImportedSurface`, `PlaneDescriptor`, `ColorMetadata`

`nitrate/crates/nitrate-pal/src/sync.rs`
Defines the `SyncTier` enum and synchronization strategies. Exists to encapsulate runtime fallback logic between efficient Timeline Semaphores (Tier A) and slower CPU Fences (Tier C).

`nitrate/crates/nitrate-pal/src/error.rs`
Defines granular error types for the Hardware Abstraction Layer. Exists to distinguish between fatal setup errors (`DeviceCreation`) and recoverable runtime errors (`Swapchain`).

`nitrate/crates/nitrate-pal/src/vulkan/mod.rs`
Bundles fragmented Vulkan sub-modules into a cohesive `VulkanDevice`. Exists to provide a single entry point for the Vulkan backend implementation.

`nitrate/crates/nitrate-pal/src/vulkan/bridge.rs` `[CRITICAL]` `[UNSAFE]` `[PROVEN]`
Wraps raw, app-created Vulkan handles into `wgpu` objects using `wgpu-hal`. Exists to implement the "Native Owns, WGPU Borrows" strategy, keeping device lifecycle control in the app while allowing wgpu to record UI commands.
→ Touch: Critical lifetime dependency; `VulkanDevice` must outlive `WgpuBridge`.
→ Proven: Spike 1, Spike 3

`nitrate/crates/nitrate-pal/src/vulkan/device.rs` `[PROVEN]`
Creates the Logical Device and selects the best Physical Device based on required extensions. Exists to ensure the hardware supports specific features like `external_memory_fd` before application startup.
→ Exports: `VulkanDevice`, `DeviceQueues`

`nitrate/crates/nitrate-pal/src/vulkan/extensions.rs`
Filters requested instance/device extensions against available ones. Exists to safely enable optional features (like Timeline Semaphores) without crashing on unsupported hardware.

`nitrate/crates/nitrate-pal/src/vulkan/queues.rs`
Iterates over queue families to find indices for Graphics, Compute, and Presentation. Exists to handle both unified and disjoint queue architectures on different GPUs.

`nitrate/crates/nitrate-pal/src/vulkan/instance.rs` `[PROVEN]`
Loads the Vulkan library and configures validation layers. Exists to bootstrap the driver connection and hook up debug callbacks.

`nitrate/crates/nitrate-pal/src/vulkan/capabilities.rs`
Queries device extensions to detect support for Timeline Semaphores and External Memory. Exists to dynamically assign the `SyncTier` based on runtime hardware features.

`nitrate/crates/nitrate-pal/src/vulkan/submit.rs` `[PROVEN]`
Native Vulkan command recording and submission with semaphore sync. Exists to provide the composition submission path independent of wgpu.
→ Exports: `CommandContext`, `SubmitParams`, `submit_commands`
→ Proven: Spike 3

`nitrate/crates/nitrate-pal/src/vulkan/import.rs` `[PROVEN]`
Wraps raw DMA-BUF file descriptors into `wgpu::Texture` objects. Currently uses staging path; true HAL import deferred until wgpu exposes external memory APIs.
→ Proven: Spike 2 (staging path)

### PAL — Export Subsystem
`nitrate/crates/nitrate-pal/src/vulkan/export/mod.rs` `[PROVEN]`
Provides a high-level interface for creating "fake" video frames. Exists to simulate a video decoder's output (allocation + export) for pipeline testing.
→ Proven: Spike 2

`nitrate/crates/nitrate-pal/src/vulkan/export/alloc.rs` `[UNSAFE]` `[PROVEN]`
Constructs `VkMemoryAllocateInfo` chains with specific export flags. Exists to ensure allocated memory is compatible with Linux DMA-BUF export mechanisms.
→ Proven: Spike 2

`nitrate/crates/nitrate-pal/src/vulkan/export/fill.rs` `[PROVEN]`
Records commands to clear an exported image with a test pattern. Exists to verify that the exported memory actually contains valid data.
→ Proven: Spike 2

### PAL — Presentation Subsystem
`nitrate/crates/nitrate-pal/src/vulkan/presentation/engine.rs` `[CRITICAL]` `[PROVEN]`
Orchestrates the Acquire-Render-Present loop. Exists to manage frame pacing and semaphore chains, abstracting triple-buffering logic from the main app.
→ Exports: `PresentationEngine`
→ Proven: Spike 1, Spike 3

`nitrate/crates/nitrate-pal/src/vulkan/presentation/types.rs`
Defines public configuration structs and the `AcquiredFrame` container. Exists to decouple the configuration logic from the engine implementation, allowing cleaner API boundaries.
→ Exports: `PresentationConfig`, `AcquiredFrame`

`nitrate/crates/nitrate-pal/src/vulkan/presentation/handle.rs` `[UNSAFE]`
Wraps `VkSwapchainKHR` creation and management. Exists to handle swapchain recreation during window resizes or surface invalidation.

`nitrate/crates/nitrate-pal/src/vulkan/presentation/sync.rs`
Manages the pool of Fences and Binary Semaphores for frame pacing. Exists to synchronize the CPU submission loop with the GPU presentation engine.

`nitrate/crates/nitrate-pal/src/vulkan/presentation/images.rs`
Creates `VkImageView` handles for swapchain images. Exists to prepare the swapchain images for use as render targets.

## Layer 3 — Domain

`nitrate/crates/nitrate-color/src/lib.rs`
Implements color space matrices and non-linear transfer functions. Exists to provide the mathematical backbone for accurate YUV→RGB conversion and HDR tone mapping.

`nitrate/crates/nitrate-decode/src/lib.rs` `[WIP]`
Defines the abstract `Decoder` trait. Exists to standardize the interface for future VA-API and Media Foundation implementations.

`nitrate/crates/nitrate-compositor/src/lib.rs` `[WIP]`
Manages the final `wgpu` render pass. Exists to combine video planes and UI layers into the final output image.

`nitrate/crates/nitrate-compositor/src/shaders/compose.wgsl`
Performs YUV→RGB conversion, tone mapping, and alpha blending in the fragment shader. Exists to execute the entire color pipeline in a single GPU pass.

`nitrate/crates/nitrate-ui/src/lib.rs` `[WIP]`
Provides the skeleton for the UI system. Exists to eventually host the Vello renderer integration.

## Layer 4 — App

`nitrate/crates/nitrate-app/src/app.rs`
Implements the `winit` event loop and high-level state management. Exists to coordinate window events with the GPU lifecycle.
→ Exports: `NitrateApp`

`nitrate/crates/nitrate-app/src/gpu.rs`
Initializes standard `wgpu` resources. Exists to support standard App-layer rendering paths distinct from the PAL's hybrid architecture.

`nitrate/crates/nitrate-app/src/main.rs`
Acts as the entry point stub. Exists to point developers toward the architecture validation spikes.

`nitrate/crates/nitrate-app/src/bin/spike1.rs` `[SPIKE]` `[PROVEN]`
Manually builds a Vulkan stack and wraps it in `wgpu`. Validates that the "Native Owns, WGPU Borrows" strategy works on the target driver.
→ Status: PASSED — orange window, 0 validation errors

`nitrate/crates/nitrate-app/src/bin/spike2/main.rs` `[SPIKE]` `[PROVEN]`
Runs the full DMA-BUF roundtrip pipeline. Validates that native memory can be exported and imported into `wgpu` correctly.
→ Status: PASSED — blue window, stable 300+ frames

`nitrate/crates/nitrate-app/src/bin/spike2/native.rs`
Issues pure-Vulkan commands to clear native images. Exists to verify the "Producer" side of the pipeline works independently.

`nitrate/crates/nitrate-app/src/bin/spike2/render.rs`
Configures the `wgpu` pipeline to render imported textures. Exists to verify the "Consumer" side of the pipeline works.

`nitrate/crates/nitrate-app/src/bin/spike3.rs` `[SPIKE]` `[PROVEN]`
Native composition with animated clear pass. Validates native command recording and semaphore-synchronized submission coexisting with wgpu bridge.
→ Status: PASSED — animated gradient, ~60fps, stable 3900+ frames

`nitrate/crates/nitrate-app/src/shaders/blit.wgsl` `[SPIKE]`
Samples a texture and outputs it to the screen. Exists to visually verify that imported textures contain the expected data.

## Layer 5 — Docs

`nitrate/README.md`
High-level project documentation. Exists to explain the vision, architecture, and build instructions to new developers.

`nitrate/TODO.md`
Active development roadmap tracking Phase 1-6 progress.

`nitrate/docs/SEMANTIC_MAP.md`
This file. Describes the purpose and status of every significant file in the codebase.

`nitrate/reference/ui-design.css`
Defines the visual aesthetic for the UI. Exists as a reference implementation for the future Vello styling.
