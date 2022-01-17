import {syscallPrint} from "compilerInternal";


let i = 0;

while (i !== 10) {
    i += 1;
    const s = "Hello World!";
    syscallPrint(1, s, 12);
}