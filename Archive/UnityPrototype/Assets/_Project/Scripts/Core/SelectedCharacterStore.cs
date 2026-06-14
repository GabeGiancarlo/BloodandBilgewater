using BloodAndBilgewater.Data;

namespace BloodAndBilgewater.Core
{
    /// <summary>
    /// Lightweight static store that survives scene loads, carrying the player's chosen
    /// character from CharacterSelect into TestIsland. Replace with an injected service
    /// once a dependency container exists.
    /// </summary>
    public static class SelectedCharacterStore
    {
        public static CharacterDefinition Selected { get; private set; }

        public static bool HasSelection => Selected != null;

        public static void Set(CharacterDefinition character)
        {
            Selected = character;
        }

        public static void Clear()
        {
            Selected = null;
        }
    }
}
