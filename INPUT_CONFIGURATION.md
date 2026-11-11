# Input configuration

PokeReader's actions can be rebound from the plugin itself, without rebuilding
the plugin and without changing Citra's controller configuration.

This document explains what is configurable, how the editor works, where the
configuration is stored and which parts of the "input isolation" problem are
and are not solved by the plugin.

## Platform model (please read this first)

PokeReader is a 3gx plugin. It is loaded **inside the 3DS process** (on a real
console through Luma3DS, or in an emulator that supports 3gx plugins).

The plugin has no access to the PC:

- it cannot enumerate or read a PC keyboard,
- it cannot enumerate or read a PC gamepad,
- it cannot consume a key before Citra turns it into a 3DS button.

The only input channel exposed by the plugin framework is the console's HID
button state. In Citra, that state is produced by Citra's input configuration:
a keyboard key or a gamepad button is mapped to a 3DS button, and the plugin
sees only the resulting 3DS button.

Because of that:

- **Keyboard and gamepad support is provided through 3DS buttons.** Map the
  keyboard keys or gamepad buttons you want to use in Citra (or use the
  physical console), then bind PokeReader to those 3DS buttons.
- The configuration stores **3DS button bindings**, not PC key names. A binding
  such as `F6` cannot be stored by the plugin because `F6` never reaches it.
- The editor lets you keep a separate **keyboard** and **gamepad** profile so
  that, for example, keyboard keys can be mapped to `ZL`/`ZR` while a gamepad
  uses a different combination. The plugin cannot tell which physical device
  produced a 3DS button, so if both profiles are enabled their bindings are
  merged.

This is the safest solution supported by the plugin environment. Claiming that
the plugin reads PC keys directly would be false.

## Suggested mapping for keyboard users (Citra)

The **Safe preset** only uses buttons that the supported Pokemon games ignore.
In Citra:

1. Open `Emulation > Configure > Controls`.
2. Map the keys you want to the New 3DS buttons `ZL` and `ZR` (and optionally
   `L`/`R` for the overlay/lock chords).
3. Enable the keyboard profile in PokeReader's **Controls** menu.

Example:

| PokeReader action | Safe preset binding | Suggested Citra keys |
| --- | --- | --- |
| Pause | `ZL` | `F6` |
| Step frame | `ZR` | `F7` |
| Resume | `ZL + ZR` | `F6` + `F7` |
| Toggle overlay | `ZL + ZR + L` | `F6` + `F7` + `F8` |
| Toggle lock | `ZL + ZR + R` | `F6` + `F7` + `F9` |

The `F6`-`F10` numbers are only a suggestion; any keys work. Because Citra maps
one key to one 3DS button, chords are pressed as several keys at the same time.

> **New 3DS only:** `ZL`/`ZR` exist on a New 3DS and in Citra when the emulated
> console is a New 3DS. On an original 3DS the safe preset cannot be used; the
> legacy preset remains available.

## Defaults (backwards compatible)

When no configuration file exists, PokeReader uses exactly the controls of the
previous versions:

| Action | Default binding |
| --- | --- |
| Pause | `Start + Select` |
| Step frame | `Select` |
| Resume | `Start` or `A` |
| Toggle overlay | `Start + Up` |
| Toggle lock | `X + Y` |

Both the keyboard and gamepad profiles are enabled by default and contain these
bindings, so existing users do not lose their controls.

On Pokemon Crystal (Virtual Console), `L + R` still pauses the game while the
pause binding is the legacy default. Editing the pause binding replaces that
shortcut.

## The Controls menu

Open the plugin (`Start + Up` by default), select **Controls** and press `A`.

```
Controls
A=Edit Y=Clear
>Devices KB+Pad
 Profile Keyboard
 Pause   Start+Select
 Step    Select
 Resume  Start/A
 Overlay Start+Up
 Lock    X+Y
 Safe preset
 Defaults
 Clear profile
 Test input
 Save config
 Default config
```

### Device selection

`Devices` cycles through:

| Value | Meaning |
| --- | --- |
| `KB+Pad` | Both profiles can trigger actions (default) |
| `KB` | Only the keyboard profile |
| `Pad` | Only the gamepad profile |
| `Off` | PokeReader actions are disabled (the overlay still draws) |

`Profile` selects which profile the editor changes. It does not enable or
disable the profile; use `Devices` for that.

### Recording a binding

1. Select an action and press `A`.
2. Release the buttons you are holding (the plugin waits for a neutral state so
   the `A` press is not captured).
3. Hold the new combination.
4. Release it.

The plugin then:

- rejects combinations with more than four buttons,
- reports a conflict if another action already uses the same combination,
- warns when the combination contains a button that the games also read,
- asks for confirmation (`A` accepts, `B` cancels).

Press `B` alone while recording to cancel. `B` alone cannot be used as a
binding because it is the cancel key.

Bindings are saved automatically after every change. The `Save config` row
retries a failed save and shows the result.

### Clearing bindings

- `Y` on an action clears that action's binding.
- `Clear profile` removes every binding of the selected profile.

An action with no binding is never triggered.

### Presets

- **Safe preset**: `ZL`/`ZR` based mapping that does not reach gameplay.
- **Defaults**: restores the legacy controls shown above.

