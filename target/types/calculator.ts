export type Calculator = {
  "version": "0.1.0",
  "name": "calculator",
  "instructions": [
    {
      "name": "create",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "initMessage",
          "type": "string"
        }
      ]
    },
    {
      "name": "add",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "subtract",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "multiply",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "divide",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "power",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "calculator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "greeting",
            "type": "string"
          },
          {
            "name": "result",
            "type": "i128"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6001,
      "name": "DivisionByZero",
      "msg": "Division by zero"
    },
    {
      "code": 6002,
      "name": "OverflowUnderflow",
      "msg": "Overflow or underflow error"
    }
  ]
};

export const IDL: Calculator = {
  "version": "0.1.0",
  "name": "calculator",
  "instructions": [
    {
      "name": "create",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "user",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "initMessage",
          "type": "string"
        }
      ]
    },
    {
      "name": "add",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "subtract",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "multiply",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "divide",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    },
    {
      "name": "power",
      "accounts": [
        {
          "name": "calculator",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "num1",
          "type": "i128"
        },
        {
          "name": "num2",
          "type": "i128"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "calculator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "greeting",
            "type": "string"
          },
          {
            "name": "result",
            "type": "i128"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6001,
      "name": "DivisionByZero",
      "msg": "Division by zero"
    },
    {
      "code": 6002,
      "name": "OverflowUnderflow",
      "msg": "Overflow or underflow error"
    }
  ]
};
