# File Responsibilities

This document explains what each source file under [src](src) is responsible for.
It is intentionally responsibility-focused: it tells you where behavior lives, not every function inside each file.

## Architecture Flow

At a high level, the runtime path looks like this:

1. [src/main.rs](src/main.rs) injects the DLL into the game process when you use the standalone injector.
2. [src/lib.rs](src/lib.rs) is the DLL entry that registers the DirectX9 ImGui hook.
3. [src/render_loop.rs](src/render_loop.rs) becomes the main per-frame coordinator once the hook is active.
4. [src/render_loop.rs](src/render_loop.rs) pulls the shared [Ds1 backend](src/memory/ds1/mod.rs) instance and delegates frame work to [src/render_loop](src/render_loop) helpers.
5. The [src/memory/ds1](src/memory/ds1) files read/write game memory, maintain freecam and XInput hook state, and expose a higher-level API to the UI.
6. The [src/ui](src/ui) files render feature windows and editors on top of that API.
7. The [src/widgets](src/widgets) files provide reusable lower-level UI building blocks used by the higher-level screens.

Another useful way to think about it is:

- Injection/bootstrap lives in [src/main.rs](src/main.rs), [src/lib.rs](src/lib.rs), and [src/dinput8_proxy.rs](src/dinput8_proxy.rs).
- Game-memory ownership lives in [src/memory](src/memory), especially [src/memory/ds1](src/memory/ds1).
- Frame orchestration lives in [src/render_loop.rs](src/render_loop.rs) and [src/render_loop](src/render_loop).
- User-facing screens live in [src/ui](src/ui).
- Reusable UI primitives live in [src/widgets](src/widgets).

## Top-Level Entry And Setup

- [src/lib.rs](src/lib.rs): Windows DLL entry for the hudhook overlay; wires the DirectX9 ImGui hook to `RenderLoop::new()`.
- [src/main.rs](src/main.rs): Standalone injector executable; locates the game process, resolves the DLL path, and injects the built DLL into Dark Souls.
- [src/config.rs](src/config.rs): Persistent configuration model and load/save logic for keybinds, colors, window layout, and freecam settings.
- [src/dinput8_proxy.rs](src/dinput8_proxy.rs): `dinput8.dll` proxy/forwarder; loads the real system `dinput8`, chainloads extra DLLs, and applies startup patches such as the no-logo patch.
- [src/render_loop.rs](src/render_loop.rs): Main in-game UI/render orchestrator; owns UI state, acquires the `Ds1` instance, runs per-frame tasks, and delegates most sections to [src/render_loop](src/render_loop) helpers.

## Memory Layer

### Shared Memory Module

- [src/memory/mod.rs](src/memory/mod.rs): Re-export layer for the game memory implementation and public constants/offsets.
- [src/memory/main_hook.rs](src/memory/main_hook.rs): Currently empty placeholder for future top-level memory hook code.

### DS1 Backend Core

- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs): Defines the `Ds1` process-backed state object plus module wiring for the DS1 backend; after refactors, it mostly holds core type definitions and shared imports.
- [src/memory/ds1/constants.rs](src/memory/ds1/constants.rs): Semantic offsets/flag constants for game structures such as `CharData1`, `CharData2`, `WorldState`, and related flag bitfields.
- [src/memory/ds1/offsets.rs](src/memory/ds1/offsets.rs): AOB patterns, hard offsets, and RVA constants used to find game memory and hook locations.
- [src/memory/ds1/refresh.rs](src/memory/ds1/refresh.rs): Pointer refresh and startup scanning logic; resolves game structures and freecam-related runtime addresses.
- [src/memory/ds1/position_state.rs](src/memory/ds1/position_state.rs): Basic player position/angle accessors.
- [src/memory/ds1/world_flags.rs](src/memory/ds1/world_flags.rs): World-state and game-cycle getters/setters such as NG+, death count, online mode, autosave, enemies, and events.
- [src/memory/ds1/player_flags.rs](src/memory/ds1/player_flags.rs): Player-facing toggles and state changes such as freecam enable, no-death/no-damage/no-move, collision/gravity, draw flags, and teleporting.
- [src/memory/ds1/item_consumption.rs](src/memory/ds1/item_consumption.rs): Infinite magic/goods and related consumption toggles.

