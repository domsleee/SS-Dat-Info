import { AnalyzeResult } from "dat-analyze/src/types";
import { Config, VideoTarget } from "./types";
import { getFromFilename } from "./data";

export function createPresets(config: Config, videoTarget: VideoTarget, processAnalyzeResult: (analyzeResult: AnalyzeResult) => void) {
  const presetSelector = document.getElementById("presetSelector")! as HTMLSelectElement;
  const presets = [
    {
      name: "Dom VM 1.09.08 with speed & inputs",
      videoId: "Ja3OmVZS2jQ",
      datFile: "replays/Village/Medium/VM 1.09.08 Dom.dat",
      offsetSeconds: -1.05,
    },
    {
      name: "Dom AE 1.19.93",
      videoId: "VQJ3hyQctoY",
      datFile: "replays/Alpine/Easy/1.19.93 Dom.dat",
      offsetSeconds: -0.21,
    },
    {
      name: "Dom AM 1.22.90",
      videoId: "drLyc1Ty9sw",
      datFile: "replays/Alpine/Medium/1.22.91 Dom.dat",
      offsetSeconds: -0.21,
    },
    {
      name: "Dom FH 0.57.06",
      videoId: "DIPZX8KbBfQ",
      datFile: "replays/Forest/Hard/0.57.06 Dom.dat",
      offsetSeconds: -0.21,
    },
    {
      name: "Dom VE 1.17.06",
      videoId: "FtyCM-26eGg",
      datFile: "replays/Village/Easy/1.17.06 Dom.dat",
      offsetSeconds: -0.22,
    },
    {
      name: "Dom VM 1.02.44",
      videoId: "qblHObF7np8",
      datFile: "replays/Village/Medium/1.02.44 Dom.dat",
      offsetSeconds: -0.21,
    }
  ];

  // add option for each preset
  for (const preset of presets) {
    const option = document.createElement("option");
    option.value = preset.videoId;
    option.text = preset.name;
    presetSelector.add(option);
  }

  // on change, load the preset
  presetSelector.addEventListener("change", async () => {
    const selectedPreset = presets.find((p) => p.videoId === presetSelector.value);
    if (!selectedPreset) {
      return;
    }

    const data = await getFromFilename(selectedPreset.datFile);
    config.videoId = selectedPreset.videoId;
    config.offsetSeconds = selectedPreset.offsetSeconds;
    videoTarget.videoTarget?.loadVideoById(selectedPreset.videoId);
    processAnalyzeResult(data);
  });
}