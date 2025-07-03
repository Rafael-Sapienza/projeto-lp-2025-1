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
        renderer,
        trashcan,
        toolbox,
        theme,
    });
    return workspace;
}
