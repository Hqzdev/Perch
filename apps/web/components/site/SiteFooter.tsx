import Link from "next/link";

export function SiteFooter() {
  return (
    <footer className="footer">
      <div className="page-wrap footer-inner">
        <Link className="brand" href="#top">
          <span className="brand-mark" aria-hidden="true">
            <span />
          </span>
          Perch
        </Link>
        <span>Source-cited answers for every website visitor.</span>
      </div>
    </footer>
  );
}
