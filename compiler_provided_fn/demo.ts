import {syscallPrint} from "compilerInternal";


let i = 0;

while (i !== 10){
    i += 1;
    syscallPrint(1, "1",1 )
}