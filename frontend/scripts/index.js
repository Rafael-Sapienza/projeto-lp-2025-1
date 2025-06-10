import { buildToolbox } from "./modules/blockly-setup.js";
import { textBlocks } from "./modules/blocks-definitions.js";

Blockly.defineBlocksWithJsonArray(Object.values(textBlocks));
const toolbox = buildToolbox(Object.values(textBlocks), "categoryToolbox");

const workspace = Blockly.inject("blocklyDiv", {
    renderer: "zelos", // Prettier format
    trashcan: true,
    toolbox,
});
