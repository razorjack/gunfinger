// The server's routes (audition::server). Every URL is relative to the page,
// whose address carries the token, so each request carries it too.

async function checked(response) {
  if (!response.ok) {
    throw new Error((await response.text()) || `${response.status} ${response.statusText}`);
  }
  return response;
}

function post(route, body) {
  return fetch(route, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body ?? {}),
  }).then(checked);
}

export async function session() {
  return (await checked(await fetch("api/session"))).json();
}

export async function envelope(file) {
  return (await checked(await fetch(`api/envelope?file=${encodeURIComponent(file)}`))).json();
}

// Window k of a file as WAV bytes; null past the file's end.
export async function audioWindow(file, k) {
  const response = await fetch(`api/audio?file=${encodeURIComponent(file)}&window=${k}`);
  if (response.status === 404) return null;
  return (await checked(response)).arrayBuffer();
}

export async function answer(question, key) {
  return (await post("api/answer", { question, key })).json();
}

export async function undo() {
  return (await post("api/undo")).json();
}

export async function reveal(file) {
  await post("api/reveal", { file });
}
