import { VideoTarget } from "./types";

declare global {
  interface Window {
    onYouTubeIframeAPIReady: () => void;
  }
}

export const videoIds = {
  ah: 'HOSM3c2Zdf0',
  vm: '_DC0g90k6eE',
  vm109: 'Ja3OmVZS2jQ',
}

export function setupVideo(videoId: string, dimensions: {width: number, height: number}): VideoTarget {
  console.log("Setup Video")
  const tag = document.createElement('script');
  const result = { videoTarget: undefined as YT.Player | undefined };

  tag.src = "https://www.youtube.com/iframe_api";
  const firstScriptTag = document.getElementsByTagName('script')[0]!;
  firstScriptTag.parentNode!.insertBefore(tag, firstScriptTag);

  let player: YT.Player | undefined = undefined;
  window.onYouTubeIframeAPIReady = () => {
    player = new YT.Player('player', {
      ...dimensions,
      videoId: videoId,
      playerVars: {
        playsinline: 1, //YT.PlaysInline.Inline,
        autoplay: 1, //YT.AutoPlay.AutoPlay,
        rel: 0, //YT.RelatedVideos.Hide,
        modestbranding: 1, //YT.ModestBranding.Modest,
        mute: 1, //YT.Mute.Muted,
        controls: 1, //YT.Controls.ShowLoadPlayer,
        showinfo: 0, //YT.ShowInfo.Hide,
        fs: 1, //YT.FullscreenButton.Show,
        cc_load_policy: 0, //YT.ClosedCaptionsLoadPolicy.UserDefault,
        iv_load_policy: 3, //YT.IvLoadPolicy.Hide,
        autohide: 1, //YT.AutoHide.HideAllControls,
        enablejsapi: 1, //YT.JsApi.Enable,
        // 'endscreen-client_20': 0  // Hide endscreen/more videos
      },
      events: {
        'onReady': (event) => {
          event.target.playVideo();
          result.videoTarget = event.target; 
        },
        'onStateChange': (event) => {
          if (event.data === YT.PlayerState.ENDED) {
            player!.seekTo(0, true);
            player!.playVideo();
          }
        }
      }
    });
  }

  return result;
}
