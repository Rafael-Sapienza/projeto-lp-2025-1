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



/*Blockly.common.setParentContainer(document.body);
function corrigirEspacoEntreToolboxEFlyout() {
  const flyout = workspace.getFlyout();
  const toolboxWidth = workspace.toolbox_.getWidth();
  const metrics = workspace.getMetrics();

  // Força o reposicionamento correto
  flyout.svgGroup_.setAttribute(
    "transform",
    `translate(${toolboxWidth + 0}, ${metrics.absoluteTop})` // ajuste fino aqui (0px ou mais)
  );
}

// Corrige após a primeira renderização
setTimeout(corrigirEspacoEntreToolboxEFlyout, 0);

// Corrige toda vez que a categoria for alterada
workspace.addChangeListener(function(event) {
  if (event.type === Blockly.Events.TOOLBOX_ITEM_SELECT) {
    setTimeout(corrigirEspacoEntreToolboxEFlyout, 3000); // pequeno delay para dar tempo do Blockly desenhar
  }
});*/
