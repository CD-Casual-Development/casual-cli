import { cli } from "../bun-helpers";
import fs from 'node:fs';

export async function GET(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response | undefined;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing quote id');
        return res;
    }

    cli('project', 'make-invoice', [['-q', pathId]], 'normal');

    fs.watch(process.env.CCLI_OUTPUT_DIR || '../public/pdfs', (eventType, filename) => {
        if (eventType === 'rename' && (filename?.includes('invoice') || filename?.includes('factuur'))) {
            res = new Response('Done', {
                headers: {
                    'HX-Redirect': `/pdfs/${filename}`
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

    return res;
}