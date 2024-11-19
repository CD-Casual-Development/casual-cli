
import fs from 'node:fs';
import { cli, form, htmlHead, htmlTail, isProd, overview, pretty, title, updateForm, type Route, type ToPage } from './bun-helpers';

const routes = fs.readdirSync('./routes').map((file) => {
  const path = file.split('.')[0];
  const route = require(`./routes/${file}`);
  return { path, route };
}).reduce((acc, { path, route }) => {
  if (path === 'home') {
    acc['/'] = route;
  } else {
    acc[path] = route;
  }
  return acc;
}, {} as Record<string, Record<string, Route>>);

if (!isProd) {
  console.debug({ routes });
}

Bun.serve({
  idleTimeout: 255,

  async fetch(req, _server) {
    try {
      // console.debug(req, server)
      const url = new URL(req.url);
      // const origin = url.origin;
      const pathParts = url.pathname.split('/');
      const path = `/${pathParts[1]}`;
      const pathId = parseInt(pathParts[2], 10);
      // const queryParams = url.searchParams;
      const method = req.method;

      const page: ToPage = (...value: string[]) => req.headers.get('Hx-Request') ? new Response(`${value.join('')}`) : new Response(`${htmlHead}${value.join('')}${htmlTail}`, { headers: { 'Content-Type': 'text/html' } });

      let res: Response;

      if (path === '/') {
        res = await routes['/'][method](req, path, pathId, page);
      } else if (routes && routes[pathParts[1]] && routes[pathParts[1]][method] && typeof routes[pathParts[1]][method] === 'function') {
        res = await routes[pathParts[1]][method](req, path, pathId, page);
      } else {
        if (method === 'GET') {
          let file = Bun.file(`./public${url.pathname}`);
          if (await file.exists()) {
            res = new Response(file);
          } else {
            res = new Response('404', { status: 404 });
          }
        } else {
          res = new Response('404', { status: 404 });
        }
      }
      return res;
    } catch (err) {
      return new Response(`Error 500: ${err}`, { status: 500 });
    }
  }
});

