#!/usr/bin/env python3
"""Splits a single-file Horizon offline build into a small page plus binary blob files.

usage: mirror_play.py SOURCE.html OUTDIR   (writes OUTDIR/horizon-offline.html and OUTDIR/horizon-offline/<name>-<n>.bin)
The page's embedded base64 <script data-blob> parts become files fetched from the same origin, so no
single file exceeds the 100 MB limit of GitHub and Vercel. Nothing is fetched from any other site.
"""
import base64, re, sys
from pathlib import Path

src, out = Path(sys.argv[1]), Path(sys.argv[2])
html = src.read_text(encoding="utf-8")
(out / "horizon-offline").mkdir(parents=True, exist_ok=True)
pat = re.compile(r'<script type="text/plain" data-blob="(\w+)" data-index="(\d+)"([^>]*)>([A-Za-z0-9+/=\s]*)</script>')

def part(m):
    name, idx, rest, body = m.groups()
    (out / "horizon-offline" / f"{name}-{idx}.bin").write_bytes(base64.b64decode(body))
    return f'<script type="text/plain" data-blob="{name}" data-index="{idx}" data-src="horizon-offline/{name}-{idx}.bin"{rest}></script>'

html, n = pat.subn(part, html)
old = """    const packed = new Uint8Array(Number(parts[0].dataset.size));
    let at = 0;
    for (const part of parts) {
      const text = atob(part.textContent);
      for (let i = 0; i < text.length; i++) packed[at + i] = text.charCodeAt(i);
      at += text.length;
      // Free the text: the bytes are kept.
      part.textContent = "";
    }"""
new = """    const chunks = await Promise.all(parts.map(async (part) => {
      if (part.dataset.src) {
        const response = await fetch(part.dataset.src);
        if (!response.ok) throw new Error(`could not load ${part.dataset.src}: ${response.status}`);
        return new Uint8Array(await response.arrayBuffer());
      }
      const text = atob(part.textContent);
      const bytes = new Uint8Array(text.length);
      for (let i = 0; i < text.length; i++) bytes[i] = text.charCodeAt(i);
      part.textContent = "";
      return bytes;
    }));
    const packed = new Uint8Array(chunks.reduce((sum, c) => sum + c.length, 0));
    let at = 0;
    for (const c of chunks) { packed.set(c, at); at += c.length; }"""
assert html.count(old) == 1, "loader not found"
html = html.replace(old, new)
(out / "horizon-offline.html").write_text(html, encoding="utf-8")
print(n, "parts,", len(html), "bytes page")
