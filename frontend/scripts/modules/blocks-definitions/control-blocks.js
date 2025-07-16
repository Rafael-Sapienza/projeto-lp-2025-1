import { blocksCategories } from "./blocks-categories.js";

export const controlBlocks = {};

const category = blocksCategories.control;

const if_block = {
    type: "if",
    category,
    colour: category.colour,
    message0: "se %1",
    args0: [
        {
            type: "input_value",
            name: "CONDITION",
            check: "Boolean",
        }
    ],
    message1: "então %1",
    args1: [
        {
            type: "input_statement",
            name: "DO",
        }
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "O número 1 é maior que o número 2",
}
controlBlocks["if_block"] = if_block;


const if_else_block = {
    type: "if_else",
    category,
    colour: category.colour,
    message0: "se %1",
    args0: [
        {
            type: "input_value",
            name: "CONDITION",
            check: "Boolean"
        }
    ],
    message1: "então %1",
    args1: [
        {
            type: "input_statement",
            name: "DO"
        }
    ],
    message2: "senão %1",
    args2: [
        {
            type: "input_statement",
            name: "ELSE"
        }
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Executa um bloco se a condição for verdadeira, senão executa outro bloco",
};
controlBlocks["if_else_block"] = if_else_block;

const repeat_block = {
    type: "repeat",
    category,
    colour: category.colour,
    message0: "repetir %1 vezes",
    args0: [
        {
            type: "input_value",
            name: "TIMES",
            check: "Number",
        }
    ],
    message1: "%1",
    args1: [
        {
            type: "input_statement",
            name: "DO",
        }
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Repete o bloco de comandos o número de vezes indicado",
    shadow: {
        TIMES: {
            shadow: {
                type: "number_shadow",
            },
        },
    },
};
controlBlocks["repeat"] = repeat_block;

const while_block = {
    type: "while",
    category,
    colour: category.colour,
    message0: "repetir enquanto %1",
    args0: [
        {
            type: "input_value",
            name: "CONDITION",
            check: "Boolean",
        }
    ],
    message1: "%1",
    args1: [
        {
            type: "input_statement",
            name: "DO",
        }
    ],
    previousStatement: null,
    nextStatement: null,
    tooltip: "Repete enquanto a condição for verdadeira",
};
controlBlocks["while"] = while_block;
