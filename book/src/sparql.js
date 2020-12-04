/*
Language: SPARQL
Website: https://www.w3.org/TR/rdf-sparql-query/
Category: query
*/

hljsSparql = function(hljs) {
  var IDENT_RE = /[a-zA-Z_][a-zA-Z0-9_:]*/;
  var KEYWORDS =
    'BASE PREFIX ' +
    'SELECT CONSTRUCT DESCRIBE ASK ' +
    'ORDER BY ASC DESC LIMIT OFFSET REDUCED DISTINCT ' +
    'FROM NAMED WHERE ' +
    'GRAPH OPTIONAL UNION FILTER a ' +
    'STR LANG LANGMATCHES DATATYPE BOUND sameTERM ' +
    'iURI isIRI isBLANK isLITERAL REGEX ';
  return {
    name: 'SPARQL',
    case_insensitive: true,
    keywords: {
      keyword: KEYWORDS,
      literals: 'true false'
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
      // Variable
      {
        className: 'variable',
        begin: /\?[a-zA-Z_]+[a-zA-Z0-9_-]*/
      },
      // URI
      {
        className: 'meta',
        begin: /<[^>]+>/
      },
    ]
  };
}
