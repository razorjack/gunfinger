// Plays a question's lanes through Web Audio, on the timeline the question
// file defines (timeline.js has the arithmetic).
//
// Scheduling. Sources are scheduled on the audio context's clock, a few
// seconds ahead of the playhead, from a cursor { t, ctx }: timeline time t
// sounds at context time ctx. Each step of the schedule covers a stretch of
// the timeline in which every sounding lane plays one piece from one window
// (or nothing), so a step is one AudioBufferSourceNode per lane. The steps
// are kept as the clock: the playhead is read from it by interpolation.
//
// Rates. With speed matching on, the timeline advances one second per second
// and a lane plays at playbackRate = its piece's speed (file seconds per
// timeline second): pitch and tempo move together, as on a turntable. With
// it off, the lane plays at rate 1 and the timeline advances 1/speed seconds
// per second, so the playhead follows that lane's own clock. Two lanes
// played together (the blend) always match speed.
//
// Changes. A lane switch, a seek or a mode change cuts every source at one
// context time `when` (just ahead of now) and schedules the new sound from
// the timeline moment the clock gives for `when`: gapless, at the same
// musical moment. A window that has not arrived yet holds the cursor: the
// playhead waits there in silence and the schedule resumes once it loads.
// A piece that starts a hair before its file (the schema allows 1 ms of
// rounding) plays from the file's start.

import { clamp, fileTime, pieceAt, pieceFrom, timelineTime } from "./timeline.js";

const LEAD = 0.06; // seconds from a decision to the sound it changes
const HORIZON = 4; // seconds of audio scheduled ahead of the playhead
const LOOP_HALF = 4; // a loop plays this long either side of its moment
export const MARKER_LEAD = 2; // a jump to a marker starts this early, to hear into it
const BLEND_GAIN = 0.5; // panned hard, a stereo source sums its two channels
const SHORTEST_STEP = 0.0005;

export class Player {
  constructor(context, windows) {
    this.context = context;
    this.windows = windows;
    this.output = context.createGain();
    this.output.connect(context.destination);
    this.question = null;
    this.generation = 0;
    this.onchange = () => {};
    this.onend = () => {};
    setInterval(() => this.tick(), 50);
  }

  open(question) {
    this.pause();
    this.question = question;
    this.lane = 0;
    this.previous = question.lanes.length > 1 ? 1 : null;
    this.blend = false;
    this.matching = true;
    this.loop = null;
    this.position = question.timeline.start;
    this.sources = [];
    this.clock = [];
    this.cursor = null;
  }

  // The lanes that sound now, with their pan: the current lane alone, or in
  // the blend the lower-numbered lane left and the other right.
  sounding() {
    if (this.blend && this.previous !== null) {
      const [left, right] = [this.lane, this.previous].sort((a, b) => a - b);
      return [{ lane: left, pan: -1 }, { lane: right, pan: 1 }];
    }
    return [{ lane: this.lane, pan: 0 }];
  }

  matchingNow() {
    return this.matching || (this.blend && this.previous !== null);
  }

  // ---- the playhead -------------------------------------------------------

  // The timeline moment that sounds at context time c.
  at(c) {
    if (!this.playing) return this.position;
    for (let index = this.clock.length - 1; index >= 0; index--) {
      const step = this.clock[index];
      if (c < step.ctx0) continue;
      if (c >= step.ctx1) return step.t1;
      return step.t0 + ((c - step.ctx0) / (step.ctx1 - step.ctx0)) * (step.t1 - step.t0);
    }
    // Before the first step sounds (just after play or a wait), the playhead
    // is where the schedule starts; with no steps, where the cursor waits.
    if (this.clock.length > 0) return this.clock[0].t0;
    return this.cursor ? this.cursor.t : this.position;
  }

  now() {
    return this.at(this.context.currentTime);
  }

  // ---- transport ----------------------------------------------------------

  play() {
    if (this.playing || !this.question) return;
    this.context.resume();
    if (this.position >= this.question.timeline.end - 0.01) this.position = this.question.timeline.start;
    this.playing = true;
    this.startAt(this.position, this.context.currentTime + LEAD);
  }

  pause() {
    if (!this.playing) return;
    this.position = this.now();
    this.cut(this.context.currentTime);
    this.playing = false;
    this.clock = [];
    this.cursor = null;
    this.onchange();
  }

  toggle() {
    this.playing ? this.pause() : this.play();
  }

