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

export function SiteInteractions() {
  useEffect(() => {
    const faqQuestions = [...document.querySelectorAll<HTMLButtonElement>("[data-faq-question]")];

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
    };
  }, []);

  return null;
}
