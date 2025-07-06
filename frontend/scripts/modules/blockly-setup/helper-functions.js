export const validateBlock = function() {
    return;
}

export const addContentWithCategory = function(block, content, contents) {
    let category = contents.find( obj =>  obj?.["name"] === block.category["name"].toUpperCase()) || null;

    if (!category) {
        contents.push({
            kind: "category",
            name: block.category["name"].toUpperCase(),
            colour: block.category["colour"],
            contents: [content],
        })
    }
    else {
        category.contents.push(content);
    }
}


// TRUSTING THAT THERE WILL BE ONLY EXPECTED TYPES
export const getVariablesByType = function(variablesArray) {
    const variablesByType = {};
    variablesArray.forEach(variable => {
        if (variable) {
            if (!(variable.type in variablesByType)) {
                variablesByType[variable.type] = [];
            }
            variablesByType[variable.type].push(variable);
        }
    });
    
    for (const type in variablesByType) {
        variablesByType[type].sort((a,b) => a.name.localeCompare(b.name));
    }

    return variablesByType;
}


export const createSetVariableBlock = function(variableType, variablesArray) {
    const setBlock = Blockly.utils.xml.createElement("block");

    const type = variableType === "Number" ? "variables_set_number" : "variables_set_string";
    setBlock.setAttribute("type", type);
    setBlock.appendChild(Blockly.Variables.generateVariableFieldDom(variablesArray[0]));

    const name = variableType === "Number" ? "NUM" : "TEXT";
    const valueElement = Blockly.utils.xml.createElement("value");
    valueElement.setAttribute("name", name);

    const fieldElement = Blockly.utils.xml.createElement("field");
    fieldElement.setAttribute("name", name);

    const shadow = variableType === "Number" ? "number_shadow" : "text_shadow";
    const shadowBlock = Blockly.utils.xml.createElement("shadow");
    shadowBlock.setAttribute("type", shadow);

    shadowBlock.appendChild(fieldElement);
    valueElement.appendChild(shadowBlock);
    setBlock.appendChild(valueElement);
    return setBlock;
}

export const createGetVariableBlock = function(variable) {
    const getBlock = Blockly.utils.xml.createElement("block");

    const blockType = variable.type === "Number" ? "variables_get_number" : "variables_get_string";
    getBlock.setAttribute("type", blockType);
    getBlock.appendChild(Blockly.Variables.generateVariableFieldDom(variable));

    return getBlock;
}
