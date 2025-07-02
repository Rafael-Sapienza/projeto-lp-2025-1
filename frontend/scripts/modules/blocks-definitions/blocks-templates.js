export const numberTemplate = {
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

export const textTemplate = {
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
