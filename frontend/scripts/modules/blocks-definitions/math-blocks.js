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
mathBlocks.bigger = bigger;

const greaterEqual = {
    type: "greater_equal",
    category,
    colour: category.colour,
    message0: "%1 ≥ %2", // or "maior ou igual a"
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
    tooltip: "Verdadeiro se o número 1 for maior ou igual ao número 2",
    shadow: {
        NUM1: { shadow: { type: "number_shadow" } },
        NUM2: { shadow: { type: "number_shadow" } },
    },
};
mathBlocks.greaterEqual = greaterEqual;

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
mathBlocks.smaller = smaller;

const lessEqual = {
    type: "less_equal",
    category,
    colour: category.colour,
    message0: "%1 ≤ %2", // or "menor ou igual a"
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
    tooltip: "Verdadeiro se o número 1 for menor ou igual ao número 2",
    shadow: {
        NUM1: { shadow: { type: "number_shadow" } },
        NUM2: { shadow: { type: "number_shadow" } },
    },
};
mathBlocks.lessEqual = lessEqual;

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
mathBlocks.equal = equal;



