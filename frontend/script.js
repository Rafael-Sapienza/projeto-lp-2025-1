/***** PRINT BLOCK *****/
Blockly.defineBlocksWithJsonArray([{
  "type": "print_block",
  "message0": "print %1",
  "args0": [
    {
      "type": "field_input",
      "name": "TEXT"
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
  "type": "assignement_block",
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
          { kind: "block", type: "print_block" },
          { kind: "block", 
            type: "if_else_block",
            inputs: {"CONDITION": {shadow:{type:"text"}}}
          },
          { kind: "block",
            type: "while_block",
            inputs:{"CONDITION": {shadow:{type:"text"}}}
          },
          { kind: "block",
            type: "assignement_block",
            inputs: {"EXPRESSION": {shadow:{type:"text"}}},
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
    const workspaceJson = Blockly.serialization.workspaces.save(workspace);
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
