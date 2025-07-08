import { createdFunctions } from "./functions-logic.js";

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

    return xmlList;
}


export const setupFunctionsToolbox = function(workspace) {
    // Register the flyout provider
    workspace.registerToolboxCategoryCallback(
        "functions_flyout",
        createFunctionsFlyout,
    );

    // Register the button callbacks
    workspace.registerButtonCallback("CREATE_FUNCTION", function(button) {
        showCreateFunctionWindow(function(blockDefinition) {
            const block = createFunctionDefinitionBlock(blockDefinition);
            Blockly.defineBlocksWithJsonArray([block]);
            const newBlock = workspace.newBlock(`function_def_${blockDefinition.name.replace(/\s+/g, "_")}`);
            newBlock.initSvg();
            newBlock.render();
        });
    });

    workspace.registerButtonCallback("DELETE_FUNCTION", function(button) {
        console.log("implement block deletion");
    });
}


function showCreateFunctionWindow(callback) {
    const mainUI = document.querySelector(".main-container");  // use your actual UI wrapper
    const parentDiv = document.querySelector("#create-functions-layer");

    const creatorDiv = document.createElement("div");
    creatorDiv.id = "function-creator";
    creatorDiv.innerHTML = `
        <div class="form-content">
            <!-- your existing form here -->
            <div class="form-line">
                <label>Nome do Bloco:</label>
                <input id="function-name" type="text" />
            </div>
            <div class="form-line">
                <label>Adicionar Caixas:</label>
                <select id="function-parameters">
                    <option value="NONE">Opcional</option>
                    <option value="Number">Número</option>
                    <option value="String">Texto</option>
                    <option value="Boolean">Verdadeiro/Falso</option>
                </select>
            </div>
            <div id="parameters-container"></div>
            <div class="form-line">
                <label>Devolver:</label>
                <select id="function-return">
                    <option value="NONE">Opcional</option>
                    <option value="Number">Número</option>
                    <option value="String">Texto</option>
                    <option value="Boolean">Verdadeiro/Falso</option>
                </select>
            </div>
            <div class="form-line">
                <button id="cancel-creation">Cancelar</button>
                <button id="confirm-creation">Criar</button>
            </div>
        </div>
    `;
    parentDiv.appendChild(creatorDiv);

    // ➕ Blur background
    if (mainUI) mainUI.classList.add("blur");

    // 🗑️ Handle Cancel button
    const cancelBtn = creatorDiv.querySelector("#cancel-creation");
    cancelBtn.addEventListener("click", () => {
        creatorDiv.remove();                        // remove modal
        if (mainUI) mainUI.classList.remove("blur"); // unblur background
    });

    // ✨ Dynamic param handling
    const select = creatorDiv.querySelector("#function-parameters");
    const parametersContainer = creatorDiv.querySelector("#parameters-container");

    select.addEventListener("change", () => {
        const type = select.value;
        const name = select.options[select.selectedIndex].innerText;
        if (type === "NONE") return;

        const parametersDiv = document.createElement("div");
        //const parametersItems = parametersContainer.querySelectorAll(".parameter-item");
        //const parametersCount = parametersItems ? parametersItems.length + 1 : 1;
        const parametersCount = parametersContainer.querySelectorAll(".parameter-item").length + 1;        parametersDiv.classList.add("parameter-item");
        parametersDiv.innerHTML = `
            <label>## Caixa ${String(parametersCount).padStart(2, '0')} [${name}]:</label>
            <input type="text" data-type="${type}"/>
            <button class="delete-parameter">🗑️</button>
        `;
        parametersContainer.appendChild(parametersDiv);
        select.value = "NONE";
    });

    parametersContainer.addEventListener("click", (e) => {
        if (e.target.classList.contains("delete-parameter")) {
            console.log(e.target);
            console.log(e.target.parentElement)
            e.target.parentElement.remove();
        }
    });


    // Submission
    const createBtn = creatorDiv.querySelector("#confirm-creation");
    createBtn.addEventListener("click", e => {
        const nameInput = creatorDiv.querySelector("#function-name");
        const blockName = nameInput.value;
        if (!blockName || blockName in createdFunctions) {
            const placeholder = blockName ? "bloco já existe." : "dê um nome ao bloco.";
            nameInput.value = "";
            nameInput.classList.add("invalid-function-name");
            nameInput.placeholder = placeholder;

            setTimeout(() => {
            nameInput.classList.remove("invalid-function-name");
                nameInput.placeholder = "";
            }, 1000);
        }
        else {
            const blockDefinition = {
                name: null,
                parameters: [],
                return: null,
                addedToWorkspace: null,
            };
            const parametersItems = parametersContainer.querySelectorAll(".parameter-item");
            for (const parameter of parametersItems) {
                const parameterInput = parameter.children[1];
                const parameterName = parameterInput.value;
                const parameterType = parameterInput.dataset.type;
                if (!parameterName || blockDefinition.parameters.some(param => param.name === parameterName)) {
                    const placeholder = parameterName ? "nome repetido" : "caixa sem nome";
                    parameterInput.value = "";
                    parameterInput.classList.add("invalid-function-name");
                    parameterInput.placeholder = placeholder;
                    setTimeout(() => {
                        parameterInput.classList.remove("invalid-function-name");
                        parameterInput.placeholder = "";
                    }, 1000);
                    return;
                }
                const returnSelect = creatorDiv.querySelector("#function-return");
                blockDefinition.parameters.push({name: parameterName, type: parameterType});
                blockDefinition.return = returnSelect.value === "NONE" ? null : returnSelect.value;
            }
            console.log(blockDefinition);
            blockDefinition.name = blockName;
            createdFunctions[blockName] = blockDefinition;
            callback(blockDefinition);
            creatorDiv.remove();
            if (mainUI) mainUI.classList.remove("blur");
        }
    });
}



function createFunctionDefinitionBlock(blockDefinition) {
    const blockType = `function_def_${blockDefinition["name"].replace(/\s+/g, "_")}`;

    const args0 = blockDefinition.parameters.map(param => ({
        type: "input_value",
        name: param.name,
        check: param.type
    }));

    const argNames = blockDefinition.parameters.map((param, i) => `%${i + 1}`).join(", ");
    const message0 = `definir ${blockDefinition.name} com ${argNames}`;

    const block = {
        type: blockType,
        message0,
        args0,
        message1: "faça %1",
        args1: [
            {
                type: "input_statement",
                name: "BODY"
            }
        ],
        colour: 230,
        tooltip: `Define function ${blockDefinition["name"]}`,
        helpUrl: ""
    };

    if (blockDefinition.return && blockDefinition.return !== "NONE") {
        block.output = blockDefinition.return;
    } else {
        block.previousStatement = null;
        block.nextStatement = null;
    }

    return block;
}
