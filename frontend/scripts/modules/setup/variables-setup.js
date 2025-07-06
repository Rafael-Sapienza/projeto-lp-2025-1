import { createSetVariableBlock, getVariablesByType, createGetVariableBlock } from "./helper-functions.js";

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
    numberVarBtn.setAttribute("text", "criar caixa para números");
    numberVarBtn.setAttribute("callbackKey", "CREATE_VARIABLE_NUMBER");
    xmlList.push(numberVarBtn);

    const stringVarBtn = Blockly.utils.xml.createElement("button");
    stringVarBtn.setAttribute("text", "criar caixa para texto");
    stringVarBtn.setAttribute("callbackKey", "CREATE_VARIABLE_STRING");
    xmlList.push(stringVarBtn);

    // OLD const variableModels = workspace.getAllVariables();
    const variables = workspace.getVariableMap().getAllVariables();
    const variablesByType = getVariablesByType(variables);
    const orderedTypes = Object.keys(variablesByType).sort();

    for (const type of orderedTypes) {
        xmlList.push(createSetVariableBlock(type, variablesByType[type]));
    }

    for (const type of orderedTypes) {
        variablesByType[type].forEach(variable => {
            xmlList.push(createGetVariableBlock(variable));
        });
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
