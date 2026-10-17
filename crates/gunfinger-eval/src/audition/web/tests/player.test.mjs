// The player under held keys (node --test). A held arrow key repeats about
// every 30 ms, faster than the player's lead from a decision to its sound
// (LEAD, 60 ms), so each change cuts sources that have not yet stopped from
// the previous one. Whatever the repeat rate, at most the sources of the
// current schedule may sound: one per sounding lane.

import assert from "node:assert/strict";
import { test } from "node:test";

import * as keys from "../keys.js";
import { Player } from "../player.js";
import { FakeContext, FakeWindows } from "./fake-audio.mjs";

const STEP = 0.005; // seconds of context time per step of the simulation
const TICK_STEPS = 10; // the player's tick runs every 50 ms
const REPEAT = 0.03; // a held key's repeat interval

function question() {
  const markers = [];
  for (let at = 10; at < 600; at += 10) markers.push({ at, kind: "listen", label: `marker at ${at} s` });
  return {
    id: "pair-0000000000000000",
    kind: "pair",
    title: "a pair",
    timeline: { start: 0, end: 600 },
    lanes: [
      { name: "A", pieces: [{ file: "/music/a.mp3", start: 0, end: 600, offset: 0, speed: 1 }] },
      { name: "B", pieces: [{ file: "/music/b.mp3", start: 0, end: 600, offset: 3.5, speed: 1.02 }] },
    ],
    markers,
    spans: [],
  };
}

function setUp(t) {
  t.mock.timers.enable({ apis: ["setInterval"] });
  const context = new FakeContext();
  const player = new Player(context, new FakeWindows());
  player.open(question());
  return { context, player };
}

// What app.js's onKey does with a key event's action.
function press(player, event) {
  const wanted = keys.action({ metaKey: false, ctrlKey: false, altKey: false, shiftKey: false, repeat: false, ...event });
  switch (wanted.name) {
    case "seek":
      return player.seek(player.now() + wanted.by);
    case "marker": {
      const marker = wanted.direction > 0 ? player.nextMarker() : player.previousMarker();
      if (marker) player.jumpTo(marker);
      return;
    }
    case "lane":
      return player.switchTo(wanted.lane);
    case "blend":
      return player.toggleBlend();
    default:
      throw new Error(`the test does not press ${wanted.name}`);
  }
}

// Plays on for `hold + after` seconds and presses the next of `events`
// every REPEAT seconds during the first `hold`, as one held key (`held`) or
// as separate presses. Returns, for each source a
// press replaced, the context time of its earliest replacement: the time
// the newly scheduled sound starts, from which the old source must be
// silent.
function holdKeys(t, context, player, events, { hold = 2, after = 3, held = true } = {}) {
  const replacedAt = new Map();
  const repeatSteps = Math.round(REPEAT / STEP);
  const steps = Math.round((hold + after) / STEP);
  let pressed = 0;
  for (let step = 1; step <= steps; step++) {
    context.advance(STEP);
    if (step % TICK_STEPS === 0) t.mock.timers.tick(50);
    if (step % repeatSteps !== 0 || step * STEP > hold) continue;
    const before = context.sources.length;
    press(player, { ...events[pressed % events.length], repeat: held && pressed > 0 });
    pressed++;
    const fresh = context.sources.slice(before);
    if (fresh.length === 0) continue;
    const cut = Math.min(...fresh.map((source) => source.startTime));
    for (const source of context.sources.slice(0, before)) {
      replacedAt.set(source, Math.min(replacedAt.get(source) ?? Infinity, cut));
    }
  }
  assert.ok(pressed >= 60, `pressed ${pressed} times`);
  return replacedAt;
}

function assertNothingStacks(context, player, replacedAt) {
  const intended = player.sounding().length;
  assert.ok(context.mostAudibleAtOnce() <= intended, `${context.mostAudibleAtOnce()} sources at once, ${intended} intended`);
  assert.ok(replacedAt.size > 60, `${replacedAt.size} sources replaced`);
  for (const [source, cut] of replacedAt) {
    const { from, to } = source.audible();
    if (to <= from || from >= cut) continue; // never heard: disconnected before it started
    assert.ok(source.stopCalls.at(-1) <= cut + 1e-9, `last stop ${source.stopCalls.at(-1)} after the cut at ${cut}`);
    assert.ok(to <= cut + 1e-9, `heard until ${to}, cut at ${cut}`);
  }
  // Still playing, with the intended sources and no more.
  assert.equal(player.playing, true);
  assert.equal(context.audibleAt(context.currentTime).length, intended);
}

test("a held left arrow never stacks sources", (t) => {
  const { context, player } = setUp(t);
  player.seek(400);
  player.play();
  const replacedAt = holdKeys(t, context, player, [{ key: "ArrowLeft" }]);
  assertNothingStacks(context, player, replacedAt);
  assert.ok(player.now() < 330, `the playhead went back to ${player.now()}`);
});

test("a held left arrow with Shift never stacks sources, also at the start", (t) => {
  const { context, player } = setUp(t);
  player.seek(590);
  player.play();
  const replacedAt = holdKeys(t, context, player, [{ key: "ArrowLeft", shiftKey: true }]);
  assertNothingStacks(context, player, replacedAt);
});

test("a held marker key never stacks sources", (t) => {
  const { context, player } = setUp(t);
  player.play();
  const replacedAt = holdKeys(t, context, player, [{ key: "ArrowDown" }]);
  assertNothingStacks(context, player, replacedAt);
  assert.ok(player.now() > 100, `the playhead reached ${player.now()}`);
});

test("a held arrow in the blend never stacks more than its two lanes", (t) => {
  const { context, player } = setUp(t);
  player.play();
  press(player, { key: "2" });
  press(player, { key: "b" });
  assert.equal(player.sounding().length, 2);
  const replacedAt = holdKeys(t, context, player, [{ key: "ArrowRight" }]);
  assertNothingStacks(context, player, replacedAt);
});

test("lane switches faster than the lead never stack sources", (t) => {
  const { context, player } = setUp(t);
  player.play();
  const replacedAt = holdKeys(t, context, player, [{ key: "2" }, { key: "1" }], { held: false });
  assertNothingStacks(context, player, replacedAt);
});
