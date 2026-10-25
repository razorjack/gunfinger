// The lanes of the open question: one waveform per lane on the shared
// timeline, the shaded spans, the markers, the loop and the playhead; under
// each waveform the file playing there (full path, copy, show its folder)
// and its placement in words. Mouse: click or drag to seek, click a marker
// to jump to it.

import * as api from "./api.js";
import { PLACEMENT_WORDS, PLACEMENTS, clock, fileTime, percent, pieceAt, signedSeconds } from "./timeline.js";

const HEIGHT = 84;
const MARKER_REACH = 5; // pixels within which a click takes the marker
const LANE_COLOURS = 9;

// The canvas's colours, from style.css's custom properties.
function colours() {
  const style = getComputedStyle(document.documentElement);
  const read = (name) => style.getPropertyValue(name).trim();
  return {
    background: read("--lane-background"),
    tick: read("--lane-tick"),
    lane: Array.from({ length: LANE_COLOURS }, (_, index) => read(`--lane-${index + 1}`)),
    dim: read("--lane-dim"),
    span: { fill: read("--span-fill"), edge: read("--span-edge") },
    loop: { fill: read("--loop-fill"), edge: read("--loop-edge") },
    playhead: read("--playhead"),
    marker: { listen: read("--marker-listen"), edge: read("--marker-edge"), join: read("--marker-join") },
  };
}

export class LanesView {
  constructor(root, tooltip, player, say) {
    this.root = root;
    this.tooltip = tooltip;
    this.player = player;
    this.say = say;
    this.envelopes = new Map(); // file -> envelope, or null while loading
    this.rows = [];
    this.drag = null;
    this.colours = colours();
  }

  open(question) {
    this.question = question;
    this.root.replaceChildren();
    this.rows = question.lanes.map((lane, index) => this.row(lane, index));
    const pieces = question.lanes.flatMap((lane) => PLACEMENTS.flatMap((placement) => lane[placement] ?? []));
    for (const file of new Set(pieces.map((piece) => piece.file))) {
      if (this.envelopes.has(file)) continue;
      this.envelopes.set(file, null);
      api
        .envelope(file)
        .then((envelope) => this.envelopes.set(file, envelope))
        .catch((error) => this.say(`no waveform for ${file}: ${error.message}`));
    }
  }

  row(lane, index) {
    const row = document.createElement("div");
    row.className = "lane";
    row.style.setProperty("--lane-colour", this.colours.lane[index % LANE_COLOURS]);
    const label = document.createElement("div");
    label.className = "lane-label";
    label.textContent = `${index + 1} · ${lane.name}`;
    const canvas = document.createElement("canvas");
    canvas.height = HEIGHT;
    const info = document.createElement("div");
    info.className = "lane-info";
    const path = document.createElement("code");
    const copy = button("copy", "Copy the full path", () => this.copy(path.textContent));
    const reveal = button("reveal", "Show the file in its folder", () =>
      api.reveal(path.textContent).catch((error) => this.say(error.message)),
    );
    const words = document.createElement("span");
    words.className = "muted";
    info.append(path, copy, reveal, words);
    row.append(label, canvas, info);
    this.root.append(row);
    this.listen(canvas);
    return { row, label, canvas, path, words };
  }

