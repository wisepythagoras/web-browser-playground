(async () => {
    const resp = await fetch('https://httpbin.org/ip', {
        method: 'GET',
    });
    console.log('Response', resp);

    // await fetch('/', {
    //     method: 'POST',
    // });
})();

(async () => {
    const response = new Response('{"a":2}');
    const json = await response.json();

    console.log(json.a);
})();

(async () => {
    // let fetch2 = fetch
    const res = await fetch2('https://httpbin.org/ip');
    const json = await res.json();
    console.log('res =', json.origin);
})();

console.log('I am right here');
