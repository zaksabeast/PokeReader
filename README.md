# PokeReader

PokeReader is a [Plug-n-play plugin](https://github.com/zaksabeast/3ds-Plug-n-play) that allows viewing information about 3ds Pokemon games, such as:

- RNG states
- Party/Wild Pokemon
- Trainer info

This information can be used to RNG shiny and high IV Pokemon, similar to lua scripts on emulators.

## Installing

1. Install [Plug-n-play](https://github.com/zaksabeast/3ds-Plug-n-play)
1. Download the [latest PokeReader release](https://github.com/zaksabeast/PokeReader/releases/latest)
1. Copy pokereader.wasm to `/pnp/pokereader.wasm` on your sd card (create the directory if it doesn't exist)

## Usage notes

PokeReader keeps track of the game's internal RNG state. If other pnp plugins are used and the RNG advances too far without PokeReader running, PokeReader will not be able to keep track of the RNG.

For Gen 6 games, PokeReader needs to be opened at least one time before the title screen so it can patch the game to get the initial seed.

## Building

1. Install rust and the wasm32-unknown-unknown target
1. Run `cargo build --release --target wasm32-unknown-unknown`

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [PKHeX](https://github.com/kwsch/PKHeX/) for Pokemon related documentation, examples, and code
- [ShinySylveon04](https://github.com/ShinySylveon04/) for building most of the UI
- [Bambo-Rambo for the DexNav and Radar addresses](https://github.com/Bambo-Rambo/TinyFinder/blob/99917164b43bf79bd7432b271cced7a4d62b8431/Subforms/NTR/NtrClient.cs#L319-L326)
