import { Editor, EditorState } from "draft-js";
import "draft-js/dist/Draft.css";
import { useState } from "react";

type Props = {
  onChange: (markdown: string) => void;  
};

function MarkdownEditor({onChange}: Props) {
  const [editorState, setEditorState] = useState(
    () => EditorState.createEmpty(),
  );

  function handleChange(editorState: EditorState) {
    setEditorState(editorState);
    onChange(editorState.getCurrentContent().getPlainText());
  }

  return <Editor editorState={editorState} onChange={handleChange} />;
}

export default MarkdownEditor;
