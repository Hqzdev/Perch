import Link from "next/link";

export function SiteHeader() {
  return (
    <header className="nav">
      <div className="page-wrap nav-inner">
        <Link className="brand" href="#top">
          <span className="brand-mark" aria-hidden="true">
            <span />
          </span>
          Perch
        </Link>
        <nav className="nav-links" aria-label="Primary navigation">
          <Link href="#demo">Demo</Link>
          <Link href="#how">How it works</Link>
          <Link href="#features">Features</Link>
          <Link href="#dashboard">Dashboard</Link>
          <Link href="#pricing">Pricing</Link>
        </nav>
        <div className="nav-actions">
          <Link className="mobile-hidden" href="#dashboard">
            Sign in
          </Link>
          <Link className="btn btn-dark" href="#how">
            Start indexing
          </Link>
        </div>
      </div>
    </header>
  );
}