  seek(t) {
    t = clamp(t, this.question.timeline);
    if (this.loop) this.loop = this.loopAround(t);
    if (this.playing) this.startAt(t, this.context.currentTime + LEAD);
    else this.position = t;
    this.onchange();
  }

  switchTo(lane) {
    if (lane >= this.question.lanes.length) return;
    if (lane !== this.lane) [this.previous, this.lane] = [this.lane, lane];
    this.blend = false;
    this.restartHere();
  }

  switchBack() {
    if (this.previous !== null) this.switchTo(this.previous);
  }

  toggleBlend() {
    if (this.previous === null) return false;
    this.blend = !this.blend;
    this.restartHere();
    return true;
  }

  toggleMatching() {
    this.matching = !this.matching;
    this.restartHere();
  }

  toggleLoop() {
    this.loop = this.loop ? null : this.loopAround(this.now());
    this.restartHere();
  }

  loopAround(t) {
    const { timeline } = this.question;
    return { start: clamp(t - LOOP_HALF, timeline), end: clamp(t + LOOP_HALF, timeline) };
  }

  // Applies a change at the moment that sounds just ahead of now.
  restartHere() {
    if (this.playing) {
      const when = this.context.currentTime + LEAD;
      this.startAt(this.at(when), when);
    }
    this.onchange();
  }

  startAt(t, when) {
    this.cut(when);
    this.generation++;
    this.cursor = { t, ctx: when, finished: false };
    this.schedule();
    this.onchange();
  }

  // Silences every source from context time `when` and ends the clock there.
  // A source due to stop before `when` keeps its stop time: a later stop()
  // call would replace it and play the source on.
  cut(when) {
    for (const { source, start, stop } of this.sources) {
      if (start >= when) source.disconnect();
      else if (stop > when) source.stop(when);
    }
    this.sources = this.sources.filter(({ start }) => start < when);
    const t = this.at(when);
    this.clock = this.clock
      .filter((step) => step.ctx0 < when)
      .map((step) => (step.ctx1 > when ? { ...step, ctx1: when, t1: t } : step));
  }

  // ---- the schedule -------------------------------------------------------

  schedule() {
    const { timeline, lanes } = this.question;
    const seconds = this.windows.seconds;
    for (;;) {
      if (this.loop && this.cursor.t >= this.loop.end - SHORTEST_STEP) this.cursor.t = this.loop.start;
      const t = this.cursor.t;
      const end = this.loop ? this.loop.end : timeline.end;
      // Before the wait below moves the cursor's time, so tick() sees the
      // end come.
      if (t >= end - SHORTEST_STEP) {
        this.cursor.finished = true;
        return;
      }
      const nowCtx = this.context.currentTime;
      if (this.cursor.ctx < nowCtx + 0.01) this.cursor.ctx = nowCtx + LEAD; // after a wait
      if (this.cursor.ctx > nowCtx + HORIZON) return;
      let stepEnd = end;
      const parts = [];
      for (const { lane, pan } of this.sounding()) {
        const piece = pieceAt(lanes[lane], t);
        if (!piece) {
          const next = pieceFrom(lanes[lane], t);
          if (next) stepEnd = Math.min(stepEnd, next.start);
          continue;
        }
        stepEnd = Math.min(stepEnd, piece.end);
        const file = Math.max(0, fileTime(piece, t));
        const k = this.windows.index(file + 1e-9);
        const buffer = this.windows.buffer(piece.file, k);
        if (!buffer) {
          if (!this.windows.settled(piece.file, k)) {
            // Not here yet: hold the cursor until it loads.
            this.windows.load(piece.file, k);
            return;
          }
          continue; // past the file's end: silent here
        }
        // A window a sample short of full counts as full, so no step of a
        // fraction of a millisecond falls between two windows.
        const windowEnd = k * seconds + (buffer.duration > seconds - 0.001 ? seconds : buffer.duration);
        if (file >= windowEnd - 1e-6) {
          stepEnd = Math.min(stepEnd, timelineTime(piece, (k + 1) * seconds));
          continue; // the file ends inside this window
        }
        stepEnd = Math.min(stepEnd, timelineTime(piece, windowEnd));
        parts.push({ piece, buffer, offset: Math.max(0, file - k * seconds), pan });
      }
      if (stepEnd - t < SHORTEST_STEP) stepEnd = t + SHORTEST_STEP;
      const matching = this.matchingNow();
      // Context seconds per timeline second in this step.
      const rate = matching || parts.length === 0 ? 1 : parts[0].piece.speed;
      const ctx0 = this.cursor.ctx;
      const ctx1 = ctx0 + rate * (stepEnd - t);
      for (const { piece, buffer, offset, pan } of parts) {
        this.playPart(buffer, offset, matching ? piece.speed : 1, pan, ctx0, ctx1);
      }
      this.clock.push({ ctx0, ctx1, t0: t, t1: stepEnd });
      this.cursor = { t: stepEnd, ctx: ctx1, finished: false };
    }
  }

