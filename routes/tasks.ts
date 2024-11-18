import { cli, parseBody, title, updateForm } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId?: number): Promise<Response> {
    let res: Response;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing task id');
        return res;
    }

    const projects = await cli('project', 'list', undefined, 'json');
    let project_id: [string | number, string][] = [];
    if (projects && Array.isArray(projects)) {
        project_id = projects.map((project) => [project.id, project.title]);
    }

    const task = await cli('project', 'get-task', pathId, 'json');
    if (task && typeof task === 'object' && !Array.isArray(task)) {
        res = new Response(`${title(task.title, 3)}${updateForm('update-task', `/tasks/${task.id}`, task, { project_id }, true)}`);
    } else {
        console.warn('No task found', { task });
        res = new Response(`Not found, received ${task}`);
    }


    return res;
}

export async function POST(req: Request, path: string, pathId: number): Promise<Response> {
    if (!req.body) {
        return new Response('No body found');
    }

    let res: Response;
    const fields = await parseBody(req.body);


    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing project id');
        return res;
    }
    if (!fields.has('title')) {
        res = new Response('Missing title');
        return res;
    }

    const id = await cli('project', 'add-task', [
        ['-p', pathId],
        ['-t', fields.get('title')],
        ['-d', fields.get('description')],
        ['--minutes-estimated', fields.get('minutes_estimated')],
        ['--minutes-spent', fields.get('minutes_spent')],
        ['--minutes-remaining', fields.get('minutes_remaining')],
        ['--minutes-billed', fields.get('minutes_billed')],
        ['--minute-rate', fields.get('minute_rate')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(id);
    } else {
        res = new Response('Done');
    }

    return res;
}

export async function PUT(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;

    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    if (!req.body) {
        res = new Response('Body required for POST');
        return res;
    }
    const fields = await parseBody(req.body);
    const id = await cli('project', 'update-task', [
        pathId,
        ['-t', fields.get('title')],
        ['-d', fields.get('description')],
        ['--minutes-estimated', fields.get('minutes_estimated')],
        ['--minutes-spent', fields.get('minutes_spent')],
        ['--minutes-remaining', fields.get('minutes_remaining')],
        ['--minutes-billed', fields.get('minutes_billed')],
        ['--minute-rate', fields.get('minute_rate')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(id);
    } else {
        res = new Response('Done');
    }

    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('project', 'remove-task', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }

    return res;
}