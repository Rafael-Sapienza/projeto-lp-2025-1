export const shadowBlocks = {};

const numberShadow = {
    type: "numberTemplate",
    message0: "%1",
    args0: [
        {
            type: "field_number",
            name: "NUM",
        },
    ],
    output: "Number",
    colour: "#52689c",
};
shadowBlocks.numberTemplate = numberShadow;

const textShadow = {
    type: "textTemplate",
    message0: "%1",
    args0: [
        {
            type: "field_input",
            name: "TEXT",
        },
    ],
    output: "String",
    colour: "#52689c",
};
shadowBlocks.textTemplate = textShadow;
