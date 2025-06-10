Blockly.defineBlocksWithJsonArray([
  {
    "type": "print",
    "message0": "print %1",
    "args0": [{ "type": "field_number", "name": "TEXT" }],
    "previousStatement": null,
    "nextStatement": null,
    "colour": 120
  },
  {
    "type": "repeat",
    "message0": "repeat %1 times %2",
    "args0": [
      { "type": "field_number", "name": "TIMES", "value": 3 },
      { "type": "input_statement", "name": "DO" }
    ],
    "previousStatement": null,
    "nextStatement": null,
    "colour": 120
  },
  {
    "type": "my_if_else",
    "message0": "if %1 then %2 else %3",
    "args0": [
      { "type": "input_value", "name": "COND" },
      { "type": "input_statement", "name": "THEN" },
      { "type": "input_statement", "name": "ELSE" }
    ],
    "previousStatement": null,
    "nextStatement": null,
    "colour": 210
  },
    {
      "type": "add_numbers",
      "message0": "%1 + %2",
      "args0": [
        { "type": "input_value", "name": "A" },
        { "type": "input_value", "name": "B" }
      ],
      "output": "Number",
      "colour": 230
    },
    {
      "type": "set_variable",
      "message0": "Set %1 to %2",
      "args0": [
        {
          "type": "field_variable",
          "name": "VAR",
          "variable": "item"
        },
        {
          "type": "input_value",
          "name": "VALUE",
          "check": ["Number", "String"]
        }
      ],
      "previousStatement": null,
      "nextStatement": null,
      "colour": 230,
      "tooltip": "Set a variable to a value",
      "helpUrl": ""
    },
    {
      "type": "variables_get",
      "message0": "%1",
      "args0": [
        {
          "type": "field_variable",
          "name": "VAR",
          "variable": "item"
        }
      ],
      "output": null,
      "colour": 330,
      "tooltip": "Returns the value of a variable.",
      "helpUrl": ""
    },
]);


toolbox = {
    "kind": "flyoutToolbox",
    "contents":[
        {
            "kind": "block",
            "type": "print"
        },
        {
            "kind": "block",
            "type": "repeat"
        },
        {
            "kind": "block",
            "type": "my_if_else"
        },
        {
            "kind": "block",
            "type": "add_numbers"
        },
        {
            "kind": "block",
            "type": "set_variable"
        },
        {
            "kind": "block",
            "type": "variables_get"
        },
    ]
}

const workspace = Blockly.inject('blocklyDiv', {toolbox});

workspace.createVariable('myVar', '', '');

console.log(Blockly.Blocks["print"]);
console.log(Blockly.Blocks["print"].init);
//console.log(Blockly.Blocks["print"].init());
Blockly.Blocks["print"].init = function() {console.log("Print function")}
console.log(Blockly.Blocks["print"]);
console.log(Blockly.Blocks["print"].init);
console.log(Blockly.Blocks["print"].init());



function execute() {
    //console.log(JSON.stringify({code: blocksJson}));
    const payload = {
        "code":[
            {
                "type":"repeat",
                "times":3,
                "do":[
                    {
                        "type":"print",
                        "text":"hello"
                    }
                ]
            }
        ]
    }

    fetch("/execute", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload)
    })
    .then(res => res.text())
    .then(result => {
        document.getElementById("output").textContent = result;
    })
    .catch(err => {
        document.getElementById("output").textContent = "Erro:\n" + err;
    });
}