### Freecam Backend

- [src/memory/ds1/freecam_state.rs](src/memory/ds1/freecam_state.rs): Freecam-specific constants and mutable runtime state shared by freecam scanning, runtime logic, diagnostics, and hook asm.
- [src/memory/ds1/freecam_hooks.rs](src/memory/ds1/freecam_hooks.rs): Naked-asm freecam hook entrypoints that patch into the game camera code.
- [src/memory/ds1/freecam_injection.rs](src/memory/ds1/freecam_injection.rs): Freecam hook installation/removal lifecycle plus camera-follow disabling.
- [src/memory/ds1/freecam_runtime.rs](src/memory/ds1/freecam_runtime.rs): Per-frame freecam runtime behavior such as mode toggling and manual movement updates.
- [src/memory/ds1/freecam_diagnostics.rs](src/memory/ds1/freecam_diagnostics.rs): Freecam diagnostics, status reporting, debug reads, and log export helpers.
- [src/memory/ds1/freecam_input_debug.rs](src/memory/ds1/freecam_input_debug.rs): Small helper for inspecting legacy freecam input-related memory values.
- [src/memory/ds1/freecam_scan_helpers.rs](src/memory/ds1/freecam_scan_helpers.rs): Pattern-scan and hook-target helper functions used to resolve freecam injection points and related code addresses.

### XInput Backend

- [src/memory/ds1/xinput_state.rs](src/memory/ds1/xinput_state.rs): XInput-specific constants and mutable hook/injection state.
- [src/memory/ds1/xinput_hooks.rs](src/memory/ds1/xinput_hooks.rs): Naked-asm XInput API hook that intercepts `XInputGetState` and optionally injects button/analog values.
- [src/memory/ds1/xinput.rs](src/memory/ds1/xinput.rs): Public XInput hook control API exposed through `Ds1`; enables/disables the hook and manages injected inputs.
- [src/memory/ds1/xinput_glue.rs](src/memory/ds1/xinput_glue.rs): Shared XInput helpers such as API resolution, stick normalization, keyboard fallback helpers, and translation-delta buffer writes.

### Shared Low-Level Helpers

- [src/memory/ds1/memory_access.rs](src/memory/ds1/memory_access.rs): Low-level memory page safety helpers and executable patch-writing utilities.

## Render Loop Helpers

- [src/render_loop/feature_sync.rs](src/render_loop/feature_sync.rs): Syncs UI toggle state from live game memory when needed.
- [src/render_loop/frame_tasks.rs](src/render_loop/frame_tasks.rs): Pre-UI per-frame work such as refresh/tick tasks before building windows.
- [src/render_loop/input_state.rs](src/render_loop/input_state.rs): Enables/disables game input based on menu/UI capture state.
- [src/render_loop/keybinds.rs](src/render_loop/keybinds.rs): Centralized hotkey processing for menu toggles, cheats, positions, utilities, and debug actions.
- [src/render_loop/layout_persistence.rs](src/render_loop/layout_persistence.rs): Saves/restores main window layout with throttling.
- [src/render_loop/menu_state.rs](src/render_loop/menu_state.rs): Handles opening/closing menu state and related UI resets.
- [src/render_loop/pending_angle.rs](src/render_loop/pending_angle.rs): Applies delayed post-warp angle writes.
- [src/render_loop/positions_section.rs](src/render_loop/positions_section.rs): Renders the stored-position UI section in the main menu.
- [src/render_loop/debug_flags_section.rs](src/render_loop/debug_flags_section.rs): Renders debug-flag toggle controls in the main menu.
- [src/render_loop/stats_section.rs](src/render_loop/stats_section.rs): Renders stat-editing UI in the main menu.
- [src/render_loop/utility_actions_section.rs](src/render_loop/utility_actions_section.rs): Renders utility actions such as moveswap, HP restore, or similar one-shot helpers.
- [src/render_loop/bonfire_section.rs](src/render_loop/bonfire_section.rs): Renders bonfire selection/warp UI.
- [src/render_loop/give_item_section.rs](src/render_loop/give_item_section.rs): Renders the give-item flow for items, rings, weapons, and armor in one place.
- [src/render_loop/ui_style.rs](src/render_loop/ui_style.rs): Applies the configured ImGui colors/styles used by the overlay UI.
- [src/render_loop/window_actions.rs](src/render_loop/window_actions.rs): Handles top-level window actions such as ejecting or animation-speed utilities.

