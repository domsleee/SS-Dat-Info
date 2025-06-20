import { Config, VideoTarget } from "./types";
import { AnalyzeResult } from "dat-analyze/src/types";
import { setupDropzone } from "./dropzone";
import { analyzeReplay } from "dat-analyze/src/analyzeReplay";

export function setupConfig(
  config: Config,
  videoTarget: VideoTarget,
  processAnalyzeResult: (analyzeResult: AnalyzeResult) => void
) {
  const { offsetSeconds, videoId, syncWithVideo } = getDOMElements();
  updateConfigDOM(config);

  videoId.addEventListener("change", () => {
    config.videoId = videoId.value;
    videoTarget.videoTarget?.loadVideoById(videoId.value);
  });

  offsetSeconds.addEventListener("change", () => {
    config.offsetSeconds = parseFloat(offsetSeconds.value);
  });

  syncWithVideo.addEventListener("change", () => {
    config.syncWithVideo = syncWithVideo.checked;
  });
  
  setupDropzone({
    processCallback: async (data: Uint8Array) => {
      syncWithVideo.checked = false; config.syncWithVideo = false;
      const analyzeResult = await analyzeReplay(data, { skipCoords: false });
      processAnalyzeResult(analyzeResult);
    }
  });
}

function getDOMElements() {
  const offsetSeconds = document.getElementById(
    "offsetSeconds"
  ) as HTMLInputElement;
  const videoId = document.getElementById("videoId") as HTMLInputElement;
  const replayFile = document.getElementById("replayFile") as HTMLInputElement;
  const syncWithVideo = document.getElementById("syncWithVideo") as HTMLInputElement;
  return { offsetSeconds, videoId, replayFile, syncWithVideo };
}

export function updateConfigDOM(config: Config) {
  const { offsetSeconds, videoId, syncWithVideo } = getDOMElements();
  offsetSeconds.value = config.offsetSeconds.toString();
  videoId.value = config.videoId;
  syncWithVideo.checked = config.syncWithVideo;
}
