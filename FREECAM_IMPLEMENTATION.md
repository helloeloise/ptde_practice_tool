# Freecam Implementation

This document describes the freecam implementation that matches the functionality of the Cheat Engine table (freecam(1).CT).

## Overview

The freecam allows you to control the camera independently from the player character. It has 4 modes that can be cycled by pressing L3+R3 (both analog stick buttons) simultaneously.

## Modes

### Mode 0: Normal Camera
- Default mode
- Camera follows player normally
- Copies transforms from camera source (+0x18) to camera target (+0x254)
- Clears control flags at [ebx+0x58] and [ebx+0x59]

### Mode 1: Conditional Freeze
- Checks if a specific input button is pressed
- If frozen (button not pressed): Sets control flags to prevent camera movement
- If not frozen: Falls through to Mode 2 behavior
- Sets [ebx+0x58] and [ebx+0x59] to 1 when frozen, 0 otherwise

### Mode 2: Free Camera
- Camera can be moved independently of player
- Clears control flags
- Updates camera motion by copying transforms from camera target (+0x254) to camera motion (+0x30)
- Calls game update function to apply the transformation

### Mode 3: Reverse Copy
- Copies transforms in reverse direction (from +0x254 to +0x30)
- Useful for setting camera position programmatically
- Clears control flags

## Technical Details

### Injection Points

The freecam uses two code injection points found via AOB scanning:

1. **Hook 1** (DARKSOULS.exe+B00AD4):
   - AOB: `89 44 24 24 8B 43 44`
   - Main camera logic hook
   - Checks for button presses
   - Executes mode-specific behavior

2. **Hook 2** (DARKSOULS.exe+BFB431):
   - AOB: `C1 EA 14 F6 C2 01`
   - Skips camera constraint checks when in freecam mode

### Camera Manager Pointer

- AOB: `A1 ? ? ? ? 8B 80 EC 00 00 00 3B C6`
- Resolves to an address containing the camera manager pointer
- Camera object is at [camera_manager+0x6F8]
- Current mode is written to [camera_object+0x24C]

### Transform Structures

The camera uses 4x4 transformation matrices (16 bytes each, 4 matrices = 64 bytes):
- Position and rotation data stored in XMM registers
- Copied using `movaps` instructions for efficient 16-byte transfers
- Three key transform locations:
  - +0x18: Source camera transform
  - +0x30: Camera motion transform
  - +0x254: Target camera transform

### Control Flags

- [ebx+0x58]: Camera control flag 1
- [ebx+0x59]: Camera control flag 2  
- [ebx+0x44]: Motion update flag (set to 1 in modes 1/2 to trigger update)

## Usage

1. Enable freecam in the Practice Tool UI (checkbox in Settings or Debug Info)
2. The freecam starts in Mode 0 (normal camera)
3. Press L3+R3 simultaneously to cycle through modes:
   - Mode 0 → 1 → 2 → 3 → 0 (cycles)
4. In Mode 2 (free camera), use controller to move camera independently
5. Disable freecam checkbox to return to normal game behavior

## Implementation Notes

- Uses dynamically scanned addresses (no hardcoded offsets)
- Addresses are found at startup during `Ds1::refresh()`
- Injection is only attempted if all required addresses are found
- Hook restoration is automatic when freecam is disabled
- No game restart required to enable/disable

## Differences from Cheat Engine Table

- Simplified: Removed extensive debug logging for better performance
- Same core functionality: All 4 modes work identically to CT
- Better error handling: Checks pointers before dereferencing
- Cleaner code: No need for external assembly files
