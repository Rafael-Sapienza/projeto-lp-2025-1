/*
Blockly.defineBlocksWithJsonArray([{
  "type": "text",
  "message0": "\"%1\"",
  "args0": [
    {
      "type": "field_input",
      "name": "TEXT",
      "text": ""
    }
  ],
  "output": "String",
  "colour": 160,
  "tooltip": "A string of text.",
  "helpUrl": "https://www.example.com"
}]);
*/

/***** EXPRESSION BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "expression_block",
  "message0": "%1",
  "args0": [
    {
      "type": "field_input",
      "name": "TEXT",
      "text": ""
    }
  ],
  "output": "String",
  "colour": 160,
  "tooltip": "A string of text.",
}]);

/***** VARIABLE DECLARATION BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "declaration_block",
  "message0": "declare %1 %2",
  "args0": [
    {
      "type": "field_dropdown",
      "name": "TYPE",
      "options": [
        ["int", "INT"],
        ["float", "FLOAT"],
        ["string", "STRING"],
        ["bool", "BOOL"]
      ]
    },
    {
      "type": "field_input",
      "name": "VARIABLE",
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "output": null,
  "colour": '#547792',
  "tooltip": "Seleciona um tipo",
  "helpUrl": ""
}]);

/***** FUNCTION DECLARATION BLOCK*****/
Blockly.defineBlocksWithJsonArray([{
  "type": "function_declaration_block",
  "message0": "def %1 %2 (%3) %4 return %5",
  "args0": [
    {
      "type": "field_dropdown",
      "name": "RETURN_TYPE",
      "options": [
        ["void","VOID"],
        ["int", "INT"],
        ["float", "FLOAT"],
        ["string", "STRING"],
        ["bool", "BOOL"]
      ]
    },
    {
      "type": "field_input",
      "name": "FUNCTION_NAME",
    },
    {
      "type": "input_value",
      "name": "FORMAL_ARGUMENTS",
      "check": "FormalArgumentList"
    },
    {
      "type": "input_statement",
      "name": "FUNCTION_BODY"
    },
    {
      "type": "input_value",
      "name": "RETURN_EXPRESSION"
    }
  ],
  "colour": '#40679E',
  "tooltip": "If-Else conditional",
  "helpUrl": ""
}]);

/***** FORMAL ARGUMENTS BLOCK*****/
Blockly.defineBlocksWithJsonArray([{
  "type": "formal_argument_block",
  "message0": "add parameter %1 %2 , %3",
  "args0": [
    {
      "type": "field_dropdown",
      "name": "ARGUMENT_TYPE",
      "options": [
        ["int", "INT"],
        ["float", "FLOAT"],
        ["string", "STRING"],
        ["bool", "BOOL"]
      ]
    },
    {
      "type": "field_input",
      "name": "FORMAL_ARGUMENT",
    },
    {
      "type": "input_value",
      "name": "NEXT_ARGUMENT",
      "check": "FormalArgumentList"
    }
  ],
  "output": "FormalArgumentList",
  "outputShape": Blockly.OUTPUT_SHAPE_ROUND,
  "colour": '#40679E',
  "tooltip": "Function definition",
  "helpUrl": ""
}]);

/***** PRINT BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "print_block",
  "message0": "print %1",
  "args0": [
    {
      "type": "input_value", // antes era field_input
      "name": "EXPRESSION"
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": '#00909E',
  "tooltip": "Prints output to the console",
  "helpUrl": ""
}]);

/***** ASSIGNMENT BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "assignment_block",
  "message0": "assign %1 = %2",
  "args0": [
    {
      "type": "field_input",
      "name": "VARIABLE",
    },
    {
      "type": "input_value",
      "name": "EXPRESSION"
    },
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": '#547792',
  "tooltip": "Assignment command",
  "helpUrl": ""
}]);


/***** IF-ELSE BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "if_else_block",
  "message0": "if %1 then %2 else %3",
  "args0": [
    {
      //"type" : "field_input",
      "type": "input_value",
      "name": "CONDITION",

      //"check": "Boolean"
    },
    {
      "type": "input_statement",
      "name": "IF_BODY"
    },
    {
      "type": "input_statement",
      "name": "ELSE_BODY"
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": '#465881',
  "tooltip": "If-Else conditional",
  "helpUrl": ""
}]);


/***** WHILE BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "while_block",
  "message0": "while %1 do %2",
  "args0": [
    {
      "type" : "input_value",
      "name": "CONDITION",
    },
    {
      "type" : "input_statement",
      "name" : "WHILE_BODY" 
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": '#94B4C1',
  "inputsInline": true, 
  "tooltip": "while loop",
  "helpUrl": ""
}]);

/***** SINGLE FUNC CALL *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "sigle_func_call_block",
  "message0": "%1 (%2)",
  "args0": [
    {
      "type": "input_value",
      "name": "FUNC_NAME",
    },
    {
      "type": "input_value",
      "name": "ACTUAL_ARGS",
    }
  ],
  "previousStatement": null,
  "nextStatement": null,
  "colour": 210,
  "tooltip": "Single func call",
  "helpUrl": ""
}]);

// INITIALIZE BLOCKLY WORKSPACE
const workspace = Blockly.inject('blocklyDiv', {
  renderer: "zelos",
  trashcan: true,
  toolbox: {
    kind: "flyoutToolbox",  // flyout lateral
    contents: [
      { kind: "block", type: "declaration_block" },
      { kind: "block", 
        type: "assignment_block",
        inputs: {"EXPRESSION": {shadow:{type:"expression_block"}} }
      },
      { kind: "block", 
        type: "print_block",
        inputs: {"EXPRESSION": {shadow:{type:"expression_block"}} }
      },
      { kind: "block", 
        type: "if_else_block",
        inputs: {"CONDITION": {shadow:{type:"expression_block"}} }
      },
      { kind: "block", 
        type: "while_block",
        inputs: {"CONDITION": {shadow:{type:"expression_block"}} }
      },
      { kind: "block", 
        type: "function_declaration_block",
        inputs: {"RETURN_EXPRESSION": {shadow:{type:"expression_block"}} }
      },
      { kind: "block", type: "formal_argument_block" },
      { kind: "block", type: "sigle_func_call_block", inputs: {"FUNC_NAME": {shadow:{type:"expression_block"}}, "ACTUAL_ARGS": {shadow:{type:"expression_block"}}}}
    ]
  }
});


async function execute() {
    const workspaceJson = Blockly.serialization.workspaces.save(workspace, {includeShadowBlocks: true});
    console.log(JSON.stringify(workspaceJson));
    try {
        const response = await fetch('/hard-interpreter', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(workspaceJson)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const output = await response.json();
        console.log(output);

        // Update the <pre id="output">
        document.getElementById('output').textContent = `Output:\n${output.join('\n')}`;

        const outputElement = document.getElementById('output');
        outputElement.textContent = `Output:\n${output.join('\n')}`;

        outputElement.scrollIntoView({ behavior: 'smooth', block: 'start' });


    } catch (error) {
        console.error('Error:', error);
        document.getElementById('output').textContent = `Error: ${error}`;
    }
}

document.querySelector("#execute").addEventListener("click", execute);