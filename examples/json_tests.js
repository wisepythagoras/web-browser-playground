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

console.log('---');

const a = JSON.stringify([123, {}]);
const b = JSON.stringify({
    a: 123,
    b: {
        c: [1, 2, 3, 4],
    }
});

console.log(a, b);
