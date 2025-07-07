export const shadowBlocks = {};

const numberShadow = {
    type: "number_shadow",
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
shadowBlocks["number_shadow"] = numberShadow;

const textShadow = {
    type: "text_shadow",
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
shadowBlocks["text_shadow"] = textShadow;
