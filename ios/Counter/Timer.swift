//
//  Timer.swift
//  Counter
//
//  Created by Justin  on 6/9/24.
//

import SwiftUI

struct Timer: View {
    @State var rust: ViewModel;
    
    public init() {
        self.rust = ViewModel()
    }
    
    var body: some View {
        VStack {
            Text("\(self.rust.timer.elapsedSecs)")
                .font(.largeTitle)
                .frame(width: 50, height: 50)
            HStack {
                if (self.rust.timer.active) {
                    Button(action: {
                        self.rust.dispatch(event: .timerPause)
                    }) {
                        Text("Stop")
                    }
                } else {
                    Button(action: {
                        self.rust.dispatch(event: .timerStart)
                    }) {
                        Text("Start")
                    }
                }
                Button(action: {
                    self.rust.dispatch(event: .timerReset)
                }) {
                    Text("Reset")
                }
            }
        }
    }
}

#Preview {
    Timer()
}
