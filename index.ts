import { $ } from 'bun';

const htmlHead = `<!DOCTYPE html>
<html lang="en">

<head>
  <title></title>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <!-- HTMX -->
  <script src="https://unpkg.com/htmx.org/dist/htmx.js"></script>
  <!-- HyperScript -->
  <script src="https://unpkg.com/hyperscript"></script>
  <!-- PicoCss -->
  <link rel="stylesheet" href="/css/pico.min.css" />
</head>

<body>
  <style>
    .row {
      display: flex;
      flex-direction: row;
    }

    aside {
      padding: 2rem;
    }

    main {
      padding: 2rem;
    }

    h1:first-letter,
    h2:first-letter {
      text-transform: uppercase;
    }

    form {
      display: grid;
      gap: 20px;
      /* define the number of grid columns */
      grid-template-columns: repeat(3, 1fr);
      margin: 2rem 0;
    }

    @keyframes fade-in {
      from {
        opacity: 0;
      }
    }

    @keyframes fade-out {
      to {
        opacity: 0;
      }
    }

    @keyframes slide-from-right {
      from {
        transform: translateX(90px);
      }
    }

    @keyframes slide-to-left {
      to {
        transform: translateX(-90px);
      }
    }

    .slide-it {
      view-transition-name: slide-it;
    }


    ::view-transition-old(slide-it) {
      animation: 180ms cubic-bezier(0.4, 0, 1, 1) both fade-out,
        600ms cubic-bezier(0.4, 0, 0.2, 1) both slide-to-left;
    }

    ::view-transition-new(slide-it) {
      animation: 180ms cubic-bezier(0, 0, 0.2, 1) 90ms both fade-in,
        600ms cubic-bezier(0.4, 0, 0.2, 1) both slide-from-right;
    }
  </style>
  <div class="row">
    <aside>
      <nav>
        <ul>
          <li><a href="/">Home</a></li>
          <li><button hx-get="/accounts" hx-swap="innerHTML transition:true" hx-target="#main"
              hx-push-url="true">Accounts</button></li>
          <li><button hx-get="/projects" hx-swap="innerHTML transition:true" hx-target="#main"
              hx-push-url="true">Projects</button></li>
          <li><button hx-get="/schedule" hx-swap="innerHTML transition:true" hx-target="#main"
              hx-push-url="true">Schedule</button></li>
          <li><button hx-get="/finance" hx-swap="innerHTML transition:true" hx-target="#main"
              hx-push-url="true">Finance</button></li>
        </ul>
      </nav>
    </aside>
    <main id="main" class="container slide-it">`;

const htmlTail = `    </main>
  </div>
</body>

</html>`;

const CLI = {
  account: [
    'list', 'ls', 'list-companies', 'list-addresses',
    'add', 'add-company', 'add-address',
    'update', 'update-company', 'update-address',
    'get', 'get-company', 'get-address',
    'remove', 'remove-company', 'remove-address'
  ],
  project: [
    'list', 'ls', 'list-tasks', 'list-quotes', 'list-invoices',
    'add', 'add-task',
    'get', 'get-task', 'complete-task',
    'get-quote', 'get-invoice',
    'make-quote', 'make-invoice',
    'update', 'update-task', 'update-quote', 'update-invoice',
    'remove', 'remove-task', 'remove-quote', 'remove-invoice'
  ],
  schedule: ['list', 'ls', 'get', 'add', 'update', 'remove'],
  finance: ['report', 'add-query', 'update-report', 'update-query', 'remove', 'remove-query']
} as const;
type Program = keyof typeof CLI;
type Command<T extends Program> = typeof CLI[T][number];

const PrintMode = [
  'normal',
  'html',
  'value',
  'json'
] as const;
type PrintMode = typeof PrintMode[number];

function pretty(json: object): string {
  return decodeURIComponent(JSON.stringify(json, undefined, 4).replaceAll('\n', '<br/>').replaceAll(' ', '&nbsp;'));
}

const isProd = Bun.env.NODE_ENV === "production";

