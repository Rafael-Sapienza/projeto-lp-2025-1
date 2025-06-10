// ========== TEXT ========== //
export const textBlocks = {};

/***** PRINT *****/
const print = {
    type: "print",
    category: "text",
    message0: "escrever na tela %1",
    args0: [
        {
            type: "field_input",
            name: "TEXT",
        },
    ],
    previousStatement: null,
    nextStatement: null,
    colour: "#5ba58c",
    tooltip: "Essa mensagem vai aparecer na tela."
};
textBlocks.print = print;
 
/***** JOIN *****/
const join = {
    type: "join",
    category: "text",
    message0: "juntar %1 com %2",
    args0: [
        {
            type: "input_value",
            name: "TEXT1",
        },
        {
            type: "input_value",
            name: "TEXT2",
        },
    ],
    output: "String",
    colour: "#5ba58c",
    inputsInline: true,
    tooltip: "Junta dois textos.",
};
textBlocks.join = join;

/***** LENGTH *****/
const length = {
    type: "length",
    category: "text",
    message0: "tamanho do texto %1",
    args0: [
        {
            type: "input_value",
            name: "TEXT",
            check: "String",
        },
    ],
    output: "String",
    colour: "#5ba58c",
};
textBlocks.length = length;
