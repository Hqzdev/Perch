const USE_CASES = [
  ["SaaS docs", "Answer implementation, billing, API, and security questions without forcing visitors to search docs."],
  ["E-commerce", "Handle product details, shipping policies, sizing, and returns from existing pages."],
  ["Agencies", "Deploy a cited assistant across multiple client websites from one repeatable workflow."],
  ["Courses", "Answer questions from syllabus, lessons, pricing, and enrollment pages."],
  ["Product sites", "Turn marketing pages into interactive answers during evaluation."]
];

export function UseCasesSection() {
  return (
    <section className="section" id="use-cases">
      <div className="page-wrap">
        <p className="section-kicker">Use cases</p>
        <h2 className="section-title">Useful anywhere the website already holds the answer.</h2>
        <div className="use-grid">
          {USE_CASES.map(([title, body]) => (
            <article className="panel use-card" key={title}>
              <h3>{title}</h3>
              <p>{body}</p>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
