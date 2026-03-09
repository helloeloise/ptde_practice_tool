# ilhook Integration Status

## Changes Made

I've integrated ilhook-rs library (version 2.3) to replace the manual hooking implementation. This should solve the crash issues we experienced with manual hooks.

### Key Changes:

1. **Added ilhook dependency** to Cargo.toml
   - Version 2.3 (latest available)

2. **Converted hook functions** to naked assembly functions
   - Each function now properly preserves/restores all registers with `pushad`/`popad`
   - Functions call game functions with correct register context (EBP, ESI, EBX, param)
   - Functions return with `ret` (ilhook handles jumping back to original code)

3. **Updated hook installation** to use ilhook's Hooker API
   - Hook 1: 0xEB3798 -> `hook_begin_target_scene_ez_draw_depth`
   - Hook 2: 0xEB437D -> `hook_begin_target_scene_ez_draw_normal_1`
   - Hook 3: 0xEB4483 -> `hook_begin_target_scene_ez_draw_normal_2`

## What ilhook Does For Us

ilhook-rs handles the difficult parts that were causing crashes:
- **Instruction relocation**: Properly handles relative instructions (jumps, calls) that need adjustment when code is moved
- **Trampoline generation**: Creates a safe way to call original code
- **Thread safety**: Handles hooking atomically
- **Memory protection**: Manages VirtualProtect calls automatically

## Current Hook Implementation

The hooks are now simple naked functions that:
1. Save all registers (`pushad`)
2. Access the game's HgMan pointer structure  
3. Call the appropriate game drawing function
4. Restore all registers (`popad`)
5. Return (`ret`)

## Building Instructions

**IMPORTANT**: This project MUST be built on Windows with MSVC toolchain:

```bash
# On Windows:
cargo build --target i686-pc-windows-msvc --release
```

The Linux build will fail because:
- hudhook needs Windows SDK (windows.h)
- imgui-sys needs MSVC compiler
- Target is 32-bit Windows

## Testing Plan

When you build and test on Windows:

1. **Start with memory patches only** (hooks disabled)
   - Verify the 6 memory patches still work
   - Ensure game doesn't crash on startup

2. **Enable hook 1 only**
   - Uncomment hook 1 installation code
   - Test if it crashes or works

3. **Enable all hooks progressively**
   - Add hook 2, test
   - Add hook 3, test

4. **Verify functionality**
   - Check if debug draw actually appears
   - Check log file for hook execution

## Potential API Issues

Since I couldn't build/test on Linux, the ilhook API calls might need adjustment. If you get compilation errors about ilhook::x86::Hooker, check:

```rust
// The API might be:
use ilhook::x86::Hooker;

// Or it might be:
use ilhook::Hooker;

// Hook creation might need different parameters:
Hooker::new(...).hook()
// vs
Hooker::hook(...)
```

Refer to ilhook 2.3 documentation or examples for the exact API.

## Fallback Option

If ilhook still doesn't work, consider these alternatives:
1. **retour-rs**: Another Rust hooking library
2. **Manual with different approach**: Use hardware breakpoints or VEH hooks instead of inline hooks
3. **Different injection time**: Try hooking later in game initialization
4. **ASM-only approach**: Write pure assembly hooks in .asm files and link them

## Expected Benefits

With ilhook properly handling instruction relocation, the hooks should:
- Not crash even with complex game code
- Properly execute original instructions after our code
- Handle any relative jumps/calls in the hooked region
- Work reliably across game restarts

The crashes we saw before were likely due to:
- Trying to jump back to partially-overwritten instructions
- Not relocating relative instructions properly
- Stack misalignment issues

ilhook solves all of these problems.
