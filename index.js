const test = (arg1) => {
    console.log(!!arg1 ? arg1.x : 'No argument', arg1);

    if (!!arg1 && !!arg1.x) {
        return myfn(arg1.x, 'This is a test');
    }

    return NaN;
};

const host = `${navigator.userAgent} - CPUs: ${navigator.hardwareConcurrency}`;

console.log(test());
console.log(test({ x: 123 }));
console.error(JSON.stringify(navigator.languages), host);
console.warn(function() {});
console.log(console.warn.toString());

// clipboard.writeText().catch((err) => console.error('Expected fail:', err));
// clipboard.writeText('Lorem ipsum dolor sit amet, consectetur adipiscing elit')
//     .then(() => {
//         console.log('Copied');
//         clipboard.readText().then((data) => {
//             console.log(`Clipboard data: ${data}`);
//         });
//     })
//     .catch((e) => console.error(e));

console.log('Here');
myfn2(234).then((r) => {
    console.log('Result: ', r);
}).catch((e) => console.error('Exception', e));
console.log(navigator);
console.log('myfn(123) =', myfn(123));
