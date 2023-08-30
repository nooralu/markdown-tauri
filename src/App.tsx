import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import "github-markdown-css/github-markdown-light.css";

function makeHtml(content: string) {
  return { __html: content };
}

function App() {
  const [content, setContent] = useState("");

  async function openFile() {
    const result = await open({
      multiple: false,
      filters: [
        {
          name: "Markdown",
          extensions: ["md"],
        },
      ],
    });
    console.log(result);
    if (result) {
      const path = result as string;
      await invoke<string>("parse_md", { path });
    }
  }

  useEffect(() => {
    listen<string>("md_parsed", (event) => {
      setContent(event.payload);
    });
  }, [content]);

  return (
    <>
      <button onClick={openFile}>Open</button>
      <article
        className="markdown-body"
        dangerouslySetInnerHTML={makeHtml(content)}
      />
    </>
  );
}

export default App;
