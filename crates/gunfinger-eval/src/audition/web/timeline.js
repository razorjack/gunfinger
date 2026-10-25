// The timeline arithmetic of the page, in one place. A question file places
// every lane with pieces (docs/audition-questions.md): a piece plays
//
//     file time = offset + speed × t      for timeline time t in [start, end)
//
// The producers compute the pieces (Rust, audition::placement); the page only
// evaluates this formula and its inverse.
//
// A lane may hold other placements beside its `pieces` (which follow every
// alignment): `main`, its main alignment throughout, and `from_start`, its
// file from its start at that alignment's speed. The producers compute them
// too; the page picks one list per lane (key O).

export const PLACEMENTS = ["pieces", "main", "from_start"];

// How the page names each placement: for the lanes together, and in a
// re-placed lane's label.
export const PLACEMENT_WORDS = {
  pieces: { lanes: "placed by every alignment", lane: "" },
  main: { lanes: "placed by the main alignment throughout", lane: "main alignment throughout" },
  from_start: { lanes: "placed from the files' starts", lane: "from its start" },
};

// The placements some lane of the question has, in the order O goes through
// them; always `pieces` first.
export function placementsOf(question) {
  return PLACEMENTS.filter((name) => question.lanes.some((lane) => lane[name]));
}

// The question's lanes under one placement. A lane without it keeps its
// pieces; `placed` says which list a lane plays.
export function placedLanes(question, placement) {
  return question.lanes.map((lane) => {
    const placed = lane[placement] ? placement : "pieces";
    return { name: lane.name, pieces: lane[placed], placed };
  });
}

export function fileTime(piece, t) {
  return piece.offset + piece.speed * t;
}

export function timelineTime(piece, seconds) {
  return (seconds - piece.offset) / piece.speed;
}

// The piece playing at t, or null where the lane is silent.
export function pieceAt(lane, t) {
  return lane.pieces.find((piece) => piece.start <= t && t < piece.end) ?? null;
}

// The piece playing at t or, in a gap, the next one; null after the last.
export function pieceFrom(lane, t) {
  return lane.pieces.find((piece) => t < piece.end) ?? null;
}

export function clamp(t, timeline) {
  return Math.min(Math.max(t, timeline.start), timeline.end);
}

// 1:02:03.4, 3:02.1, -0:09.4
export function clock(seconds) {
  const sign = seconds < 0 ? "-" : "";
  const tenths = Math.round(Math.abs(seconds) * 10);
  const hours = Math.floor(tenths / 36000);
  const minutes = Math.floor(tenths / 600) % 60;
  const rest = ((tenths % 600) / 10).toFixed(1).padStart(4, "0");
  return hours > 0
    ? `${sign}${hours}:${String(minutes).padStart(2, "0")}:${rest}`
    : `${sign}${minutes}:${rest}`;
}

export function percent(speed) {
  return `${(speed * 100).toFixed(2)}%`;
}

export function signedSeconds(seconds) {
  return `${seconds >= 0 ? "+" : "−"}${Math.abs(seconds).toFixed(2)} s`;
}
