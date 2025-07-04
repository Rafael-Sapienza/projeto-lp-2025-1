// Export a simple category object
export const variablesCategory = {
    kind: "category",
    name: "VARIÁVEIS",
    colour: "#5832a8",
    custom: "variables_flyout"
};

// Export a function that returns some blocks for the flyout
export function createVariablesFlyout(workspace) {
    const xmlList = [];

    const numberVarBtn = Blockly.utils.xml.createElement("button");
    numberVarBtn.setAttribute("text", "Criar variável Numérica");
    numberVarBtn.setAttribute("callbackKey", "CREATE_VARIABLE_NUMBER");
    xmlList.push(numberVarBtn);

    const stringVarBtn = Blockly.utils.xml.createElement("button");
    stringVarBtn.setAttribute("text", "Criar variável de Texto");
    stringVarBtn.setAttribute("callbackKey", "CREATE_VARIABLE_STRING");
    xmlList.push(stringVarBtn);

    const variableModels = workspace.getAllVariables();
    if (variableModels.length > 0) {
        // variables_set
        const setBlock = Blockly.utils.xml.createElement("block");
        setBlock.setAttribute("type", "variables_set_string");
        setBlock.appendChild(
            Blockly.Variables.generateVariableFieldDom(variableModels[0])
        );
        // <value name="VALUE"> with <shadow>
        const valueElem = Blockly.utils.xml.createElement("value");
        valueElem.setAttribute("name", "TEXT");

        const shadowBlock = Blockly.utils.xml.createElement("shadow");
        shadowBlock.setAttribute("type", "textTemplate"); // or "math_number" for numbers

        // Optionally, pre-fill the shadow block with a default value:
        const field = Blockly.utils.xml.createElement("field");
        field.setAttribute("name", "TEXT"); // "NUM" if it's math_number
        shadowBlock.appendChild(field);

        // Attach the shadow to the value
        valueElem.appendChild(shadowBlock);

        // Attach the value to the set block
        setBlock.appendChild(valueElem);
        xmlList.push(setBlock);
    }
    for (const variable of variableModels) {
        // variables_get
        const getBlock = Blockly.utils.xml.createElement("block");
        getBlock.setAttribute("type", "variables_get");
        getBlock.appendChild(
            Blockly.Variables.generateVariableFieldDom(variable)
        );
        xmlList.push(getBlock);

    }

    return xmlList;
}

export function setupVariablesToolbox(workspace) {
    // Register the flyout provider
    workspace.registerToolboxCategoryCallback(
        "variables_flyout",
        createVariablesFlyout
    );

    // Register the button callbacks
    workspace.registerButtonCallback("CREATE_VARIABLE_NUMBER", function(button) {
        Blockly.Variables.createVariableButtonHandler(button.getTargetWorkspace(), null, "Number");
    });

    workspace.registerButtonCallback("CREATE_VARIABLE_STRING", function(button) {
        Blockly.Variables.createVariableButtonHandler(button.getTargetWorkspace(), null, "String");
    });
}
