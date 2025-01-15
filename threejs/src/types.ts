import { AnalyzeResult, RowData } from "analyze/src/types";

export interface Config {
  offsetSeconds: number;
  videoId: string;
  syncWithVideo: boolean;
}

export interface VideoTarget {
  videoTarget?: YT.Player;
}

export interface AnalyzeResultContainer {
  analyzeResult: AnalyzeResult;
  data: RowData[];
}

export interface TextFields {
  nameText: HTMLParagraphElement;
  timeText: HTMLElement;
  speedText1: HTMLElement;
  speedText2: HTMLElement;
}

export interface MainLoopContainer {
  textFields: TextFields;
  analyzeResult: AnalyzeResult;
};