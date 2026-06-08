using BloodAndBilgewater.Core;
using BloodAndBilgewater.Data;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace BloodAndBilgewater.UI
{
    /// <summary>
    /// Drives the character-select screen. Call <see cref="SelectCharacter"/> from each
    /// character button, then <see cref="StartGame"/> to enter the test island.
    /// </summary>
    public class CharacterSelectController : MonoBehaviour
    {
        [Tooltip("Optional default selection used if none is chosen.")]
        [SerializeField] private CharacterDefinition defaultCharacter;

        private void Awake()
        {
            if (defaultCharacter != null && !SelectedCharacterStore.HasSelection)
            {
                SelectedCharacterStore.Set(defaultCharacter);
            }
        }

        public void SelectCharacter(CharacterDefinition character)
        {
            SelectedCharacterStore.Set(character);
        }

        public void StartGame()
        {
            if (!SelectedCharacterStore.HasSelection)
            {
                Debug.LogWarning("No character selected; staying on character select.");
                return;
            }

            SceneManager.LoadScene(SceneNames.TestIsland);
        }

        public void BackToMenu()
        {
            SceneManager.LoadScene(SceneNames.MainMenu);
        }
    }
}
