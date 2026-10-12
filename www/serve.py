#!/usr/bin/env python3
import functools
import http.server
import sys
from pathlib import Path

root = Path(__file__).parent / "dist"
port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000


class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()


handler = functools.partial(Handler, directory=str(root))
http.server.ThreadingHTTPServer(("localhost", port), handler).serve_forever()
