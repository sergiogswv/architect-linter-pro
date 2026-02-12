/**
 * This file contains extremely long functions
 * to test the complexity scoring component
 */

export class HugeClass {
  /**
   * This function is intentionally very long (>200 lines)
   * to trigger complexity violations
   */
  massiveFunction() {
    let result = 0;

    // Block 1: Initialize data
    const data1 = 1;
    const data2 = 2;
    const data3 = 3;
    const data4 = 4;
    const data5 = 5;
    const data6 = 6;
    const data7 = 7;
    const data8 = 8;
    const data9 = 9;
    const data10 = 10;

    // Block 2: Process data
    result += data1;
    result += data2;
    result += data3;
    result += data4;
    result += data5;
    result += data6;
    result += data7;
    result += data8;
    result += data9;
    result += data10;

    // Block 3: More processing
    for (let i = 0; i < 10; i++) {
      result += i;
    }

    // Block 4: Even more processing
    for (let i = 0; i < 10; i++) {
      result += i * 2;
    }

    // Block 5: Complex logic
    if (result > 100) {
      result = result / 2;
    } else {
      result = result * 2;
    }

    // Block 6: More complex logic
    if (result > 50) {
      result = result - 10;
    } else {
      result = result + 10;
    }

    // Block 7: Array operations
    const arr = [1, 2, 3, 4, 5];
    arr.forEach(x => result += x);

    // Block 8: Object operations
    const obj = { a: 1, b: 2, c: 3 };
    result += obj.a + obj.b + obj.c;

    // Block 9: String operations
    const str = "hello world";
    result += str.length;

    // Block 10: Number operations
    result = Math.floor(result);
    result = Math.ceil(result);
    result = Math.round(result);

    // Block 11: More iterations
    for (let i = 0; i < 20; i++) {
      result += i;
      if (i % 2 === 0) {
        result *= 1.1;
      }
    }

    // Block 12: Nested loops
    for (let i = 0; i < 5; i++) {
      for (let j = 0; j < 5; j++) {
        result += i * j;
      }
    }

    // Block 13: Switch statement
    switch (result % 3) {
      case 0:
        result += 10;
        break;
      case 1:
        result += 20;
        break;
      case 2:
        result += 30;
        break;
    }

    // Block 14: Try-catch
    try {
      result = result / 1;
    } catch (e) {
      result = 0;
    }

    // Block 15: More conditionals
    if (result > 1000) {
      result = 1000;
    }
    if (result < 0) {
      result = 0;
    }

    // Block 16: Array transformations
    const mapped = arr.map(x => x * 2);
    result += mapped.reduce((a, b) => a + b, 0);

    // Block 17: Filter operations
    const filtered = arr.filter(x => x > 2);
    result += filtered.length;

    // Block 18: Complex calculations
    result = (result * 1.5) + (result / 2) - (result * 0.3);

    // Block 19: More math
    result = Math.abs(result);
    result = Math.sqrt(result);
    result = Math.pow(result, 2);

    // Block 20: Final operations
    result = Math.floor(result);

    // Block 21: Additional padding to reach 200+ lines
    const temp1 = result + 1;
    const temp2 = result + 2;
    const temp3 = result + 3;
    const temp4 = result + 4;
    const temp5 = result + 5;

    result = temp1 + temp2 + temp3 + temp4 + temp5;

    // Block 22: More unnecessary operations
    for (let k = 0; k < 10; k++) {
      result += k;
    }

    // Block 23: Redundant checks
    if (result !== undefined) {
      result = result || 0;
    }

    // Block 24: More redundant code
    if (result !== null) {
      result = result ?? 0;
    }

    // Block 25: Even more code
    const final1 = result;
    const final2 = result * 2;
    const final3 = result * 3;

    // Block 26: Last block
    return final1 + final2 + final3;
  }

  /**
   * Another huge function
   */
  anotherMassiveFunction() {
    let counter = 0;

    for (let i = 0; i < 100; i++) {
      counter++;
      if (i % 2 === 0) {
        counter += 2;
      }
      if (i % 3 === 0) {
        counter += 3;
      }
      if (i % 5 === 0) {
        counter += 5;
      }
      if (i % 7 === 0) {
        counter += 7;
      }
    }

    for (let j = 0; j < 50; j++) {
      counter--;
      if (j % 2 === 1) {
        counter -= 2;
      }
    }

    for (let k = 0; k < 30; k++) {
      counter *= 1.01;
    }

    for (let m = 0; m < 20; m++) {
      counter /= 1.01;
    }

    // More code to make it long
    const arr1 = Array(50).fill(0);
    arr1.forEach(x => counter++);

    const arr2 = Array(50).fill(1);
    arr2.forEach(x => counter += x);

    return Math.floor(counter);
  }
}

/**
 * Another class with long functions
 */
export class AnotherHugeClass {
  processEverything() {
    let value = 0;

    // Padding to reach line count
    value += 1;
    value += 2;
    value += 3;
    value += 4;
    value += 5;
    value += 6;
    value += 7;
    value += 8;
    value += 9;
    value += 10;
    value += 11;
    value += 12;
    value += 13;
    value += 14;
    value += 15;
    value += 16;
    value += 17;
    value += 18;
    value += 19;
    value += 20;
    value += 21;
    value += 22;
    value += 23;
    value += 24;
    value += 25;
    value += 26;
    value += 27;
    value += 28;
    value += 29;
    value += 30;
    value += 31;
    value += 32;
    value += 33;
    value += 34;
    value += 35;
    value += 36;
    value += 37;
    value += 38;
    value += 39;
    value += 40;
    value += 41;
    value += 42;
    value += 43;
    value += 44;
    value += 45;
    value += 46;
    value += 47;
    value += 48;
    value += 49;
    value += 50;

    return value;
  }
}
