# Debug Draw Hooks - Rust Port

This module contains the Rust port of the debug draw functionality from `debug_draw.cpp`.

## Overview

The module implements three assembly hook functions that inject custom drawing behavior into Dark Souls PTDE:

1. **BeginTargetSceneEzDrawDepth** - Hooked at `0xEB3798`
2. **BeginTargetSceneEzDrawNormal1** - Hooked at `0xEB437D`
3. **BeginTargetSceneEzDrawNormal2** - Hooked at `0xEB4483`

## Usage

To apply the debug draw patches, call the `apply_debug_draw_patches()` function during your initialization:

```rust
use crate::memory::ds1::debug_draw_hooks;

// In your initialization code (e.g., DllMain equivalent)
unsafe {
    debug_draw_hooks::apply_debug_draw_patches()
        .expect("Failed to apply debug draw patches");
}
```

## Memory Patches

The module applies the following memory patches:

- **0xFD6529**: 19 bytes NOPed
- **0xFD6546**: 3 bytes NOPed
- **0xC03F1B**: Set to `0x50`
- **0xC03180**: Set to `0x50`
- **0xC064E4**: Set to `0x140`
- **0xC064EB**: Set to `0x140`

## Implementation Details

### Naked Functions

The hook functions use the `#[naked]` attribute and inline assembly to preserve the exact behavior of the C++ code. Each function:

1. Accesses the HgMan pointer structure
2. Calls game functions for draw planning
3. Jumps back to the original code execution

### Safety

All functions in this module are `unsafe` because they:
- Directly manipulate process memory
- Use inline assembly
- Hook into game code at specific addresses

## Differences from C++

- No DLL injection logic (handled by your existing injection mechanism)
- No DINPUT8 proxy functionality
- Pure memory patching and hooking only
- Uses `windows-sys` instead of WinAPI

## Notes

- This code is 32-bit x86 specific
- Addresses are hardcoded for Dark Souls PTDE
- Requires the game to be running and the module to be injected into the game process
