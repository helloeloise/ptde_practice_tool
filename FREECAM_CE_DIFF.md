# Freecam CE vs Rust Comparison

This document tracks the current differences between the Cheat Engine table in `selfconfig/freecam(1).CT` and the Rust implementation in `src/memory/ds1/mod.rs`.

## What matches the CE script now

- There is one injected hook block with runtime mode dispatch, matching the CE `newmem` structure.
- Mode 0 copies the current camera buffer into the storage buffer.
- Mode 1 performs the freeze check before falling through to motion logic.
- Mode 2 always goes to motion logic.
- Mode 3 copies the stored camera state back to the current camera buffer.
- Hook 2 bypasses the camera constraint check when freecam is active.

Relevant refs:
- [Cheat Table](selfconfig/freecam(1).CT)
- [Rust hook](src/memory/ds1/mod.rs)

## Remaining differences from the CE script

### 1. Address resolution is dynamic in Rust

The CE script uses fixed addresses such as:
- `DARKSOULS.exe+B00AD4`
- `DARKSOULS.exe+BFB431`

The Rust version uses AOB scanning and RVA selection in:
- [src/memory/ds1/offsets.rs](src/memory/ds1/offsets.rs)
- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)

This is intentional, because the tool is trying to support the normal executable build as well as debug/release variants.

### 2. Rust has configuration-driven mode forcing

The CE script increments `freeCamMode` on L3+R3 and writes that mode directly.

The Rust build adds a config-controlled toggle:
- `freecam.force_mode2_active_only`

This currently controls whether active modes are coerced to memory mode 2.

Relevant refs:
- [src/config.rs](src/config.rs)
- [src/render_loop.rs](src/render_loop.rs)
- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)

### 3. Rust keeps a fallback static storage buffer

The CE table assumes the camera pointers are always valid and writes directly to `[eax+254]` / `[eax+30]`.

The Rust code has a fallback `FREECAM_STORAGE_BUFFER` for cases where `+0x254` is null on the target build.

Relevant ref:
- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)

### 4. Rust guards the motion-function call

The CE table always calls the motion function at the motion site.

The Rust version has a safety gate:
- `FREECAM_MOTION_CALL_SAFE`

This was added because direct calls could crash in pause/menu states on the target executable.

Relevant ref:
- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)

### 5. Rust currently does not rely on the CE hardcoded local state names

The CE script uses local symbols like `freeCamMode` and `isGameFreeCam`.

The Rust version uses Rust statics for the same roles, plus debug counters and telemetry.

Relevant ref:
- [src/memory/ds1/mod.rs](src/memory/ds1/mod.rs)

## Likely explanation for the current no-movement symptom

Based on the logs and the code comparison, the most likely causes are:

1. The target executable’s live camera buffer layout differs from the CE script’s assumptions.
2. The motion-function path may not be writing to the same data the renderer ultimately consumes.
3. The normal executable may need a different RVA/call target than the CE debug table.

The fact that the AOB scans are matching means the hook points themselves are probably valid. The remaining failure is more likely a buffer-layout or motion-path mismatch than a missing injection.

## What to inspect next

1. Verify which camera buffer the renderer actually reads from on the normal exe.
2. Compare the target build’s camera update function target against the CE call site.
3. Check whether `+0x30` or `+0x254` is the authoritative write target on this build.

## Current recommendation

If movement still does not appear after the latest CE-alignment fixes, the next step should be a narrow probe that logs the exact buffer contents before and after the motion call on the normal executable build.
