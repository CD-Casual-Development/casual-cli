import { cli, waitTillFileCreatedAndRedirect } from "../bun-helpers";
import fs from 'node:fs';

export async function GET(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response | undefined;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing quote id');
        return res;
    }

    cli('project', 'make-invoice', [['-q', pathId]], 'normal');

    res = await waitTillFileCreatedAndRedirect(['invoice', 'factuur']);

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