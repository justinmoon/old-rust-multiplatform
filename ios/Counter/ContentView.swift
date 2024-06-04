//
//  ContentView.swift
//  Counter
//
//  Created by Justin  on 6/4/24.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text(sayHi())
                .padding()
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
