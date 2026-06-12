import ReactDOMServer from 'react-dom/server';
import React from 'react';
import App from './App';

export function render(url: string, prefetchedState: any) {
  // Irrigates state into global variables inside Node/Nitro context
  if (typeof global !== 'undefined') {
    (global as any).__VYZORIX_PREFETCHED_STATE__ = prefetchedState;
  }

  // Generate plain static markup
  const html = ReactDOMServer.renderToString(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );

  return { html };
}
