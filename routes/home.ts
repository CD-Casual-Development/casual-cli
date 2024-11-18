import { title, type ToPage } from "../bun-helpers";

export function GET(req: Request, path: string, pathId: number, page: ToPage): Promise<Response> {
    return Promise.resolve(page(`${title('Home')}<p>Nothing here</p>`));
}