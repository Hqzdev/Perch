const FLOW = [
  ["Crawl", "Perch reads allowed website pages and stores page-level source metadata."],
  ["Index", "Clean text is chunked, embedded, and stored with tenant and source boundaries."],
  ["Retrieve", "A visitor question searches only the tenant index and ranks relevant passages."],
  ["Answer", "The model receives retrieved context and returns a streamed answer with citations."]
];

export function TechnicalTrustSection() {
  return (
    <section className="section dark" id="security">
      <div className="page-wrap trust-tech">
        <div>
          <p className="section-kicker" style={{ color: "#fdb022" }}>
            Trust boundary
          </p>
          <h2 className="section-title">The source is visible, so the answer is accountable.</h2>
          <p className="section-copy">
            The strategic point is not that Perch uses AI. The point is that every answer can be traced back to the customer&apos;s own website content.
          </p>
        </div>
        <div className="flow">
          {FLOW.map(([label, body]) => (
            <div className="flow-row" key={label}>
              <strong>{label}</strong>
              <p>{body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
