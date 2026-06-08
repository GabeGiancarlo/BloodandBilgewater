using UnityEngine;

namespace BloodAndBilgewater.Player
{
    /// <summary>
    /// Simple top-down Rigidbody2D movement for first-playable testing.
    /// Uses the legacy Input Manager axes (Horizontal/Vertical) so it works without
    /// extra package setup. Swap to the Input System later.
    /// </summary>
    [RequireComponent(typeof(Rigidbody2D))]
    public class PlayerMovement2D : MonoBehaviour
    {
        [Tooltip("Movement speed in units per second.")]
        [SerializeField] private float moveSpeed = 5f;

        private Rigidbody2D _body;
        private Vector2 _input;

        private void Awake()
        {
            _body = GetComponent<Rigidbody2D>();
            _body.gravityScale = 0f;
            _body.freezeRotation = true;
        }

        private void Update()
        {
            _input.x = Input.GetAxisRaw("Horizontal");
            _input.y = Input.GetAxisRaw("Vertical");
            if (_input.sqrMagnitude > 1f)
            {
                _input = _input.normalized;
            }
        }

        private void FixedUpdate()
        {
            _body.MovePosition(_body.position + _input * (moveSpeed * Time.fixedDeltaTime));
        }
    }
}
