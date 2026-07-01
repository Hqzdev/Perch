import { BrowserMockup } from "@/components/product/BrowserMockup";

export function HeroSection() {
  return (
    <section className="hero" id="top">
      <div className="page-wrap hero-grid">
        <div className="hero-copy">
          <div className="status-pill">
            <span className="status-dot" />
            Now indexing 12,400+ pages a day
          </div>
          <h1>
            Add an AI assistant to your website in <span>one line</span> of script.
          </h1>
          <p>
            Perch reads your site, indexes every page, and answers visitor questions with source-cited answers drawn only from your content.
          </p>
          <div className="hero-actions">
            <a className="btn btn-primary" href="#how">
              Start indexing your site
            </a>
            <a className="btn btn-light" href="#demo">
              View demo
            </a>
          </div>
          <div className="hero-proof">
            <span className="check">Live in minutes</span>
            <span className="check">Every answer cited</span>
            <span className="check">Domain allow-listing</span>
          </div>
        </div>
        <BrowserMockup />
      </div>
    </section>
  );
}
