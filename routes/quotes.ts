import { cli, pretty } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId?: number): Promise<Response> {
    let res: Response;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing quote id');
        return res;
    }

    const quote = await cli('project', 'get-quote', pathId, 'json');
    if (quote && typeof quote === 'object') {
        res = new Response(`${pretty(quote)}`);
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