async function cli<T extends Program>(program: T, command: Command<T>, args?: [string, string | number][] | number, print_mode: PrintMode = 'html'): Promise<string | Record<string, any> | any[] | void> {
  // console.log({ program, command, args });
  let argsString: '' | number | { raw: string; } = '';

  if (typeof args === 'number') {
    argsString = args;
  } else if (args && args.length > 0) {
    argsString = {
      raw: args.map(([param, value]) => {
        if (param.startsWith('-') && value) {
          return `${param} ${value}`;
        } else {
          return false;
        }
      }).filter(Boolean).join(' ')
    };
  }

  console.debug({ isProd, print_mode, program, command, argsString, args });
  let cmd;
  try {
    if (isProd) {
      cmd = await $`casual-cli -m ${print_mode} ${program} ${command} ${argsString}`;
    } else {
      cmd = await $`cargo run --quiet -- -m ${print_mode} ${program} ${command} ${argsString}`;
    }
    console.debug({ cmd, data: cmd.stdout.toString() });
    if (cmd) {
      if (cmd.exitCode !== 0) {
        console.error('cli failed', cmd.stderr.toString());
      } else if (print_mode === 'json') {
        return JSON.parse(cmd.stdout.toString());
      } else if (print_mode === 'value') {
        return cmd.stdout.toString();
      } else {
        return decodeURIComponent(cmd.stdout.toString());
      }
    }
    console.warn('BunShell returned nothing');
  } catch (err) {
    console.error('Cli error:', err);
    return 'Server error';
  }
}

