import { creatorDivHTML } from "../interface/function-creation.js";
import { createdFunctions } from "./functions-logic.js";
import { functionsCategory } from "../blocks-definitions/blocks-library.js";
import { createSetFunctionBlock } from "./helper-functions.js";

function createFunctionsFlyout(workspace) {
    const xmlList = [];

    const createBlockBtn = Blockly.utils.xml.createElement("button");
    createBlockBtn.setAttribute("text", "criar bloco");
    createBlockBtn.setAttribute("callbackKey", "CREATE_FUNCTION");
    xmlList.push(createBlockBtn);

    const deleteBlockBtn = Blockly.utils.xml.createElement("button");
    deleteBlockBtn.setAttribute("text", "apagar bloco");
    deleteBlockBtn.setAttribute("callbackKey", "DELETE_FUNCTION");
    xmlList.push(deleteBlockBtn);

     // Add a separator line
    const sep = Blockly.utils.xml.createElement("label");
    sep.setAttribute("text", "!-------------!");
    xmlList.push(sep);

    // Add SET function blocks to the flyout
    for (const functionName in createdFunctions) {

        xmlList.push(createSetFunctionBlock(functionName, createdFunctions[functionName].parameters));
        /*createdFunctions[functionName].parameters.forEach(parameter => {
            xmlList.push(//CREATE A FUNCTION TO GENERATE/RETURN THE BLOCKS);
        });*/
    }   
    return xmlList;
}


export const setupFunctionsToolbox = function(workspace) {
    // Register the flyout provider
    workspace.registerToolboxCategoryCallback(
        "functions_flyout",
        createFunctionsFlyout,
    );

    // Register the button callbacks
    workspace.registerButtonCallback("CREATE_FUNCTION", button => {
        showCreateFunctionWindow(blockDefinition => {
            const blockSuffix = blockDefinition.name.replace(/\s+/g, "_");
            defineFunctionBlocks(blockDefinition);
            renderFunctionBlocks(workspace, blockSuffix)
        });
    });

    workspace.registerButtonCallback("DELETE_FUNCTION", button => {
        console.log("implement block deletion");
    });
}


function showCreateFunctionWindow(callback) {
    const mainContainer = document.querySelector(".main-container");  
    const parentDiv = document.querySelector("#create-functions-layer");

    const creatorDiv = document.createElement("div");
    creatorDiv.id = "function-creator";
    creatorDiv.innerHTML = creatorDivHTML;
    parentDiv.appendChild(creatorDiv);
    toggleNodeActivation(mainContainer);

    // Cancel creation logic
    const cancelBtn = creatorDiv.querySelector("#cancel-creation");
    cancelBtn.addEventListener("click", () => {
        creatorDiv.remove();                        
        toggleNodeActivation(mainContainer);
    });

    // Handle parameters logic
    const parametersSelect = creatorDiv.querySelector("#function-parameters");
    const parametersContainer = creatorDiv.querySelector("#parameters-container");

    // Adding a new parameter
    parametersSelect.addEventListener("change", () => {
        const selectType = parametersSelect.value;
        if (selectType === "NONE") {
            return;
        }

        const parametersDiv = document.createElement("div");
        parametersDiv.classList.add("parameter-item");
        parametersDiv.innerHTML = `
            <label>
                <img src="${getTypeIconPath(selectType)}" class="parameter-icon"/>
            </label>
            <input type="text" data-type="${selectType}"/>
            <label>
                <img src="${getTypeIconPath("delete-parameter")}" class="delete-parameter"/>
            </label>
        `;
        // Add the parameter and reset the parameter input element
        parametersContainer.appendChild(parametersDiv);
        parametersSelect.value = "NONE";
    });

    // Delete parameter
    parametersContainer.addEventListener("click", (e) => {
        if (e.target.classList.contains("delete-parameter")) {
            e.target.closest(".parameter-item")?.remove();
        }
    });


    // Submission
    const createBtn = creatorDiv.querySelector("#confirm-creation");
    createBtn.addEventListener("click", e => {
        const functionNameInput = creatorDiv.querySelector("#function-name");
        const blockName = functionNameInput.value;
        if (!blockName || blockName in createdFunctions) {
            const placeholder = blockName ? "bloco já existe." : "dê um nome ao bloco.";
            highlighInvalidInput(functionNameInput, placeholder, 1000);
            return;
        }

        const returnSelect = creatorDiv.querySelector("#function-return");

        const blockDefinition = {
            name: blockName,
            parameters: [],
            return: returnSelect.value === "NONE" ? null : returnSelect.value,
            addedToWorkspace: null,
        };
        const functionParameters = parametersContainer.querySelectorAll(".parameter-item");
        for (const parameter of functionParameters) {
            const parameterInput = parameter.children[1];
            const parameterName = parameterInput.value;
            const parameterType = parameterInput.dataset.type;
            if (!parameterName || blockDefinition.parameters.some(param => param.name === parameterName)) {
                const placeholder = parameterName ? "nome repetido" : "caixa sem nome";
                highlighInvalidInput(parameterInput, placeholder, 1000);
                return;
            }
            blockDefinition.parameters.push({name: parameterName, type: parameterType});
        }
        createdFunctions[blockName] = blockDefinition;
        callback(blockDefinition);
        creatorDiv.remove();
        toggleNodeActivation(mainContainer);
    });
}


