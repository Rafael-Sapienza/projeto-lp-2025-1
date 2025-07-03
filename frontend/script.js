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


Blockly.defineBlocksWithJsonArray([{
  "type": "declaration_block",
  "message0": "tipo %1 %2",
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
  "colour": 230,
  "tooltip": "Seleciona um tipo",
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
  "colour": 160,
  "tooltip": "Prints output to the console",
  "helpUrl": ""
}]);

/***** ASSIGNMENT BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "assignment_block",
  "message0": "%1 = %2",
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
  "colour": 210,
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
  "colour": 210,
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
  "colour": 210,
  "inputsInline": true, 
  "tooltip": "while loop",
  "helpUrl": ""
}]);

// INITIALIZE BLOCKLY WORKSPACE
const workspace = Blockly.inject('blocklyDiv', {
  renderer: "zelos", // Prettier format
  trashcan: true,
  toolbox: {
    //kind: "flyoutToolbox",
    kind: "categoryToolbox",
    contents: [
      {
        kind: "category",
        name: "Custom Blocks",
        colour: "#5CA699",
        contents: [
          { kind: "block", 
            type: "print_block",
            inputs: {"EXPRESSION": {shadow:{type:"expression_block"}} }
          },
          { kind: "block", type: "declaration_block"},
          { kind: "block", 
            type: "if_else_block",
            inputs: {"CONDITION": {shadow:{type:"expression_block"}}}
          },
          { kind: "block",
            type: "while_block",
            inputs:{"CONDITION": {shadow:{type:"expression_block"}}}
          },
          { kind: "block",
            type: "assignment_block",
            inputs: {"EXPRESSION": {shadow:{type:"expression_block"}}},
          }
        ]
      },
      {
        kind: "category",
        name: "Logic",
        colour: "%{BKY_LOGIC_HUE}",
        contents: [
          { kind: "block", type: "logic_compare" },
          { kind: "block", type: "logic_boolean" }
        ]
      },
      {
        kind: "category",
        name: "Math",
        colour: "%{BKY_MATH_HUE}",
        contents: [
          { kind: "block", type: "math_number" }
        ]
      }
    ]
  }
});


async function execute() {
    const workspaceJson = Blockly.serialization.workspaces.save(workspace, {includeShadowBlocks: true});
    console.log(JSON.stringify(workspaceJson));
    try {
        const response = await fetch('/execute', {
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

    } catch (error) {
        console.error('Error:', error);
        document.getElementById('output').textContent = `Error: ${error}`;
    }
}
