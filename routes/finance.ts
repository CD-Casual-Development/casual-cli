import { cli, title, type ToPage } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;

    const report = await cli('finance', 'report');
    res = page(
        title('Finance'),
        typeof report === 'string' ? report : 'No report found'
    );
    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('finance', 'remove', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }
    return res;
}