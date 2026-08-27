CodeMirror.defineMode('sml', () => ({
  startState: () => ({ comment: 0, string: false }),
  token(stream, state) {
    if (state.comment) {
      while (!stream.eol()) {
        if (stream.match('(*')) state.comment++;
        else if (stream.match('*)')) { if (--state.comment === 0) break; }
        else stream.next();
      }
      return 'comment';
    }
    if (state.string) {
      while (!stream.eol()) {
        const ch = stream.next();
        if (ch === '\\') stream.next();
        else if (ch === '"') { state.string = false; break; }
      }
      return 'string';
    }
    if (stream.eatSpace()) return null;
    if (stream.match('(*')) { state.comment = 1; return this.token(stream, state); }
    if (stream.match(/#?"/)) { state.string = true; return this.token(stream, state); }
    if (stream.match(/~?(?:0wx[\da-fA-F]+|0w\d+|0x[\da-fA-F]+|\d+(?:\.\d+)?(?:[eE]~?\d+)?)/)) return 'number';
    if (stream.match(/'[a-zA-Z][\w']*/)) return 'variable-2';
    if (stream.match(/[a-zA-Z_][\w'.]*/)) {
      return /^(?:abstype|and|andalso|as|case|datatype|do|else|end|eqtype|exception|fn|fun|functor|handle|if|in|include|infix|infixr|let|local|nonfix|of|op|open|orelse|raise|rec|sharing|sig|signature|struct|structure|then|type|val|where|while|with|withtype)$/.test(stream.current()) ? 'keyword' : /^(?:true|false|nil|NONE|SOME)$/.test(stream.current()) ? 'atom' : 'variable';
    }
    if (stream.match(/[!%&$#+\-/:<=>?@\\~`^|*]+/)) return 'operator';
    stream.next();
    return null;
  }
}));
