export function GET(req: Request, path: string, pathId?: number) {
    return page(`${title('Home')}<p>Nothing here</p>`);
}