# PokeReader

PokeReader is a 3gx plugin that allows viewing information about 3ds Pokemon games, such as:

- RNG states
- Party/Wild Pokemon
- Trainer info

This information can be used to RNG shiny and high IV Pokemon, similar to lua scripts on emulators.

## Commands
- Start + Up (D-Pad): Show/Hide the plugin
- X + Y: Lock/Unlock inputs to the plugin
- D-Pad keys: Navigate the plugin when unlocked (right & left to get in and out of the different tabs)
- Start + Select: Pause game and enable manual frame advancing
- Select: Advance one frame while paused
- Start or A: Unpause game

## Installing

1. Update to the latest [Luma](https://github.com/LumaTeam/Luma3DS/releases) or set up [Citra](https://github.com/citra-emu/citra).
1. Download the [latest PokeReader release](https://github.com/zaksabeast/PokeReader/releases/latest).
1. Rename the `pokereader.3gx` file to `default.3gx`, then copy it to `/luma/plugins/` on your sd card (create the directory if it doesn't exist).
   - Optionally, rename the `pokereader.3gx` file to `plugin.3gx`, then copy it to `/luma/plugins/<title_id>/` for every Pokemon game.

## Building

1. Install rust and the armv6k-nintendo-3ds target, devkitarm, and [3gxtool](https://gitlab.com/thepixellizeross/3gxtool)
1. Run `make`

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [PKHeX](https://github.com/kwsch/PKHeX/) for Pokemon related documentation, examples, and code
- [ShinySylveon04](https://github.com/ShinySylveon04/) for building most of the UI
- [Bambo-Rambo for the DexNav and Radar addresses](https://github.com/Bambo-Rambo/TinyFinder/blob/99917164b43bf79bd7432b271cced7a4d62b8431/Subforms/NTR/NtrClient.cs#L319-L326)
