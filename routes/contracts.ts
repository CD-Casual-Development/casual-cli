import { type ToPage, cli, overview, form, updateForm, title, parseBody } from "../bun-helpers";


function decodeDateField(field: string, fields: Map<string, string>): string | undefined {
    if (fields.has(`${field}_date`) && fields.get(`${field}_date`)) {
        if (fields.get(`${field}_date`) === 'null') {
            fields.delete(`${field}_date`);
            return undefined;
        }

        let date = decodeURIComponent(fields.get(`${field}_date`)!);
        console.log({ date }, date.split(':').length);
        if (date.split(':').length === 2) {
            fields.set(`${field}_date`, date + ':00');
        } else if (date.split(':').length === 1) {
            fields.set(`${field}_date`, date + ':00:00');
        } else {
            fields.set(`${field}_date`, date);
        }
    }
    return fields.get(`${field}_date`);
}

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response | undefined> {
    let res: Response;

    if (!pathId && Number.isNaN(pathId)) {
        res = new Response('Missing contract id');
        return res;
    }
    const contract = await cli('account', 'get-contract', pathId, 'json');

    if (contract && typeof contract === 'object' && !Array.isArray(contract)) {
        if (!contract.recipient_id) {
            return new Response(`Client not found id:${contract.recipient_id}`);
        }
        const client = await cli('account', 'get', contract.recipient_id, 'json');
        if (!client || typeof client !== 'object' || Array.isArray(client)) {
            return new Response('Client not found');
        }
        res = page(
            title(contract.contract_type + 'Contract'),
            updateForm('update-contract', `/contracts/${pathId}`, contract, { recipient_id: [contract.recipient_id, client.name] }, true)
        );
        return res;
    }

    const contracts = await cli('account', 'list-contracts');
    res = page(
        overview('contracts', typeof contracts === 'string' ? contracts : 'No contracts found'),
        form('add-contract', '/contracts', ['title', 'description', 'client_id'])
    );
    return res;
}

export async function POST(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    if (!req.body) {
        return new Response('No body found');
    }

    const fields = await parseBody(req.body);

    let res: Response;
    if (!fields.has('sender_id')) {
        res = new Response('Missing sender_id');
        return res;
    }

    if (!fields.has('recipient_id')) {
        res = new Response('Missing recipient_id');
        return res;
    }

    /*
    
        #[arg(short, long)]
        pub sender_id: i64,
        #[arg(short, long)]
        pub recipient_id: i64,
        #[arg(short, long)]
        pub contract_type: Option<String>,
        #[arg(short, long)]
        pub invoice_period_months: Option<i64>,
        #[arg(short, long)]
        pub monthly_rate: Option<i64>,
        #[arg(long)]
        pub contract_url: Option<String>,
    */
    const id = await cli('account', 'add-contract', [
        ['-s', fields.get('sender_id')],
        ['-r', fields.get('recipient_id')],
        ['-c', fields.get('contract_type')],
        ['-i', fields.get('invoice_period_months')],
        ['--monthly-rate', fields.get('monthly_rate')],
        ['--contract-url', fields.get('contract_url')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/contracts/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to new contract</button>`);
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

    const getDateField = (field: string) => decodeDateField(field, fields);

    const id = await cli('account', 'update-contract', [
        pathId,
        ['-s', fields.get('sender_id')],
        ['-r', fields.get('recipient_id')],
        ['-c', fields.get('contract_type')],
        ['-i', fields.get('invoice_period_months')],
        ['--start-date', getDateField('start')],
        ['--end-date', getDateField('end')],
        ['--auto-renew', fields.get('auto_renew')],
        ['--cancel-date', getDateField('cancel')],
        ['--send-date', getDateField('send')],
        ['--monthly-rate', fields.get('monthly_rate')],
        ['--contract-url', fields.get('contract_url')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/contracts/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to updated contract</button>`);
    } else {
        res = new Response('done');
    }
    return res;
}