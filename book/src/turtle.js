/*
Language: Turtle
Website: https://www.w3.org/TR/turtle/
Category: idl
*/

hljsTurtle = function(hljs) {
  var IDENT_RE = /[a-zA-Z_][a-zA-Z0-9_:]*/;
  var KEYWORDS =
    'prefix base a';
  return {
    name: 'Turtle',
    keywords: {
      keyword: KEYWORDS,
    },
    contains: [
      {
        className: 'comment',
        begin: /#/, end: /$/
      },
      {
        className: 'string',
        begin: /"/,
        end: /"((@[a-zA-Z]+[a-zA-Z0-9_\-]*)|(\^\^((<[^>]+>)|([a-zA-Z_]+[a-zA-Z0-9:_\-]*))))?/,
        contains: [hljs.BACKSLASH_ESCAPE]
      },
      {
        className: 'number',
        begin: '\\b(\\d[\\d_]*(\\.[0-9_]+)?([eE][+-]?[0-9_]+)?)',
        relevance: 0
      },
      // Known prefixes
      {
        className: 'title',
        begin: /(rdf|rdfs|xsd|owl):([a-zA-Z_]+[a-zA-Z0-9_-]*)?/
      },
      // URI
      {
        className: 'meta',
        begin: /<[^>]+>/
      },
    ]
  };
}
