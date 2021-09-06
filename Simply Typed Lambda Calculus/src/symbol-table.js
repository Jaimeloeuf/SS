module.exports.SymbolTableImpl = class SymbolTableImpl {
  constructor() {
    this.table = [];
  }
  push(scope) {
    this.table.push(scope);
  }
  lookup(key) {
    // Go up the scopes from the current innermost scope to look for the value
    for (let i = this.table.length - 1; i >= 0; i--) {
      const val = this.table[i].get(key);
      if (val !== undefined) return val;
    }

    // @todo Should error out instead
    return undefined;
  }
};

module.exports.Scope = class Scope {
  constructor() {
    // Key/Value pair of Name/Type pair
    this.map = {};
  }
  add(key, val) {
    this.map[key] = val;
  }
  get(key) {
    return this.map[key];
  }
};
