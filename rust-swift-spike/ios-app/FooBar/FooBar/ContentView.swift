//
//  ContentView.swift
//  FooBar
//
//  Created by User Name on 9/1/23.
//

import SwiftUI

struct ContentView: View {
    let greeting: String = hello()
    let sum: UInt32 = add(a: 5, b: 3)

    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
            Text(greeting)
            Text("5 + 3 = \(sum)")
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
