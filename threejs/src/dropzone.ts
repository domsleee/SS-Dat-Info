export function setupDropzone({ processCallback }: DropzoneConfig): void {
  // Get DOM elements with proper typing
  const dropzone = getElementOrThrow<HTMLTableCellElement>("dropzone");
  const fileInput = getElementOrThrow<HTMLInputElement>("replayFile");
  const label = getElementOrThrow<HTMLLabelElement>("dropzoneLabel");
  const subLabel = getElementOrThrow<HTMLLabelElement>("dropZoneSubLabel");

  // Update text with error handling
  function updateDropzoneText(message: string, isError = false): void {
    subLabel.textContent = message;
    if (isError) {
      dropzone.classList.add("error");
      setTimeout(() => {
        subLabel.textContent = "Drag and drop replay file here";
        dropzone.classList.remove("error");
      }, 3000);
    }
  }

  // Process file with proper error handling
  async function processFile(file: File): Promise<void> {
    updateDropzoneText("Processing...");

    try {
      const arrayBuffer = await file.arrayBuffer();
      const hexData = Buffer.from(new Uint8Array(arrayBuffer)).toString("hex");

      await processCallback(hexData);
      updateDropzoneText(`Processed ${file.name}`);
    } catch (err) {
      console.error("Error processing file:", err);
      updateDropzoneText("Error processing file", true);
    }
  }

  // Event handlers with proper typing
  function handleDragOver(e: DragEvent): void {
    e.preventDefault();
    e.stopPropagation();
    dropzone.classList.add("dragover");
  }

  function handleDragLeave(e: DragEvent): void {
    e.preventDefault();
    e.stopPropagation();
    dropzone.classList.remove("dragover");
  }

  async function handleDrop(e: DragEvent): Promise<void> {
    e.preventDefault();
    e.stopPropagation();
    dropzone.classList.remove("dragover");

    const file = e.dataTransfer?.files[0];
    if (file) {
      await processFile(file);
    }
  }

  async function handleChange(e: Event): Promise<void> {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) {
      await processFile(file);
    }
  }

  // Event listeners
  dropzone.addEventListener("dragover", handleDragOver);
  dropzone.addEventListener("dragleave", handleDragLeave);
  dropzone.addEventListener("drop", handleDrop);
  fileInput.addEventListener("change", handleChange);

  // Prevent browser defaults
  document.addEventListener("dragover", (e) => e.preventDefault());
  document.addEventListener("drop", (e) => e.preventDefault());
}

function getElementOrThrow<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) {
    throw new Error(`Element with id '${id}' not found`);
  }
  return element as T;
}

interface DropzoneConfig {
  processCallback: (hexData: string) => Promise<unknown>;
}
