// myfn2(123)
//     .then((a) => {
//         console.log('Result:', a);
//     })
//     .catch((e) => {
//         console.error('Error:', e);
//     });

console.log('-->', clipboard.writeText('dsa') instanceof Promise);
// clipboard.writeText('Lorem ipsum dolor sit amet, consectetur adipiscing elit')
//     .then(() => {
//         console.log('Copied');
//         clipboard.readText().then((data) => {
//             console.log(`Clipboard data: ${data}`);
//         });
//     })
//     .catch((e) => console.error(e))
//     .finally(() => console.log('Finally'));

document.getElementById('test');
console.log('Got it');
document.getElementById('a');
// document.getElementById();
console.log(Date.now());

(async () => {
    console.log('Here');
    const res = await myfn2(123);
    console.log('Result:', res);

    await clipboard.writeText('Lorem ipsum dolor sit amet, consectetur adipiscing elit');
    console.log('Copied text');
    
    try {
        const copiedText = await clipboard.readText();
        console.log(`Clipboard data: ${copiedText}`);
    } catch (e) {
        console.log('Exception:', e);
    }
})();
