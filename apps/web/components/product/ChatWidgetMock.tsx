type ChatWidgetMockProps = {
  dark?: boolean;
};

export function ChatWidgetMock({ dark = false }: ChatWidgetMockProps) {
  return (
    <div className={dark ? "chat-widget demo-chat" : "chat-widget"}>
      <div className="chat-widget-header">
        <span className="chat-avatar" aria-hidden="true">
          <span />
        </span>
        <strong>Ask Acme</strong>
        <small>powered by Perch</small>
      </div>
      <div className="chat-body">
        <div className="bubble user">How do I reset my API key without breaking my integration?</div>
        <div className="bubble assistant">
          Roll the key from Settings and API Keys. Your old key keeps working for 24 hours, so you can update your integration first and let the old key expire after migration.
          <div className="sources">
            <span className="source">1 docs/security</span>
            <span className="source">2 docs/api-keys</span>
          </div>
        </div>
      </div>
    </div>
  );
}
