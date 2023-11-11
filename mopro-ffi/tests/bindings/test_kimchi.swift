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

func testMoproKimchi() {
    do {
        // Create a new instance of MoproKimchi
        let moproKimchi = MoproKimchi()

        // Create a proof
        let serializedProof = try moproKimchi.createProof()
        print("Proof created successfully. Size: \(serializedProof.count)")

        // Verify the proof
        let isProofValid = try moproKimchi.verifyProof(proof: serializedProof)
        print("Proof verification result: \(isProofValid)")

    } catch let error as MoproError {
        print("MoproError: \(error)")
    } catch {
        print("Unexpected error: \(error)")
    }
}

// Execute the test
testKimchiBench()

// Execute the test
testMoproKimchi()
