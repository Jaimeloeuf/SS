module.exports = {
  root: true,
  env: {
    node: true,
    es6: true,
  },
  parserOptions: {
    ecmaVersion: 2017,
  },
  extends: ["eslint:recommended"],
  rules: {
    "comma-dangle": ["error", "only-multiline"],
  },
};
