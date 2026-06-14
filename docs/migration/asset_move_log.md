# Asset Move Log — Unity → Bevy Restoration

Date: 2026-06-14
Branch: `unity-migration-snapshot`

This log records how Unity-phase art was relocated into the Bevy-first asset
structure. Asset **file names are preserved verbatim**; only **directory** names
were normalized (lowercased, spaces → underscores) and a small number of typos
were corrected. The migration was performed by
[`tools/art_pipeline/migrate_unity_art.ps1`](../../tools/art_pipeline/migrate_unity_art.ps1)
and [`tools/art_pipeline/flatten_long_paths.ps1`](../../tools/art_pipeline/flatten_long_paths.ps1).

Originals remain preserved in three places: the live Unity project
(`Archive/UnityPrototype/`), the nested-repo commit `c17145d`, and an out-of-repo
filesystem backup (`BloodandBilgewater_UnityArtBackup_2026-06-14`).

## Source → destination roots

| Source (under `My 2d game ARggg/Assets/_Project/`) | Destination (under `assets/`) | Kind |
| --- | --- | --- |
| `Art/Aseprite/**/*.aseprite` | `source/aseprite/**` | Source art (source of truth) |
| `Art/Aseprite/Characters/PlayerDefault/**/*.png,*.json` | `textures/characters/player_default/**` | Runtime / raw frames |
| `Art/Aseprite/Props/**/*.png` | `textures/props/**` | Runtime |
| `Art/References/source/*.png` | `source/references/` | Reference images |
| `UI/Fonts/legacy/**` | `fonts/` | Fonts (see licensing note in audit) |

## Directory renames (normalization + typo fixes)

| Old directory | New directory | Reason |
| --- | --- | --- |
| `Art/Aseprite/Characters/PlayerDefault` | `source/aseprite/characters/player_default` | case/spacing |
| `Art/Aseprite/Characters/PlayerDefault/musicain` (PNG) | `textures/characters/player_default/musician` | **typo fix** (musicain → musician) |
| `Art/Aseprite/Tilesets/Beach` | `source/aseprite/tilesets/beach` | case |
| `Art/Aseprite/Tilesets/Ocean` | `source/aseprite/tilesets/ocean` | case |
| `Art/Aseprite/Tilesets/ShallowWater` | `source/aseprite/tilesets/shallow_water` | case |
| `Art/Aseprite/Tilesets/island grass` | `source/aseprite/tilesets/island_grass` | spacing |
| `Art/Aseprite/Tilesets/rockey stone shore` | `source/aseprite/tilesets/rocky_stone_shore` | **typo fix** (rockey → rocky) + spacing |
| `Art/Aseprite/Tilesets/cliff_highland` | `source/aseprite/tilesets/cliff_highland` | (unchanged) |
| `Art/Aseprite/Tilesets/haunted` | `source/aseprite/tilesets/haunted` | (unchanged) |
| `Art/Aseprite/Tilesets/mangrove` | `source/aseprite/tilesets/mangrove` | (unchanged) |
| `Art/Aseprite/Tilesets/village` | `source/aseprite/tilesets/village` | (unchanged) |
| `Art/Aseprite/Tilesets/volcanic` | `source/aseprite/tilesets/volcanic` | (unchanged) |
| `Art/Aseprite/UI/Icons` | `source/aseprite/ui/icons` | case |
| `Art/Aseprite/UI/Menus` | `source/aseprite/ui/menus` | case |
| `Art/Aseprite/UI/TitleScreen` | `source/aseprite/ui/title_screen` | case/spacing |
| `Art/Aseprite/Ships` | `source/aseprite/ships` | case |
| `Art/Aseprite/Props` | `source/aseprite/props` | case |

> Note: misspelled **asset file names** (e.g. `charater-select-banner.aseprite`,
> `empty-chararter-select.aseprite`) were intentionally **not** renamed, to avoid
> breaking any references; they are flagged for review in the preservation manifest.

### Long-path flatten renames (2026-06-14)

Deep AI-generation frame directories shortened to keep all paths under the Windows 260-char limit. File names unchanged.

| Old directory | New directory |
| --- | --- |
| `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\Heres_the_revised_visual_description_A_gaunt_undea\animations\The_character_stands_in_a_relaxed_alert_posture_su` | `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\Heres_the_revised_visual_description_A_gaunt_undea\animations\The_char~cb4b` |
| `assets\textures\characters\player_default\helmsman\Create_a_full-body_pixel_art_character_of_a_cursed\Create_a_full-body_pixel_art_character_of_a_cursed\animations\The_character_stands_firmly_with_feet_planted_main` | `assets\textures\characters\player_default\helmsman\Create_a_full-body_pixel_art_character_of_a_cursed\Create_a_full-body_pixel_art_character_of_a_cursed\animations\The_char~a370` |
| `assets\textures\characters\player_default\doctor\Create_a_Doctor_Surgeon_class_a_grim_undead_pirate\Create_a_Doctor_Surgeon_class_a_grim_undead_pirate\animations\The_creature_stands_perfectly_still_with_its_arms` | `assets\textures\characters\player_default\doctor\Create_a_Doctor_Surgeon_class_a_grim_undead_pirate\Create_a_Doctor_Surgeon_class_a_grim_undead_pirate\animations\The_crea~f1ec` |
| `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\standing_gun_to_eye_copy_2\animations\The_character_stands_firmly_bracing_their_shoulder\south-east-2fcb7412` | `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\standing_gun_to_eye_copy_2\animations\The_character_stands_firmly_bracing_their_shoulder\south-ea~9566` |
| `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\standing_gun_to_eye_copy_2\animations\The_character_stands_firmly_bracing_their_shoulder\south-east-366d3854` | `assets\textures\characters\player_default\marksman\Heres_the_revised_visual_description_A_gaunt_undea\standing_gun_to_eye_copy_2\animations\The_character_stands_firmly_bracing_their_shoulder\south-ea~90fb` |
