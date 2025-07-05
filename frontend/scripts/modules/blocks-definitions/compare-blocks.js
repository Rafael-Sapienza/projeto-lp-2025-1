import { blocksCategories } from "./blocks-categories.js";

export const compareBlocks = {};

const category = blocksCategories.compare;

const bigger = {
    type: "bigger",
    category,
    colour: category.colour,
    message0: "%1 > %2",
    args0: [
        {
            type: "input_value",
            name: "NUM1",
            check: "Number",
        },
        {
            type: "input_value",
            name: "NUM2",
            check: "Number",
        },

    ],
    output: "Boolean",
    inputsInline: true,
    tooltip: "O número 1 é maior que o número 2",
    shadow: {
        NUM1: {
            shadow: {
                type: "number_shadow",
            },
        },
        NUM2: {
            shadow: {
                type: "number_shadow",
            },
        },
    },
}
compareBlocks.bigger = bigger;

const smaller = {
    type: "smaller",
    category,
    colour: category.colour,
    message0: "%1 < %2",
    args0: [
        {
            type: "input_value",
            name: "NUM1",
            check: "Number",
        },
        {
            type: "input_value",
            name: "NUM2",
            check: "Number",
        },

    ],
    output: "Boolean",
    inputsInline: true,
    tooltip: "O número 1 é menor que o número 2",
    shadow: {
        NUM1: {
            shadow: {
                type: "number_shadow",
            },
        },
        NUM2: {
            shadow: {
                type: "number_shadow",
            },
        },
    },
}
compareBlocks.smaller = smaller;

const equal = {
    type: "equal",
    category,
    colour: category.colour,
    message0: "%1 = %2",
    args0: [
        {
            type: "input_value",
            name: "NUM1",
            check: "Number",
        },
        {
            type: "input_value",
            name: "NUM2",
            check: "Number",
        },

    ],
    output: "Boolean",
    inputsInline: true,
    tooltip: "O número 1 é igual ao número 2",
    shadow: {
        NUM1: {
            shadow: {
                type: "number_shadow",
            },
        },
        NUM2: {
            shadow: {
                type: "number_shadow",
            },
        },
    },
}
compareBlocks.equal = equal;

