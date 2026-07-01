const PLANS = [
  {
    name: "Basic",
    price: "$299",
    body: "For a single website launch.",
    features: ["One site crawl", "Embeddable widget", "Source citations", "Basic styling"]
  },
  {
    name: "Standard",
    price: "$799",
    body: "For teams that need control and analytics.",
    features: ["Dashboard", "Question analytics", "Scheduled reindex", "Widget customization"],
    featured: true
  },
  {
    name: "Premium",
    price: "$2k+",
    body: "For self-hosted or multi-tenant deployments.",
    features: ["Self-host setup", "Multi-tenant deployment", "CI/CD support", "Custom integration"]
  }
];

export function PricingSection() {
  return (
    <section className="section" id="pricing">
      <div className="page-wrap">
        <p className="section-kicker">Pricing</p>
        <h2 className="section-title">Package the outcome, not the model calls.</h2>
        <div className="pricing-grid">
          {PLANS.map((plan) => (
            <article className={plan.featured ? "panel pricing-card featured" : "panel pricing-card"} key={plan.name}>
              <div>
                <h3>{plan.name}</h3>
                <p>{plan.body}</p>
              </div>
              <div className="price">{plan.price}</div>
              <ul>
                {plan.features.map((feature) => (
                  <li key={feature}>{feature}</li>
                ))}
              </ul>
              <a className={plan.featured ? "btn btn-primary" : "btn btn-light"} href="#how">
                Start with {plan.name}
              </a>
            </article>
          ))}
        </div>
      </div>
    </section>
  );
}
