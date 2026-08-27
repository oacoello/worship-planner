/**
 * Extracts a subtle WinUI Mica/Acrylic mesh gradient from an image DataURL or URL
 */
export function extractMeshGradient(imageSrc, isDark = true) {
  return new Promise((resolve) => {
    const defaultMesh = isDark
      ? 'radial-gradient(at 0% 0%, rgba(98, 100, 167, 0.18) 0px, transparent 55%), radial-gradient(at 100% 0%, rgba(0, 120, 212, 0.14) 0px, transparent 50%), radial-gradient(at 50% 100%, rgba(135, 100, 184, 0.12) 0px, transparent 50%)'
      : 'radial-gradient(at 0% 0%, rgba(98, 100, 167, 0.08) 0px, transparent 55%), radial-gradient(at 100% 0%, rgba(0, 120, 212, 0.07) 0px, transparent 50%), radial-gradient(at 50% 100%, rgba(135, 100, 184, 0.06) 0px, transparent 50%)';

    if (!imageSrc) {
      return resolve(defaultMesh);
    }

    const img = new Image();
    img.crossOrigin = 'Anonymous';

    img.onload = () => {
      try {
        const canvas = document.createElement('canvas');
        canvas.width = 40;
        canvas.height = 40;
        const ctx = canvas.getContext('2d');
        if (!ctx) return resolve(defaultMesh);

        ctx.drawImage(img, 0, 0, 40, 40);

        // Sample 3 key ambient points: top-left, top-right, bottom-center
        const p1 = ctx.getImageData(5, 5, 1, 1).data;
        const p2 = ctx.getImageData(35, 5, 1, 1).data;
        const p3 = ctx.getImageData(20, 35, 1, 1).data;

        // Ensure vibrant but legible ambient opacity
        const op1 = isDark ? 0.22 : 0.09;
        const op2 = isDark ? 0.18 : 0.07;
        const op3 = isDark ? 0.15 : 0.06;

        const mesh = `radial-gradient(at 5% 0%, rgba(${p1[0]}, ${p1[1]}, ${p1[2]}, ${op1}) 0px, transparent 50%), radial-gradient(at 95% 10%, rgba(${p2[0]}, ${p2[1]}, ${p2[2]}, ${op2}) 0px, transparent 55%), radial-gradient(at 50% 90%, rgba(${p3[0]}, ${p3[1]}, ${p3[2]}, ${op3}) 0px, transparent 60%)`;

        resolve(mesh);
      } catch {
        resolve(defaultMesh);
      }
    };

    img.onerror = () => {
      resolve(defaultMesh);
    };

    img.src = imageSrc;
  });
}