  playPart(buffer, offset, playbackRate, pan, start, stop) {
    const source = this.context.createBufferSource();
    source.buffer = buffer;
    source.playbackRate.value = playbackRate;
    let node = source;
    if (pan !== 0) {
      const gain = this.context.createGain();
      gain.gain.value = BLEND_GAIN;
      const panner = this.context.createStereoPanner();
      panner.pan.value = pan;
      source.connect(gain).connect(panner);
      node = panner;
    }
    node.connect(this.output);
    source.start(start, offset);
    source.stop(stop);
    const entry = { source, start, stop, playbackRate };
    this.sources.push(entry);
    source.onended = () => {
      this.sources = this.sources.filter((other) => other !== entry);
    };
  }

  tick() {
    if (!this.question) return;
    if (this.playing) {
      this.schedule();
      const nowCtx = this.context.currentTime;
      this.clock = this.clock.filter((step) => step.ctx1 > nowCtx - 2);
      if (this.cursor.finished && nowCtx >= this.cursor.ctx) {
        this.position = this.cursor.t;
        this.playing = false;
        this.clock = [];
        this.onend();
      }
    }
    this.prefetch();
    this.onchange();
  }

  // ---- windows worth having soon -----------------------------------------

  prefetch() {
    const { lanes } = this.question;
    const t = this.now();
    const wanted = [];
    const add = (lane, at) => {
      const piece = pieceAt(lanes[lane], at) ?? pieceFrom(lanes[lane], at);
      if (!piece) return;
      const file = Math.max(0, fileTime(piece, Math.max(at, piece.start)));
      wanted.push([piece.file, this.windows.index(file + 1e-9)]);
    };
    const sounding = this.sounding().map(({ lane }) => lane);
    for (const lane of sounding) for (const ahead of [0, 8, 20, 40, -10]) add(lane, t + ahead);
    for (let lane = 0; lane < lanes.length; lane++) for (const ahead of [0, 12]) add(lane, t + ahead);
    for (const marker of [this.previousMarker(t), this.nextMarker(t)]) {
      if (marker) for (const lane of sounding) add(lane, marker.at - MARKER_LEAD);
    }
    if (this.loop) for (const lane of sounding) add(lane, this.loop.start);
    this.windows.want(wanted);
  }

  // ---- markers ------------------------------------------------------------

  // Up and Down jump to a marker less MARKER_LEAD, so the moment the marker
  // stands for is the one heard after the lead-in. They pass over `join`
  // markers, which only show where a lane moves to another excerpt.
  stops() {
    return this.question.markers.filter((marker) => marker.kind !== "join");
  }

  nextMarker(t = this.now()) {
    return this.stops().find((marker) => marker.at > t + MARKER_LEAD + 0.05) ?? null;
  }

  previousMarker(t = this.now()) {
    return this.stops().findLast((marker) => marker.at < t + MARKER_LEAD - 0.05) ?? null;
  }

  jumpTo(marker) {
    this.seek(marker.at - MARKER_LEAD);
  }

  // ---- for the page and the checks ----------------------------------------

  // What each lane plays at the playhead.
  fileTimes(t = this.now()) {
    return this.question.lanes.map((lane) => {
      const piece = pieceAt(lane, t);
      return piece ? { file: piece.file, seconds: fileTime(piece, t), piece } : null;
    });
  }

  snapshot() {
    const nowCtx = this.context.currentTime;
    return {
      lane: this.lane,
      previous: this.previous,
      playing: this.playing,
      position: this.now(),
      matching: this.matching,
      blend: this.blend,
      loop: this.loop,
      sounding: this.sounding(),
      playbackRates: this.sources
        .filter(({ start, stop }) => start <= nowCtx + HORIZON && stop > nowCtx)
        .map(({ playbackRate }) => playbackRate),
      fileTimes: this.fileTimes().map((at) => at && { file: at.file, seconds: at.seconds }),
      contextState: this.context.state,
      windows: this.windows.stats(),
    };
  }
}
