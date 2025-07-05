import { blocksCategories } from "./blocks-categories.js";

export const variableBlocks = {};
const category = blocksCategories.variable;

const variablesSetNumber = {
    type: "variables_set_number",
    colour: category.colour,
    message0: "guardar número %1 em %2",
    args0: [
        {
            type: "input_value",
            name: "NUM",
            check: "Number",
        },
        {
            type: "field_variable",
            name: "VAR",
            variable: "num",
        },
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Atribui um valor numérico à variável.",
};
variableBlocks["variables_set_number"] = variablesSetNumber;

const variablesSetString = {
    type: "variables_set_string",
    colour: category.colour,
    message0: "guardar texto %1 em %2",
    args0: [
        {
            type: "input_value",
            name: "TEXT",
            check: "String",
        },
        {
            type: "field_variable",
            name: "VAR",
            variable: "text",
        },
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Atribui um texto à variável.",
};
variableBlocks["variables_set_string"] = variablesSetString;


const variablesGetNumber = {

}


Blockly.defineBlocksWithJsonArray([
    {
        "type": "variables_get_number",
        "message0": "%1 %2",
        "args0": [
            {
                "type": "field_image",
                "src": "https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Numbers_icon.svg/24px-Numbers_icon.svg.png",
                "width": 15,
                "height": 15,
                "alt": "#"
            },
            {
                "type": "field_variable",
                "name": "VAR",
                "variable": "num",
                "variableTypes": ["Number"]
            }
        ],
        "output": "Number",
        "colour": "#4CAF50",
        "tooltip": "Número armazenado nesta variável.",
        "helpUrl": ""
    },
    {
        "type": "variables_get_string",
        "message0": "%1 %2",
        "args0": [
            {
                "type": "field_image",
                "src": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/0b/Text_icon.svg/24px-Text_icon.svg.png",
                "width": 15,
                "height": 15,
                "alt": "T"
            },
            {
                "type": "field_variable",
                "name": "VAR",
                "variable": "text",
                "variableTypes": ["String"]
            }
        ],
        "output": "String",
        "colour": "#2196F3",
        "tooltip": "Texto armazenado nesta variável.",
        "helpUrl": ""
    }
]);
