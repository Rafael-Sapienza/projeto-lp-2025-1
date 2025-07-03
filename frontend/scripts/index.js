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
            'size': 18,
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


const main = {
    type: "main", 
    message0: "%1 Iniciar",
    args0: [
        {
            type: "field_image",
            src: "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24'%3e%3cpath fill='%233CB827' d='M12 4c-4.41 0-8 3.59-8 8s3.59 8 8 8 8-3.59 8-8-3.59-8-8-8M9.5 16.5v-9l7 4.5z' opacity='.3'/%3e%3cpath fill='%233CB827' d='M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2m0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8'/%3e%3cpath fill='%233CB827' d='m9.5 16.5 7-4.5-7-4.5z'/%3e%3c/svg%3e",
            width: 30,
            height: 30,
            alt: "▶️",
            name: "PLAY_BUTTON"
        }
    ],
    nextStatement: null,
    tooltip: "Inicia o código",
    colour:  "#A5D49D",
}
Blockly.defineBlocksWithJsonArray([main]);
const mainBlock = workspace.newBlock("main");
mainBlock.initSvg();
mainBlock.render();
// You can optionally move it to (x,y) coordinates
mainBlock.moveBy(50, 50);
mainBlock.setDeletable(false);
mainBlock.setMovable(false);
