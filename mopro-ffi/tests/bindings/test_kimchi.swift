import mopro
import Foundation

func testKimchiBench() {
    do {
        // Call kimchi_bench function
        try kimchiBench()
        print("kimchiBeench executed successfully")
    } catch let error as MoproError {
        print("MoproError: \(error)")
    } catch {
        print("Unexpected error: \(error)")
    }
}

// Execute the test
testKimchiBench()