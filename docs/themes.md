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

| Name                     | Description                                                                 | Default Value                                                                 |
|--------------------------|-----------------------------------------------------------------------------|------------------------------------------------------------------------------|
| background_color         | The color for the background.                                               | (red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0)                                |
| text_color               | The color for all text in the GUI.                                          | (red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0)                                |
| player_head_color        | The color of the player head.                                               | (red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0)                                |
| player_body_color        | The color of all other player body parts.                                   | (red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0)                                |
| obstacles_grayscale      | Renders obstacles in grayscale when true.                                   | true                                                                         |
| obstacles_color_variation| How much randomness is added to the obstacle color. 0.0 is no variation from base, 1.0 is completely random colors. | 0.1                                                                          |
| obstacles_base_color     | Base color for obstacles. Randomness is added afterwards.                   | (red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0)                                |
| player_broken_color      | Color when body part is broken (Default is yellow).                         | (red: 1.0, green: 1.0, blue: 0.2, alpha: 1.0)                                |
| player_final_color       | Color when broken body part is hit again resulting in the player dying.     | (red: 1.0, green: 0.2, blue: 0.2, alpha: 1.0)                                |
| player_head_texture      | Path to image texture of player head. Is an empty string by default.        | ""                                                                           |
| walls_color              | Color for the walls that are to the left and right of the player.           | (red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0)                             |
| music_path               | Path to background music to play.                                           | ""                                                                           |
| bone_break_path          | Path to sound to play when bone breaks.                                     | ""                                                                           |

> Note: All paths are relative to themes/

Have fun creating your own themes!
