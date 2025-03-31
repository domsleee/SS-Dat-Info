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

  let player;
  window.onYouTubeIframeAPIReady = () => {
    player = new YT.Player('player', {
      ...dimensions,
      videoId: videoId,
      playerVars: {
        'playsinline': 1,
        'autoplay': 1,       // Auto-play the video on load
        'rel': 0,            // Disable related videos
        'modestbranding': 1, // Minimal YouTube branding
        'controls': 1,       // Show video controls
        'showinfo': 0,       // Hide video title and uploader
        'fs': 1,            // Allow fullscreen
        'cc_load_policy': 0, // Don't show closed captions by default
        'iv_load_policy': 3, // Hide video annotations
        'autohide': 1,       // Hide video controls when playing
        'enablejsapi': 1,     // Enable JavaScript API
        // 'endscreen-client_20': 0  // Hide endscreen/more videos
      },
      events: {
        'onReady': (event) => {
          event.target.playVideo();
          result.videoTarget = event.target; 
        },
        'onStateChange': (event) => {
          if (event.data === YT.PlayerState.ENDED) {
            player.seekTo(0);
            player.playVideo();
          }
        }
      }
    });
  }

  return result;
}
  