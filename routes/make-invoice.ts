import { cli } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing quote id');
        return res;
    }

    cli('project', 'make-invoice', [['-q', pathId]], 'normal');
    res = new Response(`<button hx-get="/projects/${pathId}" hx-swap="innerHTML transition:true" hx-target="#main">Processing...<br/>click to refresh</button>`);

    return res;
}