import { buildToolbox, injectWorkspace } from "./modules/blockly-setup.js";
import { customBlocks } from "./modules/blocks-definitions.js";

Blockly.defineBlocksWithJsonArray(Object.values(customBlocks));
const toolbox = buildToolbox(Object.values(customBlocks), "categoryToolbox");
const workspace = injectWorkspace("blocklyDiv", toolbox);

