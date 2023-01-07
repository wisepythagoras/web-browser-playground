const test = (arg1) => {
    console.log(!!arg1 ? arg1.x : 'No argument', arg1);

    if (!!arg1 && !!arg1.x) {
        return myfn(arg1.x, 'This is a test');
    }

    return NaN;
};

console.log(test());
console.log(test({ x: 123 }));
