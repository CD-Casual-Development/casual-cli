import { type ToPage, cli, overview, form, updateForm, title, parseBody } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;

    if (pathId && !Number.isNaN(pathId)) {
        const contract = await cli('account', 'get-contract', pathId, 'json');

        if (contract && typeof contract === 'object' && !Array.isArray(contract)) {
            const client = await cli('account', 'get', contract.client_id, 'json');
            if (!client || typeof client !== 'object' || Array.isArray(client)) {
                return new Response('Client not found');
            }
            res = page(
                title(contract.title),
                updateForm('update-contract', `/contracts/${pathId}`, contract, { client_id: [contract.client_id, client.name] }, true)
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
        ['-t', fields.get('contract_type')],
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