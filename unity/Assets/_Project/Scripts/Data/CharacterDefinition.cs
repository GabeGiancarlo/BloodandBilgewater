using UnityEngine;

namespace BloodAndBilgewater.Data
{
    /// <summary>
    /// Designer-authored definition of a selectable character / crew role.
    /// Create assets via: Assets > Create > Blood and Bilgewater > Character Definition.
    /// </summary>
    [CreateAssetMenu(
        fileName = "CharacterDefinition",
        menuName = "Blood and Bilgewater/Character Definition",
        order = 0)]
    public class CharacterDefinition : ScriptableObject
    {
        [Tooltip("Stable unique id, e.g. 'shipwright'.")]
        public string characterId;

        [Tooltip("Player-facing name.")]
        public string displayName;

        [Tooltip("Crew role, e.g. 'Shipwright'.")]
        public string roleName;

        [Tooltip("Portrait shown on the character-select screen.")]
        public Sprite portrait;

        [Tooltip("Default in-world sprite for this character.")]
        public Sprite defaultSprite;

        [TextArea]
        [Tooltip("Short flavor/description text.")]
        public string shortDescription;
    }
}
