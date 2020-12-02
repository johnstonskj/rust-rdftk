/*
Language: Turtle
Website: https://www.w3.org/TR/turtle/
Category: idl
*/

hljsTurtle = function(hljs) {
  var IDENT_RE = /[a-zA-Z_][a-zA-Z0-9_:]*/;
  var KEYWORDS =
    'prefix base a';
    ;
  return {
    name: 'Turtle',
    keywords: {
      keyword:
        KEYWORDS,
    },
    contains: [
      {
        className: 'comment',
        variants: [
           { begin: /#/, end: /$/ },
        ],
      },
      {
        className: 'string',
        variants: [
           { begin: /"/, end: /"(@[a-zA-Z]+[a-zA-Z0-9_-]*)?/ },
        ],
        contains: [hljs.BACKSLASH_ESCAPE]
      },
      {
        className: 'number',
        variants: [
          { begin: '\\b(\\d[\\d_]*(\\.[0-9_]+)?([eE][+-]?[0-9_]+)?)' }
        ],
        relevance: 0
      },
      // Data type
      {
        className: 'variable',
        variants: [
          { begin: /\^\^/, end: /[ \t\n\r]/ }
        ]
      },
      // URI
      {
        className: 'meta',
        begin: /<[^>]+>/
      },
    ]
  };
}

// module.exports = hljsSmithy;