### Testing

`Test input` shows the buttons currently held and whether each action would
trigger. This is useful to verify that Citra is delivering the expected 3DS
buttons.

## Configuration file

### Location

PokeReader writes the configuration to:

```
sdmc:/luma/plugins/PokeReader-input.cfg
```

If that location is not writable (for example because the plugin was installed
somewhere else), it falls back to:

```
sdmc:/PokeReader-input.cfg
```

The file is read from both locations at startup, primary first. If neither file
exists, the legacy defaults are used and no file is created until the user
changes something.

If the SD card is unavailable the plugin keeps the configuration in memory for
the current session and reports `Save failed`.

### Format

The file is a small, versioned, line based text file:

```
# PokeReader input configuration
# See INPUT_CONFIGURATION.md for the file format.
version=1
active=keyboard,gamepad

# keyboard
keyboard.pause=Start+Select
keyboard.step_frame=Select
keyboard.resume=Start|A
keyboard.toggle_overlay=Start+Up
keyboard.toggle_lock=X+Y

# gamepad
gamepad.pause=Start+Select
gamepad.step_frame=Select
gamepad.resume=Start|A
gamepad.toggle_overlay=Start+Up
gamepad.toggle_lock=X+Y
```

- `version` is the format version. Files without a version are treated as
  version 0 and migrated key by key.
- `active` is `none`, `keyboard`, `gamepad` or `keyboard,gamepad`.
- `+` joins buttons that must be held together (a chord).
- `|` separates alternative chords for the same action.
- `-` means "no binding".
- Button names: `A`, `B`, `X`, `Y`, `Start`, `Select`, `L`, `R`, `ZL`, `ZR`,
  `Up`, `Down`, `Left`, `Right`.
- Keys that are not recognised are ignored, so files written by newer versions
  do not break older plugins.
- Invalid values are reported in the UI and fall back to the default for that
  key.

A binding may contain at most four buttons and at most two alternatives.

### Versioning and migration

The current format version is `1`.

- A file with no `version` line is migrated from version 0 (the same key names,
  no version marker).
- Missing keys keep their default value, so a partial file is valid.
- If the file version is **newer** than the plugin, the plugin uses the
  defaults in memory, reports `Config too new` and refuses to overwrite the
  file. This prevents a newer configuration from being destroyed by an older
  build.

## Input isolation

"Input isolation" means that pressing a PokeReader binding does not also press
the equivalent button in the game.

The supported games read the standard 3DS buttons (`A`, `B`, `X`, `Y`, `Start`,
`Select`, `L`, `R` and the D-Pad). They never read `ZL`/`ZR` on the titles
supported by PokeReader. Therefore:

- Bindings made only of `ZL`/`ZR` are **not forwarded to gameplay** by
  construction. The **Safe preset** uses this property.
- Frame stepping and resuming are **deferred until the bound input is
  released**. The game is kept frozen until the buttons are up, so it never
  samples the button that triggered the action. This is implemented in
  `reader_core/src/input/manager.rs` and used by `handle_freeze` in
  `3gx/sources/main.c`.
- Pausing itself happens at the end of the frame in which the pause chord is
  pressed, because the plugin's hook runs when the frame is presented. That
  frame has already read the input. This is why the default pause binding
  (`Start + Select`, kept for backwards compatibility) can still advance a
  dialog once. Use the Safe preset (`ZL`) to avoid it.
- If a binding contains a gameplay button, the editor warns
  (`Game sees this!`) before saving. The plugin cannot mask that button from the
  game: the HID shared memory block is mapped read-only by the HID service, and
  the plugin does not patch per-game input functions.

### What was explicitly not done

- **No writes to HID shared memory.** The block is mapped read-only for the
  game and plugin; forcing a writable mapping is not guaranteed on hardware and
  would race with the HID service.
- **No per-game input function hooking.** It would require reverse engineered
  addresses for every supported game and update, and could break RNG
  behaviour.
- **No PC keyboard/gamepad API.** None exists in the plugin environment.

These limitations are reported here instead of being hidden behind a UI that
appears to work.

## Building the plugin

See [README.md](README.md#building). The generated plugin is
`out/default.3gx`. Rename it (for example
`PokeReader-Configurable-Inputs.3gx`) before copying it to
`/luma/plugins/` if you want to keep the original plugin.

## Supported environments

- Luma3DS 3gx plugins (real hardware, Old 3DS and New 3DS).
- Citra builds with 3gx plugin support.
- The configuration file is a plain text file on the SD card; no additional
  service or setting is required.

## Troubleshooting

| Symptom | Cause / fix |
| --- | --- |
| `Config too new` | The file was written by a newer PokeReader. Update the plugin or delete the file. |
| `Save failed` | The SD card or the plugin directory is not writable. The configuration still works for this session. |
| Safe preset does nothing | The console is an Old 3DS (no `ZL`/`ZR`) or Citra is emulating an Old 3DS. Use the legacy preset. |
| A game action still happens | The binding contains a button the game reads. Rebind to `ZL`/`ZR` or use the Safe preset. |
| Keyboard does nothing | The keyboard keys are not mapped to 3DS buttons in Citra, or the keyboard profile is disabled in `Devices`. |
