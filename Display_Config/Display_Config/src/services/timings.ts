
type Timer = 'totalStartup' | 'createApp' | 'registerPlugins' | 'mountApp';

class Timings {
  timers: Set<Timer> = new Set<Timer>();
  getTimer(name: Timer): Timer {
    performance.mark(`${name}-start`);
    this.timers.add(name);
    return name;
  }

  endTimer(name: Timer): void {
    performance.mark(`${name}-end`);
  }

  getResult(name: Timer): number {
    if (!this.timers.has(name)) {
      throw new Error(`Timer ${name} was not started`);
    }
    return performance.measure('', `${name}-start`, `${name}-end`).duration;
  }
}

export const timings = new Timings();
