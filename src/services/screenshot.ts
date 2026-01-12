
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';

export const screenshotService = {
  /**
   * Captures the current window using Rust-side capture and applies a 'blog effect'.
   * Returns the path where the screenshot was saved, or null if cancelled.
   */
  async captureAndSave(sectionName: string = 'app'): Promise<string | null> {
    try {
      // 1. Capture the window using our Rust command
      // This returns a base64 encoded PNG
      const base64Png = await invoke<string>('capture_window');
      const dataUrl = `data:image/png;base64,${base64Png}`;

      // 2. Apply blog effect
      const finalDataUrl = await this.applyBlogEffect(dataUrl);

      // 3. Prepare filename: celerix-{section}-{date-time}.png
      const now = new Date();
      const date = now.toISOString().split('T')[0]; // YYYY-MM-DD
      const time = now.toLocaleTimeString('en-GB', { hour12: false }).replace(/:/g, '-'); // HH-mm-ss
      const fileName = `celerix-${sectionName}-${date}-${time}.png`;

      // 4. Save the image
      const filePath = await save({
        filters: [{
          name: 'Image',
          extensions: ['png']
        }],
        defaultPath: fileName
      });

      if (filePath) {
        // Convert dataUrl to Uint8Array
        const base64Data = finalDataUrl.split(',')[1];
        const binaryData = Uint8Array.from(atob(base64Data), c => c.charCodeAt(0));
        await writeFile(filePath, binaryData);
        console.log('Screenshot saved to:', filePath);
        return filePath;
      }
      return null;
    } catch (error) {
      console.error('Screenshot failed:', error);
      throw error;
    }
  },

  /**
   * Applies a 'blog effect' to a base64 image:
   * - Rounded corners & Shadow
   * - Gradient background
   * - Padding
   */
  async applyBlogEffect(imageDataUrl: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => {
        const padding = 100; // Increased padding to give the shadow more room
        const borderRadius = 12;
        const shadowBlur = 50; // Slightly more blur for a softer, larger shadow
        const shadowOpacity = 0.4; // Slightly more opacity to compensate for larger area

        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');
        if (!ctx) {
          reject(new Error('Failed to get canvas context'));
          return;
        }

        // Set canvas dimensions
        canvas.width = img.width + padding * 2;
        canvas.height = img.height + padding * 2;

        // 1. Clear Background (Transparent)
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // 2. Draw Shadow by filling a rounded rectangle
        // We do this first because the image will be drawn on top.
        ctx.save();
        ctx.shadowColor = `rgba(0, 0, 0, ${shadowOpacity})`;
        ctx.shadowBlur = shadowBlur;
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 20;
        
        // We use a solid fill to ensure the shadow is rendered.
        // We inset it slightly (0.5px) so it doesn't leak out from under the image's rounded corners.
        ctx.fillStyle = 'white'; 
        this.roundRect(ctx, padding + 1, padding + 1, img.width - 2, img.height - 2, borderRadius);
        ctx.fill();
        ctx.restore();

        // 3. Draw Image with Rounded Corners
        // We use a path to clip the image to match the rounded rectangle
        ctx.save();
        this.roundRect(ctx, padding, padding, img.width, img.height, borderRadius);
        ctx.clip();
        ctx.drawImage(img, padding, padding);
        ctx.restore();

        resolve(canvas.toDataURL('image/png'));
      };
      img.onerror = reject;
      img.src = imageDataUrl;
    });
  },

  /**
   * Helper to draw rounded rectangles
   */
  roundRect(ctx: CanvasRenderingContext2D, x: number, y: number, width: number, height: number, radius: number) {
    ctx.beginPath();
    ctx.moveTo(x + radius, y);
    ctx.lineTo(x + width - radius, y);
    ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
    ctx.lineTo(x + width, y + height - radius);
    ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
    ctx.lineTo(x + radius, y + height);
    ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
    ctx.lineTo(x, y + radius);
    ctx.quadraticCurveTo(x, y, x + radius, y);
    ctx.closePath();
  }
};
