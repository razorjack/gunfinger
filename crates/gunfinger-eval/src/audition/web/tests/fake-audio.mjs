// Stand-ins for the parts of Web Audio and of windows.js that the player
// uses, for `node --test`. The test moves the context's clock by hand.
//
// A source follows the specification's rules for stop(): the latest call
// before the source has stopped replaces every earlier one, and a call
// after it stopped changes nothing. Each source keeps every stop time asked
// of it and when it was disconnected, so a test can tell exactly when it
// was audible.

export class FakeContext {
  constructor() {
    this.currentTime = 0;
    this.state = "running";
    this.destination = new FakeNode(this);
    this.sources = [];
  }

  resume() {
    return Promise.resolve();
  }

  createGain() {
    const node = new FakeNode(this);
    node.gain = { value: 1 };
    return node;
  }

  createStereoPanner() {
    const node = new FakeNode(this);
    node.pan = { value: 0 };
    return node;
  }

  createBufferSource() {
    const source = new FakeSource(this);
    this.sources.push(source);
    return source;
  }

  // Moves the clock on and fires `onended` for the sources that stopped.
  advance(seconds) {
    this.currentTime += seconds;
    for (const source of this.sources) {
      if (!source.ended && source.started() && source.end() <= this.currentTime) {
        source.ended = true;
        source.onended?.();
      }
    }
  }

  // The most sources audible at one moment, over everything played so far.
  mostAudibleAtOnce() {
    const edges = [];
    for (const source of this.sources) {
      const { from, to } = source.audible();
      if (to > from) edges.push([from, 1], [to, -1]);
    }
    // At a shared instant the source ending goes first: a stop at t and a
    // start at t do not overlap.
    edges.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    let now = 0;
    let most = 0;
    for (const [, change] of edges) {
      now += change;
      most = Math.max(most, now);
    }
    return most;
  }

  audibleAt(t) {
    return this.sources.filter((source) => {
      const { from, to } = source.audible();
      return from <= t && t < to;
    });
  }
}

class FakeNode {
  constructor(context) {
    this.context = context;
    this.outputs = new Set();
  }

  connect(node) {
    this.outputs.add(node);
    return node;
  }

  disconnect() {
    this.outputs.clear();
  }
}

class FakeSource extends FakeNode {
  constructor(context) {
    super(context);
    this.buffer = null;
    this.playbackRate = { value: 1 };
    this.startTime = null;
    this.offset = 0;
    this.stopTime = Infinity;
    this.stopCalls = [];
    this.disconnectedAt = Infinity;
    this.ended = false;
    this.onended = null;
  }

  started() {
    return this.startTime !== null;
  }

  start(when = 0, offset = 0) {
    this.startTime = Math.max(when, this.context.currentTime);
    this.offset = offset;
  }

  stop(when = 0) {
    this.stopCalls.push(when);
    if (this.ended || this.context.currentTime >= this.end()) return;
    this.stopTime = Math.max(when, this.context.currentTime);
  }

  disconnect() {
    super.disconnect();
    this.disconnectedAt = Math.min(this.disconnectedAt, this.context.currentTime);
  }

  // When the source stops: at its stop time or where its buffer runs out.
  end() {
    const bufferEnd = this.startTime + (this.buffer.duration - this.offset) / this.playbackRate.value;
    return Math.min(this.stopTime, bufferEnd);
  }

  // The context times in which it is heard: [from, to), empty when to <= from.
  audible() {
    if (!this.started()) return { from: 0, to: 0 };
    return { from: this.startTime, to: Math.min(this.end(), this.disconnectedAt) };
  }
}

// Every window of every file is loaded and full. A buffer names its file and
// window, and `wanted` holds the windows the player last asked to keep.
export class FakeWindows {
  constructor(seconds = 30) {
    this.seconds = seconds;
    this.wanted = [];
  }

  index(fileSeconds) {
    return Math.floor(fileSeconds / this.seconds);
  }

  buffer(file, k) {
    if (k < 0) return undefined;
    return { duration: this.seconds, length: this.seconds * 44100, numberOfChannels: 2, file, k };
  }

  settled() {
    return true;
  }

  load() {
    return Promise.resolve(null);
  }

  want(windows) {
    this.wanted = windows;
  }

  stats() {
    return {};
  }
}