  async copy(text) {
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // Without the clipboard API: select a text area and copy.
      const area = document.createElement("textarea");
      area.value = text;
      document.body.append(area);
      area.select();
      document.execCommand("copy");
      area.remove();
    }
    this.say(`copied ${text}`);
  }

  // ---- mouse --------------------------------------------------------------

  timeAt(canvas, clientX) {
    const box = canvas.getBoundingClientRect();
    const { start, end } = this.question.timeline;
    const share = Math.min(Math.max((clientX - box.left) / box.width, 0), 1);
    return start + share * (end - start);
  }

  markerNear(canvas, clientX) {
    const box = canvas.getBoundingClientRect();
    const { start, end } = this.question.timeline;
    return (
      this.question.markers.find((marker) => {
        const x = box.left + ((marker.at - start) / (end - start)) * box.width;
        return Math.abs(x - clientX) <= MARKER_REACH;
      }) ?? null
    );
  }

  listen(canvas) {
    canvas.addEventListener("pointerdown", (event) => {
      canvas.setPointerCapture(event.pointerId);
      this.drag = { canvas, t: this.timeAt(canvas, event.clientX), marker: this.markerNear(canvas, event.clientX), moved: false };
    });
    canvas.addEventListener("pointermove", (event) => {
      if (this.drag?.canvas === canvas) {
        this.drag.t = this.timeAt(canvas, event.clientX);
        this.drag.moved = true;
      }
      this.hover(canvas, event);
    });
    canvas.addEventListener("pointerup", () => {
      const drag = this.drag;
      this.drag = null;
      if (!drag) return;
      if (drag.marker && !drag.moved) this.player.jumpTo(drag.marker);
      else this.player.seek(drag.t);
    });
    canvas.addEventListener("pointerleave", () => (this.tooltip.hidden = true));
  }

  hover(canvas, event) {
    const marker = this.markerNear(canvas, event.clientX);
    const t = this.timeAt(canvas, event.clientX);
    const span = this.question.spans.find(
      (span) => span.start <= t && t < span.end && span.lanes.includes(this.rows.findIndex((row) => row.canvas === canvas)),
    );
    const text = marker ? `${clock(marker.at)} · ${marker.label}` : span ? `${clock(t)} · ${span.label}` : clock(t);
    this.tooltip.textContent = text;
    this.tooltip.hidden = false;
    this.tooltip.style.left = `${event.clientX + 12}px`;
    this.tooltip.style.top = `${event.clientY + 14}px`;
  }

  // ---- drawing ------------------------------------------------------------

  draw() {
    if (!this.question) return;
    const player = this.player;
    const t = this.drag ? this.drag.t : player.now();
    const sounding = new Set(player.sounding().map(({ lane }) => lane));
    player.lanes.forEach((lane, index) => {
      const { row, label, canvas, path, words } = this.rows[index];
      const placed = lane.placed === "pieces" ? "" : ` · ${PLACEMENT_WORDS[lane.placed].lane}`;
      const named = `${index + 1} · ${lane.name}${placed}`;
      if (label.textContent !== named) label.textContent = named;
      this.drawLane(canvas, lane, index, t, sounding.has(index));
      row.classList.toggle("current", index === player.lane);
      row.classList.toggle("sounding", sounding.has(index));
      const piece = pieceAt(lane, t) ?? lane.pieces.find((candidate) => candidate.start > t) ?? lane.pieces.at(-1);
      if (path.textContent !== piece.file) path.textContent = piece.file;
      words.textContent = this.words(piece, t);
    });
  }

  // The placement of the piece at t in words.
  words(piece, t) {
    const at = pieceAt({ pieces: [piece] }, t) ? `at ${clock(fileTime(piece, t))} of the file` : "silent here";
    const label = piece.label ? ` · ${piece.label}` : "";
    return `${at} · offset ${signedSeconds(piece.offset)} · speed ${percent(piece.speed)}${label}`;
  }

  drawLane(canvas, lane, index, t, sounding) {
    const width = Math.max(1, Math.floor(canvas.clientWidth * devicePixelRatio));
    if (canvas.width !== width) canvas.width = width;
    const height = canvas.height;
    const context = canvas.getContext("2d");
    const colours = this.colours;
    const { start, end } = this.question.timeline;
    const x = (time) => ((time - start) / (end - start)) * width;
    const timeOf = (pixel) => start + (pixel / width) * (end - start);
    const line = Math.max(1, devicePixelRatio);
    context.fillStyle = colours.background;
    context.fillRect(0, 0, width, height);

    // A span is shaded, with a band along its top that stays visible over a
    // loud waveform; the loop is shaded between two edges.
    for (const span of this.question.spans) {
      if (!span.lanes.includes(index)) continue;
      context.fillStyle = colours.span.fill;
      context.fillRect(x(span.start), 0, x(span.end) - x(span.start), height);
      context.fillStyle = colours.span.edge;
      context.fillRect(x(span.start), 0, x(span.end) - x(span.start), 2 * line);
    }
    const loop = this.player.loop;
    if (loop) {
      context.fillStyle = colours.loop.fill;
      context.fillRect(x(loop.start), 0, x(loop.end) - x(loop.start), height);
      context.fillStyle = colours.loop.edge;
      context.fillRect(Math.round(x(loop.start)), 0, line, height);
      context.fillRect(Math.round(x(loop.end)) - line, 0, line, height);
    }

    // The waveform: for each pixel column, the extremes of the envelope's
    // buckets over the file time that column plays (dim), and their RMS
    // level (bright), which keeps the shape of a loud master.
    const middle = height / 2;
    const colour = sounding ? colours.lane[index % LANE_COLOURS] : colours.dim;
    const peaks = new Path2D();
    const levels = new Path2D();
    for (const piece of lane.pieces) {
      const envelope = this.envelopes.get(piece.file);
      const first = Math.max(0, Math.floor(x(piece.start)));
      const last = Math.min(width, Math.ceil(x(piece.end)));
      if (!envelope) {
        levels.moveTo(first, middle);
        levels.lineTo(last, middle);
        continue;
      }
      for (let pixel = first; pixel < last; pixel++) {
        const from = fileTime(piece, Math.max(timeOf(pixel), piece.start)) / envelope.bucket_seconds;
        const to = fileTime(piece, Math.min(timeOf(pixel + 1), piece.end)) / envelope.bucket_seconds;
        let low = 0;
        let high = 0;
        let level = 0;
        for (let bucket = Math.max(0, Math.floor(from)); bucket <= Math.floor(to) && bucket < envelope.min.length; bucket++) {
          low = Math.min(low, envelope.min[bucket]);
          high = Math.max(high, envelope.max[bucket]);
          level = Math.max(level, envelope.rms[bucket]);
        }
        peaks.moveTo(pixel + 0.5, middle - (high / 127) * middle);
        peaks.lineTo(pixel + 0.5, middle - (low / 127) * middle + 1);
        levels.moveTo(pixel + 0.5, middle - (level / 127) * middle);
        levels.lineTo(pixel + 0.5, middle + (level / 127) * middle + 1);
      }
    }
    context.strokeStyle = colour;
    context.globalAlpha = 0.35;
    context.stroke(peaks);
    context.globalAlpha = 1;
    context.stroke(levels);

    // Where the lane moves to another piece.
    context.fillStyle = colours.tick;
    for (const piece of lane.pieces.slice(1)) context.fillRect(Math.round(x(piece.start)), height - 6, 1, 6);

    // Markers one can jump to carry a flag at the top; `join` markers,
    // which Up and Down pass over, are a plain line.
    for (const marker of this.question.markers) {
      const at = Math.round(x(marker.at));
      context.fillStyle = colours.marker[marker.kind] ?? colours.tick;
      context.fillRect(at, 0, line, height);
      if (marker.kind !== "join") triangle(context, at + line / 2, 0, 4 * line, 1);
    }

    // The playhead, twice as wide, with a cap at either end.
    const playhead = Math.round(x(t));
    context.fillStyle = colours.playhead;
    context.fillRect(playhead - line / 2, 0, 2 * line, height);
    triangle(context, playhead + line / 2, 0, 4 * line, 1);
    triangle(context, playhead + line / 2, height, 4 * line, -1);
  }
}

// A triangle pointing into the lane from its top (direction 1) or bottom
// (-1), centred on x.
function triangle(context, x, y, size, direction) {
  context.beginPath();
  context.moveTo(x - size, y);
  context.lineTo(x + size, y);
  context.lineTo(x, y + direction * size * 1.2);
  context.closePath();
  context.fill();
}

function button(text, title, action) {
  const element = document.createElement("button");
  element.type = "button";
  element.textContent = text;
  element.title = title;
  element.addEventListener("click", (event) => {
    event.currentTarget.blur();
    action();
  });
  return element;
}
