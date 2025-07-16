import { customBlocks, templateBlocks, variablesCategory, functionsCategory } from "./modules/blocks-definitions/blocks-library.js";
import { registerBlocks } from "./modules/blockly-setup/blocks-setup.js"
import { buildToolbox, injectWorkspace, setupRunBtn } from "./modules/blockly-setup/workspace-setup.js";
import { setupVariablesToolbox } from "./modules/blockly-setup/variables-setup.js";
import { setupFunctionsToolbox } from "./modules/blockly-setup/functions-setup.js";
import { easyTheme } from "./modules/interface/blockly-themes.js";

let selectedCustomBlocks = [];
for (const category in customBlocks) {
    selectedCustomBlocks = [...selectedCustomBlocks, ...Object.values(customBlocks[category])];
};

let selectedTemplateBlocks = [];
for (const category in templateBlocks) {
    selectedTemplateBlocks = [...selectedTemplateBlocks, ...Object.values(templateBlocks[category])];
};

registerBlocks([...selectedCustomBlocks, ...selectedTemplateBlocks]);
const toolbox = buildToolbox(selectedCustomBlocks, "categoryToolbox");
toolbox.contents.push(variablesCategory, functionsCategory);

const workspace = injectWorkspace("blocklyDiv", toolbox, easyTheme);
setupVariablesToolbox(workspace);
setupFunctionsToolbox(workspace);
setupRunBtn("easy_run", workspace);
