using BloodAndBilgewater.Core;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace BloodAndBilgewater.UI
{
    /// <summary>
    /// Hooks for the main-menu buttons. Wire these to UI Button OnClick events.
    /// </summary>
    public class MainMenuController : MonoBehaviour
    {
        /// <summary>Start a new game (goes to character select for now).</summary>
        public void NewGame()
        {
            SelectedCharacterStore.Clear();
            SceneManager.LoadScene(SceneNames.CharacterSelect);
        }

        public void OpenCharacterSelect()
        {
            SceneManager.LoadScene(SceneNames.CharacterSelect);
        }

        public void QuitGame()
        {
#if UNITY_EDITOR
            UnityEditor.EditorApplication.isPlaying = false;
#else
            Application.Quit();
#endif
        }
    }
}
