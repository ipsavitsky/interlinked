import { file } from "bun";

const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:3000";

console.log(`Starting frontend server, backend is at ${backendUrl}`);

Bun.serve({
  port: 3001,
  async fetch(req) {
    const url = new URL(req.url);

    let reqPath = url.pathname;
    if (reqPath === "/") {
      reqPath = "/index.html";
    }

    if (reqPath === "/config.js") {
      const configContent = `window.BACKEND_URL = "${backendUrl}";`;
      return new Response(configContent, {
        headers: { "Content-Type": "application/javascript" },
      });
    }

    const filePath = import.meta.dir + reqPath;

    try {
      const data = file(filePath);
      return new Response(data);
    } catch (e) {
      return new Response("Not Found", { status: 404 });
    }
  },
});
