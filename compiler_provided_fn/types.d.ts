declare module "compilerInternal" {
    export function syscallPrint(rdi : number, s : string, size : number) : void
}