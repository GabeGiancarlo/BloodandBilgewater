# Unity → Bevy System Map

Date: 2026-06-14

The Unity prototype's C# scripts are preserved as **reference only** under
`Archive/UnityPrototype/Assets/_Project/Scripts/`. They are **not** ported directly to Rust.
This table maps each script to its intended Bevy 0.14 equivalent. Many equivalents already
exist in the restored Bevy project (`src/`).

| Unity script | Responsibility | Bevy 0.14 equivalent | Status in restored `src/` |
| --- | --- | --- | --- |
| `Player/PlayerMovement2D.cs` | Rigidbody2D top-down movement from input axes | A movement **system** reading `ButtonInput<KeyCode>`/`Axis<GamepadAxis>` and writing `Transform`/velocity on the player entity | `src/gameplay/player/` + `src/input/` exist |
| `Camera/CameraFollow2D.cs` | Smoothed `SmoothDamp` follow camera | A `camera_follow` system lerping the `Camera2d` transform toward the player each frame | `src/rendering/camera.rs` exists |
| `Data/CharacterDefinition.cs` | `ScriptableObject` describing a selectable class (id, name, role, portrait, sprite, desc) | A `CharacterDefinition` **asset** deserialized from **RON/JSON** in `assets/data/classes/` via `serde` + a custom `AssetLoader` | `src/gameplay/classes/` exists; data dir `assets/data/classes/` created |
| `Core/SelectedCharacterStore.cs` | Static store carrying the chosen character across scenes | A Bevy **`Resource`** (e.g. `SelectedCharacter`) inserted on select, read on spawn | add `Resource` in `src/gameplay/classes` or `src/ui` |
| `UI/CharacterSelectController.cs` | Character-select screen logic | Bevy **UI** + a `CharacterSelect` app **state**; systems run on `OnEnter`/`Update(in_state)` | `src/ui/character_select.rs` exists; `src/app/state.rs` has states |
| `UI/MainMenuController.cs` | Main-menu navigation / scene loads | App **state transitions** (`MainMenu → CharacterSelect → InGame`) driven by UI button systems | `src/ui/title_menu.rs`, `src/ui/menu.rs`, `src/app/state.rs` exist |
| `Core/SceneNames.cs` | String constants for scene names | Bevy **`States` enum** variants (no string scene names needed) | `src/app/state.rs` |

## Notes

- Unity "scenes" (`MainMenu`, `CharacterSelect`, `TestIsland`) become **Bevy states**, not
  separate scene files. `src/app/state.rs` is the home for the state enum.
- Unity `ScriptableObject` data (character definitions, roles, ship ranks) becomes
  **data-driven assets** in `assets/data/` (RON/JSON + `serde`), aligning with the Bevy
  project's existing `gameplay/classes` and `gameplay/ship` modules.
- Input: Unity legacy `Input.GetAxisRaw` maps to Bevy `ButtonInput<KeyCode>` for keyboard and
  `Gamepad` APIs for controller. Controller support is a Proof 01 TODO.
- Do **not** copy C# idioms (MonoBehaviour lifecycle, `[SerializeField]`) into Rust; use ECS
  systems, components, and resources instead.
