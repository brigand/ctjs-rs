{
  const ctjs = Object.freeze({
    array: (items) => '[' + items.join(', ') + ']',
    vec: (items) => 'vec![' + items.join(', ') + ']',
    str: (text) => raw_string(text),
    f32: (number) => to_floating(number, 'f32'),
    f64: (number) => to_floating(number, 'f64'),
    float: (number) => to_floating(number, null),
    range: (start, end, step=1) => Array.from({length: (end - start) / step+1}, (_, i)=> start + i * step),
    json: (value) => `serde_json::json!(${JSON.stringify(value, null, 2)})`,
    parse_attrs: (attrs, allow = null) => {
      return parse_attrs(attrs || [], allow ? [].concat(allow) : null);
    },
  });

  const raw_string = (text) => {
    let raw = /[\\"\n\r\t]/.test(text);
    let matches = text.match(/"(#*)/g) || [];
    let close = matches.sort((a, b) => b.length - a.length)[0];
    let hash_count = close && close.length > 1 ? close.length : 1;
    let hash = '#'.repeat(hash_count);

    if (raw) {
      return `r${hash}"${text}"${hash}`
    } else {
      return `"${text}"`;
    }
  };

  const to_floating = (num, ty = null) => {
    let ns = String(num);
    if (!ns.includes('.')) {
      ns += '.0';
    }
    if (ty) {
      ns += '_' + ty;
    }
    return ns;
  };

  const get = (initial, [...parts]) => {
    let current = initial;

    if (current == null) {
      return undefined;
    }

    parts.every((part) => {
      if (current != null) {
        current = current[part];
        return current != null;
      }
    });

    return current;
  };

  const parse_attrs = (attrs, allow) => {
    const kv = Object.create(null);

    attrs.forEach(attr => {
      const segments = get(attr, ['path', 'segments']); 
      if (!segments || segments.length !== 1) {
        return;
      }

      const ident = get(attr, ['path', 'segments', 0, 'ident']);
      const match = allow ? allow.includes(ident) : true;
      if (!ident || !match) {
        return;
      }

      if (!attr.tokens) {
        return;
      }

      const group = get(attr, ['tokens', 0, 'group']);

      // match #[allowed_name(key = "value")
      if (!group || group.delimiter !== 'parenthesis') {
        return;
      }


      const key = get(group, ['stream', 0, 'ident']);
      const punct = get(group, ['stream', 1, 'punct', 'op']);
      const value = get(group, ['stream', 2, 'lit']);

      if (punct !== '=') {
        return;
      }

      kv[key] = value;
    });

    return kv;
  }

  this.ctjs = ctjs;
}
