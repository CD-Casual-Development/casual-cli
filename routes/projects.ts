import { cli, form, overview, parseBody, title, updateForm, type ToPage } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;
    if (pathId && !Number.isNaN(pathId)) {
        const project = await cli('project', 'get', pathId, 'json');


        if (project && typeof project === 'object' && !Array.isArray(project)) {
            const tasks = await cli('project', 'list-tasks', pathId);
            const quotes = await cli('project', 'list-quotes', [['-p', pathId]]);
            const invoices = await cli('project', 'list-invoices', [['-p', pathId]]);
            const accounts = await cli('account', 'ls', undefined, 'json');
            let client_id: [string | number, string][] = [];
            if (accounts && Array.isArray(accounts)) {
                client_id = accounts.map((account) => [account.id, account.name]);
            }

            res = page(
                title(project.title),
                `<button hx-get="/make-quote/${pathId}" hx-swap="outerHTML" hx-target="this" class="quote-button outline" title="Make quote">📝</button>
                <br/>
                <br/>`,
                updateForm('update-project', `/projects/${pathId}`, project, { client_id }, true),
                '<br/>',
                overview('tasks', typeof tasks === 'string' ? tasks : 'No tasks found', 2, 'task-view'),
                form('add-task', '/tasks/' + pathId, ['title', 'description', 'minutes_estimated', 'minutes_spent', 'minutes_remaining', 'minutes_billed', 'minute_rate'], undefined, true),
                '<br/>',
                overview('quotes', typeof quotes === 'string' ? quotes.replaceAll('/usr/src/app/public', '') : 'No quotes found', 2, 'quote-view'),
                '<br/>',
                overview('invoices', typeof invoices === 'string' ? invoices.replaceAll('/usr/src/app/public', '') : 'No invoices found', 2, 'invoice-view')
            );
            return res;
        } else {
            console.error('project id not found', pathId);
        }
    }

    const projects = await cli('project', 'ls');
    const clients = await cli('account', 'ls', undefined, 'json');
    let client_id: [string | number, string][] = [];
    if (clients && Array.isArray(clients)) {
        client_id = clients.map((client) => [client.id, client.name]);
    }
    res = page(
        overview('projects', typeof projects === 'string' ? projects : 'No projects found'),
        form('add-project', '/projects', ['title', 'description', 'client_id'], { client_id })
    );
    return res;
}

export async function POST(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    if (!req.body) {
        return new Response('No body found');
    }

    const fields = await parseBody(req.body);

    let res: Response;
    if (!fields.has('title')) {
        res = new Response('Missing title');
        return res;
    }

    const id = await cli('project', 'add', [
        ['-t', fields.get('title')],
        ['-d', fields.get('description')],
        ['-c', fields.get('client_id')]
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/projects/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to new project</button>`);
    } else {
        res = new Response('done');
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

    const id = await cli('project', 'update', [
        pathId,
        ['-t', fields.get('title')],
        ['-d', fields.get('description')],
        ['-c', fields.get('client_id')]
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/projects/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to updated project</button>`);
    } else {
        res = new Response('done');
    }
    return res;
}

export async function DELETE(req: Request, path: string, pathId: number): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing id');
        return res;
    }

    const id = await cli('project', 'remove', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }
    return res;
}