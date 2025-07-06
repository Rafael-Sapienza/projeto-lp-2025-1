import { addContentWithCategory } from "./helper-functions.js"
import { easyInterpretation } from "../../api/easy-mode-api.js"

export const buildToolbox = function(blocks=[], kind="flyoutToolbox") {
    const toolbox = {
        kind,
        contents: [],
    };

    blocks.forEach( block => {
        const content = {
            kind: "block",
            type: block.type,
        };

        if (block.hasOwnProperty("shadow")) {
            content.inputs = block.shadow;
        }

        if (kind === "categoryToolbox") {
            addContentWithCategory(block, content, toolbox.contents);
        }
        else if (kind === "flyoutToolbox") {
            toolbox.contents.push(content);
        }
    });
    
    return toolbox;
}

export const injectWorkspace = function(divId, toolbox, theme, renderer="zelos", trashcan=true) {
    const workspace = Blockly.inject(divId, {
        toolbox,
        theme,
        renderer,
        trashcan,
    });
    return workspace;
}

export const setupRunBtn = function(type, workspace) {
    const runBtn = workspace.newBlock(type);
    runBtn.initSvg();
    runBtn.render();
    runBtn.moveBy(50, 50);
    runBtn.setDeletable(false);
    runBtn.setMovable(false);

    let lastSelectedBlockId = null;

    workspace.addChangeListener(function(event) {
        if (
          event.type === Blockly.Events.SELECTED
        ) {
            const newId = event.newElementId;
            if (newId && newId !== lastSelectedBlockId) {
                const block = workspace.getBlockById(newId);
                if (block && block.type === "easy_run") {
                  console.log("Main block clicked!!!!!");
                  easyInterpretation(workspace);
                }
            }
            lastSelectedBlockId = newId;
        }
    });
}
