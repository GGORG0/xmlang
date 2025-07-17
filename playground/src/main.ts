import "./style.css";
import "@xterm/xterm/css/xterm.css";

import xmlangUrl from "../../target/wasm32-wasip1/release/xmlang.wasm?url";

import type { Instance } from "@wasmer/sdk";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { runWasix, init, Directory } from "@wasmer/sdk";
import { EditorView, basicSetup } from "codemirror";
import { xml } from "@codemirror/lang-xml";
import { oneDark } from "@codemirror/theme-one-dark";

async function main() {
  await init();

  const term = new Terminal({ cursorBlink: true, convertEol: true });
  const fit = new FitAddon();
  term.loadAddon(fit);
  term.open(document.getElementById("terminal")!);
  fit.fit();
  window.addEventListener("resize", () => {
    fit.fit();
  });

  term.writeln("### Loading... ###");

  const initialProgram = await getCode();

  const dir = new Directory({
    "/playground.xml": initialProgram,
  });

  const saveOnEdit = EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      const code = update.state.doc.toString();
      localStorage.setItem("playgroundCode", code);
    }
  });

  const editor = new EditorView({
    doc: initialProgram,
    extensions: [
      basicSetup,
      xml(),
      EditorView.theme({
        "&": { height: "100%", width: "100%" },
        ".cm-scroller": { overflow: "auto" },
      }),
      EditorView.lineWrapping,
      oneDark,
      saveOnEdit,
    ],
    parent: document.getElementById("editor")!,
  });

  const module = await WebAssembly.compileStreaming(fetch(xmlangUrl));

  async function run() {
    term.clear();

    const code = editor.state.doc.toString();
    localStorage.setItem("playgroundCode", code);
    await dir.writeFile("/playground.xml", code);

    const instance = await runWasix(module, {
      args: ["/app/playground.xml"],
      mount: { "/app": dir },
    });

    let listener = connectStreams(instance, term);

    await instance.wait();
    listener.dispose();
    term.writeln("\n### Program finished. ###");
  }

  document.getElementById("run")?.addEventListener("click", run);

  await run();
}

const encoder = new TextEncoder();

function connectStreams(instance: Instance, term: Terminal) {
  const stdin = instance.stdin?.getWriter();
  const listener = term.onData((data) => {
    if (data === "\r") {
      data += "\n";
    }
    stdin?.write(encoder.encode(data)).catch(console.error);
    term.write(data);
  });

  instance.stdout.pipeTo(
    new WritableStream({ write: (chunk) => term.write(chunk) }),
  );
  instance.stderr.pipeTo(
    new WritableStream({ write: (chunk) => term.write(chunk) }),
  );

  return listener;
}

const defaultCode = `<program>
  <print>Hello, World!</print>
</program>`;

async function getCode(): Promise<string> {
  const params = new URLSearchParams(window.location.search);
  const gistId = params.get("gist");
  const owner = params.get("owner");
  const repo = params.get("repo");
  const branch = params.get("branch");
  const file = params.get("file");

  if (gistId && (owner || repo || branch || file)) {
    alert(
      "Warning: Both Gist and GitHub repo parameters are present. Only one source will be loaded. Repo takes precedence if all repo parameters are set.",
    );
  }

  if ((owner || repo || branch || file) && !(owner && repo && branch && file)) {
    alert(
      "Error: To load from a GitHub repo, you must specify owner, repo, branch, and file parameters.",
    );
    return localStorage.getItem("playgroundCode") || defaultCode;
  }

  async function tryLoad(fetcher: () => Promise<string>, source: string) {
    if (localStorage.getItem("playgroundCode")) {
      if (
        !confirm(
          `Loading code from a ${source} will overwrite your saved code. Continue?`,
        )
      ) {
        return localStorage.getItem("playgroundCode") || "";
      }
    }
    try {
      const code = await fetcher();
      history.replaceState(null, "", "/");
      localStorage.setItem("playgroundCode", code);
      return code;
    } catch (error) {
      console.error(`Failed to fetch from ${source}:`, error);
      alert(`Failed to load code from ${source}.`);
      return localStorage.getItem("playgroundCode") || defaultCode;
    }
  }

  if (owner && repo && branch && file) {
    return tryLoad(() => getFromRepo(owner, repo, branch, file), "GitHub repo");
  } else if (gistId) {
    return tryLoad(() => getGist(gistId), "Gist");
  } else {
    return localStorage.getItem("playgroundCode") || defaultCode;
  }
}

async function getGist(id: string): Promise<string> {
  const response = await fetch(`https://api.github.com/gists/${id}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch gist: ${response.statusText}`);
  }

  const gistData = await response.json();
  const files = gistData.files;
  const firstFileKey = Object.keys(files)[0];
  if (!firstFileKey) {
    throw new Error("Gist contains no files");
  }

  const content = files[firstFileKey].content;
  if (content == null) {
    throw new Error("File content not available");
  }

  return content;
}

async function getFromRepo(
  owner: string,
  repo: string,
  branch: string,
  path: string,
): Promise<string> {
  const url = `https://raw.githubusercontent.com/${owner}/${repo}/${branch}/${path}`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch from repo: ${response.statusText}`);
  }
  return response.text();
}

main();
