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
  }

  this.ctjs = ctjs;
}
