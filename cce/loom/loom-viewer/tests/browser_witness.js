// browser_witness.js — E4b Exit-Zeuge: "wasm-Viewer verifiziert R1/N-
// Dateien im Headless-Browser-Test". Startet einen lokalen statischen
// Server fuer web/, laedt die Seite in echtem, headless Chromium
// (Playwright) und ruft window.runViewer(...) fuer (a) das reale R1
// (Valid) und (b) eine deliberat beschaedigte Kopie (N — Reject) auf.
//
// Voraussetzung: `bash build_wasm.sh` wurde zuvor ausgefuehrt (erzeugt
// web/pkg/, nicht versioniert). `npm install playwright` im selben
// Verzeichnis wie diese Datei (package.json liegt daneben).
//
// Aufruf: node tests/browser_witness.js

const fs = require('fs');
const path = require('path');
const http = require('http');
const { chromium } = require('playwright');

const WEB_DIR = path.join(__dirname, '..', 'web');
const R1_PATH = path.join(__dirname, '..', '..', 'golden', 'r1.loom');
const CHROMIUM_PATH = process.env.LOOM_CHROMIUM_PATH || '/opt/pw-browsers/chromium';

function serveStatic(dir) {
  const server = http.createServer((req, res) => {
    let file = decodeURIComponent(req.url.split('?')[0]);
    if (file === '/') file = '/index.html';
    const full = path.join(dir, file);
    if (!full.startsWith(dir)) {
      res.writeHead(403);
      res.end();
      return;
    }
    fs.readFile(full, (err, data) => {
      if (err) {
        res.writeHead(404);
        res.end('not found: ' + file);
        return;
      }
      const ext = path.extname(full);
      const type =
        { '.html': 'text/html', '.js': 'application/javascript', '.wasm': 'application/wasm' }[
          ext
        ] || 'application/octet-stream';
      res.writeHead(200, { 'Content-Type': type });
      res.end(data);
    });
  });
  return new Promise((resolve) => {
    server.listen(0, '127.0.0.1', () => resolve(server));
  });
}

async function main() {
  if (!fs.existsSync(path.join(WEB_DIR, 'pkg', 'loom_viewer.js'))) {
    console.error('web/pkg/ fehlt — zuerst `bash build_wasm.sh` ausfuehren.');
    process.exit(2);
  }

  const r1Bytes = fs.readFileSync(R1_PATH);
  const r1Base64 = r1Bytes.toString('base64');

  // N-Datei: dieselbe reale Struktur, EIN Byte im Payload-Bereich
  // gekippt — bricht den Frame-Digest, L0 muss ablehnen (Reject), nicht
  // nur "kein Treffer" oder ein stiller Leerwert.
  const corrupted = Buffer.from(r1Bytes);
  corrupted[corrupted.length - 5] ^= 0xff;
  const nBase64 = corrupted.toString('base64');

  const server = await serveStatic(WEB_DIR);
  const port = server.address().port;

  const browser = await chromium.launch({
    executablePath: CHROMIUM_PATH,
    args: ['--no-sandbox'],
  });
  try {
    const page = await browser.newPage();
    const consoleErrors = [];
    page.on('pageerror', (e) => consoleErrors.push(String(e)));

    await page.goto(`http://127.0.0.1:${port}/index.html`, { waitUntil: 'load' });
    await page.waitForFunction('window.__loomViewerReady === true', { timeout: 10000 });

    const r1Result = await page.evaluate((b64) => window.runViewer(b64), r1Base64);
    const nResult = await page.evaluate((b64) => window.runViewer(b64), nBase64);

    console.log('--- R1 (echt, gueltig) ---');
    console.log(r1Result);
    console.log('--- N (beschaedigte Kopie) ---');
    console.log(nResult);

    const failures = [];
    if (!r1Result.includes('MANIFEST:')) failures.push('R1: MANIFEST-Ansicht fehlt');
    if (!r1Result.includes('Valid')) failures.push('R1: erwartetes Verdikt Valid fehlt');
    if (!nResult.startsWith('reject:')) failures.push('N: erwartetes reject:-Praefix fehlt');
    if (consoleErrors.length > 0) failures.push('Browser meldete Fehler: ' + consoleErrors.join('; '));

    if (failures.length > 0) {
      console.error('FEHLGESCHLAGEN:\n  - ' + failures.join('\n  - '));
      process.exitCode = 1;
    } else {
      console.log('ERFOLG: wasm-Viewer verifiziert R1 (Valid) und N (Reject) im Headless-Browser.');
    }
  } finally {
    await browser.close();
    server.close();
  }
}

main().catch((e) => {
  console.error('FEHLER:', e);
  process.exit(1);
});
