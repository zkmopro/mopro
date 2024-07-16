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
    
    func testCircomProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveCircom"].tap()
        var predicate = NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")
        var elementQuery = app.staticTexts.containing(predicate)
        XCTAssert(elementQuery.count == 1)
        app.buttons["verifyCircom"].tap()
        predicate = NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")
        elementQuery = app.staticTexts.containing(predicate)
        XCTAssert(elementQuery.count == 1)
    }
    
    func testHalo2ProveVerify() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launch()
        
        app.buttons["proveHalo2"].tap()
        var predicate = NSPredicate(format: "label CONTAINS[c] %@", "1️⃣")
        var elementQuery = app.staticTexts.containing(predicate)
        XCTAssert(elementQuery.count == 1)
        app.buttons["verifyHalo2"].tap()
        predicate = NSPredicate(format: "label CONTAINS[c] %@", "2️⃣")
        elementQuery = app.staticTexts.containing(predicate)
        XCTAssert(elementQuery.count == 1)
    }
}
