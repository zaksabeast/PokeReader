# PokeReader

PokeReader is a 3gx plugin that allows viewing information about 3ds Pokemon games, such as:

- RNG states
- Party/Wild Pokemon
- Trainer info

This information can be used to RNG shiny and high IV Pokemon, similar to lua scripts on emulators.

## Commands

All of these controls can be changed from the **Controls** menu inside the plugin.
The bindings below are the defaults, which match the controls of previous versions.

- Start + Up (D-Pad): Show/Hide the plugin
- X + Y: Lock/Unlock inputs to the plugin
- D-Pad keys: Navigate the plugin when unlocked (right & left to get in and out of the different tabs)
- Start + Select: Pause game and enable manual frame advancing
- Select: Advance one frame while paused
- Start or A: Unpause game
- SOS Menu Commands:
    - X + Right (D-Pad): Set SOS Caller slot to current Ally slot
    - X + Up/Down: Manually increment/decrement Caller slot

### Configurable controls

PokeReader 0.8.0 adds a controls editor and a conflict-free preset that avoids
sending PokeReader inputs to the game. See
[INPUT_CONFIGURATION.md](INPUT_CONFIGURATION.md) for the full guide.

A short summary:

- Open the plugin and select **Controls**.
- Pick the profile to edit (keyboard or gamepad) and press **A** on an action.
- Hold the new combination and release it. Conflicts and inputs that the game
  also uses are reported before the binding is saved.
- **Y** clears the selected binding, and **Save config** writes the file to
  `sdmc:/luma/plugins/PokeReader-input.cfg` (fallback:
  `sdmc:/PokeReader-input.cfg`).

PokeReader runs inside the 3DS process, so it can only see 3DS buttons. There is
no plugin API to read PC keyboard or gamepad keys directly. With Citra, map the
keys you want in Citra's input settings (for example `F6` to `ZL` and `F7` to
`ZR`) and then bind PokeReader to those 3DS buttons. The **Safe preset** uses
`ZL`/`ZR`, which the supported games do not read, so those inputs do not reach
gameplay.

## Installing

1. Update to the latest [Luma](https://github.com/LumaTeam/Luma3DS/releases) or set up [Citra](https://github.com/citra-emu/citra).
1. Download and unzip the [latest PokeReader release](https://github.com/zaksabeast/PokeReader/releases/latest).
1. Copy `default.3gx` to `/luma/plugins/default.3gx` on your sd card (create the directory if it doesn't exist).

## Building

1. Install rust and the armv6k-nintendo-3ds target, devkitarm, and [3gxtool](https://gitlab.com/thepixellizeross/3gxtool)
1. Run `make`

The build output is `out/default.3gx`.

Other useful commands:

```sh
make test   # host unit tests (uses the test_stubs feature)
make lint   # clippy for the 3DS target
make format # rustfmt
```

A prebuilt container with devkitARM and the pinned Rust toolchain is used by
CI: `zaksabeast/devkitarm-rust`.

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [PKHeX](https://github.com/kwsch/PKHeX/) for Pokemon related documentation, examples, and code
- [ShinySylveon04](https://github.com/ShinySylveon04/) for building most of the UI
- [Bambo-Rambo for the DexNav and Radar addresses](https://github.com/Bambo-Rambo/TinyFinder/blob/99917164b43bf79bd7432b271cced7a4d62b8431/Subforms/NTR/NtrClient.cs#L319-L326)
