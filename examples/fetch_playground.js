(async () => {
    const resp = await fetch('https://httpbin.org/ip', {
        method: 'GET',
    });
    console.log('Response', resp);

    await fetch('/', {
        method: 'POST',
    });
})();
