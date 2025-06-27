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
                type: "numberTemplate",
            },
        },
        NUM2: {
            shadow: {
                type: "numberTemplate",
            },
        },
    },
}
mathBlocks.sum = sum;


