import { textBlocks } from "./text-blocks.js";
import { mathBlocks } from "./math-blocks.js";
//import { compareBlocks } from "./compare-blocks.js";
import { controlBlocks } from "./control-blocks.js";
import { shadowBlocks } from "./shadow-blocks.js";
import { variableBlocks } from "./variable-blocks.js";
import { runBlocks } from "./run-blocks.js";

export { variablesCategory, functionsCategory } from "./blocks-categories.js";

export const customBlocks = {
    textBlocks, 
    mathBlocks,
    //compareBlocks,
    controlBlocks,
}

export const templateBlocks = {
    shadowBlocks,
    variableBlocks,
    runBlocks,
}
