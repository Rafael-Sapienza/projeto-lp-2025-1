import { buildToolbox, injectWorkspace } from "./modules/blockly-setup.js";
import { customBlocks } from "./modules/blocks-definitions/blocks-state.js";
import { numberTemplate, textTemplate } from "./modules/blocks-definitions/blocks-templates.js";

console.log(customBlocks);
let allBlocks = [];
for (const category in customBlocks) {
    allBlocks = [...allBlocks, ...Object.values(customBlocks[category])];
};


Blockly.defineBlocksWithJsonArray([numberTemplate, textTemplate]);
Blockly.defineBlocksWithJsonArray(allBlocks);

const myTheme = Blockly.Theme.defineTheme( 
    'myTheme', 
    {
        'base': Blockly.Themes.Classic,
        'fontStyle': {
            'family': 'Sour Gummy',
            'size': 15,
        },
        componentStyles: {
            workspaceBackgroundColour: '#FFF9F0',
            toolboxBackgroundColour: '#FCE4D6',
            toolboxForegroundColour: '#3E2723',
            flyoutBackgroundColour: '#FFF3E0',
            flyoutForegroundColour: '#4E342E',
            flyoutOpacity: 0.95,
            scrollbarColour: '#D7CCC8',
            insertionMarkerColour: '#FF7043',
            insertionMarkerOpacity: 0.5,
            markerColour: '#F50057',
            cursorColour: ''
            },

        startHats: true,
    }
);

const toolbox = buildToolbox(allBlocks, "categoryToolbox");
const workspace = injectWorkspace("blocklyDiv", toolbox, myTheme);

Blockly.common.setParentContainer(document.body);


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
});


document.querySelector("button").addEventListener("click", execute);

async function execute() {
    const workspaceJson = Blockly.serialization.workspaces.save(workspace);
    console.log(JSON.stringify(workspaceJson));
    try {
        const response = await fetch('/execute', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(workspaceJson)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const output = await response.json();
        console.log(output);

        // Update the <pre id="output">
        document.getElementById('output').textContent = `Output:\n${output.join('\n')}`;

    } catch (error) {
        console.error('Error:', error);
        document.getElementById('output').textContent = `Error: ${error}`;
    } 
}
