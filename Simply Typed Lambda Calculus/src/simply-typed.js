/*
 * Generated by PEG.js 0.10.0.
 *
 * http://pegjs.org/
 */

"use strict";

function peg$subclass(child, parent) {
  function ctor() { this.constructor = child; }
  ctor.prototype = parent.prototype;
  child.prototype = new ctor();
}

function peg$SyntaxError(message, expected, found, location) {
  this.message  = message;
  this.expected = expected;
  this.found    = found;
  this.location = location;
  this.name     = "SyntaxError";

  if (typeof Error.captureStackTrace === "function") {
    Error.captureStackTrace(this, peg$SyntaxError);
  }
}

peg$subclass(peg$SyntaxError, Error);

peg$SyntaxError.buildMessage = function(expected, found) {
  var DESCRIBE_EXPECTATION_FNS = {
        literal: function(expectation) {
          return "\"" + literalEscape(expectation.text) + "\"";
        },

        "class": function(expectation) {
          var escapedParts = "",
              i;

          for (i = 0; i < expectation.parts.length; i++) {
            escapedParts += expectation.parts[i] instanceof Array
              ? classEscape(expectation.parts[i][0]) + "-" + classEscape(expectation.parts[i][1])
              : classEscape(expectation.parts[i]);
          }

          return "[" + (expectation.inverted ? "^" : "") + escapedParts + "]";
        },

        any: function(expectation) {
          return "any character";
        },

        end: function(expectation) {
          return "end of input";
        },

        other: function(expectation) {
          return expectation.description;
        }
      };

  function hex(ch) {
    return ch.charCodeAt(0).toString(16).toUpperCase();
  }

  function literalEscape(s) {
    return s
      .replace(/\\/g, '\\\\')
      .replace(/"/g,  '\\"')
      .replace(/\0/g, '\\0')
      .replace(/\t/g, '\\t')
      .replace(/\n/g, '\\n')
      .replace(/\r/g, '\\r')
      .replace(/[\x00-\x0F]/g,          function(ch) { return '\\x0' + hex(ch); })
      .replace(/[\x10-\x1F\x7F-\x9F]/g, function(ch) { return '\\x'  + hex(ch); });
  }

  function classEscape(s) {
    return s
      .replace(/\\/g, '\\\\')
      .replace(/\]/g, '\\]')
      .replace(/\^/g, '\\^')
      .replace(/-/g,  '\\-')
      .replace(/\0/g, '\\0')
      .replace(/\t/g, '\\t')
      .replace(/\n/g, '\\n')
      .replace(/\r/g, '\\r')
      .replace(/[\x00-\x0F]/g,          function(ch) { return '\\x0' + hex(ch); })
      .replace(/[\x10-\x1F\x7F-\x9F]/g, function(ch) { return '\\x'  + hex(ch); });
  }

  function describeExpectation(expectation) {
    return DESCRIBE_EXPECTATION_FNS[expectation.type](expectation);
  }

  function describeExpected(expected) {
    var descriptions = new Array(expected.length),
        i, j;

    for (i = 0; i < expected.length; i++) {
      descriptions[i] = describeExpectation(expected[i]);
    }

    descriptions.sort();

    if (descriptions.length > 0) {
      for (i = 1, j = 1; i < descriptions.length; i++) {
        if (descriptions[i - 1] !== descriptions[i]) {
          descriptions[j] = descriptions[i];
          j++;
        }
      }
      descriptions.length = j;
    }

    switch (descriptions.length) {
      case 1:
        return descriptions[0];

      case 2:
        return descriptions[0] + " or " + descriptions[1];

      default:
        return descriptions.slice(0, -1).join(", ")
          + ", or "
          + descriptions[descriptions.length - 1];
    }
  }

  function describeFound(found) {
    return found ? "\"" + literalEscape(found) + "\"" : "end of input";
  }

  return "Expected " + describeExpected(expected) + " but " + describeFound(found) + " found.";
};

