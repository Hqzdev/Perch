const STEPS = [
  {
    title: "Add your site URL",
    body: "Enter your domain. Perch discovers reachable pages, respects crawl limits, and builds a source map.",
    color: "#4b5563"
  },
  {
    title: "Perch crawls and indexes",
    body: "Pages are cleaned, chunked, embedded, and stored per tenant for fast retrieval.",
    color: "#2e90fa"
  },
  {
    title: "Install one script tag",
    body: "Drop the widget into your site and let visitors ask questions with cited answers.",
    color: "#fdb022"
  }
];

export function HowItWorksSection() {
  return (
    <section className="section dark" id="how">
      <div className="page-wrap">
        <p className="section-kicker" style={{ color: "#fdb022" }}>
          How it works
        </p>
        <h2 className="section-title">Point Perch at your site. Paste one tag. Done.</h2>
        <div className="steps-grid">
          {STEPS.map((step, index) => (
            <article className="panel" key={step.title}>
              <div className="step-number" style={{ background: step.color, color: index === 2 ? "#18181b" : "#ffffff" }}>
                {index + 1}
              </div>
              <h3>{step.title}</h3>
              <p>{step.body}</p>
            </article>
          ))}
        </div>
        <div className="code-box">
          <div className="browser-top" style={{ borderColor: "rgba(255,255,255,.08)", background: "#0c1712" }}>
            <span className="window-dot red" />
            <span className="window-dot yellow" />
            <span className="window-dot slate" />
            <span className="address" style={{ background: "#14241d", borderColor: "rgba(255,255,255,.1)", color: "#8fa69c" }}>
              index.html
            </span>
          </div>
          <pre>{`<script src="https://cdn.perch.ai/widget.js" data-perch-key="pk_live_8f2c...a91"></script>`}</pre>
        </div>
      </div>
    </section>
  );
}
