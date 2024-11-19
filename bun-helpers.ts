import { $ } from 'bun';

export type ToPage = (...html: string[]) => Response;

export const map_input_type = {
  checkbox: (val: string) => val.startsWith('is_'),
  'datetime-local': (val: string) => val.endsWith('_at') || val.endsWith('date'),
  email: (val: string) => val.endsWith('email'),
  number: (val: string) =>
    (val.endsWith('number') && !(val.startsWith('commerce_') || val.startsWith('vat_')))
    || val.endsWith('rate')
    || val.startsWith('minutes_')
    || val.endsWith('id')
    || val.endsWith('percentage')
    || val.startsWith('total')
    || val.endsWith('discount')
    || val.endsWith('months'),
  password: (val: string) =>
    val.startsWith('password'),
  url: (val: string) =>
    val.endsWith('url')
    || val.endsWith('link')
    || val.endsWith('logo'),
  tel: (val: string) =>
    val.endsWith('phone'),
};

export function validateInputType(field_name: string) {
  for (const [type, check] of Object.entries(map_input_type)) {
    if (check(field_name)) {
      return type;
    }
  }
  return 'text';
}

export const map_input_autofill = {
  off: (val: string) => val.endsWith('id'),
  'address-line1': (val: string) => val.endsWith('street') || val.endsWith('address'),
  'address-level2': (val: string) => val.endsWith('city'),
  tel: (val: string) => val.endsWith('phone'),
  email: (val: string) => val.endsWith('email'),
  url: (val: string) => val.endsWith('url'),
  name: (val: string) => val === 'name',
  organization: (val: string) => val === 'company_name',
  'country-name': (val: string) => val.endsWith('country'),
  'postal-code': (val: string) => val.endsWith('postalcode'),
};

export function validateAutofill(field_name: string) {
  for (const [type, check] of Object.entries(map_input_autofill)) {
    if (check(field_name)) {
      return type;
    }
  }
  return 'on';
}

export function disabledField(field_name: string) {
  return field_name === 'id' || field_name === 'created_at' || field_name === 'updated_at';
}

