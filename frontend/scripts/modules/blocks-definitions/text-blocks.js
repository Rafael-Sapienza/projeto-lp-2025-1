import { blocksCategories } from "./blocks-categories.js";

export const textBlocks = {};

// ========== TEXT ========== //
const category = blocksCategories.text;
/***** PRINT *****/
const print = {
    type: "print",
    category: category,
    colour: category.colour,
    message0: "escrever na tela %1",
    args0: [
        {
            type: "field_input",
            name: "TEXT",
        },
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Essa mensagem vai aparecer na tela."
};
textBlocks.print = print;
 
/***** JOIN *****/
const join = {
    type: "join",
    category: category,
    colour: category.colour,
    message0: "juntar %1 com %2",
    args0: [
        {
            type: "input_value",
            name: "TEXT1",
        },
        {
            type: "input_value",
            name: "TEXT2",
        },
    ],
    output: "String",
    inputsInline: true,
    tooltip: "Junta dois textos.",
    shadow: {
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
};
textBlocks.join = join;

/***** LENGTH *****/
const length = {
    type: "length",
    category: category,
    colour: category.colour,
    message0: "tamanho do texto %1",
    args0: [
        {
            type: "input_value",
            name: "TEXT",
            check: "String",
        },
    ],
    output: "Number",
    tooltip: "Retorna o tamanho de um texto.",
    shadow: {
        TEXT: {
            shadow: {
                type: "text",
            }
        }
    }
};
textBlocks.length = length;
