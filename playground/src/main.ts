import "./style.css";
import "@xterm/xterm/css/xterm.css";

import xmlangUrl from "../../target/wasm32-wasip1/release/xmlang.wasm?url";

import type { Instance } from "@wasmer/sdk";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { runWasix, init, Directory } from "@wasmer/sdk";
import { EditorView, basicSetup } from "codemirror";
import { xml } from "@codemirror/lang-xml";

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

  const initialProgram = `<program>
  <print>Hello, World!</print>
</program>`;

  const dir = new Directory({
    "/playground.xml": initialProgram,
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
    ],
    parent: document.getElementById("editor")!,
  });

  const module = await WebAssembly.compileStreaming(fetch(xmlangUrl));

  async function run() {
    term.clear();

    const code = editor.state.doc.toString();
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
    new WritableStream({ write: (chunk) => term.write(chunk) })
  );
  instance.stderr.pipeTo(
    new WritableStream({ write: (chunk) => term.write(chunk) })
  );

  return listener;
}

main();
