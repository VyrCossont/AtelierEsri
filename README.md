# Atelier Esri

An Atelier-inspired game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run-native target/wasm32-unknown-unknown/release/atelier_esri.wasm
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

# License

[MIT license](LICENSE.md), except for some assets as noted below.

# Assets

## Folders

`asset_originals` is not directly consumed by the build script, and contains project files in formats like [Pixaki](https://pixaki.com/)/[Aseprite](https://www.aseprite.org/) documents (graphics), [PICO-8](https://www.lexaloffle.com/pico-8.php) cartridges (music, sound effects), and [Tiled](https://www.mapeditor.org/) (tilesets, maps).

[`assets`](assets) is intended for consumption by the [build script](build.rs).

## Licensing

Some assets are from [OpenGameArt](https://opengameart.org/) and come with their own license terms.

I created some of the assets with reference to AI-generated images from [Artbreeder](https://www.artbreeder.com/), for which the [Artbreeder ToS](https://www.artbreeder.com/terms.pdf) requires outputs to be licensed under the [CC0](https://creativecommons.org/share-your-work/public-domain/cc0/) public domain license. 

I created some of the other assets from reference to AI-generated images from [Waifu Labs](https://waifulabs.com/), for which I can't find a ToS.

In both cases, the Pixaki and Aseprite files will contain the AI-generated reference, as well as my own pixel-art interpretation. Copyright for AI-generated images is an evolving area, and what counts as a protectable copyrighted derivative of a public domain work seems to be a large grey area, so I'm not sure if I have a copyright on the pixel-art versions, in which case the project's MIT license applies, or if they fall under CC0 (for Artbreeder references) or some other license (for Waifu Labs references). Use at your own risk, or ask a lawyer (and then please tell me what they said).

## Character portraits

- [`Allie.png`](asset_originals/Allie.png)/[`Allie.pixaki`](asset_originals/Allie.pixaki)/[`Allie.aseprite`](asset_originals/Allie.aseprite) (Waifu Labs reference)
- [`Esri.png`](asset_originals/Esri.png)/[`Esri.pixaki`](asset_originals/Esri.pixaki)/[`Esri.aseprite`](asset_originals/Esri.aseprite) (Waifu Labs reference)
- [`Sae.png`](asset_originals/Sae.png)/[`Sae.pixaki`](asset_originals/Sae.pixaki)/[`Sae.aseprite`](asset_originals/Sae.aseprite) (Artbreeder reference)

## Sprites

- [`gungirl.png`](assets/gungirl.png): [Gun Girl + Riflemen + Shielded Rifleman + Tiles](https://opengameart.org/content/gun-girl-riflemen-shielded-rifleman-tiles) by [Spring Spring](https://opengameart.org/users/spring-spring) (CC-BY-3.0)
- [`roguelikeitems.aseprite`](asset_originals/roguelikeitems.aseprite): [Roguelike/RPG Items](https://opengameart.org/content/roguelikerpg-items) by [@JoeCreates](https://twitter.com/joecreates) (CC-BY-SA-3.0)
- [`fantasy-tileset.aseprite`](asset_originals/fantasy-tileset.aseprite): [32x32 fantasy tileset](https://opengameart.org/content/32x32-fantasy-tileset) by [@jeromBD](https://twitter.com/jeromBD) (CC-BY-SA-3.0)

## Fonts

- [`tiny_font.png`](assets/tiny_font.png): [Tiny Bitmap Font](https://opengameart.org/content/tiny-bitmap-font) by [Binary Moon](https://www.binarymoon.co.uk/) (CC0)

## Tiles

- [`Kenney_MonochromeRPG_extended.png`](asset_originals/Kenney_monochromerpg_extended.png), [`Village.tmx`](asset_originals/Village.tmx): [Monochrome RPG](https://opengameart.org/content/monochrome-rpg) by [Kenney](https://kenney.nl/) and extended by me (CC0)
