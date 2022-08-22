# espflash-cli-redesign

This repository contains a work-in-progress command-line interface to replace the existing one in [espflash](https://github.com/esp-rs/espflash).

A number of types have been adapted (or copied verbatim) from [espflash](https://github.com/esp-rs/espflash) just to get the CLI functioning correctly; these types may or may not change, but are outside the scope of the project.

## Open Questions

Both `cargo-espflash` and `espflash` now require a `flash` subcommand for flashing to a target device. This is done to eliminate the issue of arguments being passed to the excutable itself (ie. prior to the subcommand) not being applied to subcommands. It's possible to accomplish this without a subcommand but:

1. the implementation is quite messy, and results in quite a bit of glue code
2. using a subcommand is more consistent with the rest of the CLI (and with `esptool.py` as well)

**Q:** Do we want to stick with `flash` for the name of this subcommand? `esptool.py` uses `write_mem`/`write_flash`, do we want to use these and get rid of the `--ram` flag, or just use `write` instead?
