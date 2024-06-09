//
//  CounterApp.swift
//  Counter
//
//  Created by Justin  on 6/4/24.
//

import SwiftUI

@main
struct CounterApp: App {
    @State var rust: ViewModel;
    
    public init() {
        self.rust = ViewModel()
    }

    var body: some Scene {

        WindowGroup {
            HStack {
                Button(action: {
                    self.rust.dispatch(event: .setRoute(route: Route.counter))
                }) {
                    Text("Counter")
                }
                Button(action: {
                    self.rust.dispatch(event: .setRoute(route: Route.timer))
                }) {
                    Text("Timer")
                }
            }
            Text(String(describing: self.rust.router.route))

            switch rust.router.route {
            case .counter:
                Counter(rust: self.rust)
            case .timer:
                Timer(rust: self.rust)
            }
        }
    }
}

