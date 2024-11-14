import { $ } from 'bun';

const Program = [
  'account',
  'project',
  'schedule',
  'finance'
] as const;

const PrintMode = [
  'normal',
  'html',
  'value',
  'json'
] as const;

function pretty(json: object): string {
  return decodeURIComponent(JSON.stringify(json, undefined, 4).replaceAll('\n', '<br/>').replaceAll(' ', '&nbsp;'));
}

const isProd = Bun.env.NODE_ENV === "production";

async function cli(program: typeof Program[number], command: string | number, args?: [string, string | number][] | number, print_mode: typeof PrintMode[number] = 'html'): Promise<string | Record<string, any> | void> {
  // console.log({ program, command, args });
  let argsString: '' | number | { raw: string } = '';

  if (typeof args === 'number') {
    argsString = args;
  } else if (args && args.length > 0) {
    argsString = {
      raw: args.map(([param, value]) => {
        if (param.startsWith('-') && value) {
          return `${param} ${value}`;
        } else if (param && !value) {
          return `${param}`;
        } else {
          return false;
        }
      }).filter(Boolean).join(' ')
    };
  }

  console.debug({ isProd, print_mode, program, command, argsString, args });
  let cmd;
  if (isProd) {
    cmd = await $`casual-cli -m ${print_mode} ${program} ${command} ${argsString}`
  } else {
    cmd = await $`cargo run --quiet -- -m ${print_mode} ${program} ${command} ${argsString}`
  }

  if (cmd) {
    if (cmd.exitCode !== 0) {
      console.error('cli failed', cmd.stderr.toString());
    } else {
      if (print_mode === 'json') {
        return JSON.parse(cmd.stdout.toString());
      } else {
        return decodeURIComponent(cmd.stdout.toString());
      }
    }
  }
}

function form(id: string, postPath: string, fieldsNames: string[], autoComplete?: Record<string, string[]>) {
  return `${title(id.replaceAll('-', ' '), 2)}<label for="result">Result:</label><span id="result"></span>
<form id="${id}" hx-post="${postPath}" hx-swap="innerHTML" hx-target="#result">
  ${fieldsNames.map(name => {
    let html = `<input placeholder="${name}" name="${name}" />`;

    if (autoComplete && autoComplete[name]) {
      html = `<input placeholder="${name}" name="${name}" list="datalist-${name}" />`;
      html += `<datalist id="datalist-${name}">${autoComplete[name].map((val) => `<option>${val}</option>`).join('')}</datalist>`;
    }

    return html;
  }).join('')}
  <button type="submit">Submit</button>
</form>`;
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
</style>
<script>
  document.querySelectorAll('.${name} [data-id]').forEach((el) => {
    const btn = document.createElement('button');
    btn.setAttribute('hx-get', \`/${name}/\${el.dataset.id}\`);
    btn.setAttribute('hx-swap', 'innerHTML${targetId === 'main' ? ' transition:true' : ''}');
    btn.setAttribute('hx-target', '#${targetId}');
    btn.innerText = 'view';
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

  console.debug({ bodyString, values });
  return values;
}

Bun.serve({
  async fetch(req, _server) {
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
              console.log({ account });
              if (account && typeof account === 'object') {
                res = new Response(`${title(account.name)}${pretty(account)}`)
                break outer;
              } else {
                console.error('project id not found', pathId);
              }
            }

            const accounts = await cli('account', 'ls');
            const companies = await cli('account', 'list-companies')
            res = new Response(`
${overview('accounts', typeof accounts === 'string' ? accounts : 'No accounts found')}
${form("add-account", "/accounts", ['name', 'phone', 'email', 'company_id', 'address_id', 'company_name', 'country', 'city', 'street', 'number', 'unit', 'postalcode', 'privacy_permissions'])}
${overview('companies', typeof companies === 'string' ? companies : 'No companies found', 2)}
${form("add-company", "/company", ['name', 'logo', 'commerce_number', 'vat_number', 'iban', 'phone', 'email', 'account_id', 'address_id', 'country', 'city', 'street', 'number', 'unit', 'postalcode'])}`);
          } break outer;

          case "/companies": {
            if (!pathId || Number.isNaN(pathId)) {
              res = new Response('Missing company id');
              break outer;
            }

            const company = await cli('account', 'get-company', pathId, 'json');
            if (company && typeof company === 'object') {
              res = new Response(`${title(company.name)}${pretty(company)}`);
            } else {
              console.log('No task found', { company })
              res = new Response(`Not found, received ${company}`);
            }
          } break outer;

          case "/projects": {
            if (pathId && !Number.isNaN(pathId)) {
              const project = await cli('project', 'get', pathId, 'json');


              if (project && typeof project === 'object') {
                const tasks = await cli('project', 'list-tasks', project.id);
                res = new Response(`${title(project.title)}
${pretty(project)}
<br/>
<button hx-get="/make-quote/${pathId}">Download quote</button>
<br/>
${overview('tasks', typeof tasks === 'string' ? tasks : 'No tasks found', 2, 'task-view')}
<article id="task-view"></article>
${form('add-task', '/task/' + pathId, ['title', 'description', 'minutes_estimated', 'minutes_spent', 'minutes_remaining', 'minutes_billed', 'minute_rate'])}
`);
                break outer;
              } else {
                console.error('project id not found', pathId);
              }
            }

            const projects = await cli('project', 'ls');

            res = new Response(`
${overview('projects', typeof projects === 'string' ? projects : 'No projects found')}
${form('add-project', '/projects', ['title', 'description', 'client_id'], { client_id: ['1', '2', '3'] })}`);
          } break outer;

          case '/make-quote': {
            if (!pathId || Number.isNaN(pathId)) {
              res = new Response('Missing task id');
              break outer;
            }

            const quote_url = await cli('project', 'make-quote', [['-p', pathId]], 'value');
            if (quote_url && typeof quote_url === 'string') {
              res = new Response();
              res.headers.set('HX-Redirect', quote_url);
            } else {
              console.log('No quote url from cli')
              res = new Response(`No url found for the quote, found ${quote_url}`);
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
              console.log('No task found', { task })
              res = new Response(`Not found, received ${task}`);
            }
          } break outer;

          case "/schedule": {
            const schedule = await cli('schedule', 'ls');
            res = new Response(`
${overview('schedule', typeof schedule === 'string' ? schedule : 'No scheduled items')}
${form('add-schedule', '/schedule', ['date'])}`);
          } break outer;

          case "/finance": {
            const report = await cli('finance', 'report');
            res = new Response(`<h1>Finance</h1>${report}`);
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
      }

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
              res = new Response(id);
            } else {
              res = new Response('Done')
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
            ]);

            if (id && typeof id === 'string') {
              res = new Response(id);
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
            ]);
            // console.log({ id });
            if (id && typeof id === 'string') {
              res = new Response(id);
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
            ]);

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
      }

      default: {
        res = new Response('404');
      } break;
    }
    return res;
  }
});
