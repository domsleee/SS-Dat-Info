
interface VideoTarget {
    seekTo(seconds: number, allowSeekAhead?: boolean): void;
    getCurrentTime(): number;
    playVideo(): void;
    pauseVideo(): void;
  }

export function setupVideo(): { videoTarget: VideoTarget | undefined } {
    console.log("ESTUP VIDEO")
    const tag = document.createElement('script');
    const result = { videoTarget: undefined as VideoTarget | undefined };
  
    tag.src = "https://www.youtube.com/iframe_api";
    var firstScriptTag = document.getElementsByTagName('script')[0];
    firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
  
    var player;
    window.onYouTubeIframeAPIReady = () => {
      console.log("onYouTubeIframeAPIReady");
      player = new YT.Player('player', {
        height: '360',
        width: '480 ',
        videoId: 'HOSM3c2Zdf0',
        playerVars: {
          'playsinline': 1
        },
        events: {
          'onReady': onPlayerReady,
        }
      });
    }
  
    function onPlayerReady(event) {
      console.log("onPlayerReady", event.target);
      event.target.playVideo();
      result.videoTarget = event.target; 
    }

    return result;
  }
  