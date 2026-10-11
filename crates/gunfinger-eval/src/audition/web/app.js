// The listening page: loads the questions, opens the first one without an
// answer, and turns keys and clicks into the player's actions and answers.
// The page knows no kinds of question: it shows what each question says.
//
// For the checks, window.audition exposes the state (snapshot()), opens a
// question by id (open()) and hands out the player (player()).

import * as api from "./api.js";
import * as keys from "./keys.js";
import { LanesView } from "./lanes.js";
import { Player } from "./player.js";
import { Queue, source } from "./queue.js";
import { clock, percent } from "./timeline.js";
import { Windows } from "./windows.js";

const element = (id) => document.getElementById(id);

const context = new AudioContext({ sampleRate: 44100 });
let session = null;
let current = null;
let windows = null;
let player = null;
let lanes = null;
let queue = null;

function say(text, kind = "info") {
  const message = element("message");
  message.textContent = text;
  message.className = kind;
}

function questions() {
  return session.files.flatMap((file) => file.questions);
}

async function start() {
  try {
    session = await api.session();
  } catch (error) {
    element("title").textContent = "Cannot load the questions";
    say(error.message, "error");
    return;
  }
  windows = new Windows(context, session.window_seconds);
  player = new Player(context, windows);
  lanes = new LanesView(element("lanes"), element("tooltip"), player, say);
  queue = new Queue(element("queue"), open);
  player.onend = () => say("the end of the timeline");
  drawLegend();
  open(questions().find((question) => !question.state) ?? questions()[0]);
  document.addEventListener("keydown", onKey);
  requestAnimationFrame(frame);
}

function open(question) {
  if (!question) return;
  current = question;
  player.open(question);
  lanes.open(question);
  queue.render(session, current);
  const all = questions();
  element("position").textContent = `question ${all.indexOf(question) + 1} of ${all.length} · ${question.kind}`;
  element("title").textContent = question.title;
  element("text").textContent = question.text;
  element("details").replaceChildren(
    ...question.details.map((line) => {
      const item = document.createElement("li");
      item.textContent = line;
      return item;
    }),
  );
  drawChoices();
}

function drawChoices() {
  const answered = current.state?.value;
  const parts = current.choices.map((choice) => {
    const part = document.createElement("span");
    part.className = choice.value === answered ? "choice chosen" : "choice";
    const key = document.createElement("kbd");
    key.textContent = choice.key.toUpperCase();
    part.append(key, ` ${choice.label}`);
    return part;
  });
  const state = document.createElement("span");
  state.className = "muted";
  state.textContent = current.state ? `answered: ${current.state.value}, ${source(current.state)}` : "not answered yet";
  element("choices").replaceChildren(...parts, state);
}

function drawLegend() {
  const legend = element("legend");
  legend.replaceChildren(
    ...keys.CONTROLS.map(([key, what]) => {
      const part = document.createElement("span");
      const kbd = document.createElement("kbd");
      kbd.textContent = key;
      part.append(kbd, ` ${what}`);
      return part;
    }),
  );
  element("help-keys").replaceChildren(
    ...keys.CONTROLS.map(([key, what]) => {
      const row = document.createElement("tr");
      row.innerHTML = "<th></th><td></td>";
      row.cells[0].textContent = key;
      row.cells[1].textContent = what;
      return row;
    }),
  );
  element("help-files").textContent = `Answers go to ${session.answers}; same and different for pairs also to ${session.verdicts}.`;
}

function frame() {
  lanes.draw();
  const t = player.now();
  const { timeline } = current;
  const reference = timeline.reference !== undefined ? `${current.lanes[timeline.reference].name} ` : "";
  element("clock").textContent = `${player.playing ? "▶" : "❚❚"} ${reference}${clock(t)} / ${clock(timeline.end)}`;
  const piece = player.fileTimes(t)[player.lane]?.piece;
  const modes = [
    `lane ${player.lane + 1} (${current.lanes[player.lane].name})`,
    player.blend ? `with lane ${player.previous + 1}, ${player.lane < player.previous ? "left" : "right"}` : null,
    player.matchingNow() ? `speed matching on${piece ? ` (plays at ${percent(piece.speed)})` : ""}` : "speed matching off (own speed)",
    player.loop ? `loop ${clock(player.loop.start)}–${clock(player.loop.end)}` : null,
  ];
  element("modes").textContent = modes.filter(Boolean).join(" · ");
  requestAnimationFrame(frame);
}

function step(direction) {
  const all = questions();
  open(all[(all.indexOf(current) + direction + all.length) % all.length]);
}

// The next question without an answer after the current one, if any.
function nextOpen() {
  const all = questions();
  const index = all.indexOf(current);
  return [...all.slice(index + 1), ...all.slice(0, index)].find((question) => !question.state) ?? null;
}

function applyStates(reply) {
  for (const question of questions()) question.state = reply.states[question.id] ?? null;
}

async function choose(key) {
  const choice = current.choices.find((candidate) => candidate.key === key);
  if (!choice) {
    say(`${key.toUpperCase()} is not an answer here; ? lists the keys`);
    return;
  }
  try {
    const reply = await api.answer(current.id, key);
    applyStates(reply);
    say(`${current.title}: ${reply.message}`, "done");
    const next = nextOpen();
    if (next) open(next);
    else {
      queue.render(session, current);
      drawChoices();
      say(`${reply.message}. Every question has an answer.`, "done");
    }
  } catch (error) {
    say(error.message, "error");
  }
}

async function undo() {
  try {
    const reply = await api.undo();
    applyStates(reply);
    say(reply.message, reply.message.includes("left there") ? "error" : "done");
    const undone = questions().find((question) => question.id === reply.question);
    if (undone) open(undone);
    else {
      queue.render(session, current);
      drawChoices();
    }
  } catch (error) {
    say(error.message, "error");
  }
}

function onKey(event) {
  const help = element("help");
  const wanted = keys.action(event);
  if (!wanted) return;
  event.preventDefault();
  context.resume();
  if (!help.hidden && wanted.name !== "help" && wanted.name !== "close") help.hidden = true;
  switch (wanted.name) {
    case "toggle":
      return player.toggle();
    case "lane":
      if (wanted.lane < current.lanes.length) player.switchTo(wanted.lane);
      else say(`this question has ${current.lanes.length} lanes`);
      return;
    case "back":
      return player.switchBack();
    case "seek":
      return player.seek(player.now() + wanted.by);
    case "marker": {
      const marker = wanted.direction > 0 ? player.nextMarker() : player.previousMarker();
      if (marker) {
        player.jumpTo(marker);
        say(`${clock(marker.at)}: ${marker.label}`);
      } else say(wanted.direction > 0 ? "no marker after this" : "no marker before this");
      return;
    }
    case "loop":
      return player.toggleLoop();
    case "matching":
      return player.toggleMatching();
    case "blend":
      if (!player.toggleBlend()) say("one lane only: nothing to play beside it");
      return;
    case "question":
      return step(wanted.direction);
    case "undo":
      return undo();
    case "help":
      help.hidden = !help.hidden;
      return;
    case "close":
      help.hidden = true;
      return;
    case "choice":
      return choose(wanted.key);
  }
}

window.audition = {
  snapshot: () => ({
    question: current?.id,
    kind: current?.kind,
    ...player.snapshot(),
    queue: questions().map((question) => ({ id: question.id, title: question.title, answer: question.state?.value ?? null })),
    message: element("message").textContent,
  }),
  open: (id) => open(questions().find((question) => question.id === id)),
  // The player itself, for checks that need its clock and sources.
  player: () => player,
};

start();
