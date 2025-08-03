// Rohit Chauhan: Method Chaining ES2022+ (Run on Bun!)

class Counter {
	
	// Private field
	#number = 0;
	
	plus() {
		this.#number++;
		return this;
	}
	
	minus() {
		this.#number--;
		return this;
	}
	
	multiply(n) {
		this.#number *= n;
		return this;
	}
	
	divide(n) {
		if(n === 0) {
			throw new Error("don't use zero");
		}
		this.#number /= n;
		return this;
	}
	
	get value() {
		return this.#number;
	}
	
	reset() {
		this.#number = 0;
		return this;
	}
	
	toString() {
		return this.#number.toString();
	}
	
	}
	
const counter = new Counter();

counter.plus().plus().plus().minus();
console.log(`${counter}`);

counter.plus().plus().multiply(8).divide(2);
console.log(`${counter}`);
