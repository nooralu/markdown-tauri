import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import _ from "lodash";
import MarkdownEditor from "./components/MarkdwonEditor";
import MarkdwonView from "./components/MarkdwonView";

function App() {
  const [viewContent, setViewContent] = useState("");

  const handleChange = _.debounce((content: string) => {
    invoke<string>("parse_md_str", { content }).then((content) => {
      setViewContent(content);
    });
  }, 100);

  return (
    <div className="flex flex-row p-[1rem] h-screen overflow-y-hidden">
      <div className="view w-1/2 h-full overflow-y-auto bg-gray-100">
        <MarkdownEditor onChange={handleChange} />
      </div>
      <div className="view w-1/2 h-full overflow-y-auto">
        <MarkdwonView content={viewContent} />
      </div>
    </div>
  );
}

export default App;
