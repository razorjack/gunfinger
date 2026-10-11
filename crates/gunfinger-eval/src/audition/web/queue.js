// The queue on the left: every loaded question under its file's title, the
// open ones marked, the answered ones with their answer. A click opens one.

export class Queue {
  constructor(root, open) {
    this.root = root;
    this.open = open;
  }

  render(session, current) {
    this.root.replaceChildren();
    for (const file of session.files) {
      const heading = document.createElement("h2");
      heading.textContent = file.title;
      heading.title = file.path;
      this.root.append(heading);
      const list = document.createElement("ol");
      for (const question of file.questions) {
        const item = document.createElement("li");
        item.className = question.state ? "answered" : "open";
        item.classList.toggle("current", question.id === current?.id);
        const answer = document.createElement("span");
        answer.className = "answer";
        answer.textContent = question.state ? question.state.value : "·";
        answer.title = question.state ? source(question.state) : "not answered";
        const title = document.createElement("span");
        title.textContent = question.title;
        item.append(answer, title);
        item.addEventListener("click", () => this.open(question));
        list.append(item);
      }
      this.root.append(list);
    }
    this.root.querySelector(".current")?.scrollIntoView({ block: "nearest" });
  }
}

export function source(state) {
  if (state.source.from === "log") return `answered here (answer log, record ${state.source.seq})`;
  const [a, b] = state.source.pair;
  return state.source.related
    ? `from the verdicts file, on a related pair: ${a} ~ ${b}`
    : `from the verdicts file: ${a} ~ ${b}`;
}
