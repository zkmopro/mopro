//
//  ExampleAppTests.swift
//  ExampleAppTests
//
//  Created by User Name on 3/6/24.
//

import XCTest

@testable import ExampleApp

final class ExampleAppTests: XCTestCase {

  override func setUpWithError() throws {
    // Put setup code here. This method is called before the invocation of each test method in the class.
  }

  override func tearDownWithError() throws {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
  }

  func testCircomMultiplier() throws {
    do {
      var inputs = [String: [String]]()
      let a = 3
      let b = 5
      let c = a * b
      inputs["a"] = [String(a)]
      inputs["b"] = [String(b)]
      let outputs: [String] = [String(c), String(a)]
      let expectedOutput: [UInt8] = serializeOutputs(outputs)

      // Generate Proof
      guard
        let generateProofResult = try generateProof2(circuitInputs: inputs) as GenerateProofResult?
      else { print("error") }
      XCTAssertFalse(generateProofResult.proof.isEmpty, "Proof should not be empty")
      XCTAssertEqual(
        Data(expectedOutput), generateProofResult.inputs,
        "Circuit outputs mismatch the expected outputs")

      guard
        let isValid = try verifyProof2(
          proof: generateProofResult.proof, publicInput: generateProofResult.inputs) as Bool?
      else { print("error") }
      XCTAssertTrue(isValid, "Proof verification should succeed")
    } catch let error as MoproError {
      print("MoproError: \(error)")
    } catch {
      print("Unexpected error: \(error)")
    }
  }
  func testPerformanceExample() throws {
    // This is an example of a performance test case.
    self.measure {
      // Put the code you want to measure the time of here.
    }
  }

}
