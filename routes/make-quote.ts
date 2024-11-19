import { cli } from "../bun-helpers";
import fs from 'node:fs';

export async function GET(req: Request, path: string, pathId?: number): Promise<Response> {
  let res: Response | undefined;

  if (!pathId || Number.isNaN(pathId)) {
    res = new Response('Missing task id');
    return res;
  }

  // This is not working so making workaround
  // let quote_url = await cli('project', 'make-quote', [['-p', pathId]], 'json');
  cli('project', 'make-quote', [['-p', pathId]], 'normal');



  fs.watch(process.env.CCLI_OUTPUT_DIR || '../public/pdfs', (eventType, filename) => {
    if (eventType === 'rename' && (filename?.includes('quote') || filename?.includes('offerte'))) {
      res = new Response('Done', {
        headers: {
          'HX-Redirect': `/public/pdfs/${filename}`
        }
      });
    }
  });

  let retries = 0;
  while (!res && retries < 10) {
    await new Promise(r => setTimeout(r, 1000));
    retries++;
  }

  if (res) {
    return res;
  }

  res = new Response(`<button hx-get="/projects/${pathId}" hx-swap="innerHTML transition:true" hx-target="#main">Processing...<br/>click to refresh</button>`, {
    headers: {
      'HX-Location': `{ "path": "/projects/${pathId}", "target": "#main", "swap": "innerHTML transition:true" }`,
    }
  });
  /*
  if (Array.isArray(quote_url)) {
    quote_url = quote_url[0] as string;
  } else if (typeof quote_url === 'object') {
    quote_url = quote_url.data as string;
  }

  console.debug({ quote_url });
  if (quote_url && typeof quote_url === 'string') {
    if (quote_url.startsWith('"')) {
      quote_url = quote_url.trimEnd().slice(1);
    }
    if (quote_url.endsWith('"')) {
      quote_url = quote_url.slice(0, quote_url.length - 1);
    }
    if (quote_url.includes('/public/')) {
      quote_url = `/${quote_url.split('/public/')[1]}`
    }

    res = new Response('Done', {
      headers: {
        'HX-Redirect': quote_url
      }
    });
    //res = new Response(`<a href="${quote_url}">Download quote</a>`);
  } else {
    console.log('No quote url from cli');
    res = new Response(`No url found for the quote, found ${quote_url}`);
  }*/

  return res;
}