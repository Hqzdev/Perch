const TEAMS = ["Northwind", "Ledgerly", "Foldbox", "Cadence", "Postmark"];

export function TrustStrip() {
  return (
    <section className="trust-strip">
      <div className="page-wrap trust-inner">
        <strong>Grounding answers at</strong>
        {TEAMS.map((team) => (
          <span key={team}>{team}</span>
        ))}
      </div>
    </section>
  );
}
