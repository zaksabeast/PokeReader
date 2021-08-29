# PokeReader

This is an experiment to replace ntr + pcalc with something more stable using modern Luma features.

PokeReader is a 3ds sysmodule that currently doesn't do anything, and is just a fun test. You most likely shouldn't install it right now.

## Building, installing, and running

1. Install make, rust, devkitarm, and devkitpro's 3ds libs
1. Run `make` to build the debug and release builds
1. Run `make` in the `launcher` directory to build the launcher
1. Copy `launcher/out/pkrd-launcher.3dsx` to your console's `/3ds` directory
1. Install `out/release/pkrd.cia` to your console

## Developing

Rather than installing the cia after every change, you may find it more convenient to copy the files at `out/<build>/000401300000CB02` to `/luma/titles/000401300000CB02` on your 3ds. Luma will load those when the sysmodule is launched.

The debug build will write logs to `/ctr-logs.txt`, so it will be slower than the release build.

If luma's crash screen is enabled, panics will show the last four characters of the causing file in r9 and the line that caused the panic in r10. While this hides two register's values, it does provide pretty quick insight into a problem area.

## How does this work?

Keep in mind, all of this is high level.

NTR works by patching several sysmodules to accomplish a few things:

1. Provide the ability to run custom code when a game is launched
1. Load plugins into memory when a game launches
1. Patch the game in a function responsible for helping with graphics to run the plugin code

By having the game's process run plugins, plugins have all permissions the game does. With the same permissions and hooking into a function responsible for graphics, plugins can easily draw to the console screen.

However patching multiple sysmodules makes NTR a fragile solution. It was great at the time, but Luma has added several extensions and patches that could provide a better solution.

PokeReader is a sysmodule that runs as a background process, and runs a service called `pkrd:game`. This service provides two commands: a setup command and a game hook command. PokeReader also comes with a launcher homebrew, so it only runs when a user wants it to.

Below are the steps of launching PokeReader to running code with game access:

1. The launcher homebrew launches PokeReader
1. PokeReader sets up `pkrd:game` and subscribes to a custom Luma notification to be notified when a game launches
1. The launcher opens a session to `pkrd:game`
1. The launcher runs the setup command, which moves `pkrd:game`'s session handle to PokeReader
1. When a game launches, PokeReader checks to see if it's a known game and patches the same function NTR patches. The patch runs `pkrd:game`'s game hook command with a copy of the `pkrd:game` session handle
1. The game runs the patched function and behaves the same as normal, except it also runs `pkrd:game`'s game hook command
1. PokeReader receives the stack pointer as an argument, opens a debug session with the game and the `gsp` module, and runs whatever code we want for that game

The `pkrd:game` session handle is given to PokeReader so games don't need to open a session, which means smaller patches.

Debug sessions are closed after they're used so other tools (like Rosalina) can freely open debug sessions as well, provided the sessions are cleaned up after use. This also prevents locks between PokeReader, the game, and the home menu.

PokeReader doesn't patch system modules, so it's more resiliant to updates and less prone to patching incorrectly. It uses debug sessions to have read/write access to the game, and also requires a debug session with `gsp` to be able to draw to the screen.

Since PokeReader's game hook command only runs when the game calls it and the game is only patched when launched, closing and reopening games should not cause crashes, which is another NTR limit.

Interestingly, Rosalina also seems to frequently open and close debug sessions, and initial testing shows this doesn't seem to impact performance by a noticable amount. Writing one large amount of data to the screen doesn't noticably impact performance either. However, writing lots of tiny amounts of data destroys performance.

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [Luma3DS](https://github.com/LumaTeam/Luma3DS) and [Wumiibo](https://github.com/hax0kartik/wumiibo) for being great references (build processes, double check my reverse engineering, etc.)
- [libctru](https://github.com/devkitPro/libctru/) for being a great reference and providing an easy way to make open source homebrew
- [PKSM](https://github.com/FlagBrew/PKSM) for providing an example of svcControlService
- [3dbrew](https://www.3dbrew.org/) for documentation about how different parts of the 3ds works
- [The rust3ds team](https://github.com/rust3ds) for the 3ds.json, initial ctru_sys, and code references to help get rust working on the 3ds
- [devkitPro](https://github.com/devkitPro/) for their toolchain
- All 3ds researchers