export const htmlHead = `<!DOCTYPE html>
  <html lang="en">
  
  <head>
    <title></title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <!-- HTMX -->
    <script src="/js/htmx.min.js"></script>
    <!-- HyperScript -->
    <script src="/js/hyperscript.min.js"></script>
    <!-- PicoCss -->
    <link rel="stylesheet" href="/css/pico/pico.min.css" />
    <link rel="stylesheet" href="/css/casual-cli.css" />
    <script src="/js/accordion.js"></script>
  </head>
  
  <body>
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

export const htmlTail = `    </main>
    </div>
  </body>
  
  </html>`;

export const CLI = {
  account: [
    'list', 'ls', 'list-companies', 'list-addresses', 'list-contracts',
    'add', 'add-company', 'add-address', 'add-contract',
    'update', 'update-company', 'update-address', 'update-contract',
    'get', 'get-company', 'get-address', 'get-contract',
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
export type Program = keyof typeof CLI;
export type Command<T extends Program> = typeof CLI[T][number];

export const PrintMode = [
  'normal',
  'html',
  'value',
  'json'
] as const;
export type PrintMode = typeof PrintMode[number];


function singular(name: string): string {
  let newName: string;

  if (name.endsWith('ies')) {
    newName = name.slice(0, -3) + 'y';
  } else if (name.endsWith('ses')) {
    newName = name.slice(0, -2);
  } else if (name.endsWith('s')) {
    newName = name.slice(0, -1);
  } else {
    newName = name;
  }

  return newName;
}

function plural(name: string): string {
  let newName: string;

  if (name.endsWith('y')) {
    newName = name.slice(0, -1) + 'ies';
  } else if (name.endsWith('s')) {
    newName = name + 'es';
  } else {
    newName = name + 's';
  }

  return newName;
}

export function pretty(json: object): string {
  return decodeURIComponent(JSON.stringify(json, undefined, 4).replaceAll('\n', '<br/>').replaceAll(' ', '&nbsp;'));
}

export const isProd = Bun.env.NODE_ENV === "production";

export async function cli<T extends Program>(program: T, command: Command<T>, args?: ([string, string | number | undefined] | number)[] | number, print_mode: PrintMode = 'html'): Promise<string | Record<string, any> | any[] | void> {
  // console.log({ program, command, args });
  let argsString: '' | number | { raw: string; } = '';

  if (typeof args === 'number') {
    argsString = args;
  } else if (args && args.length > 0) {
    argsString = {
      raw: args.map((arg) => {
        if (typeof arg === 'number') {
          return `${arg}`;
        } else {
          const [param, value] = arg;
          if (param.startsWith('-') && value) {
            return `${param} ${value}`;
          } else {
            return false;
          }
        }
      }).filter(Boolean).join(' ')
    };
  }

  let cmd;
  try {
    if (isProd) {
      cmd = await $`casual-cli -m ${print_mode} ${program} ${command} ${argsString}`;
    } else {
      cmd = await $`cargo run --quiet -- -m ${print_mode} ${program} ${command} ${argsString}`;
    }

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

export function updateForm(id: string, putPath: string, fields: Record<string, string | number>, autoComplete?: Record<string, string[] | [string | number, string][]>, isCollapsable: boolean = false): string {
  return `${isCollapsable ? `
      <style>
        details>summary::after {
          margin-top: 11px;
          float: left;
          margin-right: 10px;
        }
        summary > h4 {
          display: inline-block;
        }
      </style>
      <details><summary>${title(id.replaceAll('-', ' '), 4)}</summary><div class="content">` : title(id.replaceAll('-', ' '), 2)}
  <label for="result">Result:</label><span id="result-${id}"></span>
  <form id="${id}" hx-put="${putPath}" hx-swap="innerHTML" hx-target="#result-${id}">
    ${Object.entries(fields).map(([name, value]) => {
    const inputArgs = `id="${id}-${name}" ${disabledField(name) ? 'readonly' : ''} autofill=${validateAutofill(name)} type="${validateInputType(name)}" ${validateInputType(name) === 'number' ? 'min="0"' : ''} ${validateInputType(name) === 'datetime-local' ? `value="${value}"` : `placeholder="${typeof value === 'string' ? decodeURIComponent(value) : value}"`}  name="${name}"`;

    let html = `<div><label for="${id}-${name}">${name.replaceAll('_', ' ')}</label>`;
    if (autoComplete && autoComplete[name]) {
      if (name.endsWith('_id')) {
        // use select
        html += `<select name="${name}" ${disabledField(name) ? 'readonly' : ''} id="${id}-${name}">
            ${autoComplete[name].map((val) => Array.isArray(val) ? `<option value="${val[0]}" ${value === val[0] ? 'selected' : ''}>${val[1]}</option>` : `<option>${val}</option>`).join('')}
          </select>`;
      } else {
        // use datalist
        html += `<input ${inputArgs} list="datalist-${name}" />`;
        html += `<datalist id="datalist-${name}">${autoComplete[name].map((val) => Array.isArray(val) ? `<option value="${val[0]}">${val[1]}</option>` : `<option>${val}</option>`).join('')}</datalist>`;
      }
    } else {
      html += `<input ${inputArgs} />`;
    }
    html += '</div>';

    return html;
  }).join('')}
    <button type="submit">Submit</button>
  </form>
  ${isCollapsable ? '</div></details>' : ''}`;
}


export function form(id: string, postPath: string, fieldsNames: string[], autoComplete?: Record<string, number | [string, number] | string[] | [string | number, string][]>, isCollapsable: boolean = false): string {
  const titleText = id.replaceAll('-', ' ');
  return `${isCollapsable ? `
      <style>
        details>summary::after {
          margin-top: 11px;
          float: left;
          margin-right: 10px;
        }
        summary > h4 {
          display: inline-block;
        }
      </style>
      <details><summary>${title(titleText, 4)}</summary><div class="content">` : title(titleText, 2)}
  <label for="result">Result:</label><span id="result-${id}"></span>
  <form id="${id}" hx-post="${postPath}" hx-swap="innerHTML" hx-target="#result-${id}">
    ${fieldsNames.map(name => {
    const inputId = `${id}-${name}`;
    const inputType = validateInputType(name);
    const inputArgs = `id="${inputId}" type="${inputType}" autofill=${validateAutofill(name)} ${inputType === 'number' ? 'min="0"' : ''} placeholder="${inputType === 'text' ? `My cool ${titleText.split(' ')[1]} ${name}` : '0'}" name="${name}"`;
    let html = `<div id="${inputId}-wrapper"><label for="${inputId}">${name.replaceAll('_', ' ')}</label>`;
    if (autoComplete && autoComplete[name]) {
      console.log({ name, value: autoComplete[name] });
      if (typeof autoComplete[name] === 'number' && autoComplete[name] > 0) {
        // use number as value and disable input
        html += `<select name="${name}" id="${inputId}" readonly><option val="${autoComplete[name]}" selected>${autoComplete[name]}</select>`;
      } else if (Array.isArray(autoComplete[name]) && autoComplete[name].length === 2 && typeof autoComplete[name][1] === 'string' && typeof autoComplete[name][0] === 'number') {
        // use select
        html += `<select name="${name}" readonly id="${inputId}"><option value='${autoComplete[name][0]}' selected>${autoComplete[name][1]}</option></select>`;
      } else if (name.endsWith('_id') && Array.isArray(autoComplete[name])) {
        // use select 
        html += `<select name="${name}" ${disabledField(name) ? 'readonly' : ''} id="${inputId}"><option value='' selected>Choose ${name}</option>
            ${autoComplete[name].map((val) => Array.isArray(val) ? `<option value="${val[0]}">${val[1]}</option>` : `<option>${val}</option>`).join('')}
          </select>`;
      } else if (Array.isArray(autoComplete[name])) {
        html += `<input ${inputArgs} list="datalist-${name}" />`;
        html += `<datalist id="datalist-${name}">${autoComplete[name].map((val) => Array.isArray(val) ? `<option value="${val[0]}">${val[1]}</option>` : `<option>${val}</option>`).join('')}</datalist>`;
      }
    } else {
      html += `<input ${inputArgs} />`;
    }
    html += '</div>';
    return html;
  }).join('')}
    <button type="submit">Submit</button>
  </form>
  ${isCollapsable ? '</div></details>' : ''}`;
}
export function title(title: string, heading: number = 1): string {
  return `<h${heading}>${decodeURIComponent(title)}</h${heading}>`;
}
export function overview(name: string, dataHtml: string, heading: number = 1, targetId: string = 'main') {
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
  </style>
  ${targetId !== 'main' ? `<article id="${targetId}"></article>` : ''}
  <script>
  
    document.querySelectorAll('.${name} [data-id]').forEach((el) => {
      if (el.querySelector('.view-button')) {
        return;
      }
      
      const docFragment = document.createDocumentFragment();
      
      if (el.dataset.recipientId !== undefined && el.dataset.recipientId !== '') {
        const btn = document.createElement('button');
        btn.setAttribute('class', 'recipient-button outline')
        btn.setAttribute('hx-get', \`/accounts/\${el.dataset.recipientId}\`);
        btn.setAttribute('hx-swap', 'innerHTML transition:true');
        btn.setAttribute('hx-target', '#main');
        btn.setAttribute('hx-push-url', 'true')
        btn.setAttribute('title', 'View recipient');
        btn.innerText = '👨👩';
        if (typeof window.htmx !== 'undefined') {
          htmx.process(btn);
        }
        docFragment.appendChild(btn);
      }
      
      const deleteBtn = document.createElement('button');
      deleteBtn.setAttribute('class', 'delete-button outline');
      deleteBtn.setAttribute('hx-delete', \`/${name}/\${el.dataset.id}\`);
      deleteBtn.setAttribute('hx-swap', 'innerHTML transition:true');
      deleteBtn.setAttribute('hx-target', '#${targetId}');
      deleteBtn.setAttribute('hx-confirm', 'This will permantly delete ${singular(name)}, are you sure?');
      deleteBtn.innerText = '❌';
      deleteBtn.setAttribute('title', 'Delete ${singular(name)}');
      if (typeof window.htmx !== 'undefined') {
        htmx.process(deleteBtn);
      }
      docFragment.appendChild(deleteBtn);
  
      ${targetId === 'quote-view' ? `
        const invoiceBtn = document.createElement('button');
        invoiceBtn.setAttribute('class', 'invoice-button outline');
        invoiceBtn.setAttribute('hx-get', \`/make-invoice/\${el.dataset.id}\`);
        invoiceBtn.setAttribute('hx-swap', 'outerHTML');
        invoiceBtn.setAttribute('hx-target', 'this');
        invoiceBtn.innerText = '💶';
        invoiceBtn.setAttribute('title', 'Make invoice');
        if (typeof window.htmx !== 'undefined') {
          htmx.process(invoiceBtn);
        }
        docFragment.appendChild(invoiceBtn);
      ` : ''}
  
  
      const btn = document.createElement('button');
      btn.setAttribute('class', 'view-button outline')
      btn.setAttribute('hx-get', \`/${name}/\${el.dataset.id}\`);
      btn.setAttribute('hx-swap', 'innerHTML${targetId === 'main' ? ' transition:true' : ''}');
      btn.setAttribute('hx-target', '#${targetId}');
      ${targetId === 'main' ? `btn.setAttribute('hx-push-url', 'true');` : ''}
      btn.innerText = '🔍';
      btn.setAttribute('title', 'View ${singular(name)}');
      if (typeof window.htmx !== 'undefined') {
        htmx.process(btn);
      }
      docFragment.appendChild(btn);
      el.appendChild(docFragment);
    });
  </script>`;
}

export async function parseBody(stream: ReadableStream): Promise<Map<string, string>> {
  const values = new Map<string, string>();
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

export type Route = (req: Request, path: string, pathId: number, page: ToPage) => Promise<Response>;

