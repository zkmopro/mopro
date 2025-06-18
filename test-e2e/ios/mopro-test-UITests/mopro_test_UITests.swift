//
//  mopro_test_UITests.swift
//  mopro-test-UITests
//
//  Created by Chance on 6/30/24.
//

import XCTest

final class mopro_test_UITests: XCTestCase {
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
        
        // In UI tests it is usually best to stop immediately when a failure occurs.
        continueAfterFailure = false
        
        // In UI tests it’s important to set the initial state - such as interface orientation - required for your tests before they run. The setUp method is a good place to do this.
    }
    
    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }
    
    func testRapidsnarkProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveRapidsnark"].tap()
        let proveText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")).firstMatch
        XCTAssertTrue(proveText.waitForExistence(timeout: 5), "The time of proof generation is over 5 secs")
        
        app.buttons["verifyRapidsnark"].tap()
        let verifyText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")).firstMatch
        XCTAssertTrue(verifyText.waitForExistence(timeout: 5), "The time of proof verification is over 5 secs")
    }
    
    func testCircomProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveCircom"].tap()
        let proveText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")).firstMatch
        XCTAssertTrue(proveText.waitForExistence(timeout: 5), "The time of proof generation is over 5 secs")
        
        app.buttons["verifyCircom"].tap()
        let verifyText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")).firstMatch
        XCTAssertTrue(verifyText.waitForExistence(timeout: 5), "The time of proof verification is over 5 secs")
    }
    
    func testHalo2ProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveHalo2"].tap()
        let proveText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")).firstMatch
        XCTAssertTrue(proveText.waitForExistence(timeout: 5), "The time of proof generation is over 5 secs")
        
        app.buttons["verifyHalo2"].tap()
        let verifyText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")).firstMatch
        XCTAssertTrue(verifyText.waitForExistence(timeout: 5), "The time of proof verification is over 5 secs")
    }

    func testNoirProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveNoir"].tap()
        let proveText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")).firstMatch
        XCTAssertTrue(proveText.waitForExistence(timeout: 5), "The time of proof generation is over 5 secs")
        
        app.buttons["verifyNoir"].tap()
        let verifyText = app.staticTexts.containing(NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")).firstMatch
        XCTAssertTrue(verifyText.waitForExistence(timeout: 5), "The time of proof verification is over 5 secs")
    }
}
