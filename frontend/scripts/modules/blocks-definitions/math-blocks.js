import { blocksCategories } from "./blocks-categories.js";

export const mathBlocks = {};

const category =  blocksCategories.math;

const sum = {
    type: "sum",
    category,
    colour: category.colour,
    message0: "%1 + %2",
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
    output: "Number",
    inputsInline: true,
    tooltip: "Soma de dois números",
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
mathBlocks.sum = sum;

const sub = {
    type: "sub",
    category,
    colour: category.colour,
    message0: "%1 - %2",
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
    output: "Number",
    inputsInline: true,
    tooltip: "Subtração de dois números",
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
mathBlocks.sub = sub;

const mult = {
    type: "mult",
    category,
    colour: category.colour,
    message0: "%1 x %2",
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
    output: "Number",
    inputsInline: true,
    tooltip: "Multiplicação de dois números",
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
mathBlocks.mult = mult;

const divi = {
    type: "divi",
    category,
    colour: category.colour,
    message0: "%1 ÷ %2",
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
    output: "Number",
    inputsInline: true,
    tooltip: "Divisão de dois números",
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
mathBlocks.divi = divi;





