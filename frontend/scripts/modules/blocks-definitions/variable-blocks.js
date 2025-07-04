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
    shadow: {
        NUM: {
            shadow: {
                type: "numberTemplate",
            },
        },
    }
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
    shadow: {
        TEXT: {
            shadow: {
                type: "textTemplate",
            },
        },
    }
};
variableBlocks["variables_set_string"] = variablesSetString;
