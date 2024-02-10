const res = JSON.parse(`{
    "k": 123,
    "test": "this is a test string",
    "nested": {
        "a": 456
    }
}`);

console.log(res.k, res.test);
console.log(res.nested.a);
