(async () => {
    const resp = await fetch('https://httpbin.org/ip', {
        method: 'GET',
    });
    console.log('Response', resp);

    await fetch('/', {
        method: 'POST',
    });
})();

(async () => {
    const response = new Response('{"a":2}');
    const json = await response.json();

    console.log(json.a);
})();