function peg$parse(input, options) {
  options = options !== void 0 ? options : {};

  var peg$FAILED = {},

      peg$startRuleFunctions = { Program: peg$parseProgram },
      peg$startRuleFunction  = peg$parseProgram,

      peg$c0 = "(",
      peg$c1 = peg$literalExpectation("(", false),
      peg$c2 = ")",
      peg$c3 = peg$literalExpectation(")", false),
      peg$c4 = function(a) {
        return a;
      },
      peg$c5 = function(l, r) {
        if (!r || !r.length) {
          return l;
        } else {
          r = r.pop();
          return { type: 'application', left: l, right: r};
        }
      },
      peg$c6 = "\u03BB",
      peg$c7 = peg$literalExpectation("\u03BB", false),
      peg$c8 = ":",
      peg$c9 = peg$literalExpectation(":", false),
      peg$c10 = "\u2192",
      peg$c11 = peg$literalExpectation("\u2192", false),
      peg$c12 = function(id, t, f) {
        return { type: 'abstraction', arg: { type: t, id: id }, body: f };
      },
      peg$c13 = function(o, e) {
        return { type: 'arithmetic', operator: o, expression: e };
      },
      peg$c14 = function(e) {
        return { type: 'is_zero', expression: e };
      },
      peg$c15 = function(expr, then, el) {
        return { type: 'conditional_expression', condition: expr, then: then, el: el };
      },
      peg$c16 = /^[ \t\r\n]/,
      peg$c17 = peg$classExpectation([" ", "\t", "\r", "\n"], false, false),
      peg$c18 = /^[a-z]/,
      peg$c19 = peg$classExpectation([["a", "z"]], false, false),
      peg$c20 = function(id) {
        return { name: id.join(''), type: 'identifier' };
      },
      peg$c21 = function(expr) {
        return expr;
      },
      peg$c22 = "true",
      peg$c23 = peg$literalExpectation("true", false),
      peg$c24 = function() {
        return { type: 'literal', value: true };
      },
      peg$c25 = "false",
      peg$c26 = peg$literalExpectation("false", false),
      peg$c27 = function() {
        return { type: 'literal', value: false };
      },
      peg$c28 = "0",
      peg$c29 = peg$literalExpectation("0", false),
      peg$c30 = function() {
        return { type: 'literal', value: 0 };
      },
      peg$c31 = "if",
      peg$c32 = peg$literalExpectation("if", false),
      peg$c33 = "then",
      peg$c34 = peg$literalExpectation("then", false),
      peg$c35 = "else",
      peg$c36 = peg$literalExpectation("else", false),
      peg$c37 = "pred",
      peg$c38 = peg$literalExpectation("pred", false),
      peg$c39 = function() {
        return 'pred';
      },
      peg$c40 = "succ",
      peg$c41 = peg$literalExpectation("succ", false),
      peg$c42 = function() {
        return 'succ';
      },
      peg$c43 = "Nat",
      peg$c44 = peg$literalExpectation("Nat", false),
      peg$c45 = function() {
        return 'Nat';
      },
      peg$c46 = "Bool",
      peg$c47 = peg$literalExpectation("Bool", false),
      peg$c48 = function() {
        return 'Bool';
      },
      peg$c49 = "iszero",
      peg$c50 = peg$literalExpectation("iszero", false),
      peg$c51 = function() {
        return 'iszero';
      },

      peg$currPos          = 0,
      peg$savedPos         = 0,
      peg$posDetailsCache  = [{ line: 1, column: 1 }],
      peg$maxFailPos       = 0,
      peg$maxFailExpected  = [],
      peg$silentFails      = 0,

      peg$result;

  if ("startRule" in options) {
    if (!(options.startRule in peg$startRuleFunctions)) {
      throw new Error("Can't start parsing from rule \"" + options.startRule + "\".");
    }

    peg$startRuleFunction = peg$startRuleFunctions[options.startRule];
  }

  function text() {
    return input.substring(peg$savedPos, peg$currPos);
  }

  function location() {
    return peg$computeLocation(peg$savedPos, peg$currPos);
  }

  function expected(description, location) {
    location = location !== void 0 ? location : peg$computeLocation(peg$savedPos, peg$currPos)

    throw peg$buildStructuredError(
      [peg$otherExpectation(description)],
      input.substring(peg$savedPos, peg$currPos),
      location
    );
  }

  function error(message, location) {
    location = location !== void 0 ? location : peg$computeLocation(peg$savedPos, peg$currPos)

    throw peg$buildSimpleError(message, location);
  }

  function peg$literalExpectation(text, ignoreCase) {
    return { type: "literal", text: text, ignoreCase: ignoreCase };
  }

  function peg$classExpectation(parts, inverted, ignoreCase) {
    return { type: "class", parts: parts, inverted: inverted, ignoreCase: ignoreCase };
  }

  function peg$anyExpectation() {
    return { type: "any" };
  }

  function peg$endExpectation() {
    return { type: "end" };
  }

  function peg$otherExpectation(description) {
    return { type: "other", description: description };
  }

  function peg$computePosDetails(pos) {
    var details = peg$posDetailsCache[pos], p;

    if (details) {
      return details;
    } else {
      p = pos - 1;
      while (!peg$posDetailsCache[p]) {
        p--;
      }

      details = peg$posDetailsCache[p];
      details = {
        line:   details.line,
        column: details.column
      };

      while (p < pos) {
        if (input.charCodeAt(p) === 10) {
          details.line++;
          details.column = 1;
        } else {
          details.column++;
        }

        p++;
      }

      peg$posDetailsCache[pos] = details;
      return details;
    }
  }

  function peg$computeLocation(startPos, endPos) {
    var startPosDetails = peg$computePosDetails(startPos),
        endPosDetails   = peg$computePosDetails(endPos);

    return {
      start: {
        offset: startPos,
        line:   startPosDetails.line,
        column: startPosDetails.column
      },
      end: {
        offset: endPos,
        line:   endPosDetails.line,
        column: endPosDetails.column
      }
    };
  }

  function peg$fail(expected) {
    if (peg$currPos < peg$maxFailPos) { return; }

    if (peg$currPos > peg$maxFailPos) {
      peg$maxFailPos = peg$currPos;
      peg$maxFailExpected = [];
    }

    peg$maxFailExpected.push(expected);
  }

  function peg$buildSimpleError(message, location) {
    return new peg$SyntaxError(message, null, null, location);
  }

  function peg$buildStructuredError(expected, found, location) {
    return new peg$SyntaxError(
      peg$SyntaxError.buildMessage(expected, found),
      expected,
      found,
      location
    );
  }

  function peg$parseProgram() {
    var s0, s1, s2, s3, s4, s5, s6, s7;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      s2 = [];
      if (input.charCodeAt(peg$currPos) === 40) {
        s3 = peg$c0;
        peg$currPos++;
      } else {
        s3 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c1); }
      }
      while (s3 !== peg$FAILED) {
        s2.push(s3);
        if (input.charCodeAt(peg$currPos) === 40) {
          s3 = peg$c0;
          peg$currPos++;
        } else {
          s3 = peg$FAILED;
          if (peg$silentFails === 0) { peg$fail(peg$c1); }
        }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          s4 = peg$parseApplication();
          if (s4 !== peg$FAILED) {
            s5 = peg$parse_();
            if (s5 !== peg$FAILED) {
              if (input.charCodeAt(peg$currPos) === 41) {
                s6 = peg$c2;
                peg$currPos++;
              } else {
                s6 = peg$FAILED;
                if (peg$silentFails === 0) { peg$fail(peg$c3); }
              }
              if (s6 === peg$FAILED) {
                s6 = null;
              }
              if (s6 !== peg$FAILED) {
                s7 = peg$parse_();
                if (s7 !== peg$FAILED) {
                  peg$savedPos = s0;
                  s1 = peg$c4(s4);
                  s0 = s1;
                } else {
                  peg$currPos = s0;
                  s0 = peg$FAILED;
                }
              } else {
                peg$currPos = s0;
                s0 = peg$FAILED;
              }
            } else {
              peg$currPos = s0;
              s0 = peg$FAILED;
            }
          } else {
            peg$currPos = s0;
            s0 = peg$FAILED;
          }
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseApplication() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parseExprAbs();
    if (s1 !== peg$FAILED) {
      s2 = [];
      s3 = peg$parseApplication();
      while (s3 !== peg$FAILED) {
        s2.push(s3);
        s3 = peg$parseApplication();
      }
      if (s2 !== peg$FAILED) {
        peg$savedPos = s0;
        s1 = peg$c5(s1, s2);
        s0 = s1;
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseExprAbs() {
    var s0;

    s0 = peg$parseExpr();
    if (s0 === peg$FAILED) {
      s0 = peg$parseAbstraction();
    }

    return s0;
  }

  function peg$parseAbstraction() {
    var s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      s2 = [];
      if (input.charCodeAt(peg$currPos) === 40) {
        s3 = peg$c0;
        peg$currPos++;
      } else {
        s3 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c1); }
      }
      while (s3 !== peg$FAILED) {
        s2.push(s3);
        if (input.charCodeAt(peg$currPos) === 40) {
          s3 = peg$c0;
          peg$currPos++;
        } else {
          s3 = peg$FAILED;
          if (peg$silentFails === 0) { peg$fail(peg$c1); }
        }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          if (input.charCodeAt(peg$currPos) === 955) {
            s4 = peg$c6;
            peg$currPos++;
          } else {
            s4 = peg$FAILED;
            if (peg$silentFails === 0) { peg$fail(peg$c7); }
          }
          if (s4 !== peg$FAILED) {
            s5 = peg$parse_();
            if (s5 !== peg$FAILED) {
              s6 = peg$parseIdentifier();
              if (s6 !== peg$FAILED) {
                if (input.charCodeAt(peg$currPos) === 58) {
                  s7 = peg$c8;
                  peg$currPos++;
                } else {
                  s7 = peg$FAILED;
                  if (peg$silentFails === 0) { peg$fail(peg$c9); }
                }
                if (s7 !== peg$FAILED) {
                  s8 = peg$parseType();
                  if (s8 !== peg$FAILED) {
                    if (input.charCodeAt(peg$currPos) === 8594) {
                      s9 = peg$c10;
                      peg$currPos++;
                    } else {
                      s9 = peg$FAILED;
                      if (peg$silentFails === 0) { peg$fail(peg$c11); }
                    }
                    if (s9 !== peg$FAILED) {
                      s10 = peg$parseApplication();
                      if (s10 !== peg$FAILED) {
                        s11 = peg$parse_();
                        if (s11 !== peg$FAILED) {
                          if (input.charCodeAt(peg$currPos) === 41) {
                            s12 = peg$c2;
                            peg$currPos++;
                          } else {
                            s12 = peg$FAILED;
                            if (peg$silentFails === 0) { peg$fail(peg$c3); }
                          }
                          if (s12 === peg$FAILED) {
                            s12 = null;
                          }
                          if (s12 !== peg$FAILED) {
                            s13 = peg$parse_();
                            if (s13 !== peg$FAILED) {
                              peg$savedPos = s0;
                              s1 = peg$c12(s6, s8, s10);
                              s0 = s1;
                            } else {
                              peg$currPos = s0;
                              s0 = peg$FAILED;
                            }
                          } else {
                            peg$currPos = s0;
                            s0 = peg$FAILED;
                          }
                        } else {
                          peg$currPos = s0;
                          s0 = peg$FAILED;
                        }
                      } else {
                        peg$currPos = s0;
                        s0 = peg$FAILED;
                      }
                    } else {
                      peg$currPos = s0;
                      s0 = peg$FAILED;
                    }
                  } else {
                    peg$currPos = s0;
                    s0 = peg$FAILED;
                  }
                } else {
                  peg$currPos = s0;
                  s0 = peg$FAILED;
                }
              } else {
                peg$currPos = s0;
                s0 = peg$FAILED;
              }
            } else {
              peg$currPos = s0;
              s0 = peg$FAILED;
            }
          } else {
            peg$currPos = s0;
            s0 = peg$FAILED;
          }
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseExpr() {
    var s0;

    s0 = peg$parseIfThen();
    if (s0 === peg$FAILED) {
      s0 = peg$parseIsZeroCheck();
      if (s0 === peg$FAILED) {
        s0 = peg$parseArithmeticOperation();
        if (s0 === peg$FAILED) {
          s0 = peg$parseZero();
          if (s0 === peg$FAILED) {
            s0 = peg$parseTrue();
            if (s0 === peg$FAILED) {
              s0 = peg$parseFalse();
              if (s0 === peg$FAILED) {
                s0 = peg$parseIdentifier();
                if (s0 === peg$FAILED) {
                  s0 = peg$parseParanExpression();
                }
              }
            }
          }
        }
      }
    }

    return s0;
  }

  function peg$parseArithmeticOperation() {
    var s0, s1, s2;

    s0 = peg$currPos;
    s1 = peg$parseOperator();
    if (s1 !== peg$FAILED) {
      s2 = peg$parseApplication();
      if (s2 !== peg$FAILED) {
        peg$savedPos = s0;
        s1 = peg$c13(s1, s2);
        s0 = s1;
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseIsZeroCheck() {
    var s0, s1, s2;

    s0 = peg$currPos;
    s1 = peg$parseIsZero();
    if (s1 !== peg$FAILED) {
      s2 = peg$parseApplication();
      if (s2 !== peg$FAILED) {
        peg$savedPos = s0;
        s1 = peg$c14(s2);
        s0 = s1;
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseOperator() {
    var s0;

    s0 = peg$parseSucc();
    if (s0 === peg$FAILED) {
      s0 = peg$parsePred();
    }

    return s0;
  }

  function peg$parseIfThen() {
    var s0, s1, s2, s3, s4, s5, s6;

    s0 = peg$currPos;
    s1 = peg$parseIf();
    if (s1 !== peg$FAILED) {
      s2 = peg$parseApplication();
      if (s2 !== peg$FAILED) {
        s3 = peg$parseThen();
        if (s3 !== peg$FAILED) {
          s4 = peg$parseApplication();
          if (s4 !== peg$FAILED) {
            s5 = peg$parseElse();
            if (s5 !== peg$FAILED) {
              s6 = peg$parseApplication();
              if (s6 !== peg$FAILED) {
                peg$savedPos = s0;
                s1 = peg$c15(s2, s4, s6);
                s0 = s1;
              } else {
                peg$currPos = s0;
                s0 = peg$FAILED;
              }
            } else {
              peg$currPos = s0;
              s0 = peg$FAILED;
            }
          } else {
            peg$currPos = s0;
            s0 = peg$FAILED;
          }
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseType() {
    var s0;

    s0 = peg$parseNat();
    if (s0 === peg$FAILED) {
      s0 = peg$parseBool();
    }

    return s0;
  }

  function peg$parse_() {
    var s0, s1;

    s0 = [];
    if (peg$c16.test(input.charAt(peg$currPos))) {
      s1 = input.charAt(peg$currPos);
      peg$currPos++;
    } else {
      s1 = peg$FAILED;
      if (peg$silentFails === 0) { peg$fail(peg$c17); }
    }
    while (s1 !== peg$FAILED) {
      s0.push(s1);
      if (peg$c16.test(input.charAt(peg$currPos))) {
        s1 = input.charAt(peg$currPos);
        peg$currPos++;
      } else {
        s1 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c17); }
      }
    }

    return s0;
  }

  function peg$parse__() {
    var s0, s1;

    s0 = [];
    if (peg$c16.test(input.charAt(peg$currPos))) {
      s1 = input.charAt(peg$currPos);
      peg$currPos++;
    } else {
      s1 = peg$FAILED;
      if (peg$silentFails === 0) { peg$fail(peg$c17); }
    }
    if (s1 !== peg$FAILED) {
      while (s1 !== peg$FAILED) {
        s0.push(s1);
        if (peg$c16.test(input.charAt(peg$currPos))) {
          s1 = input.charAt(peg$currPos);
          peg$currPos++;
        } else {
          s1 = peg$FAILED;
          if (peg$silentFails === 0) { peg$fail(peg$c17); }
        }
      }
    } else {
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseIdentifier() {
    var s0, s1, s2, s3, s4;

    s0 = peg$currPos;
    s1 = peg$currPos;
    peg$silentFails++;
    s2 = peg$parseReservedWord();
    peg$silentFails--;
    if (s2 === peg$FAILED) {
      s1 = void 0;
    } else {
      peg$currPos = s1;
      s1 = peg$FAILED;
    }
    if (s1 !== peg$FAILED) {
      s2 = peg$parse_();
      if (s2 !== peg$FAILED) {
        s3 = [];
        if (peg$c18.test(input.charAt(peg$currPos))) {
          s4 = input.charAt(peg$currPos);
          peg$currPos++;
        } else {
          s4 = peg$FAILED;
          if (peg$silentFails === 0) { peg$fail(peg$c19); }
        }
        if (s4 !== peg$FAILED) {
          while (s4 !== peg$FAILED) {
            s3.push(s4);
            if (peg$c18.test(input.charAt(peg$currPos))) {
              s4 = input.charAt(peg$currPos);
              peg$currPos++;
            } else {
              s4 = peg$FAILED;
              if (peg$silentFails === 0) { peg$fail(peg$c19); }
            }
          }
        } else {
          s3 = peg$FAILED;
        }
        if (s3 !== peg$FAILED) {
          s4 = peg$parse_();
          if (s4 !== peg$FAILED) {
            peg$savedPos = s0;
            s1 = peg$c20(s3);
            s0 = s1;
          } else {
            peg$currPos = s0;
            s0 = peg$FAILED;
          }
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseParanExpression() {
    var s0, s1, s2, s3, s4, s5, s6, s7;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.charCodeAt(peg$currPos) === 40) {
        s2 = peg$c0;
        peg$currPos++;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c1); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          s4 = peg$parseExpr();
          if (s4 !== peg$FAILED) {
            s5 = peg$parse_();
            if (s5 !== peg$FAILED) {
              if (input.charCodeAt(peg$currPos) === 41) {
                s6 = peg$c2;
                peg$currPos++;
              } else {
                s6 = peg$FAILED;
                if (peg$silentFails === 0) { peg$fail(peg$c3); }
              }
              if (s6 !== peg$FAILED) {
                s7 = peg$parse_();
                if (s7 !== peg$FAILED) {
                  peg$savedPos = s0;
                  s1 = peg$c21(s4);
                  s0 = s1;
                } else {
                  peg$currPos = s0;
                  s0 = peg$FAILED;
                }
              } else {
                peg$currPos = s0;
                s0 = peg$FAILED;
              }
            } else {
              peg$currPos = s0;
              s0 = peg$FAILED;
            }
          } else {
            peg$currPos = s0;
            s0 = peg$FAILED;
          }
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseTrue() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c22) {
        s2 = peg$c22;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c23); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c24();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseFalse() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 5) === peg$c25) {
        s2 = peg$c25;
        peg$currPos += 5;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c26); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c27();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseZero() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.charCodeAt(peg$currPos) === 48) {
        s2 = peg$c28;
        peg$currPos++;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c29); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c30();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseReservedWord() {
    var s0;

    s0 = peg$parseIf();
    if (s0 === peg$FAILED) {
      s0 = peg$parseThen();
      if (s0 === peg$FAILED) {
        s0 = peg$parseElse();
        if (s0 === peg$FAILED) {
          s0 = peg$parsePred();
          if (s0 === peg$FAILED) {
            s0 = peg$parseSucc();
            if (s0 === peg$FAILED) {
              s0 = peg$parseNat();
              if (s0 === peg$FAILED) {
                s0 = peg$parseBool();
                if (s0 === peg$FAILED) {
                  s0 = peg$parseIsZero();
                  if (s0 === peg$FAILED) {
                    s0 = peg$parseFalse();
                  }
                }
              }
            }
          }
        }
      }
    }

    return s0;
  }

  function peg$parseIf() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 2) === peg$c31) {
        s2 = peg$c31;
        peg$currPos += 2;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c32); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          s1 = [s1, s2, s3];
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseThen() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c33) {
        s2 = peg$c33;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c34); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          s1 = [s1, s2, s3];
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseElse() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c35) {
        s2 = peg$c35;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c36); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          s1 = [s1, s2, s3];
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parsePred() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c37) {
        s2 = peg$c37;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c38); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c39();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseSucc() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c40) {
        s2 = peg$c40;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c41); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c42();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseNat() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 3) === peg$c43) {
        s2 = peg$c43;
        peg$currPos += 3;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c44); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c45();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseBool() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 4) === peg$c46) {
        s2 = peg$c46;
        peg$currPos += 4;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c47); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c48();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  function peg$parseIsZero() {
    var s0, s1, s2, s3;

    s0 = peg$currPos;
    s1 = peg$parse_();
    if (s1 !== peg$FAILED) {
      if (input.substr(peg$currPos, 6) === peg$c49) {
        s2 = peg$c49;
        peg$currPos += 6;
      } else {
        s2 = peg$FAILED;
        if (peg$silentFails === 0) { peg$fail(peg$c50); }
      }
      if (s2 !== peg$FAILED) {
        s3 = peg$parse_();
        if (s3 !== peg$FAILED) {
          peg$savedPos = s0;
          s1 = peg$c51();
          s0 = s1;
        } else {
          peg$currPos = s0;
          s0 = peg$FAILED;
        }
      } else {
        peg$currPos = s0;
        s0 = peg$FAILED;
      }
    } else {
      peg$currPos = s0;
      s0 = peg$FAILED;
    }

    return s0;
  }

  peg$result = peg$startRuleFunction();

  if (peg$result !== peg$FAILED && peg$currPos === input.length) {
    return peg$result;
  } else {
    if (peg$result !== peg$FAILED && peg$currPos < input.length) {
      peg$fail(peg$endExpectation());
    }

    throw peg$buildStructuredError(
      peg$maxFailExpected,
      peg$maxFailPos < input.length ? input.charAt(peg$maxFailPos) : null,
      peg$maxFailPos < input.length
        ? peg$computeLocation(peg$maxFailPos, peg$maxFailPos + 1)
        : peg$computeLocation(peg$maxFailPos, peg$maxFailPos)
    );
  }
}

module.exports = {
  SyntaxError: peg$SyntaxError,
  parse:       peg$parse
};