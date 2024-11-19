import { cli, overview, form, type ToPage } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;

    const schedule = await cli('schedule', 'ls');
    res = page(
        overview('schedule', typeof schedule === 'string' ? schedule : 'No scheduled items'),
        form('add-schedule', '/schedule', ['date'])
    );
    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('schedule', 'remove', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }
    return res;
}