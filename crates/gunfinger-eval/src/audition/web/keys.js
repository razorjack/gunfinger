// The page's keys. Control keys are fixed here; answers take their keys from
// the question's choices, which the schema check keeps off the control keys
// (schema::CONTROL_KEYS: b j k l m o z). Keys with Cmd, Ctrl or Alt are left to
// the browser, so Cmd-Z, Cmd-C and Cmd-R work as usual.

export const CONTROLS = [
  ["Space", "play or pause"],
  ["1 2 3 …", "switch to that lane at the same moment"],
  ["Tab", "back to the lane played before"],
  ["← →", "seek 5 s (Shift: 30 s)"],
  ["↑ ↓", "previous or next marker, from 2 s before it"],
  ["L", "loop 8 s around the moment (seeking moves the loop)"],
  ["M", "speed matching on or off"],
  ["O", "place the lanes by every alignment, the main alignment throughout, or from their files' starts"],
  ["B", "this lane and the one before together, lower lane left"],
  ["J K", "next or previous question"],
  ["Z", "undo the latest answer, then the one before"],
  ["?", "this help"],
];

const SEEK = 5;
const SEEK_LARGE = 30;

// A held key repeats only these. A held answer key must not go on to answer
// the questions that open after it, nor a held Z undo several answers.
const REPEATING = new Set(["seek", "marker"]);

// The action a key event asks for, or null; { name: "held" } for a repeat
// to ignore.
export function action(event) {
  if (event.metaKey || event.ctrlKey || event.altKey) return null;
  const wanted = named(event);
  if (wanted && event.repeat && !REPEATING.has(wanted.name)) return { name: "held" };
  return wanted;
}

function named(event) {
  switch (event.key) {
    case " ":
      return { name: "toggle" };
    case "Tab":
      return { name: "back" };
    case "ArrowLeft":
      return { name: "seek", by: event.shiftKey ? -SEEK_LARGE : -SEEK };
    case "ArrowRight":
      return { name: "seek", by: event.shiftKey ? SEEK_LARGE : SEEK };
    case "ArrowUp":
      return { name: "marker", direction: -1 };
    case "ArrowDown":
      return { name: "marker", direction: 1 };
    case "?":
      return { name: "help" };
    case "Escape":
      return { name: "close" };
  }
  if (/^[1-9]$/.test(event.key)) return { name: "lane", lane: Number(event.key) - 1 };
  const letter = event.key.length === 1 ? event.key.toLowerCase() : "";
  switch (letter) {
    case "l":
      return { name: "loop" };
    case "m":
      return { name: "matching" };
    case "o":
      return { name: "placement" };
    case "b":
      return { name: "blend" };
    case "j":
      return { name: "question", direction: 1 };
    case "k":
      return { name: "question", direction: -1 };
    case "z":
      return { name: "undo" };
  }
  return /^[a-z]$/.test(letter) ? { name: "choice", key: letter } : null;
}
