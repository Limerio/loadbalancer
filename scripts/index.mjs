import { createServer } from "node:http";

const ports = [8001, 8002];

for (const port of ports) {
  const server = createServer((req, res) => {
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end(`Hello ${port}!\n`);
  });

  server.listen(port, "127.0.0.1", () => {
    console.log(`Listening on 127.0.0.1:${port}`);
  });
}
