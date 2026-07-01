const FAQ = [
  ["Does it hallucinate?", "Perch is designed to answer from retrieved site content and show citations. If the site does not contain enough source material, the assistant should say so."],
  ["Can it answer outside my website?", "No. The retrieval path is scoped by tenant and site, so the widget answers from the indexed website boundary."],
  ["How do citations work?", "Each retrieved chunk keeps source metadata. The answer includes links back to the pages used as evidence."],
  ["How long does indexing take?", "Small marketing sites can index in minutes. Larger docs sites depend on crawl limits, page count, and embedding throughput."],
  ["Can I customize the widget?", "Yes. The dashboard preview includes accent color, placement, source links, and basic tone controls."],
  ["Can agencies use this for clients?", "Yes. The strongest agency package is repeatable setup, dashboard access, custom styling, and optional self-hosting."]
];

export function FaqSection() {
  return (
    <section className="section" id="faq">
      <div className="page-wrap">
        <p className="section-kicker">FAQ</p>
        <h2 className="section-title">Straight answers before someone installs it.</h2>
        <div className="faq-grid">
          {FAQ.map(([question, answer]) => (
            <article className="panel faq-card" key={question}>
              <h3>{question}</h3>
              <p>{answer}</p>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
