import "github-markdown-css/github-markdown-light.css";

function makeHtml(markdown: string) {
  return { __html: markdown };
}

type MarkdownViewProps = {
  content: string;
};

function MarkdwonView({ content }: MarkdownViewProps) {
  return (
    <article
      className="markdown-body"
      dangerouslySetInnerHTML={makeHtml(content)}
    />
  );
}

export default MarkdwonView;
