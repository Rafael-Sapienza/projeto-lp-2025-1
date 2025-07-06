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
            //variableTypes: ["Number"]
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
            //variableTypes: ["String"]
        },
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Atribui um texto à variável.",
};
variableBlocks["variables_set_string"] = variablesSetString;


const variablesGetNumber = {
    type: "variables_get_number",
    colour: category.colour,
    message0: "%1 %2",
    args0: [
        {
            type: "field_image",
            src: "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M12 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM4 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z'/%3e%3cpath d='M4 2.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5zm0 4a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm3-6a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm3-6a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5z'/%3e%3c/svg%3e",
            width: 15,
            height: 15,
            alt: "#"
        },
        {
            type: "field_variable",
            name: "VAR",
            variable: "num",
            //"variable": "Number",
            //"variableTypes": ["Number"]
        }
    ],
    output: "Number",
    tooltip: "Número armazenado nesta variável.",
    helpUrl: ""
}
variableBlocks["variables_get_number"] = variablesGetNumber;


const variablesGetString = {
    type: "variables_get_string",
    colour: category.colour,
    message0: "%1 %2",
    args0: [
        {
            type: "field_image",
            src: "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M2.678 11.894a1 1 0 0 1 .287.801 11 11 0 0 1-.398 2c1.395-.323 2.247-.697 2.634-.893a1 1 0 0 1 .71-.074A8 8 0 0 0 8 14c3.996 0 7-2.807 7-6s-3.004-6-7-6-7 2.808-7 6c0 1.468.617 2.83 1.678 3.894m-.493 3.905a22 22 0 0 1-.713.129c-.2.032-.352-.176-.273-.362a10 10 0 0 0 .244-.637l.003-.01c.248-.72.45-1.548.524-2.319C.743 11.37 0 9.76 0 8c0-3.866 3.582-7 8-7s8 3.134 8 7-3.582 7-8 7a9 9 0 0 1-2.347-.306c-.52.263-1.639.742-3.468 1.105'/%3e%3cpath d='M4 5.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8m0 2.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 1-.5-.5'/%3e%3c/svg%3e",
            width: 15,
            height: 15,
            alt: "#"
        },
        {
            type: "field_variable",
            name: "VAR",
            variable: "text",
            //"variable": "String",
            //"variableTypes": ["String"]
        }
    ],
    output: "String",
    tooltip: "Texto armazenado nesta variável.",
    helpUrl: ""
}
variableBlocks["variables_get_string"] = variablesGetString;
