# PokeReader

PokeReader is a 3gx plugin that allows viewing information about 3ds Pokemon games, such as:

- RNG states
- Party/Wild Pokemon
- Trainer info

This information can be used to RNG shiny and high IV Pokemon, similar to lua scripts on emulators.

## Commands
- X+Y to Lock/Unlock
- D-pad keys to navigate when unlocked (right & left to get in and out of the different tabs)
- Start + Select to pause game and enable manual frame advancing
- Select to advance Main RNG frames once
- Start or A to unpause
- Start + Up (D-Pad) to Show/Hide the plugin

## Installing

1. Update to the latest [Luma](https://github.com/LumaTeam/Luma3DS/releases) or set up [Citra](https://github.com/citra-emu/citra).
2. Download the [latest PokeReader release](https://github.com/zaksabeast/PokeReader/releases/latest).
3. Rename the `pokereader.3gx` file to `default.3gx`, then copy it to `/luma/plugins/` on your sd card (create the directory if it doesn't exist).
   - Optionally, rename the `pokereader.3gx` file to `plugin.3gx`, then copy it to `/luma/plugins/<title_id>/` for every Pokemon game.

## Building

1. Install rust and the armv6k-nintendo-3ds target, devkitarm, and [3gxtool](https://gitlab.com/thepixellizeross/3gxtool)
2. Run `make`

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [PKHeX](https://github.com/kwsch/PKHeX/) for Pokemon related documentation, examples, and code
- [ShinySylveon04](https://github.com/ShinySylveon04/) for building most of the UI
- [Bambo-Rambo for the DexNav and Radar addresses](https://github.com/Bambo-Rambo/TinyFinder/blob/99917164b43bf79bd7432b271cced7a4d62b8431/Subforms/NTR/NtrClient.cs#L319-L326)
