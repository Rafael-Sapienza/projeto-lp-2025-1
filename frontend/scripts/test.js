import { textBlocks } from "./modules/blocks-definitions.js";

Blockly.defineBlocksWithJsonArray(Object.values(textBlocks));

const workspace = Blockly.inject("blocklyDiv", {
    renderer: "zelos", // Prettier format
    trashcan: true,
    toolbox: {
        //kind: "flyoutToolbox",
        kind: "categoryToolbox",
        contents: [
            {
                kind: "category",
                name: "TEXTO",
                colour: "#5ba58c",
                contents: [
                    { kind: "block", type: "print" },
                    { 
                        kind: "block", 
                        type: "length",
                        inputs: {
                            TEXT: {
                                shadow: {
                                    type: "text",
                                }
                            }
                        }
                    },
                    {
                        kind: "block",
                        type: "join",
                        inputs: {
                            TEXT1: {
                                shadow: {
                                    type: "text",
                                },
                            },
                            TEXT2: {
                                shadow: {
                                    type: "text",
                                },
                            },
                        },
                    },
                ],
            },
        ],
    },
});
