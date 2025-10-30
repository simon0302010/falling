# Creating a new theme

To make a new theme, create a file called YOUR_THEME_NAME.theme.ron in assets/themes/.
Also create an entry for your theme in assets/themes/manifest.ron like this:

```
ThemeManifest(
    themes: [
        ThemeManifestEntry(
            path: "themes/default.theme.ron",
            name: "Default",
        ),
        ThemeManifestEntry(
            path: "themes/YOUR_THEME_NAME.theme.ron",
            name: "YOUR_THEME_NAME",
        ),
    ]
)
```

A good base for a new theme is the default theme file.
Here is a list of all things that can be specified using a theme file:

- background_color: The color for the background.
- text_color: The color for all text in the GUI.
- player_head_color: The color of the player head.
- player_body_color: The color of all other player body parts.
- obstacles_grayscale: Renders obstacles in grayscale when true.
- obstacles_color_variation: How much randomness is added to the obstacle color. 0.0 is no variation from base, 1.0 is completely random colors.
- obstacles_base_color: Base color for obstacles. Randomess is added afterwards.
- player_broken_color: Color when body part is broken (Default is yellow).
- player_final_color: Color when broken body part is hit again resulting in the player dying.
- player_head_texture: Path to image texture of player head. Is an empty string by default.
- walls_color: Color for the walls tht are to the left and right of the player.
- music_path: Path to background music to play.

> Note: All paths are relative to themes/

Have fun creating your own themes!