// Uses the user blockDefinition to build/define the new block on blockly
function defineFunctionBlocks(blockDefinition) {
    const argNames = blockDefinition.parameters.map((param, i) => `%${i + 1}`).join(", ");

    // DEF block definition
    const defBlockType = `function_def_${blockDefinition["name"].replace(/\s+/g, "_")}`;

    const defArgs0 = blockDefinition.parameters.map(param => ({
        type: "field_label",
        name: param.name,
        text: param.name,
        check: param.type
    }));
    const defMessage0 = `DEFINIR ${blockDefinition.name}${argNames.length > 0 ? ": " + argNames : ""}`;

    const defBlock = {
        type: defBlockType,
        message0: defMessage0,
        args0: defArgs0,
        message1: "%1",
        args1: [
            {
                type: "input_statement",
                name: "BODY"
            }
        ],
        colour: functionsCategory.colour,
        tooltip: `Definir o bloco ${blockDefinition["name"]}`,
        helpUrl: ""
    };

    if (blockDefinition.return) {
        defBlock.message2 = "DEVOLVER %1";
        defBlock.args2 = [
            {
                type: "input_value",
                name: "RETURN",
                check: blockDefinition.return,
            },
        ];
        defBlock.output = blockDefinition.return;
    }

    // SET block definition
    const setBlockType = `function_set_${blockDefinition["name"].replace(/\s+/g, "_")}`;

    const setArgs0 = blockDefinition.parameters.map(param => ({
        type: "input_value",
        name: param.name,
        text: param.name,
        check: param.type
    }));
    const setMessage0 = `CHAMAR ${blockDefinition.name}${argNames.length > 0 ? ": " + argNames : ""}`;

    const setBlock = {
        type: setBlockType,
        message0: setMessage0,
        args0: setArgs0,
        colour: functionsCategory.colour,
        tooltip: `Usar o bloco ${blockDefinition["name"]}`,
        helpUrl: ""
    };

    if (blockDefinition.return) {
        setBlock.output = blockDefinition.return;
    }
    else {
        setBlock.previousStatement = null;
        setBlock.nextStatement = null;
    }

    Blockly.defineBlocksWithJsonArray([defBlock, setBlock]);

    // GET blocks definitions (for every parameter)
    for (const param of blockDefinition.parameters) {
        const blockType = `function_param_get_${blockDefinition.name}_${param.name}`;
        const getBlock = {
            type: blockType,
            message0: `%1`,
            args0: [
                {
                    type: "field_label",
                    name: "PARAM_NAME",
                    text: param.name,
                },
            ],
            output: param.type,
            colour: functionsCategory.colour,
            tooltip: `Usa o parâmetro '${param.name}' da função '${blockDefinition.name}'`,
            helpUrl: ""
        };

        Blockly.defineBlocksWithJsonArray([getBlock]);
    }

}

function renderFunctionBlocks(workspace, blockSuffix) {
    const defBlock = workspace.newBlock(`function_def_${blockSuffix}`);
    defBlock.initSvg();
    defBlock.render();
    defBlock.setDeletable(false);

    const blockReturn = defBlock.getInput("RETURN");
    if (blockReturn && blockReturn.connection && !blockReturn.connection.isConnected()) {
        const shadows = {
            Number: "number_shadow",
            String: "text_shadow",
        }
        const shadow = shadows[blockReturn.connection.check?.find( returnType =>  shadows[returnType])];
        console.log(shadow)
        if (shadow) {
            const shadowBlock = workspace.newBlock(shadow);
            shadowBlock.setShadow(true);
            shadowBlock.initSvg();
            shadowBlock.render();
            blockReturn.connection.connect(shadowBlock.outputConnection);
        }
    }
}

/************ HELPERS *****************/
export const toggleNodeActivation = function(node) {
    if (!node) {
        return;
    }
    node.classList.toggle("blur");
}

export const getTypeIconPath = function(type) {
    if (type === "String") {
        return "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M2.678 11.894a1 1 0 0 1 .287.801 11 11 0 0 1-.398 2c1.395-.323 2.247-.697 2.634-.893a1 1 0 0 1 .71-.074A8 8 0 0 0 8 14c3.996 0 7-2.807 7-6s-3.004-6-7-6-7 2.808-7 6c0 1.468.617 2.83 1.678 3.894m-.493 3.905a22 22 0 0 1-.713.129c-.2.032-.352-.176-.273-.362a10 10 0 0 0 .244-.637l.003-.01c.248-.72.45-1.548.524-2.319C.743 11.37 0 9.76 0 8c0-3.866 3.582-7 8-7s8 3.134 8 7-3.582 7-8 7a9 9 0 0 1-2.347-.306c-.52.263-1.639.742-3.468 1.105'/%3e%3cpath d='M4 5.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7a.5.5 0 0 1-.5-.5M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8m0 2.5a.5.5 0 0 1 .5-.5h4a.5.5 0 0 1 0 1h-4a.5.5 0 0 1-.5-.5'/%3e%3c/svg%3e";
    }
    else if (type === "Number") {
        return "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M12 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM4 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z'/%3e%3cpath d='M4 2.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5zm0 4a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm3-6a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm3-6a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5zm0 3a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v4a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5z'/%3e%3c/svg%3e";
    }
    else if (type === "Boolean") {
        return "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z'/%3e%3cpath d='M10.97 4.97a.75.75 0 0 1 1.071 1.05l-3.992 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425z'/%3e%3c/svg%3e";
    }
    else if (type === "delete-parameter") {
        return "data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='%23664FC2' viewBox='0 0 16 16'%3e%3cpath d='M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z'/%3e%3cpath d='M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z'/%3e%3c/svg%3e";
    }
    else {
        return " ";
    }
}

export const highlighInvalidInput = function(inputNode, message, time) {
    inputNode.value = "";
    inputNode.classList.add("invalid-function-name");
    inputNode.placeholder = message;

    setTimeout(() => {
        inputNode.classList.remove("invalid-function-name");
        inputNode.placeholder = "";
    }, time);
}

