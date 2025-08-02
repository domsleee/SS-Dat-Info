// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function fetchCardIds(renderer: string) {
  // await new Promise((resolve) => setTimeout(resolve, 1000));
  return [0];
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function fetchResolutions(renderer: string) {
  // await new Promise((resolve) => setTimeout(resolve, 1000));
  return [
    "640x480",
    "800x600",
    "1024x768",
    "1600x1200",
    "1280x720",
    "1600x1900",
    "1920x1080",
    "2560x1440",
    "3840x2160",
    "2560x1080",
    "3440x1440",
    "5120x2160",
  ];
}
