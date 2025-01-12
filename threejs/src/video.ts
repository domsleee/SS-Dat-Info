
interface VideoTarget {
    seekTo(seconds: number, allowSeekAhead?: boolean): void;
    getCurrentTime(): number;
    playVideo(): void;
    pauseVideo(): void;
  }

export const videoIds = {
  ah: 'HOSM3c2Zdf0',
  vm: '_DC0g90k6eE',
  vm109: 'Ja3OmVZS2jQ',
}

export function setupVideo(videoId: string, dimensions: {width: number, height: number}): { videoTarget: VideoTarget | undefined } {
    console.log("Setup Video")
    const tag = document.createElement('script');
    const result = { videoTarget: undefined as VideoTarget | undefined };
  
    tag.src = "https://www.youtube.com/iframe_api";
    let firstScriptTag = document.getElementsByTagName('script')[0]!;
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
          'enablejsapi': 1     // Enable JavaScript API
        },
        events: {
          'onReady': onPlayerReady,
          'onStateChange': onPlayerStateChange
        }
      });
    }
  
    function onPlayerReady(event) {
      console.log("onPlayerReady", event.target);
      event.target.playVideo();
      result.videoTarget = event.target; 
    }

    function onPlayerStateChange(event) {
      // YT.PlayerState.ENDED = 0
      if (event.data === 0) {
        player.seekTo(0);
        player.playVideo();
      }
    }

    return result;
  }
  