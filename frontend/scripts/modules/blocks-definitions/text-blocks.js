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
            type: "input_value",
            name: "TEXT",
        },
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Essa mensagem vai aparecer na tela.",
    shadow: {
        TEXT: {
            shadow: {
                type: "text_shadow",
            }
        }
    }
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
            check: "String",
        },
        {
            type: "input_value",
            name: "TEXT2",
            check: "String",
        },
    ],
    output: "String",
    inputsInline: true,
    tooltip: "Junta dois textos.",
    shadow: {
        TEXT1: {
            shadow: {
                type: "text_shadow",
            },
        },
        TEXT2: {
            shadow: {
                type: "text_shadow",
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
                type: "text_shadow",
            }
        }
    }
};
textBlocks.length = length;

