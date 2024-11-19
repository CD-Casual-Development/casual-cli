import { cli, pretty } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId?: number): Promise<Response> {
    let res: Response;


    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing invoice id');
        return res;
    }

    const invoice = await cli('project', 'get-invoice', pathId, 'json');
    if (invoice && typeof invoice === 'object' && !Array.isArray(invoice)) {
        res = new Response(
            `<embed width="100%"
                style="aspect-ratio: 4 / 3; min-height: 300px;"
                src="${invoice.invoice_url.startsWith(process.env.CCLI_OUTPUT_DIR)
                ? invoice.invoice_url.replace(process.env.CCLI_OUTPUT_DIR, '/pdfs')
                : invoice.invoice_url}" type="application/pdf" />
            <br/>
            ${pretty(invoice)}`);
    } else {
        console.warn('No invoice found', { invoice });
        res = new Response(`Not found, received ${invoice}`);
    }

    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('project', 'remove-invoice', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }
    return res;
}