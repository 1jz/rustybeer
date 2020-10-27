# RustyBeer
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-7-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/drodil/rustybeer/blob/master/LICENSE)
[![Contributors](https://img.shields.io/github/contributors/drodil/rustybeer.svg?style=flat)]()
[![Issues](https://img.shields.io/github/issues-raw/drodil/rustybeer.svg?maxAge=25000)](https://github.com/drodil/rustybeer/issues)
[![PRs](https://img.shields.io/github/issues-pr/drodil/rustybeer.svg?style=flat)](https://github.com/drodil/rustybeer/pulls)

RustyBeer is a CLI tool written in Rust, to calculate values used in the process of brewing beer.

## Installation

If you don't already have the toolset installed, you will first need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).
From the root of the repository, run the following command:

```shell
cargo build
```

You can now run it with:
```shell
cargo run <subcommand>
```

## Files and Folders

- **src** - The folder containing all source code
  - **calculators** - The folder containing calculators to be used in lib or CLI tool
  - **commands** - The folder containing subcommands for CLI
  - **main.rs** - The file containing the main function
- **Cargo.toml** - The file containing build and dependency infomation
- **LICENSE** - The file containing the terms that this code package is released under
- **README.md** - The file you are currently reading
- **CONTRIBUTING.md** - Contribution guidelines for this repository

## Acronyms

Beer brewing has a lot of acronyms that have a meaning. This table is to help
out with figuring out what everything means:

Acronum      | Description
-------------|---------------------------------
ABV          | Alcohol By Volume
OG           | Original Gravity
FG           | Final Gravity
SG           | Specific Gravity
IBU          | International Bittering Units

## Functionality

Below is a table of the features currently implemented.

Implemented                   | Function      | Description                                    | Usage
------------------------------|---------------|------------------------------------------------|-------
:white_check_mark:            | ABV           | Calculates ABV from OG and FG or FG from OG and ABV | `abv --og <Original gravity> (--fg <Final gravity>) (--abv <Alcohol by volume>)`
:hourglass_flowing_sand:      | Boil-off Gravity| Calculates the volume needed to be boiled down to for a desired SG | `boil_off --current_gravity <current_gravity> --wort_volume <wort_volume> <--target_volume <target_volume>|--desired_gravity <desired_gravity>>`
:white_check_mark:            | Dilution      | Calculates the SG after dilution               | `diluting --sg <Current specific gravity> --cv <Current volume> --tv <Target volume>`
:white_check_mark:            | Num Of Bottles  | Calculates the number of bottles required for a given volume | `num_of_bottles --volume <volume>`
:white_check_mark:            | Priming       | Beer Priming Calculator                        | `priming --temp <Beer temperature> --amount <Beer volume> --co2_volumes <co2_volumes>`
:white_check_mark:            | SG Correction | Corrects SG reading according to the difference between the measurement temperature and the calibration temperature | `sg_correction --sg <Specific gravity reading> --ct <Calibration temperature> --mt <Measurement temperature>`
:white_check_mark:            | Beer style    | Finds beer styles matching given parameters    | `beer_style (--og <Original gravity>) (--fg <Final gravity>) (--abv <Alcohol by volume>) (--ibu <International bittering units> (--color <SRM color>)`
:white_check_mark:            | ABV <-> ABW    | Calculates alcohol by weight (ABW) from  alcohol by volume (ABV) | `abv_abw --percent <alcohol percentage> (--total_volume <total beer volume in ml>) (--total_density <density of beer in g/cm³) (--reverse)`

This list will expand as ideas and suggestions come in.

## Other Tasks to Do

See [Issues](https://github.com/drodil/rustybeer/issues)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://drodil.kapsi.fi"><img src="https://avatars0.githubusercontent.com/u/1178319?v=4" width="100px;" alt=""/><br /><sub><b>drodil</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=drodil" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/mlatief"><img src="https://avatars3.githubusercontent.com/u/462098?v=4" width="100px;" alt=""/><br /><sub><b>mlatief</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=mlatief" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/ProgrammerJoe93"><img src="https://avatars3.githubusercontent.com/u/56159225?v=4" width="100px;" alt=""/><br /><sub><b>Joseph Russell</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=ProgrammerJoe93" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/flauntingspade4"><img src="https://avatars1.githubusercontent.com/u/48335751?v=4" width="100px;" alt=""/><br /><sub><b>flauntingspade4</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=flauntingspade4" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/i-jey"><img src="https://avatars1.githubusercontent.com/u/25993326?v=4" width="100px;" alt=""/><br /><sub><b>Ilakkiyan Jeyakumar</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=i-jey" title="Code">💻</a></td>
    <td align="center"><a href="http://linkedin.com/in/tommilligan477"><img src="https://avatars2.githubusercontent.com/u/12255914?v=4" width="100px;" alt=""/><br /><sub><b>Tom Milligan</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=tommilligan" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/rogercyyu"><img src="https://avatars0.githubusercontent.com/u/45835736?v=4" width="100px;" alt=""/><br /><sub><b>Roger Y</b></sub></a><br /><a href="https://github.com/drodil/rustybeer/commits?author=rogercyyu" title="Code">💻</a></td>
  </tr>
</table>

<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
=======

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)
