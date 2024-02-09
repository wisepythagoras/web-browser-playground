let person = new Person('John', 19);
person.sayHello();

if (Person.is(person)) {
    console.log('person is a Person class instance.');
}
if (!Person.is('Hello')) {
    console.log('\'Hello\' string is not a Person class instance.');
}

console.log(Person.staticProperty);
console.log(person.inheritedProperty);
console.log(Person.prototype.inheritedProperty === person.inheritedProperty);

console.log('--->');
console.log(myPerson.sayHello());
