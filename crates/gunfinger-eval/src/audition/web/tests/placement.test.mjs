// The lanes' placements (node --test): O moves an aligned lane from its
// pieces (every alignment) to its main alignment throughout and to its file
// from its start, and back. In every placement the reference lane, the
// playhead, the sources scheduled, the windows wanted and the rates with and
// without speed matching follow the placement in force.

import assert from "node:assert/strict";
import { test } from "node:test";

import * as keys from "../keys.js";
import { Player } from "../player.js";
import { placedLanes, placementsOf } from "../timeline.js";
import { FakeContext, FakeWindows } from "./fake-audio.mjs";

const B = "/music/b.m4a";

// Two rips of one recording: lane B follows alignment 1 (offset +0.44 s)
// until A 320.3 s, then a repeat of its section at 1:12 (alignment 2,
// offset -235.23 s), as session 40's pack item 1 did.
function question() {
  return {
    id: "pair-0000000000000001",
    kind: "pair",
    title: "two rips",
    timeline: { start: 0, end: 387.5, reference: 0 },
    lanes: [
      { name: "A", pieces: [{ file: "/music/a.opus", start: 0, end: 387.5, offset: 0, speed: 1 }] },
      {
        name: "B",
        pieces: [
          { file: B, start: 0, end: 320.3, offset: 0.44, speed: 0.9594, label: "alignment 1: 3298 hits" },
          { file: B, start: 320.3, end: 387.5, offset: -235.23, speed: 0.9601, label: "alignment 2: 361 hits" },
        ],
        main: [{ file: B, start: 0, end: 387.5, offset: 0.44, speed: 0.9594, label: "alignment 1: 3298 hits, throughout" }],
        from_start: [{ file: B, start: 0, end: 387.5, offset: 0, speed: 0.9594, label: "from the file's start" }],
      },
    ],
    markers: [],
    spans: [],
  };
}

function setUp(t) {
  t.mock.timers.enable({ apis: ["setInterval"] });
  const context = new FakeContext();
  const windows = new FakeWindows();
  const player = new Player(context, windows);
  player.open(question());
  return { context, windows, player };
}

function placementKey(player) {
  const wanted = keys.action({ key: "o", metaKey: false, ctrlKey: false, altKey: false, shiftKey: false, repeat: false });
  assert.equal(wanted.name, "placement");
  return player.nextPlacement();
}

// The file time each source of the current schedule starts at, and its rate.
function scheduled(context, from) {
  return context.sources
    .slice(from)
    .filter((source) => source.stopTime > context.currentTime)
    .map((source) => ({
      file: source.buffer.file,
      seconds: source.buffer.k * 30 + source.offset,
      rate: source.playbackRate.value,
      at: source.startTime,
    }));
}

const close = (actual, expected, what) =>
  assert.ok(Math.abs(actual - expected) < 1e-6, `${what}: ${actual}, expected ${expected}`);

test("a lane without other placements keeps its pieces, and O says so", (t) => {
  const { player } = setUp(t);
  const plain = question();
  delete plain.lanes[1].main;
  delete plain.lanes[1].from_start;
  player.open(plain);

  assert.deepEqual(placementsOf(plain), ["pieces"]);
  assert.equal(placementKey(player), false);
  assert.equal(player.placement, "pieces");
  assert.deepEqual(player.lanes[1].pieces, plain.lanes[1].pieces);
});

test("O goes through every alignment, the main alignment and the file's start, and back", (t) => {
  const { player } = setUp(t);
  const at = 350; // inside alignment 2's piece

  assert.equal(player.placement, "pieces");
  close(player.fileTimes(at)[1].seconds, -235.23 + 0.9601 * at, "every alignment");
  assert.equal(placementKey(player), true);
  assert.equal(player.placement, "main");
  close(player.fileTimes(at)[1].seconds, 0.44 + 0.9594 * at, "main alignment");
  assert.equal(placementKey(player), true);
  assert.equal(player.placement, "from_start");
  close(player.fileTimes(at)[1].seconds, 0.9594 * at, "from the start");
  assert.equal(placementKey(player), true);
  assert.equal(player.placement, "pieces");
  // The reference lane plays its own time in every placement.
  for (const placement of ["pieces", "main", "from_start"]) {
    assert.deepEqual(placedLanes(question(), placement)[0].pieces, question().lanes[0].pieces);
  }
});

test("a question opens placed by every alignment, whatever the last one used", (t) => {
  const { player } = setUp(t);
  placementKey(player);
  assert.equal(player.placement, "main");

  player.open(question());

  assert.equal(player.placement, "pieces");
  assert.equal(player.lanes[1].placed, "pieces");
});

test("a change of placement while playing keeps the moment and re-places lane B", (t) => {
  const { context, windows, player } = setUp(t);
  player.switchTo(1);
  player.seek(340);
  player.play();
  for (let step = 0; step < 100; step++) {
    context.advance(0.01);
    if (step % 5 === 4) t.mock.timers.tick(50);
  }
  const before = player.now();
  const first = scheduled(context, 0)[0];
  assert.equal(first.file, B);
  close(first.seconds, -235.23 + 0.9601 * 340, "the repeat's file time at A 340 s");
  close(first.rate, 0.9601, "the repeat's rate");

  const sources = context.sources.length;
  placementKey(player);
  t.mock.timers.tick(50);

  const [now] = scheduled(context, sources);
  // The new sound starts at the moment the clock gives for its start time.
  const moment = player.at(now.at);
  assert.ok(Math.abs(moment - before) < 0.2, `moved from ${before} to ${moment}`);
  close(now.seconds, 0.44 + 0.9594 * moment, "the main alignment's file time");
  close(now.rate, 0.9594, "the main alignment's rate");
  // The windows kept are lane B's under the main alignment.
  const wantedB = windows.wanted.filter(([file]) => file === B).map(([, k]) => k);
  assert.ok(wantedB.includes(Math.floor((0.44 + 0.9594 * moment) / 30)), `wanted ${wantedB}`);
  assert.ok(!wantedB.includes(Math.floor((-235.23 + 0.9601 * moment) / 30)), `wanted ${wantedB}`);
  // No more than the one lane sounds.
  assert.equal(context.audibleAt(context.currentTime + 0.1).length, 1);
});

test("without speed matching the playhead follows lane B's own clock in the new placement", (t) => {
  const { context, player } = setUp(t);
  player.switchTo(1);
  player.toggleMatching();
  placementKey(player);
  placementKey(player); // from the file's start
  player.seek(100);
  player.play();
  for (let step = 0; step < 300; step++) {
    context.advance(0.01);
    if (step % 5 === 4) t.mock.timers.tick(50);
  }

  const [source] = scheduled(context, 0);
  assert.equal(source.rate, 1);
  close(source.seconds, 0.9594 * 100, "B from its start at A 100 s");
  // 3 s of B's own time are 3 / 0.9594 s of A's timeline.
  const elapsed = player.now() - 100;
  assert.ok(Math.abs(elapsed - (context.currentTime - source.at) / 0.9594) < 0.02, `${elapsed}`);
});

test("the control letters are the server's (schema::CONTROL_KEYS), O among them", () => {
  const press = (key) => keys.action({ key, metaKey: false, ctrlKey: false, altKey: false, shiftKey: false, repeat: false });
  const controls = [..."abcdefghijklmnopqrstuvwxyz"].filter((key) => press(key).name !== "choice");

  assert.deepEqual(controls, [..."bjklmoz"]);
  assert.equal(press("O").name, "placement");
});
