import { BadRepository } from './bad.repository'; // ❌ VIOLATION: forbidden import
import { CircularA } from './circular-a'; // Part of circular dependency

/**
 * Bad Controller - combines multiple violations:
 * 1. Forbidden import (controller -> repository)
 * 2. Extremely long function
 * 3. Part of circular dependency chain
 */
export class BadController {
  private repo: BadRepository; // ❌ Should not exist
  private circularA: CircularA;

  constructor() {
    this.repo = new BadRepository();
    this.circularA = new CircularA();
  }

  /**
   * This is a HUGE function (>200 lines)
   * It violates complexity rules
   */
  massiveControllerFunction() {
    let result = 0;

    // Access repository directly (violation)
    const data = this.repo.findAll();

    // Extremely long and unnecessary code below
    for (let i = 0; i < 100; i++) {
      result += i;
      if (i % 2 === 0) {
        result *= 1.1;
      }
      if (i % 3 === 0) {
        result += 10;
      }
      if (i % 5 === 0) {
        result -= 5;
      }
      if (i % 7 === 0) {
        result *= 0.9;
      }
    }

    const temp1 = result + 1;
    const temp2 = result + 2;
    const temp3 = result + 3;
    const temp4 = result + 4;
    const temp5 = result + 5;
    const temp6 = result + 6;
    const temp7 = result + 7;
    const temp8 = result + 8;
    const temp9 = result + 9;
    const temp10 = result + 10;

    result = temp1 + temp2 + temp3 + temp4 + temp5;
    result += temp6 + temp7 + temp8 + temp9 + temp10;

    for (let j = 0; j < 50; j++) {
      result += j * 2;
      if (j % 2 === 1) {
        result -= j;
      }
    }

    const arr = Array(30).fill(0);
    arr.forEach((_, idx) => {
      result += idx;
    });

    for (let k = 0; k < 20; k++) {
      for (let m = 0; m < 5; m++) {
        result += k * m;
      }
    }

    if (result > 1000) {
      result = 1000;
    } else if (result > 500) {
      result = 500;
    } else if (result > 100) {
      result = 100;
    } else {
      result = 0;
    }

    switch (result % 10) {
      case 0: result += 100; break;
      case 1: result += 200; break;
      case 2: result += 300; break;
      case 3: result += 400; break;
      case 4: result += 500; break;
      case 5: result += 600; break;
      case 6: result += 700; break;
      case 7: result += 800; break;
      case 8: result += 900; break;
      case 9: result += 1000; break;
    }

    const obj1 = { a: 1, b: 2 };
    const obj2 = { c: 3, d: 4 };
    const obj3 = { e: 5, f: 6 };

    result += obj1.a + obj1.b;
    result += obj2.c + obj2.d;
    result += obj3.e + obj3.f;

    try {
      result = result / 1;
    } catch (e) {
      result = 0;
    }

    for (let n = 0; n < 15; n++) {
      result += Math.random() * 10;
    }

    result = Math.floor(result);
    result = Math.ceil(result);
    result = Math.round(result);
    result = Math.abs(result);

    const finalArr = data.map(x => x.id);
    result += finalArr.length;

    return result;
  }

  anotherBadMethod() {
    // More direct repository access
    return this.repo.findById(1);
  }
}
