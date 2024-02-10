const res = JSON.parse(`{
    "k": 123,
    "test": "this is a test string",
    "nested": {
        "a": 456,
        "arr": [1, 2, 4, {}]
    }
}`);

console.log(res.k, res.test);
console.log(res.nested.a);

for (const v of res.nested.arr) {
    console.log(v);
}