function form(id: string, postPath: string, fieldsNames: string[], autoComplete?: Record<string, string[] | [string | number, string][]>, isCollapsable: boolean = false): string {
  return `${isCollapsable ? `
    <style>
      details>summary::after {
        margin-top: 11px;
        float: left;
        margin-right: 10px;
      }
      summary > h3 {
        display: inline-block;
      }
    </style>
    <details><summary>${title(id.replaceAll('-', ' '), 3)}</summary>` : title(id.replaceAll('-', ' '), 2)}
<label for="result">Result:</label><span id="result-${id}"></span>
<form id="${id}" hx-post="${postPath}" hx-swap="innerHTML" hx-target="#result-${id}">
  ${fieldsNames.map(name => {
    let html = `<input placeholder="${name}" name="${name}" />`;


    if (autoComplete && autoComplete[name]) {
      html = `<input placeholder="${name}" name="${name}" list="datalist-${name}" />`;
      html += `<datalist id="datalist-${name}">${autoComplete[name].map((val) => Array.isArray(val) ? `<option value="${val[0]}">${val[1]}</option>` : `<option>${val}</option>`).join('')}</datalist>`;
    }

    return html;
  }).join('')}
  <button type="submit">Submit</button>
</form>
${isCollapsable ? '</details>' : ''}`;
}
function title(title: string, heading: number = 1): string {
  return `<h${heading}>${decodeURIComponent(title)}</h${heading}>`;
}
function overview(name: string, dataHtml: string, heading: number = 1, targetId: string = 'main') {
  return `${title(name, heading)}
<div class="${name}">${dataHtml}</div>
<style>
 .${name} {
    display: grid;
    gap: 20px;
    /* define the number of grid columns */
    grid-template-columns: repeat(4, 1fr);
    margin: 2rem 0;
  }
  .${name} span {
    border: var(--pico-border-width) solid var(--pico-primary-border);
    border-radius: var(--pico-border-radius); 
    padding-left: 8px;
    line-height: 2;
  }
  .${name} button {
    float: right;
    padding: 5px 18px;
  }
  
  .${name} span button {
    width: calc(100% + 8px);
    margin-top: 12px;
  }
</style>
${targetId !== 'main' ? `<article id="${targetId}"></article>` : ''}
<script>
  document.querySelectorAll('.${name} [data-recipient-id]').forEach((el) => {
    if (el.querySelector('.recipient-button')) {
      return;
    }

    const btn = document.createElement('button');
    btn.setAttribute('class', 'recipient-button')
    btn.setAttribute('hx-get', \`/accounts/\${el.dataset.recipientId}\`);
    btn.setAttribute('hx-swap', 'innerHTML transition:true');
    btn.setAttribute('hx-target', '#main');
    btn.setAttribute('hx-push-url', 'true')
    btn.innerText = 'Recipient';
    if (typeof window.htmx !== 'undefined') {
      htmx.process(btn);
    }
    el.appendChild(btn);
  });

  document.querySelectorAll('.${name} [data-id]').forEach((el) => {
    if (el.querySelector('.view-button')) {
      return;
    }

    ${targetId === 'quote-view' ? `
      const invoiceBtn = document.createElement('button');
      invoiceBtn.setAttribute('class', 'invoice-button');
      invoiceBtn.setAttribute('hx-get', \`/make-invoice/\${el.dataset.id}\`);
      invoiceBtn.innerText = '💶';
      invoiceBtn.setAttribute('title', 'Make invoice');
      if (typeof window.htmx !== 'undefined') {
        htmx.process(invoiceBtn);
      }
      el.appendChild(invoiceBtn);
    ` : ''}


    const btn = document.createElement('button');
    btn.setAttribute('class', 'view-button')
    btn.setAttribute('hx-get', \`/${name}/\${el.dataset.id}\`);
    btn.setAttribute('hx-swap', 'innerHTML${targetId === 'main' ? ' transition:true' : ''}');
    btn.setAttribute('hx-target', '#${targetId}');
    ${targetId === 'main' ? `btn.setAttribute('hx-push-url', 'true');` : ''}
    btn.innerText = '🔍';
    btn.setAttribute('title', 'View ${name.endsWith('s') ? name.slice(0, -1) : name}');
    if (typeof window.htmx !== 'undefined') {
      htmx.process(btn);
    }
    el.appendChild(btn);
  });
</script>`;
}

async function parseBody(stream: ReadableStream) {
  const values = new Map();
  let bodyString = '';
  for await (const chunk of stream.values({ preventCancel: true })) {
    bodyString += Buffer.from(chunk).toString();
    //const [key, value] = Buffer.from(chunk).toString().split("=");
    //console.debug({ key, value, chunk })
    //values.set(key, value);
  }
  bodyString.split('&')
    .map(pair =>
      pair.split('=')
    ).forEach(([key, value]) => {
      values.set(key, value);
    });

  // console.debug({ bodyString, values });
  return values;
}

Bun.serve({
  /* ts-ignore */
  idleTimeout: 255,

  async fetch(req, _server) {
    try {
      // console.debug(req, server)
      const url = new URL(req.url);
      // const origin = url.origin;
      const pathParts = url.pathname.split('/');
      const path = `/${pathParts[1]}`;
      const pathId = parseInt(pathParts[2], 10);
      // const queryParams = url.searchParams;
      const method = req.method;

      let res: Response;

      outer: switch (method) {
        case "GET": {
          if (pathParts.includes('.ccli')) {
            res = new Response(Bun.file(url.pathname));
            break outer;
          }

          const page = (value: string) => req.headers.get('Hx-Request') ? new Response(`${value}`) : new Response(`${htmlHead}${value}${htmlTail}`, { headers: { 'Content-Type': 'text/html' } });

          switch (path) {

            case "/": {
              res = new Response(Bun.file('./index.html'));
            } break outer;

            case "/hello": {
              res = new Response('Hello World!');
            } break outer;

            case "/accounts": {
              if (pathId && !Number.isNaN(pathId)) {
                const account = await cli('account', 'get', pathId, 'json');

                if (account && typeof account === 'object' && !Array.isArray(account)) {
                  res = page(`${title(account.name)}${pretty(account)}`);
                  break outer;
                } else {
                  console.error('project id not found', { pathId, account });
                }
              }

              const accounts = await cli('account', 'ls');
              const companies = await cli('account', 'list-companies');

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
              if (companiesJson && Array.isArray(companiesJson)) {
                address_id = companiesJson.map((company) => [company.address_id, company.name]);
              }
              if (accountJson && Array.isArray(accountJson)) {
                address_id.concat(accountJson.map((account) => [account.address_id, account.name]));
              }

              res = page(`
${overview('accounts', typeof accounts === 'string' ? accounts : 'No accounts found')}
${form("add-account", "/accounts", ['name', 'phone', 'email', 'company_id', 'address_id', 'company_name', 'country', 'city', 'street', 'number', 'unit', 'postalcode', 'privacy_permissions'], { company_id, address_id }, true)}
${overview('companies', typeof companies === 'string' ? companies : 'No companies found', 2)}
${form("add-company", "/company", ['name', 'logo', 'commerce_number', 'vat_number', 'iban', 'phone', 'email', 'account_id', 'address_id', 'country', 'city', 'street', 'number', 'unit', 'postalcode'], { account_id, address_id }, true)}`);
            } break outer;

            case "/companies": {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing company id');
                break outer;
              }

              const company = await cli('account', 'get-company', pathId, 'json');
              if (company && typeof company === 'object' && !Array.isArray(company)) {
                res = page(`${title(company.name)}${pretty(company)}`);
              } else {
                console.log('No task found', { company });
                res = new Response(`Not found, received ${company}`);
              }
            } break outer;

            case "/projects": {
              if (pathId && !Number.isNaN(pathId)) {
                const project = await cli('project', 'get', pathId, 'json');


                if (project && typeof project === 'object' && !Array.isArray(project)) {
                  const tasks = await cli('project', 'list-tasks', project.id);
                  const quotes = await cli('project', 'list-quotes', [['-p', pathId]]);
                  const invoices = await cli('project', 'list-invoices', [['-p', pathId]]);

                  res = page(`${title(project.title)}
${pretty(project)}
<br/>
<button hx-get="/make-quote/${pathId}" hx-swap="outerHTML" hx-target="this" title="Make quote">📝</button>
<br/>
${overview('tasks', typeof tasks === 'string' ? tasks : 'No tasks found', 2, 'task-view')}
${form('add-task', '/task/' + pathId, ['title', 'description', 'minutes_estimated', 'minutes_spent', 'minutes_remaining', 'minutes_billed', 'minute_rate'], undefined, true)}
<br/>
${overview('quotes', typeof quotes === 'string' ? quotes.replaceAll('/usr/src/app/public', '') : 'No quotes found', 2, 'quote-view')}
<br/>
${overview('invoices', typeof invoices === 'string' ? invoices.replaceAll('/usr/src/app/public', '') : 'No invoices found', 2, 'invoice-view')}
`);
                  break outer;
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
              res = page(`
${overview('projects', typeof projects === 'string' ? projects : 'No projects found')}
${form('add-project', '/projects', ['title', 'description', 'client_id'], { client_id })}`);
            } break outer;

            case '/make-quote': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing task id');
                break outer;
              }

              // This is not working so making workaround
              // let quote_url = await cli('project', 'make-quote', [['-p', pathId]], 'json');
              cli('project', 'make-quote', [['-p', pathId]], 'normal');
              res = new Response(`<button hx-get="/projects/${pathId}" hx-swap="innerHTML transition:true" hx-target="#main">Processing...<br/>click to refresh</button>`);
              /*
              if (Array.isArray(quote_url)) {
                quote_url = quote_url[0] as string;
              } else if (typeof quote_url === 'object') {
                quote_url = quote_url.data as string;
              }

              console.debug({ quote_url });
              if (quote_url && typeof quote_url === 'string') {
                if (quote_url.startsWith('"')) {
                  quote_url = quote_url.trimEnd().slice(1);
                }
                if (quote_url.endsWith('"')) {
                  quote_url = quote_url.slice(0, quote_url.length - 1);
                }
                if (quote_url.includes('/public/')) {
                  quote_url = `/${quote_url.split('/public/')[1]}`
                }

                res = new Response('Done', {
                  headers: {
                    'HX-Redirect': quote_url
                  }
                });
                //res = new Response(`<a href="${quote_url}">Download quote</a>`);
              } else {
                console.log('No quote url from cli');
                res = new Response(`No url found for the quote, found ${quote_url}`);
              }*/
            } break outer;

            case '/make-invoice': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing quote id');
                break outer;
              }

              cli('project', 'make-invoice', [['-q', pathId]], 'normal');
              res = new Response(`<button hx-get="/projects/${pathId}" hx-swap="innerHTML transition:true" hx-target="#main">Processing...<br/>click to refresh</button>`);
            } break outer;

            case '/quotes': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing quote id');
                break outer;
              }

              const quote = await cli('project', 'get-quote', pathId, 'json');
              if (quote && typeof quote === 'object') {
                res = new Response(`${pretty(quote)}`);
              } else {
                console.log('No quote found', { quote });
                res = new Response(`Not found, received ${quote}`);
              }
            } break outer;

            case '/invoices': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing invoice id');
                break outer;
              }

              const invoice = await cli('project', 'get-invoice', pathId, 'json');
              if (invoice && typeof invoice === 'object') {
                res = new Response(`${pretty(invoice)}`);
              } else {
                console.log('No invoice found', { invoice });
                res = new Response(`Not found, received ${invoice}`);
              }
            } break outer;

            case '/tasks': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing task id');
                break outer;
              }

              const task = await cli('project', 'get-task', pathId, 'json');
              if (task && typeof task === 'object') {
                res = new Response(`${pretty(task)}`);
              } else {
                console.log('No task found', { task });
                res = new Response(`Not found, received ${task}`);
              }
            } break outer;

            case "/schedule": {
              const schedule = await cli('schedule', 'ls');
              res = page(`
${overview('schedule', typeof schedule === 'string' ? schedule : 'No scheduled items')}
${form('add-schedule', '/schedule', ['date'])}`);
            } break outer;

            case "/finance": {
              const report = await cli('finance', 'report');
              res = page(`<h1>Finance</h1>${report}`);
            } break outer;

            default: {
              let file = Bun.file(`./public${url.pathname}`);
              if (await file.exists()) {
                res = new Response(file);
              } else {
                res = new Response('404');
              }
            } break outer;
          }
        } break; // GET

        case "POST": {
          if (!req.body) {
            res = new Response('Body required for POST');
            break outer;
          }

          const fields = await parseBody(req.body);

          switch (path) {
            case "/accounts": {
              if (!fields.has('name')) {
                res = new Response('Missing name');
                break outer;
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
            } break outer;

            case '/company': {
              if (!fields.has('name')) {
                res = new Response('Missing name');
                break outer;
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
            } break outer;

            case '/projects': {
              if (!fields.has('title')) {
                res = new Response('Missing title');
                break outer;
              }

              const id = await cli('project', 'add', [
                ['-t', fields.get('title')],
                ['-d', fields.get('description')],
                ['-c', fields.get('client_id')]
              ], 'value');
              // console.log({ id });
              if (id && typeof id === 'string') {
                res = new Response(`<button hx-get="/projects/${id}" hx-swap="innerHTML transition:true" hx-target="#main">Go to new project</button>`);
              } else {
                res = new Response('done');
              }
            } break outer;

            case '/task': {
              if (!pathId || Number.isNaN(pathId)) {
                res = new Response('Missing project id');
                break outer;
              }
              if (!fields.has('title')) {
                res = new Response('Missing title');
                break outer;
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
            } break outer;

            default: {
              res = new Response('404');
            } break outer;
          }
        } break; // POST

        case 'DELETE': {
          if (!pathId || Number.isNaN(pathId)) {
            res = new Response('Missing id');
            break outer;
          }

          switch (path) {
            case '/accounts': {
              const id = await cli('account', 'remove', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/companies': {
              const id = await cli('account', 'remove-company', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/projects': {
              const id = await cli('project', 'remove', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/tasks': {
              const id = await cli('project', 'remove-task', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/quotes': {
              const id = await cli('project', 'remove-quote', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/invoices': {
              const id = await cli('project', 'remove-invoice', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/schedule': {
              const id = await cli('schedule', 'remove', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;

            case '/finance': {
              const id = await cli('finance', 'remove', pathId, 'value');
              if (id && typeof id === 'string') {
                res = new Response('Done');
              } else {
                res = new Response('Failed');
              }
            } break outer;


            default: {
              res = new Response('404');
            } break outer;
          }
        }

        default: {
          res = new Response('404');
        } break;
      }
      console.debug({ res });
      return res;
    } catch (err) {
      return new Response(`Error 500: ${err}`, { status: 500 });
    }
  }
});
