import {syscallPrint} from "compilerInternal";
import {isTrue} from "./demo2";


if (isTrue()) {
    syscallPrint(1, "its true", 8);
}

if (!isTrue()) {
    syscallPrint(1, "its not true",12 );
}