// Simple Node.js dev server with hot reload and error overlay
const http = require('http');
const fs = require('fs');
const path = require('path');
const WebSocket = require('ws');
const chokidar = require('chokidar');

const PORT = 3000;
const BUILD_DIR = path.resolve(__dirname, '../build');

// Serve static files
defaultMime = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.wasm': 'application/wasm',
  '.css': 'text/css',
};

const server = http.createServer((req, res) => {
  let filePath = path.join(BUILD_DIR, req.url === '/' ? '/index.html' : req.url);
  fs.readFile(filePath, (err, data) => {
    if (err) {
      res.writeHead(404);
      res.end('Not found');
      return;
    }
    const ext = path.extname(filePath);
    res.writeHead(200, { 'Content-Type': defaultMime[ext] || 'application/octet-stream' });
    res.end(data);
  });
});

// WebSocket for hot reload and error overlay
const wss = new WebSocket.Server({ server });
let sockets = [];
wss.on('connection', ws => {
  sockets.push(ws);
  ws.on('close', () => {
    sockets = sockets.filter(s => s !== ws);
  });
});

function broadcast(msg) {
  sockets.forEach(ws => {
    if (ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(msg));
  });
}

// Watch for changes in build dir and notify clients
chokidar.watch(BUILD_DIR).on('change', file => {
  broadcast({ type: 'reload' });
});

// Watch for error file
def ERROR_FILE = path.join(BUILD_DIR, 'build-error.txt');
fs.watchFile(ERROR_FILE, () => {
  fs.readFile(ERROR_FILE, 'utf8', (err, data) => {
    if (!err && data) {
      broadcast({ type: 'error', message: data });
    }
  });
});

server.listen(PORT, () => {
  console.log(`Dev server running at http://localhost:${PORT}`);
});
