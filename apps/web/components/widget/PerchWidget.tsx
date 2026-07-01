"use client";

import { FormEvent, useEffect, useMemo, useRef, useState } from "react";

type MessageRole = "assistant" | "visitor";

type Source = {
  label: string;
  path: string;
};

type Message = {
  id: string;
  role: MessageRole;
  text: string;
  sources?: Source[];
};

type IndexedAnswer = {
  match: string[];
  text: string;
  sources: Source[];
};

const indexedAnswers: IndexedAnswer[] = [
  {
    match: ["api", "key", "reset", "roll", "security"],
    text: "Open Settings, go to API Keys, and roll the active key. Perch keeps the previous key valid for 24 hours so your integration can migrate without downtime.",
    sources: [
      { label: "API key rotation", path: "docs/api-keys" },
      { label: "Security policy", path: "docs/security" }
    ]
  },
  {
    match: ["install", "script", "tag", "website", "embed"],
    text: "Install Perch by adding one script tag before the closing body tag. After that, the widget loads asynchronously and starts answering from indexed site content.",
    sources: [
      { label: "Widget install", path: "docs/install" },
      { label: "Async loading", path: "docs/performance" }
    ]
  },
  {
    match: ["pricing", "plan", "cost", "price", "trial"],
    text: "Most teams start with Standard because it includes analytics, widget customization, scheduled reindexing, and up to 10,000 indexed pages.",
    sources: [
      { label: "Pricing", path: "pricing" },
      { label: "Plan limits", path: "docs/limits" }
    ]
  },
  {
    match: ["hallucination", "source", "citation", "answer", "trust"],
    text: "Perch answers only from approved indexed pages and attaches sources to every grounded claim. If it cannot find enough context, it should ask for clarification or route the visitor to your team.",
    sources: [
      { label: "Grounded answers", path: "docs/retrieval" },
      { label: "Fallback rules", path: "docs/handoffs" }
    ]
  }
];

const starterMessages: Message[] = [
  {
    id: "welcome",
    role: "assistant",
    text: "Hi, I am Perch. Ask me about this site and I will answer with sources.",
    sources: [
      { label: "Current page", path: "perch.ai" }
    ]
  }
];

const prompts = [
  "How do I install Perch?",
  "How are answers sourced?",
  "What plan should I start with?"
];

class PerchAnswerService {
  answer(question: string): Message {
    const normalizedQuestion = question.toLowerCase();
    const matchedAnswer = indexedAnswers.find((answer) =>
      answer.match.some((keyword) => normalizedQuestion.includes(keyword))
    );

    const fallbackAnswer = {
      text: "Perch indexed this site and found the strongest match in the product docs. The short version: it crawls your pages, retrieves relevant passages, and gives visitors a cited answer inside the widget.",
      sources: [
        { label: "Product overview", path: "docs/overview" },
        { label: "Retrieval flow", path: "docs/retrieval" }
      ]
    };

    const result = matchedAnswer ?? fallbackAnswer;

    return {
      id: crypto.randomUUID(),
      role: "assistant",
      text: result.text,
      sources: result.sources
    };
  }
}

export function PerchWidget() {
  const answerService = useMemo(() => new PerchAnswerService(), []);
  const [isOpen, setIsOpen] = useState(false);
  const [input, setInput] = useState("");
  const [isThinking, setIsThinking] = useState(false);
  const [messages, setMessages] = useState<Message[]>(starterMessages);
  const messagesRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesRef.current?.scrollTo({
      top: messagesRef.current.scrollHeight,
      behavior: "smooth"
    });
  }, [messages, isThinking]);

  function sendQuestion(question: string) {
    const trimmedQuestion = question.trim();

    if (trimmedQuestion.length === 0 || isThinking) {
      return;
    }

    setInput("");
    setMessages((currentMessages) => [
      ...currentMessages,
      {
        id: crypto.randomUUID(),
        role: "visitor",
        text: trimmedQuestion
      }
    ]);
    setIsThinking(true);

    window.setTimeout(() => {
      setMessages((currentMessages) => [
        ...currentMessages,
        answerService.answer(trimmedQuestion)
      ]);
      setIsThinking(false);
    }, 720);
  }

  function submitQuestion(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    sendQuestion(input);
  }

  return (
    <div className="perch-widget" data-open={isOpen}>
      {isOpen ? (
        <section className="perch-widget-panel" aria-label="Perch assistant">
          <div className="perch-widget-header">
            <span className="perch-widget-mark" aria-hidden="true">
              <svg width="18" height="18" viewBox="0 0 32 32">
                <path d="M23 8c-6 0-9 3.5-10.5 7.5C11 19 9 20.5 7 21.5c3.5 1.5 8 1.5 11-1 2.2-1.8 3-4.2 3.2-6.2.9.8 1.8 2 2.3 3.6.9-3 .5-6.2-.5-9.4z" fill="currentColor" />
                <path d="M6 24h13" stroke="currentColor" strokeWidth="2.1" strokeLinecap="round" />
              </svg>
            </span>
            <div>
              <h2>Ask this site</h2>
              <p>Grounded in indexed pages</p>
            </div>
            <button className="perch-widget-icon-button" type="button" aria-label="Close assistant" onClick={() => setIsOpen(false)}>
              ×
            </button>
          </div>

          <div className="perch-widget-messages" ref={messagesRef}>
            {messages.map((message) => (
              <article className="perch-message" data-role={message.role} key={message.id}>
                <p>{message.text}</p>
                {message.sources ? (
                  <div className="perch-sources" aria-label="Sources">
                    {message.sources.map((source) => (
                      <span key={source.path}>
                        <strong>{source.label}</strong>
                        {source.path}
                      </span>
                    ))}
                  </div>
                ) : null}
              </article>
            ))}
            {isThinking ? (
              <div className="perch-thinking" aria-label="Perch is typing">
                <span />
                <span />
                <span />
              </div>
            ) : null}
          </div>

          <div className="perch-prompts" aria-label="Suggested questions">
            {prompts.map((prompt) => (
              <button type="button" key={prompt} onClick={() => sendQuestion(prompt)}>
                {prompt}
              </button>
            ))}
          </div>

          <form className="perch-widget-form" onSubmit={submitQuestion}>
            <input
              aria-label="Ask Perch"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              placeholder="Ask about pricing, install, security..."
            />
            <button type="submit" disabled={input.trim().length === 0 || isThinking} aria-label="Send question">
              <svg width="17" height="17" viewBox="0 0 24 24" fill="none">
                <path d="M5 12h14M13 6l6 6-6 6" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round" />
              </svg>
            </button>
          </form>
        </section>
      ) : null}

      <button className="perch-widget-launcher" type="button" aria-label="Open assistant" onClick={() => setIsOpen(true)}>
        <span className="perch-widget-mark" aria-hidden="true">
          <svg width="18" height="18" viewBox="0 0 32 32">
            <path d="M23 8c-6 0-9 3.5-10.5 7.5C11 19 9 20.5 7 21.5c3.5 1.5 8 1.5 11-1 2.2-1.8 3-4.2 3.2-6.2.9.8 1.8 2 2.3 3.6.9-3 .5-6.2-.5-9.4z" fill="currentColor" />
            <path d="M6 24h13" stroke="currentColor" strokeWidth="2.1" strokeLinecap="round" />
          </svg>
        </span>
        <span>Ask this site</span>
      </button>
    </div>
  );
}
