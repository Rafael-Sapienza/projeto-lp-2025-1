import { blocksCategories } from "./blocks-categories.js";

export const controlBlocks = {};

const category = blocksCategories.control;

const if_block = {
    type: "if",
    category,
    colour: category.colour,
    message0: "se %1 então",
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
    tooltip: "O número 1 é maior que o número 2",
}
controlBlocks["if_block"] = if_block;
