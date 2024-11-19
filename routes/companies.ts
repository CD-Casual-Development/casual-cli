import { cli, overview, parseBody, title, updateForm, type ToPage } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;
    if (!pathId || Number.isNaN(pathId)) {
        res = new Response('Missing company id');
        return res;
    }

    const company = await cli('account', 'get-company', pathId, 'json');
    if (company && typeof company === 'object' && !Array.isArray(company)) {
        const accounts = await cli('account', 'ls', undefined, 'json');
        const companyAccounts = await cli('account', 'ls', [['-c', pathId]]);
        let account_id: [string | number, string][] = [];
        if (accounts && Array.isArray(accounts)) {
            account_id = accounts.map((account) => [account.id, account.name]);
        }
        const addresses = await cli('account', 'list-addresses', undefined, 'json');
        let address_id: [string | number, string][] = [];
        if (addresses && Array.isArray(addresses)) {
            address_id = addresses.map((address) => [address.id, `${address.city}, ${address.street} ${address.number}${address.unit}`]);
        }



        res = page(
            title(company.name),
            updateForm('update-company', `/companies/${company.id}`, company, { account_id, address_id }, true),
            company.logo ? `<img src="${company.logo}" alt="logo" />` : '',
            '<br/>',
            overview('accounts', typeof companyAccounts === 'string' ? companyAccounts : 'No accounts found', 2, 'account-view')
        );

    } else {
        console.warn('No task found', { company });
        res = new Response(`Not found, received ${company}`);
    }
    return res;
}

export async function POST(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    if (!req.body) {
        return new Response('No body found');
    }

    let res: Response;

    const fields = await parseBody(req.body);

    if (!fields.has('name')) {
        res = new Response('Missing name');
        return res;
    }

    const id = await cli('account', 'add-company', [
        ['-n', fields.get('name')],
        ['-l', fields.get('logo')],
        ['-c', fields.get('commerce_number')],
        ['-v', fields.get('vat_number')],
        ['-i', fields.get('iban')],
        ['-p', fields.get('phone')],
        ['-e', fields.get('email')],
        ['-a', fields.get('account_id')],
        ['--address-id', fields.get('address_id')],
        ['--country', fields.get('country')],
        ['--city', fields.get('city')],
        ['-s', fields.get('street')],
        ['--number', fields.get('number')],
        ['--unit', fields.get('unit')],
        ['--postalcode', fields.get('postalcode')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/companies/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to new company</button>`);
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

    const id = await cli('account', 'update-company', [
        pathId,
        ['-n', fields.get('name')],
        ['-l', fields.get('logo')],
        ['-c', fields.get('commerce_number')],
        ['-v', fields.get('vat_number')],
        ['-i', fields.get('iban')],
        ['-p', fields.get('phone')],
        ['-e', fields.get('email')],
        ['-a', fields.get('account_id')],
        ['--address-id', fields.get('address_id')]
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/companies/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to updated company</button>`);
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

    const id = await cli('account', 'remove-company', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }

    return res;
}