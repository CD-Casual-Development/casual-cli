import { cli, title, updateForm, overview, form, parseBody, type ToPage } from "../bun-helpers";

export async function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    let res: Response;
    if (pathId && !Number.isNaN(pathId)) {
        const account = await cli('account', 'get', pathId, 'json');

        if (account && typeof account === 'object' && !Array.isArray(account)) {
            const companies = await cli('account', 'list-companies', undefined, 'json');
            const addresses = await cli('account', 'list-addresses', undefined, 'json');
            let company_id: [string | number, string][] = [];
            if (companies && Array.isArray(companies)) {
                company_id = companies.map((company) => [company.id, company.name]);
            }
            let address_id: [string | number, string][] = [];
            if (addresses && Array.isArray(addresses)) {
                address_id = addresses.map((address) => [address.id, `${address.city}, ${address.street} ${address.number}${address.unit}`]);
            }


            res = page(`${title(account.name)}
            ${updateForm('update-account', `/accounts/${account.id}`, account, { company_id, address_id }, true)}`);
            return res;
        } else {
            console.error('project id not found', { pathId, account });
        }
    }

    const accounts = await cli('account', 'ls');
    const companies = await cli('account', 'list-companies');
    const addresses = await cli('account', 'list-addresses', undefined, 'json');

    const accountJson = await cli('account', 'ls', undefined, 'json');
    let account_id: [string | number, string][] = [];
    if (accountJson && Array.isArray(accountJson)) {
        account_id = accountJson.map((account) => [account.id, account.name]);
    }

    const companiesJson = await cli('account', 'list-companies', undefined, 'json');
    let company_id: [string | number, string][] = [];
    if (companiesJson && Array.isArray(companiesJson)) {
        company_id = companiesJson.map((company) => [company.id, company.name]);
    }

    let address_id: [string | number, string][] = [];
    if (addresses && Array.isArray(addresses)) {
        address_id = addresses.map((address) => [address.id, `${address.city}, ${address.street} ${address.number}${address.unit}`]);
    }

    return page(`
${overview('accounts', typeof accounts === 'string' ? accounts : 'No accounts found')}
${form("add-account", "/accounts", ['name', 'phone', 'email', 'company_id', 'address_id', 'company_name', 'country', 'city', 'street', 'number', 'unit', 'postalcode', 'privacy_permissions'], { company_id, address_id }, true)}
${overview('companies', typeof companies === 'string' ? companies : 'No companies found', 2)}
${form("add-company", "/companies", ['name', 'logo', 'commerce_number', 'vat_number', 'iban', 'phone', 'email', 'account_id', 'address_id', 'country', 'city', 'street', 'number', 'unit', 'postalcode'], { account_id, address_id }, true)}`);
}

export async function POST(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    if (!req.body) {
        return new Response('No body found');
    }
    let res: Response;

    const fields = await parseBody(req.body);

    if (!fields.has('name')) {
        res = new Response('Missing name');
    }

    const id = await cli('account', 'add', [
        ['-n', fields.get('name')],
        ['-p', fields.get('phone')],
        ['-c', fields.get('company_id')],
        ['-a', fields.get('address_id')],
        ['--company-name', fields.get('company_name')],
        ['--country', fields.get('country')],
        ['--city', fields.get('city')],
        ['-s', fields.get('street')],
        ['--number', fields.get('number')],
        ['-u', fields.get('unit')],
        ['--postalcode', fields.get('postalcode')],
        ['--privacy-permissions', fields.get('privacy_permissions')],
    ]);
    //  console.log({ id });
    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/accounts/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to new account</button>`);
    } else {
        res = new Response('Done');
    }

    return res;
}

export async function PUT(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
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

    const id = await cli('account', 'update', [
        pathId,
        ['-n', fields.get('name')],
        ['-p', fields.get('phone')],
        ['-c', fields.get('company_id')],
        ['-a', fields.get('address_id')],
        ['--company-name', fields.get('company_name')],
        ['--country', fields.get('country')],
        ['--city', fields.get('city')],
        ['-s', fields.get('street')],
        ['--number', fields.get('number')],
        ['-u', fields.get('unit')],
        ['--postalcode', fields.get('postalcode')],
        ['--privacy-permissions', fields.get('privacy_permissions')],
    ], 'value');

    if (id && typeof id === 'string') {
        res = new Response(`<button hx-get="/accounts/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to updated account</button>`);
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

    const id = await cli('account', 'remove', pathId, 'value');
    if (id && typeof id === 'string') {
        res = new Response('Done');
    } else {
        res = new Response('Failed');
    }

    return res;
}