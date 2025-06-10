import { addContentWithCategory } from "./helper-functions.js"

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
            // TODO: Implement
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

function injectWorkspace(divId, toolbox, renderer="zelos", trashcan=true) {
    const workspace = Blockly.inject(divId, {
        renderer: renderer,
        trashcan: true,
        toolbox: toolbox,
    });
    return workspace;
}
