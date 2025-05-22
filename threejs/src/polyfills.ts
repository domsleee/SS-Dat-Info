import { Buffer } from "buffer";
declare global {
  interface Window {
    Buffer: typeof Buffer;
  }
  const Buffer: typeof import("buffer").Buffer;
}

window.Buffer = Buffer;
