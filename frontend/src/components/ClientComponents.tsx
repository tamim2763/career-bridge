"use client"

import dynamic from "next/dynamic"

// Lazy load heavy components that don't need to block initial render
// These are client-only components
const VisualEditsMessenger = dynamic(() => import("../visual-edits/VisualEditsMessenger"), {
  ssr: false,
});

const ErrorReporter = dynamic(() => import("@/components/ErrorReporter"), {
  ssr: false,
});

export function ClientComponents() {
  return (
    <>
      <ErrorReporter />
      <VisualEditsMessenger />
    </>
  );
}

