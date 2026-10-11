// Audio windows: the server sends a file in windows of `seconds` (30 s):
// window k holds file time [k × seconds, (k + 1) × seconds) as 16-bit stereo
// WAV, decoded here into an AudioBuffer of 32-bit floats (10.6 MB per
// window). A mix is never held whole: the cache keeps at most `budget`
// windows and drops the least recently used one that is not wanted now.
//
// The player asks for the windows it is about to play (`load`, `buffer`);
// `want` names every window worth having soon (around the playhead of each
// lane and at the nearest markers), which starts their loading in that order
// and protects them from eviction.
//
// A window past the file's end is `missing` for good. A window that failed
// to load (the NAS gone, FFmpeg failed) is tried again after RETRY_MS; the
// player waits for it meanwhile and `onerror` tells the listener.

import * as api from "./api.js";

const RETRY_MS = 3000;

export class Windows {
  constructor(context, seconds, budget = 24) {
    this.context = context;
    this.seconds = seconds;
    this.budget = budget;
    this.loaded = new Map(); // key -> { buffer, used }
    this.loading = new Map(); // key -> Promise<AudioBuffer|null>
    this.missing = new Set(); // keys past a file's end
    this.failed = new Map(); // key -> when it last failed (performance.now())
    this.wanted = new Set();
    this.uses = 0;
    this.onload = () => {};
    this.onerror = () => {};
  }

  index(fileSeconds) {
    return Math.floor(fileSeconds / this.seconds);
  }

  key(file, k) {
    return `${k}\u0000${file}`;
  }

  // The decoded window, or undefined while it is not loaded.
  buffer(file, k) {
    const entry = this.loaded.get(this.key(file, k));
    if (entry) entry.used = ++this.uses;
    return entry?.buffer;
  }

  // Whether a window is loaded, or known to hold no audio.
  settled(file, k) {
    const key = this.key(file, k);
    return k < 0 || this.loaded.has(key) || this.missing.has(key);
  }

  load(file, k) {
    const key = this.key(file, k);
    if (this.loaded.has(key)) return Promise.resolve(this.loaded.get(key).buffer);
    if (this.missing.has(key) || k < 0) return Promise.resolve(null);
    if (performance.now() - (this.failed.get(key) ?? -Infinity) < RETRY_MS) return Promise.resolve(null);
    if (!this.loading.has(key)) {
      const started = performance.now();
      const promise = api
        .audioWindow(file, k)
        .then((bytes) => (bytes ? this.context.decodeAudioData(bytes) : null))
        .then((buffer) => {
          this.loading.delete(key);
          this.failed.delete(key);
          if (buffer) {
            this.loaded.set(key, { buffer, used: ++this.uses });
            this.last = { file, k, milliseconds: performance.now() - started };
            this.evict();
          } else {
            this.missing.add(key);
          }
          this.onload();
          return buffer;
        })
        .catch((error) => {
          this.loading.delete(key);
          this.failed.set(key, performance.now());
          this.onerror(file, k, error);
          return null;
        });
      this.loading.set(key, promise);
    }
    return this.loading.get(key);
  }

  // [[file, k], ...] in order of need: loads them, and keeps them.
  want(windows) {
    this.wanted = new Set(windows.map(([file, k]) => this.key(file, k)));
    for (const [file, k] of windows) this.load(file, k);
  }

  evict() {
    while (this.loaded.size > this.budget) {
      let oldest = null;
      for (const [key, entry] of this.loaded) {
        if (this.wanted.has(key)) continue;
        if (!oldest || entry.used < oldest[1].used) oldest = [key, entry];
      }
      if (!oldest) return;
      this.loaded.delete(oldest[0]);
    }
  }

  // For the checks: how much audio the page holds.
  stats() {
    let bytes = 0;
    for (const { buffer } of this.loaded.values()) bytes += buffer.length * buffer.numberOfChannels * 4;
    return { windows: this.loaded.size, loading: this.loading.size, megabytes: bytes / 1e6, last: this.last };
  }
}