## UI Layer

- [src/ui/mod.rs](src/ui/mod.rs): UI module export surface.
- [src/ui/player.rs](src/ui/player.rs): Main player-related UI/editor surface.
- [src/ui/bonfire.rs](src/ui/bonfire.rs): Bonfire UI/data handling.
- [src/ui/items.rs](src/ui/items.rs): Shared item data/UI support used by give-item style flows.
- [src/ui/tas_runner.rs](src/ui/tas_runner.rs): TAS runner UI/state integration.

### Debug Window

- [src/ui/debug_info.rs](src/ui/debug_info.rs): Debug window owner/coordinator; owns debug window state, window layout persistence behavior, and delegates render/update details to submodules.
- [src/ui/debug_info/update.rs](src/ui/debug_info/update.rs): Refreshes all cached debug-window values from `Ds1`.
- [src/ui/debug_info/render_sections.rs](src/ui/debug_info/render_sections.rs): Renders non-equipment debug sections such as summary, freecam diagnostics, XInput diagnostics, and keyboard input display.
- [src/ui/debug_info/render_equipment.rs](src/ui/debug_info/render_equipment.rs): Renders the large equipment-editing section of the debug window.

## Widget Layer

- [src/widgets/mod.rs](src/widgets/mod.rs): Shared widget traits, exports, constants, and scaling helpers.
- [src/widgets/flag.rs](src/widgets/flag.rs): Boolean/toggle-style widget behavior.
- [src/widgets/group.rs](src/widgets/group.rs): Composite/container widget for grouping child widgets.
- [src/widgets/label.rs](src/widgets/label.rs): Text label/display widget.
- [src/widgets/nudge_position.rs](src/widgets/nudge_position.rs): Widget for nudging position values incrementally.
- [src/widgets/position.rs](src/widgets/position.rs): Position-edit/display widget.
- [src/widgets/radial_menu.rs](src/widgets/radial_menu.rs): Radial menu widget and interaction behavior.
- [src/widgets/savefile_manager.rs](src/widgets/savefile_manager.rs): Save-file management widget behavior.
- [src/widgets/stats_editor.rs](src/widgets/stats_editor.rs): Stats editing widget behavior.
- [src/widgets/store_value.rs](src/widgets/store_value.rs): Generic store/restore value widget helper.

## Practical Reading Order

If you are new to the codebase, this order gives a good high-level picture:

1. [src/lib.rs](src/lib.rs)
2. [src/render_loop.rs](src/render_loop.rs)
3. [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)
4. [src/memory/ds1/refresh.rs](src/memory/ds1/refresh.rs)
5. [src/memory/ds1](src/memory/ds1) freecam files
6. [src/memory/ds1](src/memory/ds1) XInput files
7. [src/ui/debug_info.rs](src/ui/debug_info.rs)
8. [src/render_loop](src/render_loop)

## Notes

- This document covers the active Rust source tree under `src/`.
- It does not document `target/`, `selfconfig/`, or root-level notes/docs, which are support artifacts rather than the runtime code path.
