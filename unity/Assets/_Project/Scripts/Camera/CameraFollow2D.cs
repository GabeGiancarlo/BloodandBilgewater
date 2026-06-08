using UnityEngine;

namespace BloodAndBilgewater.CameraRig
{
    /// <summary>
    /// Minimal smoothed follow camera for a 2D target. Replace with Cinemachine once the
    /// package is installed.
    /// </summary>
    public class CameraFollow2D : MonoBehaviour
    {
        [Tooltip("Target to follow (usually the player).")]
        [SerializeField] private Transform target;

        [Tooltip("Smoothing time; 0 = snap instantly.")]
        [SerializeField] private float smoothTime = 0.15f;

        [Tooltip("Offset from the target. Keep z negative for a 2D camera.")]
        [SerializeField] private Vector3 offset = new Vector3(0f, 0f, -10f);

        private Vector3 _velocity;

        public void SetTarget(Transform newTarget)
        {
            target = newTarget;
        }

        private void LateUpdate()
        {
            if (target == null)
            {
                return;
            }

            Vector3 desired = target.position + offset;
            transform.position = Vector3.SmoothDamp(
                transform.position, desired, ref _velocity, smoothTime);
        }
    }
}
