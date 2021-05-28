module.exports.SymbolTableImpl = class SymbolTableImpl {
  constructor() {
    this.table = [];
  }
  push(scope) {
    this.table.push(scope);
  }
  lookup(x) {
    // Huh can liddat meh....
    for (let i = this.table.length - 1; i >= 0; i--) {
      const val = this.table[i].get(x);
      if (val !== undefined) return val;
    }

    return undefined;
  }
};

module.exports.Scope = class Scope {
  constructor() {
    // Key/Value pair of Name/Type pair
    this.map = {};
  }
  add(x, val) {
    this.map[x] = val;
  }
  get(x) {
    return this.map[x];
  }
};
