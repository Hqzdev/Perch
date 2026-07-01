import { DashboardPreviewSection } from "@/components/landing/DashboardPreviewSection";
import { FaqSection } from "@/components/landing/FaqSection";
import { FeaturesSection } from "@/components/landing/FeaturesSection";
import { FinalCtaSection } from "@/components/landing/FinalCtaSection";
import { HeroSection } from "@/components/landing/HeroSection";
import { HowItWorksSection } from "@/components/landing/HowItWorksSection";
import { LiveDemoSection } from "@/components/landing/LiveDemoSection";
import { PricingSection } from "@/components/landing/PricingSection";
import { TechnicalTrustSection } from "@/components/landing/TechnicalTrustSection";
import { TrustStrip } from "@/components/landing/TrustStrip";
import { UseCasesSection } from "@/components/landing/UseCasesSection";
import { SiteFooter } from "@/components/site/SiteFooter";
import { SiteHeader } from "@/components/site/SiteHeader";

export default function Home() {
  return (
    <>
      <SiteHeader />
      <main>
        <HeroSection />
        <TrustStrip />
        <LiveDemoSection />
        <HowItWorksSection />
        <FeaturesSection />
        <UseCasesSection />
        <TechnicalTrustSection />
        <DashboardPreviewSection />
        <PricingSection />
        <FaqSection />
        <FinalCtaSection />
      </main>
      <SiteFooter />
    </>
  );
}
