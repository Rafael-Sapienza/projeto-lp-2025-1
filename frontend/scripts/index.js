import { buildToolbox, injectWorkspace } from "./modules/blockly-setup.js";
import { customBlocks } from "./modules/blocks-definitions/blocks-state.js";
import { numberTemplate } from "./modules/blocks-definitions/blocks-templates.js";

console.log(customBlocks);
let allBlocks = [];
for (const category in customBlocks) {
    allBlocks = [...allBlocks, ...Object.values(customBlocks[category])];
};


Blockly.defineBlocksWithJsonArray([numberTemplate]);
Blockly.defineBlocksWithJsonArray(allBlocks);
const toolbox = buildToolbox(allBlocks, "categoryToolbox");
const workspace = injectWorkspace("blocklyDiv", toolbox);


