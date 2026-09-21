for (const code of document.querySelectorAll('code.language-repl')) {
    const source = code.textContent.trim();
    const url = `https://nassau.takashiidobe.com/#code=${encodeURIComponent(source)}`;
    const snippet = code.closest('pre');
    const example = document.createElement('div');
    example.className = 'repl-example';
    code.className = 'language-sml';
    snippet.replaceWith(example);
    example.append(snippet);

    const details = document.createElement('details');
    details.className = 'repl-embed';
    const summary = document.createElement('summary');
    summary.textContent = 'Run this example in Nassau';
    details.append(summary);

    const frame = document.createElement('iframe');
    frame.src = url;
    frame.title = 'Runnable Standard ML example in the Nassau REPL';
    frame.loading = 'lazy';
    frame.referrerPolicy = 'no-referrer';
    details.append(frame);

    const link = document.createElement('a');
    link.href = url;
    link.target = '_blank';
    link.rel = 'noopener noreferrer';
    link.textContent = 'Open this example in the Nassau REPL';
    details.append(link);
    example.append(details);
}
