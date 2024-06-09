import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var rust: FfiApp;
    var count: Int32;
    var timer: TimerState;
    
    public init() {
        let rust = FfiApp()
        let state = rust.getState()

        self.count = state.count
        self.timer = state.timer
        self.rust = rust

        self.rust.listenForUpdates(updater: self)

    }
    
    func update(update: Update) {
        switch update {
        case .countChanged(count: let count):
            self.count = count
        case .timer(state: let timer):
            self.timer = timer
        }
    }
    
    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }
}
