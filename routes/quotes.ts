import { cli, pretty } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId?: number): Promise<Response> {
    let res: Response;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing quote id');
        return res;
    }

    const quote = await cli('project', 'get-quote', pathId, 'json');
    if (quote && typeof quote === 'object' && !Array.isArray(quote)) {
        res = new Response(
            `<embed width="100%"
                style="aspect-ratio: 4 / 3; min-height: 300px;"
                src="${quote.quote_url.startsWith(process.env.CCLI_OUTPUT_DIR)
                ? quote.quote_url.replace(process.env.CCLI_OUTPUT_DIR, '/pdfs')
                : quote.quote_url}" type="application/pdf" />
            <br/>
            ${pretty(quote)}`);
    } else {
        console.warn('No quote found', { quote });
        res = new Response(`Not found, received ${quote}`);
    }


    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('project', 'remove-quote', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }
    return res;
}