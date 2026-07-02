"use client";

import { useEffect } from "react";

function setFaqState(item: HTMLElement, open: boolean) {
  const indicator = item.querySelector<HTMLElement>(".faq-indicator");
  item.dataset.open = open ? "true" : "false";

  if (!indicator) {
    return;
  }

  indicator.textContent = open ? "−" : "+";
  indicator.style.background = open ? "#84a36a" : "#eef3ea";
  indicator.style.color = open ? "#ffffff" : "#5d7550";
}

function formatCount(value: number, format?: string) {
  if (format === "comma") {
    return Math.round(value).toLocaleString("en-US");
  }

  if (value % 1 !== 0) {
    return value.toFixed(1);
  }

  return Math.round(value).toString();
}

export function SiteInteractions() {
  useEffect(() => {
    const faqQuestions = [...document.querySelectorAll<HTMLButtonElement>("[data-faq-question]")];
    const counters = [...document.querySelectorAll<HTMLElement>("[data-countup]")];
    const animations = counters.map((counter) => {
      const target = Number(counter.dataset.countup);
      const format = counter.dataset.fmt;
      const startedAt = performance.now();
      const duration = 900;
      let frame = 0;

      const tick = (time: number) => {
        const progress = Math.min((time - startedAt) / duration, 1);
        const eased = 1 - Math.pow(1 - progress, 3);
        counter.textContent = formatCount(target * eased, format);

        if (progress < 1) {
          frame = requestAnimationFrame(tick);
        }
      };

      frame = requestAnimationFrame(tick);

      return () => {
        cancelAnimationFrame(frame);
      };
    });

    const faqHandlers = faqQuestions.map((question) => {
      const handler = () => {
        const item = question.closest<HTMLElement>(".faq-item");

        if (!item) {
          return;
        }

        const shouldOpen = item.dataset.open !== "true";
        const group = item.parentElement?.querySelectorAll<HTMLElement>(".faq-item") ?? [];
        group.forEach((currentItem) => setFaqState(currentItem, currentItem === item ? shouldOpen : false));
      };

      question.addEventListener("click", handler);

      return () => {
        question.removeEventListener("click", handler);
      };
    });

    return () => {
      faqHandlers.forEach((removeHandler) => removeHandler());
      animations.forEach((removeAnimation) => removeAnimation());
    };
  }, []);

  return null;
}
